pub mod extension;
pub mod analyzer;
pub mod application;
pub mod formatter;
pub mod reader;

use chrono::{DateTime, Utc};
use prost::Message;
use prost::encoding::{WireType, encode_key, encode_varint};
use sha2::{Digest, Sha256};
use std::usize;

pub mod define {
    include!(concat!(env!("OUT_DIR"), "/als.rs"));
}

use define::DataPack;

use crate::als::proto::define::{DataFrame, Room, data_frame};

fn encode_frame(frame: &DataFrame, buf: &mut Vec<u8>) {
    let frame_bytes = frame.encode_to_vec();
    encode_key(16, WireType::LengthDelimited, buf);
    encode_varint(frame_bytes.len() as u64, buf);
    buf.extend_from_slice(&frame_bytes);
}

/// Custom DataPack encoder that puts frames before control messages
/// This matches the expected order that the analyzer expects
fn encode_data_pack_custom_order(data_pack: &DataPack) -> Vec<u8> {
    let mut buf = Vec::new();

    // Encode frames first (field number 16)
    if !data_pack.frames.is_empty() {
        for frame in &data_pack.frames {
            encode_frame(frame, &mut buf);
        }
    }

    // Then encode control messages in field number order
    if let Some(control) = &data_pack.control {
        match control {
            define::data_pack::Control::Data(value) => {
                // Field number 2, wire type Varint
                encode_key(2, WireType::Varint, &mut buf);
                encode_varint(if *value { 1 } else { 0 }, &mut buf);
            }
            define::data_pack::Control::Pong(value) => {
                // Field number 10, wire type Varint
                encode_key(10, WireType::Varint, &mut buf);
                encode_varint(if *value { 1 } else { 0 }, &mut buf);
            }
            define::data_pack::Control::SegmentStartedAt(value) => {
                // Field number 14, wire type Varint
                encode_key(14, WireType::Varint, &mut buf);
                encode_varint(*value as u64, &mut buf);
            }
            define::data_pack::Control::CacheEnded(value) => {
                // Field number 15, wire type Varint
                encode_key(15, WireType::Varint, &mut buf);
                encode_varint(if *value { 1 } else { 0 }, &mut buf);
            }
        }
    }

    buf
}

/// Calculate SHA-256 digest for binary data and return as hex string
pub fn calculate_digest(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub timestamp: DateTime<Utc>,
    pub data_pack: DataPack,
    pub raw_data: Vec<u8>,
}

impl PacketInfo {
    pub fn len(&self) -> u16 {
        (self.raw_data.len() + 9) as u16
    }

    /// Calculate SHA-256 digest for the protobuf segment
    pub fn protobuf_digest(&self) -> String {
        calculate_digest(&self.raw_data)
    }

    /// Calculate SHA-256 digest for each DataFrame
    pub fn frame_digests(&self) -> Vec<(usize, String)> {
        self.data_pack
            .frames
            .iter()
            .enumerate()
            .map(|(index, frame)| {
                let frame_bytes = frame.encode_to_vec();
                (index, calculate_digest(&frame_bytes))
            })
            .collect()
    }

    pub fn create_segment_started_packet(timestamp: DateTime<Utc>) -> Self {
        Self {
            timestamp: timestamp,
            data_pack: DataPack {
                control: Some(define::data_pack::Control::SegmentStartedAt(
                    timestamp.timestamp_micros(),
                )),
                frames: vec![],
            },
            raw_data: vec![],
        }
    }

    pub fn create_room_frame(timestamp: DateTime<Utc>, room: Room) -> Self {
        Self {
            timestamp,
            data_pack: DataPack {
                control: Some(define::data_pack::Control::Data(true)),
                frames: vec![DataFrame {
                    message: Some(data_frame::Message::Room(room)),
                }],
            },
            raw_data: vec![],
        }
    }

    pub fn create_cache_end(timestamp: DateTime<Utc>) -> Self {
        Self {
            timestamp,
            data_pack: DataPack {
                control: Some(define::data_pack::Control::CacheEnded(true)),
                frames: vec![],
            },
            raw_data: vec![],
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let data_pack_bytes = self.protobuf_to_vec(); // keep bytes order
        // let data_pack_bytes = self.data_pack.encode_to_vec(); // do not keep the bytes order but workable for replay
        let len = 9 + data_pack_bytes.len() as u16;
        buf.extend_from_slice(&len.to_be_bytes());
        buf.push(0x01); // live mark
        buf.extend_from_slice(&self.timestamp.timestamp_micros().to_be_bytes());
        buf.extend_from_slice(&data_pack_bytes);
        buf
    }

    pub fn protobuf_to_vec(&self) -> Vec<u8> {
        encode_data_pack_custom_order(&self.data_pack)
    }

    pub fn frame_to_vec(frame: &DataFrame) -> Vec<u8> {
        let mut buf = Vec::new();
        encode_frame(frame, &mut buf);
        buf
    }
}

macro_rules! if_some {
    ($var:ident in $expr:expr, $block:block) => {
        if let Some(ref mut $var) = $expr {
            $block
        }
    };
}