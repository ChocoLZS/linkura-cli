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
use std::time::Duration;
use tokio::time::sleep;

mod packet {
    use anyhow::{Context, Result};
    /// MRS协议的RPC包类型常量
    const MRS_RPC_ID_BEGIN: u16 = 0xff00;
    #[allow(unused)]
    const MRS_PROTOCOL_VERSION: u16 = 0x0100;
    /// MRS协议的包类型枚举（小端序）
    ///
    /// reference: mrs.js
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(u16)]
    pub enum MrsPacketType {
        /// 握手包 - 建立连接时使用
        VersionCheck = MRS_RPC_ID_BEGIN | 0x00,
        /// 客户端密钥包 - 发送公钥进行身份验证
        KeyExchangeRequest = MRS_RPC_ID_BEGIN | 0x01,
        /// 加入房间包 - 请求加入指定房间
        JoinRoom = 0x05,
        /// 心跳保活包 - 维持连接活跃状态
        KeepAliveResponse = MRS_RPC_ID_BEGIN | 0xa2,

        #[allow(unused)]
        KeyExchangeResponse = MRS_RPC_ID_BEGIN | 0x02,
        #[allow(unused)]
        KeepAliveRequest = MRS_RPC_ID_BEGIN | 0xa1,
        #[allow(unused)]
        ConnectionClose = MRS_RPC_ID_BEGIN | 0xc0,
        #[allow(unused)]
        ConnectionCloseHardLimitOver = MRS_RPC_ID_BEGIN | 0xc1,
        #[allow(unused)]
        End = MRS_RPC_ID_BEGIN | 0xff,
    }
    impl MrsPacketType {
        pub fn from_u16(value: u16) -> Option<Self> {
            match value {
                0xff00 => Some(MrsPacketType::VersionCheck),
                0xff01 => Some(MrsPacketType::KeyExchangeRequest),
                0x0005 => Some(MrsPacketType::JoinRoom),
                0xffa2 => Some(MrsPacketType::KeepAliveResponse),
                0xff02 => Some(MrsPacketType::KeyExchangeResponse),
                0xffa1 => Some(MrsPacketType::KeepAliveRequest),
                0xffc0 => Some(MrsPacketType::ConnectionClose),
                0xffc1 => Some(MrsPacketType::ConnectionCloseHardLimitOver),
                0xffff => Some(MrsPacketType::End),
                _ => None,
            }
        }

        /// 获取包类型的字符串描述
        pub fn as_str(&self) -> &'static str {
            match self {
                MrsPacketType::VersionCheck => "VersionCheck",
                MrsPacketType::KeyExchangeRequest => "KeyExchangeRequest",
                MrsPacketType::JoinRoom => "JoinRoom",
                MrsPacketType::KeepAliveResponse => "KeepAliveResponse",
                _ => "Unknown Packet Type",
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct MrsPacket {
        pub sequence_number: u32,
        pub packet_type: MrsPacketType,
        pub payload: Vec<u8>,
    }

    impl MrsPacket {
        pub fn new(sequence_number: u32, packet_type: MrsPacketType, payload: Vec<u8>) -> Self {
            Self {
                sequence_number,
                packet_type,
                payload,
            }
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            let total_length = 12 + self.payload.len(); // 4字节长度 + 4字节序号 + 2字节option + 2字节类型 + payload
            let mut buffer = Vec::with_capacity(total_length);

            // 1-4字节: 包总长 (小端，减去前4字节)
            buffer.extend_from_slice(&((total_length - 4) as u32).to_le_bytes());

            // 5-8字节: sequence number (小端)
            buffer.extend_from_slice(&self.sequence_number.to_le_bytes());

            // 9-10字节: option (默认为0)
            buffer.extend_from_slice(&0u16.to_le_bytes());

            // 11-12字节: 包类型 (小端)
            buffer.extend_from_slice(&(self.packet_type as u16).to_le_bytes());

            // payload
            buffer.extend_from_slice(&self.payload);

            buffer
        }

        pub fn from_bytes(data: &[u8]) -> Result<Self> {
            if data.len() < 12 {
                return Err(anyhow::anyhow!("Packet too short"));
            }

            let total_length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
            if data.len() != total_length + 4 {
                return Err(anyhow::anyhow!("Packet length mismatch"));
            }

            let sequence_number = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
            let _option = u16::from_le_bytes([data[8], data[9]]);
            let packet_type_value = u16::from_le_bytes([data[10], data[11]]);

            let packet_type = MrsPacketType::from_u16(packet_type_value).ok_or_else(|| {
                anyhow::anyhow!("Unknown packet type: 0x{:04x}", packet_type_value)
            })?;

            let payload = data[12..].to_vec();

            Ok(Self {
                sequence_number,
                packet_type,
                payload,
            })
        }

        pub fn len(&self) -> usize {
            12 + self.payload.len()
        }

        /// 获取包的十六进制字符串表示（用于调试）
        pub fn to_hex_string(&self) -> String {
            hex::encode(self.to_bytes())
        }


        pub fn log_send(&self) {
            tracing::info!(
                "Sending packet {}",
                self.packet_type.as_str()
            );
            tracing::debug!(
                "Sending packet: type={}, seq={}, payload_len={}",
                self.packet_type.as_str(),
                self.sequence_number,
                self.payload.len()
            );
        }
        pub fn log_sent(&self) {
            tracing::info!(
                "{} packet sent",
                self.packet_type.as_str()
            );
        }
    }

    pub trait MrsPacketFactory {
        type Args;
        fn generate(sequence: u32, args: Self::Args) -> Result<MrsPacket>;
    }

    pub struct VersionCheck;
    impl MrsPacketFactory for VersionCheck {
        type Args = ();
        fn generate(sequence: u32, _args: Self::Args) -> Result<MrsPacket> {
            const SUFFIX: &str = "0200000003006d72730400000208006d72735f726f6f6d00000002";
            Ok(MrsPacket::new(
                sequence,
                MrsPacketType::VersionCheck,
                hex::decode(SUFFIX)?,
            ))
        }
    }

    pub struct KeyExchangeRequest;
    impl MrsPacketFactory for KeyExchangeRequest {
        type Args = String; // 公钥字符串
        fn generate(sequence: u32, pub_key_string: Self::Args) -> Result<MrsPacket> {
            let payload = hex::decode(pub_key_string).context("Failed to decode public key")?;
            Ok(MrsPacket::new(
                sequence,
                MrsPacketType::KeyExchangeRequest,
                payload,
            ))
        }
    }

    pub struct JoinRoom;
    impl MrsPacketFactory for JoinRoom {
        type Args = (u32, u16); // 房间ID和玩家ID
        fn generate(sequence: u32, args: Self::Args) -> Result<MrsPacket> {
            let (room_id, player_id) = args;
            let mut payload = Vec::new();

            // room_id (4字节，小端)
            payload.extend_from_slice(&room_id.to_le_bytes());
            payload.extend_from_slice(&[0u8; 4]); // padding

            // player_id (2字节，小端)
            payload.extend_from_slice(&player_id.to_le_bytes());
            payload.extend_from_slice(&[0u8; 2]); // padding

            const SUFFIX: &str = "000000000100000002000000726cff050000008501000000";
            payload.extend_from_slice(&hex::decode(SUFFIX)?);

            Ok(MrsPacket::new(sequence, MrsPacketType::JoinRoom, payload))
        }
    }

    pub struct KeepAliveResponse;
    impl MrsPacketFactory for KeepAliveResponse {
        type Args = ();
        fn generate(sequence: u32, _args: Self::Args) -> Result<MrsPacket> {
            Ok(MrsPacket::new(
                sequence,
                MrsPacketType::KeepAliveResponse,
                vec![],
            ))
        }
    }

    pub struct PacketGenerator {
        current_sequence: u32,
    }

    impl PacketGenerator {
        pub fn new() -> Self {
            Self {
                current_sequence: 1,
            }
        }

        pub fn next_sequence(&mut self) -> u32 {
            let seq = self.current_sequence;
            self.current_sequence = self.current_sequence.wrapping_add(1);
            seq
        }

        pub fn generate<T>(&mut self, args: T::Args) -> Result<MrsPacket>
        where
            T: MrsPacketFactory,
        {
            let sequence = self.next_sequence();
            T::generate(sequence, args)
        }
    }
}
const RECEIVE_DATA_INTERVAL: Duration = Duration::from_millis(10);
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct MrsConnectionInfo {
    pub host: String,
    pub port: u16,
    pub room_id: u32,
    pub player_id: u16,
}

pub struct MrsClient {
    stream: Option<TcpStream>,
    pub_key_string: String,
    data_index: u32,
    running_signal: Arc<AtomicBool>,
    packet_generator: packet::PacketGenerator,
    connection_info: MrsConnectionInfo,
}

impl MrsClient {
    pub fn new(
        running_signal: Arc<AtomicBool>,
        connection_info: MrsConnectionInfo,
    ) -> Result<Self> {
        let p_k = EphemeralSecret::random(&mut OsRng);
        let pub_key = p_k.public_key();
        let pub_key_string = hex::encode(pub_key.to_sec1_bytes());

        std::fs::create_dir_all("data").context("Failed to create data directory")?;
        Ok(Self {
            stream: None,
            pub_key_string,
            data_index: 0,
            running_signal,
            packet_generator: packet::PacketGenerator::new(),
            connection_info,
        })
    }

    pub fn connect(&mut self) -> Result<()> {
        tracing::info!(
            "Connecting to server at {}:{}...",
            self.connection_info.host, self.connection_info.port
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
        let client_key_packet = self
            .packet_generator
            .generate::<packet::KeyExchangeRequest>(self.pub_key_string.clone())?;
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
        match packet.packet_type {
            packet::MrsPacketType::VersionCheck => {
                if let Err(e) = self.send_handshake() {
                    tracing::error!("Failed to send handshake response: {}", e);
                }
            }
            packet::MrsPacketType::KeyExchangeResponse => {
                if let Err(e) = self.send_join_room() {
                    tracing::error!("Failed to send join room request: {}", e);
                }
            }
            packet::MrsPacketType::KeepAliveRequest => {
                if let Err(e) = self.send_keepalive() {
                    tracing::error!("Failed to send keepalive response: {}", e);
                }
            }
            packet::MrsPacketType::End | packet::MrsPacketType::ConnectionClose => {
                return Ok(false);
            }
            _ => {
                // 暂时不处理其他类型的包
                tracing::warn!(
                    "Received packet type: {}, but not handling it",
                    packet.packet_type.as_str()
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
                let data = &buffer[0..bytes_read];
                tracing::trace!(
                    "Received {} bytes from server: {}",
                    bytes_read,
                    hex::encode(data)
                );
                match packet::MrsPacket::from_bytes(data) {
                    Ok(packet) => {
                        tracing::debug!(
                            "Received packet: type={:?}, seq={}, payload_len={}",
                            packet.packet_type,
                            packet.sequence_number,
                            packet.payload.len()
                        );
                        let Ok(_) = self.handle_packet(packet) else {
                            return Ok(false);
                        };
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse packet: {}, saving raw data", e);
                    }
                }

                // 保存接收到的原始数据
                let file_path = format!("data/data_{}.bin", self.data_index);
                let mut file = File::create(&file_path)
                    .with_context(|| format!("Failed to create file {}", file_path))?;
                file.write_all(data)
                    .with_context(|| format!("Failed to write data to {}", file_path))?;
                tracing::debug!(
                    "Data saved to {} ({} bytes)",
                    file_path,
                    bytes_read
                );
                self.data_index += 1;
                Ok(true)
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 非阻塞模式下没有数据可读，正常情况
                Ok(true)
            }
            Err(e) => Err(anyhow::anyhow!("Error reading from socket: {}", e)),
        }
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

    pub async fn run(&mut self) -> Result<()> {
        let mut reconnect_attempts = 0u32;

        loop {
            if !self.running_signal.load(Ordering::Relaxed) {
                tracing::info!("Received shutdown signal, exiting...");
                break;
            }

            // 如果没有连接，尝试重连
            if self.stream.is_none() {
                if reconnect_attempts >= MAX_RECONNECT_ATTEMPTS {
                    return Err(anyhow::anyhow!("Max reconnection attempts reached"));
                }

                if reconnect_attempts > 0 {
                    tracing::info!(
                        "Reconnection attempt {} of {}",
                        reconnect_attempts + 1,
                        MAX_RECONNECT_ATTEMPTS
                    );
                    sleep(RECONNECT_DELAY).await;
                }

                match self.connect() {
                    Ok(_) => {
                        reconnect_attempts = 0;
                    }
                    Err(e) => {
                        tracing::error!("Failed to connect: {}", e);
                        reconnect_attempts += 1;
                        continue;
                    }
                }
            }

            // 处理不同状态的逻辑
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

            // 根据l4抓包数据获取的大概间隔
            sleep(RECEIVE_DATA_INTERVAL).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOM_ID: u32 = 1919810;
    const PLAYER_ID: u16 = 11451;

    const VERSION_CHECK: &str =
        "2300000001000000000000ff0200000003006d72730400000208006d72735f726f6f6d00000002"; // first packet
    const KEEPALIVE_RESPONSE: &str = "08000000040000000000a2ff"; // 4th packet

    #[test]
    fn test_all_packets() {
        let mut room_bytes = Vec::new();
        room_bytes.extend_from_slice(&ROOM_ID.to_le_bytes());
        room_bytes.extend_from_slice(&[0u8; 4]);
        let mut player_bytes = Vec::new();
        player_bytes.extend_from_slice(&PLAYER_ID.to_le_bytes());
        player_bytes.extend_from_slice(&[0u8; 2]);
        let join_room = format!(
            "2c0000000300000000000500{}{}000000000100000002000000726cff050000008501000000",
            hex::encode(room_bytes),
            hex::encode(player_bytes)
        );
        let mut packet_gen = packet::PacketGenerator::new();

        // 测试握手包生成
        let handshake_packet = packet_gen.generate::<packet::VersionCheck>(()).unwrap();
        assert_eq!(
            handshake_packet.packet_type,
            packet::MrsPacketType::VersionCheck
        );
        assert_eq!(
            handshake_packet.to_bytes(),
            hex::decode(VERSION_CHECK).unwrap()
        );
        assert_eq!(handshake_packet.to_hex_string(), VERSION_CHECK);

        let p_k = EphemeralSecret::random(&mut OsRng);
        let pub_key = p_k.public_key();
        let pub_key_string = hex::encode(pub_key.to_sec1_bytes());

        // 测试密钥交换包生成
        let key_exchange_packet = packet_gen
            .generate::<packet::KeyExchangeRequest>(pub_key_string.clone())
            .unwrap();
        assert_eq!(
            key_exchange_packet.packet_type,
            packet::MrsPacketType::KeyExchangeRequest
        );
        assert_eq!(
            key_exchange_packet.to_bytes(),
            hex::decode(format!("4900000002000000000001ff{}", pub_key_string)).unwrap()
        ); // 2nd packet

        // 测试加入房间包生成
        let join_room_packet = packet_gen
            .generate::<packet::JoinRoom>((ROOM_ID, PLAYER_ID))
            .unwrap();
        assert_eq!(
            join_room_packet.packet_type,
            packet::MrsPacketType::JoinRoom
        );
        assert_eq!(join_room_packet.len(), 48);
        assert_eq!(
            join_room_packet.to_bytes(),
            hex::decode(&join_room).unwrap()
        );

        // 测试心跳包生成
        let keepalive_packet = packet_gen
            .generate::<packet::KeepAliveResponse>(())
            .unwrap();
        assert_eq!(
            keepalive_packet.packet_type,
            packet::MrsPacketType::KeepAliveResponse
        );
        assert_eq!(keepalive_packet.len(), 12);
        assert_eq!(
            keepalive_packet.to_bytes(),
            hex::decode(KEEPALIVE_RESPONSE).unwrap()
        );
    }
}
