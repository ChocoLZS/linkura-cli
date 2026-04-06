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
            MrsPacketType::KeyExchangeResponse => "KeyExchangeResponse",
            MrsPacketType::KeepAliveRequest => "KeepAliveRequest",
            MrsPacketType::ConnectionClose => "ConnectionClose",
            MrsPacketType::ConnectionCloseHardLimitOver => "ConnectionCloseHardLimitOver",
            MrsPacketType::End => "End",
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

        let packet_type = MrsPacketType::from_u16(packet_type_value)
            .ok_or_else(|| anyhow::anyhow!("Unknown packet type: 0x{:04x}", packet_type_value))?;

        let payload = data[12..].to_vec();

        Ok(Self {
            sequence_number,
            packet_type,
            payload,
        })
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        12 + self.payload.len()
    }

    /// 获取包的十六进制字符串表示（用于调试）
    #[allow(dead_code)]
    pub fn to_hex_string(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn log_send(&self) {
        tracing::info!("Sending packet {}", self.packet_type.as_str());
        tracing::debug!(
            "Sending packet: type={}, seq={}, payload_len={}",
            self.packet_type.as_str(),
            self.sequence_number,
            self.payload.len()
        );
    }
    pub fn log_sent(&self) {
        tracing::info!("{} packet sent", self.packet_type.as_str());
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

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdh::EphemeralSecret;
    use rand_core::OsRng;
    const ROOM_ID: u32 = 1919810;
    const PLAYER_ID: u16 = 11451;

    const VERSION_CHECK: &str =
        "2300000001000000000000ff0200000003006d72730400000208006d72735f726f6f6d00000002"; // first packet
    const KEEPALIVE_RESPONSE: &str = "08000000040000000000a2ff"; // 4th packet

    #[test]
    fn test_packet_type_conversion() {
        assert_eq!(
            MrsPacketType::from_u16(0xff00),
            Some(MrsPacketType::VersionCheck)
        );
        assert_eq!(
            MrsPacketType::from_u16(0xff01),
            Some(MrsPacketType::KeyExchangeRequest)
        );
        assert_eq!(
            MrsPacketType::from_u16(0x0005),
            Some(MrsPacketType::JoinRoom)
        );
        assert_eq!(
            MrsPacketType::from_u16(0xffa2),
            Some(MrsPacketType::KeepAliveResponse)
        );
        assert_eq!(MrsPacketType::from_u16(0x9999), None); // 未知类型
    }

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
        let mut packet_gen = PacketGenerator::new();

        // 测试握手包生成
        let handshake_packet = packet_gen.generate::<VersionCheck>(()).unwrap();
        assert_eq!(handshake_packet.packet_type, MrsPacketType::VersionCheck);
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
            .generate::<KeyExchangeRequest>(pub_key_string.clone())
            .unwrap();
        assert_eq!(
            key_exchange_packet.packet_type,
            MrsPacketType::KeyExchangeRequest
        );
        assert_eq!(
            key_exchange_packet.to_bytes(),
            hex::decode(format!("4900000002000000000001ff{}", pub_key_string)).unwrap()
        ); // 2nd packet

        // 测试加入房间包生成
        let join_room_packet = packet_gen
            .generate::<JoinRoom>((ROOM_ID, PLAYER_ID))
            .unwrap();
        assert_eq!(join_room_packet.packet_type, MrsPacketType::JoinRoom);
        assert_eq!(join_room_packet.len(), 48);
        assert_eq!(
            join_room_packet.to_bytes(),
            hex::decode(&join_room).unwrap()
        );

        // 测试心跳包生成
        let keepalive_packet = packet_gen.generate::<KeepAliveResponse>(()).unwrap();
        assert_eq!(
            keepalive_packet.packet_type,
            MrsPacketType::KeepAliveResponse
        );
        assert_eq!(keepalive_packet.len(), 12);
        assert_eq!(
            keepalive_packet.to_bytes(),
            hex::decode(KEEPALIVE_RESPONSE).unwrap()
        );
    }
}
