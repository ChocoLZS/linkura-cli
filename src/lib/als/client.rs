use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Sender},
};
use std::thread::{self, JoinHandle, sleep};
use std::time::Duration;

use super::packet;

const RECEIVE_DATA_INTERVAL: Duration = Duration::from_millis(10);
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const RECONNECT_DELAY: Duration = Duration::from_secs(5);
const KEEP_ALIVE_INTERVAL: Duration = Duration::from_millis(1350);

#[derive(Debug, PartialEq)]
enum ClientState {
    Auth,
    Join,
    ReceivingData,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub receive_interval: Duration,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay: Duration,
    pub data_directory: String,
    ///
    /// mrs keepalive request 包默认10秒一次
    ///
    /// 考虑到程序启动延迟以及网络波动，设置为20秒
    ///
    pub timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            receive_interval: RECEIVE_DATA_INTERVAL,
            max_reconnect_attempts: MAX_RECONNECT_ATTEMPTS,
            reconnect_delay: RECONNECT_DELAY,
            data_directory: "data".to_string(),
            timeout: Duration::from_secs(20),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub room_id: String,
    pub token: String,
}

#[derive(Debug)]
struct ClientRuntimeState {
    pub receive_buffer: Vec<u8>, // 接收缓冲区，用于处理TCP分片
    pub saved_buffer: Vec<u8>,   // 保存原始数据
    pub state: ClientState,
    pub last_received: std::time::Instant,
    pub data_prefix: String,
    pub data_index: u64,
}

