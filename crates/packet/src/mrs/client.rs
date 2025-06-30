use anyhow::{Context, Result};
use p256::ecdh::EphemeralSecret;
use rand_core::OsRng;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::sleep;
use std::time::Duration;

use super::packet;

const RECEIVE_DATA_INTERVAL: Duration = Duration::from_millis(10);
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct MrsClientConfig {
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

impl Default for MrsClientConfig {
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
pub struct MrsConnectionInfo {
    pub host: String,
    pub port: u16,
    pub room_id: u32,
    pub player_id: u16,
}

#[derive(Debug)]
struct MrsClientRuntimeState {
    pub receive_buffer: Vec<u8>, // 接收缓冲区，用于处理TCP分片
    pub saved_buffer: Vec<u8>,   // 保存原始数据
    pub state: packet::MrsPacketType,
    pub last_received: std::time::Instant,
    pub data_prefix: String,
    pub data_index: u64,
}

impl MrsClientRuntimeState {
    fn new() -> Self {
        Self {
            receive_buffer: Vec::new(),
            saved_buffer: Vec::new(),
            state: packet::MrsPacketType::VersionCheck,
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
        self.state = packet::MrsPacketType::VersionCheck;
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

pub struct MrsClient {
    stream: Option<TcpStream>,
    running_signal: Arc<AtomicBool>,
    packet_generator: packet::PacketGenerator,
    connection_info: MrsConnectionInfo,
    config: MrsClientConfig,
    runtime_state: MrsClientRuntimeState,
}

impl MrsClient {
    pub fn new(
        running_signal: Arc<AtomicBool>,
        connection_info: MrsConnectionInfo,
        config: Option<MrsClientConfig>,
    ) -> Result<Self> {
        Self::with_config(running_signal, connection_info, config.unwrap_or_default())
    }

    pub fn with_config(
        running_signal: Arc<AtomicBool>,
        connection_info: MrsConnectionInfo,
        config: MrsClientConfig,
    ) -> Result<Self> {
        Ok(Self {
            stream: None,
            running_signal,
            packet_generator: packet::PacketGenerator::new(),
            connection_info,
            config,
            runtime_state: MrsClientRuntimeState::new(),
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
        if !self.runtime_state.receive_buffer.is_empty()
            || !self.runtime_state.saved_buffer.is_empty()
        {
            tracing::warn!(
                "MrsClient disconnected with remaining data in receive && saved buffer, saving to file"
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

    fn send_handshake(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().context("No active connection")?;
        let version_check_packet = self.packet_generator.generate::<packet::VersionCheck>(())?;
        let version_check_bytes = version_check_packet.to_bytes();
        version_check_packet.log_send();
        stream
            .write_all(&version_check_bytes)
            .context("Failed to send version check packet")?;
        version_check_packet.log_sent();
        // 发送客户端公钥
        let p_k = EphemeralSecret::random(&mut OsRng);
        let pub_key = p_k.public_key();
        let pub_key_string = hex::encode(pub_key.to_sec1_bytes());
        let client_key_packet = self
            .packet_generator
            .generate::<packet::KeyExchangeRequest>(pub_key_string)?;
        let client_key_bytes = client_key_packet.to_bytes();
        client_key_packet.log_send();
        stream
            .write_all(&client_key_bytes)
            .context("Failed to send client key packet")?;
        client_key_packet.log_sent();
        Ok(())
    }

    fn send_join_room(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().context("No active connection")?;
        let join_room_packet = self.packet_generator.generate::<packet::JoinRoom>((
            self.connection_info.room_id,
            self.connection_info.player_id,
        ))?;
        let join_room_bytes = join_room_packet.to_bytes();
        join_room_packet.log_send();
        stream
            .write_all(&join_room_bytes)
            .context("Failed to send join room packet")?;
        join_room_packet.log_sent();
        Ok(())
    }

    ///
    /// 与服务端流程建立：
    /// 1. tcp建立连接
    /// 2. 服务端主动push version check包
    /// 3. 客户端发送握手包（VersionCheck）和密钥交换包（KeyExchangeRequest）
    /// 4. 服务端发送密钥交换响应包（KeyExchangeResponse）
    /// 5. 客户端发送加入房间包（JoinRoom）
    /// 6. 服务端发送数据和心跳包（KeepAliveRequest）
    /// 7. 客户端发送心跳包（KeepAliveResponse）
    ///
    fn handle_packet(&mut self, packet: packet::MrsPacket) -> Result<bool> {
        tracing::debug!("Handling packet: {}", packet.packet_type.as_str());

        match packet.packet_type {
            packet::MrsPacketType::VersionCheck => {
                if self.runtime_state.state != packet::MrsPacketType::VersionCheck {
                    tracing::warn!(
                        "Received unexpected VersionCheck packet in state: {}",
                        self.runtime_state.state.as_str()
                    );
                    return Ok(false);
                }
                self.send_handshake()
                    .context("Failed to send handshake response")?;
                self.runtime_state.state = packet::MrsPacketType::KeyExchangeRequest;
            }
            packet::MrsPacketType::KeyExchangeResponse => {
                if self.runtime_state.state != packet::MrsPacketType::KeyExchangeRequest {
                    tracing::warn!(
                        "Received unexpected KeyExchangeResponse packet in state: {}",
                        self.runtime_state.state.as_str()
                    );
                    return Ok(false);
                }
                self.send_join_room()
                    .context("Failed to send join room request")?;
                self.runtime_state.state = packet::MrsPacketType::JoinRoom;
            }
            packet::MrsPacketType::KeepAliveRequest => {
                self.send_keepalive()
                    .context("Failed to send keepalive response")?;
            }
            packet::MrsPacketType::End | packet::MrsPacketType::ConnectionClose => {
                tracing::info!("Received connection close signal from server");
                return Ok(false);
            }
            _ => {
                tracing::warn!(
                    "Received unknown packet type: {:x}",
                    packet.packet_type as u16
                );
            }
        }
        Ok(true)
    }

    fn receive_data(&mut self) -> Result<bool> {
        let stream = self.stream.as_mut().context("No active connection")?;

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
        while self.runtime_state.receive_buffer.len() >= 4 {
            let packet_length = u32::from_le_bytes([
                self.runtime_state.receive_buffer[0],
                self.runtime_state.receive_buffer[1],
                self.runtime_state.receive_buffer[2],
                self.runtime_state.receive_buffer[3],
            ]) as usize
                + 4;

            if self.runtime_state.receive_buffer.len() >= packet_length {
                let packet_data = self.runtime_state.receive_buffer[0..packet_length].to_vec();

                self.runtime_state.receive_buffer.drain(0..packet_length);

                match packet::MrsPacket::from_bytes(&packet_data) {
                    Ok(packet) => {
                        tracing::debug!(
                            "Received packet: type={}, seq={}, payload_len={}",
                            packet.packet_type.as_str(),
                            packet.sequence_number,
                            packet.payload.len()
                        );

                        if !self.handle_packet(packet)? {
                            return Err(anyhow::anyhow!(
                                "Packet handling requested connection close"
                            ));
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse packet: {}", e);
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

    fn send_keepalive(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().context("No active connection")?;

        // 使用新的packet系统发送keepalive消息
        let keepalive_packet = self
            .packet_generator
            .generate::<packet::KeepAliveResponse>(())?;
        let keepalive_bytes = keepalive_packet.to_bytes();
        keepalive_packet.log_send();
        stream
            .write_all(&keepalive_bytes)
            .context("Failed to send keepalive packet")?;
        keepalive_packet.log_sent();
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

            let result = match self.receive_data() {
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
}

impl Drop for MrsClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ROOM_ID: u32 = 1919810;
    const PLAYER_ID: u16 = 11451;

    #[test]
    fn test_client_config() {
        let config = MrsClientConfig::default();
        assert_eq!(config.receive_interval, Duration::from_millis(10));
        assert_eq!(config.max_reconnect_attempts, 5);
        assert_eq!(config.reconnect_delay, Duration::from_secs(5));
        assert_eq!(config.data_directory, "data");

        let custom_config = MrsClientConfig {
            receive_interval: Duration::from_millis(20),
            max_reconnect_attempts: 3,
            reconnect_delay: Duration::from_secs(10),
            data_directory: "custom_data".to_string(),
            timeout: Duration::from_secs(10),
        };

        let running_signal = Arc::new(AtomicBool::new(true));
        let connection_info = MrsConnectionInfo {
            host: "localhost".to_string(),
            port: 8080,
            room_id: ROOM_ID,
            player_id: PLAYER_ID,
        };

        let _client =
            MrsClient::with_config(running_signal, connection_info, custom_config).unwrap();
    }
}