impl ClientRuntimeState {
    fn new() -> Self {
        Self {
            receive_buffer: Vec::new(),
            saved_buffer: Vec::new(),
            state: ClientState::Auth,
            last_received: std::time::Instant::now(),
            data_prefix: format!(
                "data_{}_",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ),
            data_index: 0,
        }
    }
    fn reset(&mut self) {
        self.receive_buffer.clear();
        self.saved_buffer.clear();
        self.state = ClientState::Auth;
        self.last_received = std::time::Instant::now();
        self.data_index = 0;
        self.data_prefix = format!(
            "data_{}_",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    fn increase_data_index(&mut self) {
        self.data_index += 1;
    }
}

pub struct Client {
    stream: Option<TcpStream>,
    running_signal: Arc<AtomicBool>,
    connection_info: ConnectionInfo,
    config: ClientConfig,
    runtime_state: ClientRuntimeState,
    // Keepalive 线程管理
    keepalive_enabled: Arc<AtomicBool>,
    keepalive_handle: Option<JoinHandle<()>>,
    keepalive_tx: Option<Sender<TcpStream>>,
}

impl Client {
    pub fn new(
        running_signal: Arc<AtomicBool>,
        connection_info: ConnectionInfo,
        config: Option<ClientConfig>,
    ) -> Result<Self> {
        Self::with_config(running_signal, connection_info, config.unwrap_or_default())
    }

    pub fn with_config(
        running_signal: Arc<AtomicBool>,
        connection_info: ConnectionInfo,
        config: ClientConfig,
    ) -> Result<Self> {
        Ok(Self {
            stream: None,
            running_signal,
            connection_info,
            config,
            runtime_state: ClientRuntimeState::new(),
            keepalive_enabled: Arc::new(AtomicBool::new(false)),
            keepalive_handle: None,
            keepalive_tx: None,
        })
    }

    fn connect(&mut self) -> Result<()> {
        tracing::info!(
            "Connecting to server at {}:{}...",
            self.connection_info.host,
            self.connection_info.port
        );
        let stream =
            TcpStream::connect((self.connection_info.host.clone(), self.connection_info.port))
                .context("Failed to connect to server")?;

        // 设置非阻塞模式以便能够响应中断信号
        stream
            .set_nonblocking(true)
            .context("Failed to set non-blocking mode")?;

        self.stream = Some(stream);
        tracing::info!("Connected to server successfully");
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.stop_keepalive_thread();
        if !self.runtime_state.receive_buffer.is_empty()
            || !self.runtime_state.saved_buffer.is_empty()
        {
            tracing::warn!(
                "Client disconnected with remaining data in receive && saved buffer, saving to file"
            );
            let data_to_save = std::mem::take(&mut self.runtime_state.receive_buffer);
            if let Err(e) = self.save_raw_data(&data_to_save, data_to_save.len(), true) {
                tracing::error!("Failed to save remaining data: {}", e);
            }
        }
        if let Some(stream) = self.stream.take() {
            let _ = stream.shutdown(std::net::Shutdown::Both);
            tracing::info!("Disconnected from server");
        }
    }

    fn state_handler(&mut self) -> Result<bool> {
        let stream = self.stream.as_mut().context("No active connection")?;
        // 客户端主动发起连接
        if self.runtime_state.state == ClientState::Auth {
            packet::AlsPacket::AuthenticateRequest {
                token: self.connection_info.token.clone(),
            }
            .send(stream)?;
            self.runtime_state.state = ClientState::Join;
            return Ok(true);
        }

        let mut buffer = [0u8; 4096];
        match stream.read(&mut buffer) {
            Ok(0) => {
                tracing::info!("Connection closed by server");
                return Ok(false);
            }
            Ok(bytes_read) => {
                let new_data = &buffer[0..bytes_read];
                tracing::trace!(
                    "Received {} bytes from server: {}",
                    bytes_read,
                    hex::encode(new_data)
                );

                self.runtime_state
                    .receive_buffer
                    .extend_from_slice(new_data);

                self.process_receive_buffer()?;
                self.runtime_state.last_received = std::time::Instant::now();
                Ok(true)
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 非阻塞模式下没有数据可读，正常情况
                if self.runtime_state.last_received.elapsed() > self.config.timeout {
                    tracing::warn!(
                        "No data received after {:?}, disconnecting",
                        self.config.timeout
                    );
                    return Ok(false);
                }
                Ok(true)
            }
            Err(e) => Err(anyhow::anyhow!("Error reading from socket: {}", e)),
        }
    }

    fn process_receive_buffer(&mut self) -> Result<()> {
        // 目前保证收到的包的结构都是完整的
        while self.runtime_state.receive_buffer.len() >= 2 {
            let packet_length = u16::from_be_bytes([
                self.runtime_state.receive_buffer[0],
                self.runtime_state.receive_buffer[1],
            ]) as usize
                + 2;

            if self.runtime_state.receive_buffer.len() >= packet_length {
                let packet_data = self.runtime_state.receive_buffer[0..packet_length].to_vec();

                self.runtime_state.receive_buffer.drain(0..packet_length);
                if self.runtime_state.state == ClientState::Join {
                    packet::AlsPacket::JoinRequest {
                        room_id: self.connection_info.room_id.clone(),
                    }
                    .send(self.stream.as_mut().context("No active connection")?)?;
                    self.runtime_state.state = ClientState::ReceivingData;

                    if let Err(e) = self.start_keepalive_thread() {
                        tracing::error!("Failed to start keepalive thread: {}", e);
                    }
                    if let Err(e) = self.send_stream_to_keepalive() {
                        tracing::error!("Failed to send stream to keepalive thread: {}", e);
                    }
                }

                self.save_raw_data(&packet_data, packet_data.len(), false)?;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// 保存原始数据
    fn save_raw_data(&mut self, data: &[u8], size: usize, force: bool) -> Result<()> {
        self.runtime_state.saved_buffer.extend_from_slice(data);
        if self.runtime_state.saved_buffer.len() >= 1024 * 1024 || force {
            tracing::info!("Save buffer exceeded 1MB, saving data to file");
            let file_path = format!(
                "{}/{}{}.bin",
                self.config.data_directory,
                self.runtime_state.data_prefix,
                self.runtime_state.data_index
            );
            let mut file = File::create(&file_path)
                .with_context(|| format!("Failed to create file {}", file_path))?;
            file.write_all(&self.runtime_state.saved_buffer)
                .with_context(|| format!("Failed to write data to {}", file_path))?;
            tracing::debug!("Raw data saved to {} ({} bytes)", file_path, size);
            self.runtime_state.increase_data_index();
            self.runtime_state.saved_buffer.clear();
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        std::fs::create_dir_all(&self.config.data_directory).with_context(|| {
            format!(
                "Failed to create data directory: {}",
                self.config.data_directory
            )
        })?;
        let mut reconnect_attempts = 0u32;

        self.runtime_state.reset();
        loop {
            if !self.running_signal.load(Ordering::Relaxed) {
                tracing::info!("Received shutdown signal, exiting...");
                break;
            }

            // 如果没有连接，尝试重连
            if self.stream.is_none() {
                if reconnect_attempts >= self.config.max_reconnect_attempts {
                    tracing::error!(
                        "Max connection attempts reached: something went wrong, maybe token is expired"
                    );
                    return Err(anyhow::anyhow!("Max connection attempts reached"));
                }
                tracing::info!(
                    "Connection attempt {} of {}",
                    reconnect_attempts + 1,
                    self.config.max_reconnect_attempts
                );
                reconnect_attempts += 1;
                // sleep(self.config.reconnect_delay);

                match self.connect() {
                    Ok(_) => {
                        // reconnect_attempts = 0;
                        self.runtime_state.reset();
                    }
                    Err(e) => {
                        tracing::error!("Failed to connect: {}", e);
                        continue;
                    }
                }
            }

            let result = match self.state_handler() {
                Ok(true) => Ok(()),
                Ok(false) => {
                    self.disconnect();
                    Ok(())
                }
                Err(e) => Err(e),
            };

            if let Err(e) = result {
                tracing::error!("Operation failed: {}", e);
                self.disconnect();
                reconnect_attempts += 1;
                continue;
            }

            sleep(self.config.receive_interval);
        }

        Ok(())
    }

    fn start_keepalive_thread(&mut self) -> Result<()> {
        if self.keepalive_handle.is_some() {
            self.stop_keepalive_thread();
        }

        let (tx, rx) = mpsc::channel::<TcpStream>();
        let running_signal = self.running_signal.clone();
        let keepalive_enabled = self.keepalive_enabled.clone();

        keepalive_enabled.store(true, Ordering::Relaxed);

        let handle = thread::spawn(move || {
            let mut last_keepalive = std::time::Instant::now();

            tracing::info!("Keepalive thread started");
            // loop获取tcp流
            loop {
                if !running_signal.load(Ordering::Relaxed)
                    || !keepalive_enabled.load(Ordering::Relaxed)
                {
                    tracing::info!("Keepalive thread received shutdown signal, exiting...");
                    break;
                }

                if let Ok(mut stream) = rx.try_recv() {
                    tracing::debug!("Keepalive thread received new stream");
                    // keepalive loop
                    loop {
                        if !running_signal.load(Ordering::Relaxed)
                            || !keepalive_enabled.load(Ordering::Relaxed)
                        {
                            tracing::info!(
                                "Keepalive thread received shutdown signal during keepalive loop"
                            );
                            break;
                        }

                        if last_keepalive.elapsed() >= KEEP_ALIVE_INTERVAL {
                            let keepalive_packet = packet::AlsPacket::KeepAliveRequest {};
                            match keepalive_packet.send(&mut stream) {
                                Ok(_) => {
                                    tracing::trace!("Keepalive packet sent successfully");
                                    last_keepalive = std::time::Instant::now();
                                }
                                Err(e) => {
                                    tracing::error!("Failed to send keepalive packet: {}", e);
                                    break;
                                }
                            }
                        }

                        sleep(Duration::from_millis(100));
                    }
                }

                sleep(Duration::from_millis(100));
            }

            tracing::info!("Keepalive thread exited");
        });

        self.keepalive_handle = Some(handle);
        self.keepalive_tx = Some(tx);

        tracing::info!("Keepalive thread started successfully");
        Ok(())
    }

    fn stop_keepalive_thread(&mut self) {
        self.keepalive_enabled.store(false, Ordering::Relaxed);

        if let Some(handle) = self.keepalive_handle.take() {
            if let Err(e) = handle.join() {
                tracing::error!("Error joining keepalive thread: {:?}", e);
            }
        }

        self.keepalive_tx = None;

        tracing::debug!("Keepalive thread stopped");
    }

    fn send_stream_to_keepalive(&mut self) -> Result<()> {
        if let (Some(tx), Some(stream)) = (&self.keepalive_tx, &self.stream) {
            match stream.try_clone() {
                Ok(cloned_stream) => {
                    if let Err(e) = tx.send(cloned_stream) {
                        tracing::error!("Failed to send stream to keepalive thread: {}", e);
                        return Err(anyhow::anyhow!(
                            "Failed to send stream to keepalive thread: {}",
                            e
                        ));
                    } else {
                        tracing::debug!("Stream sent to keepalive thread");
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to clone stream for keepalive thread: {}", e);
                    return Err(anyhow::anyhow!(
                        "Failed to clone stream for keepalive thread: {}",
                        e
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIVE_ID: &str = "test_room";
    const TOKEN: &str = "1145141919810";

    #[test]
    fn test_client_config() {
        let config = ClientConfig::default();
        assert_eq!(config.receive_interval, Duration::from_millis(10));
        assert_eq!(config.max_reconnect_attempts, 5);
        assert_eq!(config.reconnect_delay, Duration::from_secs(5));
        assert_eq!(config.data_directory, "data");

        let custom_config = ClientConfig {
            receive_interval: Duration::from_millis(20),
            max_reconnect_attempts: 3,
            reconnect_delay: Duration::from_secs(10),
            data_directory: "custom_data".to_string(),
            timeout: Duration::from_secs(10),
        };

        let running_signal = Arc::new(AtomicBool::new(true));
        let connection_info = ConnectionInfo {
            host: "localhost".to_string(),
            port: 8080,
            room_id: LIVE_ID.to_string(),
            token: TOKEN.to_string(),
        };

        let _client = Client::with_config(running_signal, connection_info, custom_config).unwrap();
    }
}
