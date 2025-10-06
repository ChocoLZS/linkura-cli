pub mod extension;
pub mod analyzer;
// pub mod application;
pub mod formatter;
pub mod reader;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use prost::Message;
use prost::encoding::{WireType, encode_key, encode_varint};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read, Seek, Write};
use std::usize;

pub mod proto {
    pub mod als {
        include!(concat!(env!("OUT_DIR"), "/als.rs"));
    }
}

use prost::bytes::Buf;
use proto::als::DataPack;

use crate::als::proto::proto::als::{DataFrame, Room, data_frame};

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
            proto::als::data_pack::Control::Data(value) => {
                // Field number 2, wire type Varint
                encode_key(2, WireType::Varint, &mut buf);
                encode_varint(if *value { 1 } else { 0 }, &mut buf);
            }
            proto::als::data_pack::Control::Pong(value) => {
                // Field number 10, wire type Varint
                encode_key(10, WireType::Varint, &mut buf);
                encode_varint(if *value { 1 } else { 0 }, &mut buf);
            }
            proto::als::data_pack::Control::SegmentStartedAt(value) => {
                // Field number 14, wire type Varint
                encode_key(14, WireType::Varint, &mut buf);
                encode_varint(*value as u64, &mut buf);
            }
            proto::als::data_pack::Control::CacheEnded(value) => {
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
                control: Some(proto::als::data_pack::Control::SegmentStartedAt(
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
                control: Some(proto::als::data_pack::Control::Data(true)),
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
                control: Some(proto::als::data_pack::Control::CacheEnded(true)),
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

impl TryFrom<MixedPacketInfo> for PacketInfo {
    type Error = anyhow::Error;
    fn try_from(mixed_packet: MixedPacketInfo) -> Result<Self> {
        let data_pack = mixed_packet
            .data_pack
            .ok_or_else(|| anyhow!("Missing data pack"))?;
        Ok(Self {
            timestamp: mixed_packet.timestamp.unwrap_or_else(|| Utc::now()),
            data_pack,
            raw_data: mixed_packet.raw_data,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MixedPacketInfo {
    pub format: MixedPacketFormat,
    pub timestamp: Option<DateTime<Utc>>,
    pub data_pack: Option<DataPack>,
    pub raw_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum MixedPacketFormat {
    // Format 1: int16 length (big endian) + int8 unused (0x00) + byte[length-3] protobuf data
    ProtobufFormat {
        length: u16,
        unused: u8,
    },
    // Format 2: int8 length + int64 timestamp
    TimestampFormat {
        length: u16,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone)]
pub struct ProtobufField {
    pub field_number: u32,
    pub wire_type: u8,
    pub field_name: String,
    pub raw_bytes: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct ControlMessageStats {
    pub data_count: u32,
    pub pong_count: u32,
    pub segment_started_at_count: u32,
    pub cache_ended_count: u32,
    pub total_control_messages: u32,
}

#[derive(Debug, Default, Clone)]
pub struct FrameMessageStats {
    pub instantiate_object_count: u32,
    pub update_object_count: u32,
    pub destroy_object_count: u32,
    pub room_count: u32,
    pub authorize_response_count: u32,
    pub join_room_response_count: u32,
    pub total_frame_messages: u32,
}

#[derive(Debug, Default, Clone)]
pub struct UnknownFieldStats {
    pub unknown_fields: std::collections::HashMap<u32, u32>, // field_number -> count
}

#[derive(Debug, Default, Clone)]
pub struct PacketAnalysisStats {
    pub total_packets: u32,
    pub packets_with_control: u32,
    pub packets_with_frames: u32,
    pub total_frames: u32,
    pub control_stats: ControlMessageStats,
    pub frame_stats: FrameMessageStats,
    pub unknown_field_stats: UnknownFieldStats,
}

pub fn parse_protobuf_fields(data: &[u8]) -> Vec<ProtobufField> {
    let mut fields = Vec::new();
    let mut cursor = std::io::Cursor::new(data);

    while cursor.position() < data.len() as u64 {
        match parse_protobuf_field(&mut cursor) {
            Ok(field) => fields.push(field),
            Err(_) => break,
        }
    }

    fields
}

fn parse_protobuf_field(cursor: &mut std::io::Cursor<&[u8]>) -> Result<ProtobufField> {
    let start_pos = cursor.position();

    // Read varint (field number and wire type)
    let tag = read_varint(cursor)?;
    let field_number = (tag >> 3) as u32;
    let wire_type = (tag & 0x7) as u8;

    let field_name = match wire_type {
        0 => "Varint".to_string(),
        1 => "64-bit".to_string(),
        2 => "Length-delimited".to_string(),
        3 => "Start group (deprecated)".to_string(),
        4 => "End group (deprecated)".to_string(),
        5 => "32-bit".to_string(),
        _ => format!("Unknown wire type {}", wire_type),
    };

    // Skip field data based on wire type
    let _field_data_start = cursor.position();
    match wire_type {
        0 => {
            // Varint
            read_varint(cursor)?;
        }
        1 => {
            // 64-bit
            if cursor.remaining() < 8 {
                return Err(anyhow!("Not enough bytes for 64-bit field"));
            }
            cursor.advance(8);
        }
        2 => {
            // Length-delimited
            let length = read_varint(cursor)?;
            if cursor.remaining() < length as usize {
                return Err(anyhow!("Not enough bytes for length-delimited field"));
            }
            cursor.advance(length as usize);
        }
        5 => {
            // 32-bit
            if cursor.remaining() < 4 {
                return Err(anyhow!("Not enough bytes for 32-bit field"));
            }
            cursor.advance(4);
        }
        _ => {
            return Err(anyhow!("Unsupported wire type: {}", wire_type));
        }
    }

    let end_pos = cursor.position();
    let field_bytes = cursor.get_ref()[(start_pos as usize)..(end_pos as usize)].to_vec();

    Ok(ProtobufField {
        field_number,
        wire_type,
        field_name,
        raw_bytes: field_bytes,
    })
}

fn read_varint(cursor: &mut std::io::Cursor<&[u8]>) -> Result<u64> {
    let mut result = 0u64;
    let mut shift = 0;

    loop {
        if cursor.remaining() == 0 {
            return Err(anyhow!("Unexpected end of data while reading varint"));
        }

        let byte = cursor.get_u8();
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift >= 64 {
            return Err(anyhow!("Varint too long"));
        }
    }

    Ok(result)
}

// Helper functions for formatting target types
fn format_instantiate_object_target(
    target: &Option<proto::als::instantiate_object::Target>,
) -> String {
    match target {
        Some(proto::als::instantiate_object::Target::CurrentPlayer(_)) => {
            "CurrentPlayer".to_string()
        }
        Some(proto::als::instantiate_object::Target::RoomAll(room)) => {
            format!(
                "RoomAll (room_id: {:?})",
                String::from_utf8_lossy(&room.room_id)
            )
        }
        Some(proto::als::instantiate_object::Target::PlayerId(player_id)) => {
            format!("PlayerId ({:?})", String::from_utf8_lossy(player_id))
        }
        None => "None".to_string(),
    }
}

fn format_update_object_target(target: &Option<proto::als::update_object::Target>) -> String {
    match target {
        Some(proto::als::update_object::Target::CurrentPlayer(_)) => "CurrentPlayer".to_string(),
        Some(proto::als::update_object::Target::RoomAll(room)) => {
            format!(
                "RoomAll (room_id: {:?})",
                String::from_utf8_lossy(&room.room_id)
            )
        }
        Some(proto::als::update_object::Target::PlayerId(player_id)) => {
            format!("PlayerId ({:?})", String::from_utf8_lossy(player_id))
        }
        None => "None".to_string(),
    }
}

impl ControlMessageStats {
    pub fn update_from_control(&mut self, control: &proto::als::data_pack::Control) {
        match control {
            proto::als::data_pack::Control::Data(_) => {
                self.data_count += 1;
            }
            proto::als::data_pack::Control::Pong(_) => {
                self.pong_count += 1;
            }
            proto::als::data_pack::Control::SegmentStartedAt(_) => {
                self.segment_started_at_count += 1;
            }
            proto::als::data_pack::Control::CacheEnded(_) => {
                self.cache_ended_count += 1;
            }
        }
        self.total_control_messages += 1;
    }
}

impl FrameMessageStats {
    pub fn update_from_frame(&mut self, frame: &proto::als::DataFrame) {
        if let Some(message) = &frame.message {
            use proto::als::data_frame::Message;
            match message {
                Message::InstantiateObject(_) => self.instantiate_object_count += 1,
                Message::UpdateObject(_) => self.update_object_count += 1,
                Message::DestroyObject(_) => self.destroy_object_count += 1,
                Message::Room(_) => self.room_count += 1,
                Message::AuthorizeResponse(_) => self.authorize_response_count += 1,
                Message::JoinRoomResponse(_) => self.join_room_response_count += 1,
            }
            self.total_frame_messages += 1;
        }
    }
}

impl UnknownFieldStats {
    pub fn update_from_raw_data(&mut self, raw_data: &[u8]) {
        let fields = parse_protobuf_fields(raw_data);
        for field in fields {
            // Check if this field number is unknown/unhandled
            if !self.is_known_field_number(field.field_number) {
                *self.unknown_fields.entry(field.field_number).or_insert(0) += 1;
            }
        }
    }

    fn is_known_field_number(&self, field_number: u32) -> bool {
        // Known field numbers from datapack.proto
        match field_number {
            // DataPack fields
            2 => true,  // data
            10 => true, // pong
            14 => true, // segment_started_at
            15 => true, // cache_ended
            16 => true, // frames
            // DataFrame fields
            128 => true, // instantiate_object
            129 => true, // update_object
            130 => true, // destroy_object
            143 => true, // room
            144 => true, // authorize_response
            147 => true, // join_room_response
            // InstantiateObject fields
            3 => true,  // current_player target
            4 => true,  // room_all target
            7 => true,  // player_id target
            8 => true,  // object_id
            9 => true,  // owner_id
            11 => true, // init_data
            // UpdateObject fields (reuses some target fields)
            6 => true, // player_id target (different field num than InstantiateObject)
            // DestroyObject fields (reuses UpdateObject target field numbers)
            // Room fields
            1 => true, // id
            // Note: fields 2 and 3 are already covered above in different contexts
            // AuthorizeResponse fields
            // (reuses id=1, role=2, allowed_room_ids=3)
            // JoinRoomResponse fields
            // (reuses room=1, joined_at=2)
            _ => false,
        }
    }
}

impl PacketAnalysisStats {
    pub fn update_from_packet(&mut self, data_pack: &DataPack) {
        self.total_packets += 1;

        if let Some(control) = &data_pack.control {
            self.packets_with_control += 1;
            self.control_stats.update_from_control(control);
        }

        if !data_pack.frames.is_empty() {
            self.packets_with_frames += 1;
            self.total_frames += data_pack.frames.len() as u32;

            for frame in &data_pack.frames {
                self.frame_stats.update_from_frame(frame);
            }
        }
    }

    pub fn update_from_raw_data(&mut self, raw_data: &[u8]) {
        self.unknown_field_stats.update_from_raw_data(raw_data);
    }

    pub fn merge_with(&mut self, other: PacketAnalysisStats) {
        self.total_packets += other.total_packets;
        self.packets_with_control += other.packets_with_control;
        self.packets_with_frames += other.packets_with_frames;
        self.total_frames += other.total_frames;

        // Merge control stats
        self.control_stats.data_count += other.control_stats.data_count;
        self.control_stats.pong_count += other.control_stats.pong_count;
        self.control_stats.segment_started_at_count += other.control_stats.segment_started_at_count;
        self.control_stats.cache_ended_count += other.control_stats.cache_ended_count;
        self.control_stats.total_control_messages += other.control_stats.total_control_messages;

        // Merge frame stats
        self.frame_stats.instantiate_object_count += other.frame_stats.instantiate_object_count;
        self.frame_stats.update_object_count += other.frame_stats.update_object_count;
        self.frame_stats.destroy_object_count += other.frame_stats.destroy_object_count;
        self.frame_stats.room_count += other.frame_stats.room_count;
        self.frame_stats.authorize_response_count += other.frame_stats.authorize_response_count;
        self.frame_stats.join_room_response_count += other.frame_stats.join_room_response_count;
        self.frame_stats.total_frame_messages += other.frame_stats.total_frame_messages;

        // Merge unknown field stats
        for (field_num, count) in other.unknown_field_stats.unknown_fields {
            *self
                .unknown_field_stats
                .unknown_fields
                .entry(field_num)
                .or_insert(0) += count;
        }
    }
}

pub fn format_statistics(writer: &mut OutputWriter, stats: &PacketAnalysisStats) -> Result<()> {
    writer.writeln("")?;
    writer.writeln("================== STATISTICS ==================")?;
    writer.writeln(&format!("Total packets analyzed: {}", stats.total_packets))?;
    writer.writeln(&format!(
        "Packets with control messages: {} ({:.1}%)",
        stats.packets_with_control,
        if stats.total_packets > 0 {
            stats.packets_with_control as f64 / stats.total_packets as f64 * 100.0
        } else {
            0.0
        }
    ))?;
    writer.writeln(&format!(
        "Packets with frames: {} ({:.1}%)",
        stats.packets_with_frames,
        if stats.total_packets > 0 {
            stats.packets_with_frames as f64 / stats.total_packets as f64 * 100.0
        } else {
            0.0
        }
    ))?;
    writer.writeln(&format!("Total frames: {}", stats.total_frames))?;
    writer.writeln("")?;

    // Control message statistics
    let control = &stats.control_stats;
    if control.total_control_messages > 0 {
        writer.writeln("Control Message Types:")?;
        if control.data_count > 0 {
            writer.writeln(&format!(
                "  Data: {} ({:.1}%)",
                control.data_count,
                control.data_count as f64 / control.total_control_messages as f64 * 100.0
            ))?;
        }
        if control.pong_count > 0 {
            writer.writeln(&format!(
                "  Pong: {} ({:.1}%)",
                control.pong_count,
                control.pong_count as f64 / control.total_control_messages as f64 * 100.0
            ))?;
        }
        if control.segment_started_at_count > 0 {
            writer.writeln(&format!(
                "  SegmentStartedAt: {} ({:.1}%)",
                control.segment_started_at_count,
                control.segment_started_at_count as f64 / control.total_control_messages as f64
                    * 100.0
            ))?;
        }
        if control.cache_ended_count > 0 {
            writer.writeln(&format!(
                "  CacheEnded: {} ({:.1}%)",
                control.cache_ended_count,
                control.cache_ended_count as f64 / control.total_control_messages as f64 * 100.0
            ))?;
        }
        writer.writeln(&format!(
            "  Total Control Messages: {}",
            control.total_control_messages
        ))?;
        writer.writeln("")?;
    }

    // Frame message statistics
    let frame = &stats.frame_stats;
    if frame.total_frame_messages > 0 {
        writer.writeln("Frame Message Types:")?;

        // Response messages
        let response_count = frame.authorize_response_count + frame.join_room_response_count;

        if response_count > 0 {
            writer.writeln("  Response Messages:")?;
            if frame.authorize_response_count > 0 {
                writer.writeln(&format!(
                    "    AuthorizeResponse: {} ({:.1}%)",
                    frame.authorize_response_count,
                    frame.authorize_response_count as f64 / frame.total_frame_messages as f64
                        * 100.0
                ))?;
            }
            if frame.join_room_response_count > 0 {
                writer.writeln(&format!(
                    "    JoinRoomResponse: {} ({:.1}%)",
                    frame.join_room_response_count,
                    frame.join_room_response_count as f64 / frame.total_frame_messages as f64
                        * 100.0
                ))?;
            }
        }

        // Object-related messages
        let object_count =
            frame.instantiate_object_count + frame.update_object_count + frame.destroy_object_count;
        if object_count > 0 {
            writer.writeln("  Object Messages:")?;
            if frame.instantiate_object_count > 0 {
                writer.writeln(&format!(
                    "    InstantiateObject: {} ({:.1}%)",
                    frame.instantiate_object_count,
                    frame.instantiate_object_count as f64 / frame.total_frame_messages as f64
                        * 100.0
                ))?;
            }
            if frame.update_object_count > 0 {
                writer.writeln(&format!(
                    "    UpdateObject: {} ({:.1}%)",
                    frame.update_object_count,
                    frame.update_object_count as f64 / frame.total_frame_messages as f64 * 100.0
                ))?;
            }
            if frame.destroy_object_count > 0 {
                writer.writeln(&format!(
                    "    DestroyObject: {} ({:.1}%)",
                    frame.destroy_object_count,
                    frame.destroy_object_count as f64 / frame.total_frame_messages as f64 * 100.0
                ))?;
            }
        }

        // Other messages
        if frame.room_count > 0 {
            writer.writeln(&format!(
                "  Room: {} ({:.1}%)",
                frame.room_count,
                frame.room_count as f64 / frame.total_frame_messages as f64 * 100.0
            ))?;
        }

        writer.writeln(&format!(
            "  Total Frame Messages: {}",
            frame.total_frame_messages
        ))?;
    }

    // Unknown field statistics
    if !stats.unknown_field_stats.unknown_fields.is_empty() {
        writer.writeln("Unknown Field Numbers Found:")?;
        let mut unknown_fields: Vec<_> = stats.unknown_field_stats.unknown_fields.iter().collect();
        unknown_fields.sort_by_key(|(field_num, _)| *field_num);

        for (field_number, count) in unknown_fields {
            writer.writeln(&format!("  Field #{}: {} occurrences", field_number, count))?;
        }
        writer.writeln("")?;
    }

    writer.writeln("================================================")?;
    writer.writeln("")?;
    Ok(())
}

// Unified formatting functions
pub fn format_packet_unified(
    writer: &mut OutputWriter,
    packet_number: usize,
    length: u16,
    timestamp: Option<DateTime<Utc>>,
    data_pack: Option<&DataPack>,
    raw_data: &[u8],
    protobuf_data: &[u8],
    format_type: &str,
) -> Result<()> {
    writer.writeln(&format!(
        "=== Packet #{}: {} bytes ===",
        packet_number, length
    ))?;
    writer.writeln(&format!("  Format: {}", format_type))?;

    if let Some(ts) = timestamp {
        let timestamp_micros = ts.timestamp_micros() as u64;
        writer.writeln(&format!(
            "  Timestamp: {} ({} / 0x{:x})",
            ts.format("%Y-%m-%d %H:%M:%S%.6f UTC"),
            timestamp_micros,
            timestamp_micros
        ))?;
    }

    // Show protobuf field analysis for all packets containing protobuf data
    if !raw_data.is_empty() {
        writer.writeln(&format!("  Raw data length: {} bytes", raw_data.len()))?;

        // Show protobuf digest
        let protobuf_digest = calculate_digest(raw_data);
        writer.writeln(&format!("  Protobuf SHA-256 digest: {}", protobuf_digest))?;

        // Show first 32 bytes in hex
        let debug_len = 32.min(raw_data.len());
        writer.writeln(&format!(
            "  Raw data (first {} bytes): {}",
            debug_len,
            raw_data[..debug_len]
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        ))?;
        writer.writeln(&format!(
            "  Protobuf data (first {} bytes): {}",
            protobuf_data.len(),
            protobuf_data
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        ))?;

        // Try to parse protobuf fields
        let fields = parse_protobuf_fields(raw_data);
        if !fields.is_empty() {
            writer.writeln("  Protobuf Fields:")?;
            for field in &fields {
                writer.writeln(&format!(
                    "    Field #{}: {} (wire type: {}, {} bytes)",
                    field.field_number,
                    field.field_name,
                    field.wire_type,
                    field.raw_bytes.len()
                ))?;

                // Show field bytes
                let field_hex = field
                    .raw_bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                writer.writeln(&format!("      Raw bytes: {}", field_hex))?;
            }
        }
    }

    // Show detailed data pack information if available
    if let Some(data_pack) = data_pack {
        print_data_pack_details_unified(writer, data_pack)?;
    }

    writer.writeln("")?; // Empty line for better readability
    Ok(())
}

fn print_data_pack_details_unified(writer: &mut OutputWriter, data_pack: &DataPack) -> Result<()> {
    if let Some(control) = &data_pack.control {
        writer.writeln("  Control message:")?;
        match control {
            proto::als::data_pack::Control::Data(data) => {
                writer.writeln(&format!("    Type: Data, Value: {}", data))?;
            }
            proto::als::data_pack::Control::Pong(pong) => {
                writer.writeln(&format!("    Type: Pong, Value: {}", pong))?;
            }
            proto::als::data_pack::Control::SegmentStartedAt(timestamp) => {
                writer.writeln(&format!(
                    "    Type: SegmentStartedAt, Timestamp: {}",
                    timestamp
                ))?;
            }
            proto::als::data_pack::Control::CacheEnded(ended) => {
                writer.writeln(&format!("    Type: CacheEnded, Value: {}", ended))?;
            }
        }
    } else {
        writer.writeln("  No control message")?;
    }

    if !data_pack.frames.is_empty() {
        writer.writeln(&format!("  Frames ({}):", data_pack.frames.len()))?;
        for (i, frame) in data_pack.frames.iter().enumerate() {
            writer.writeln(&format!("    Frame #{}: ", i + 1))?;

            // Calculate and display frame digest
            let frame_bytes = frame.encode_to_vec();
            let frame_digest = calculate_digest(&frame_bytes);
            writer.writeln(&format!("      DataFrame SHA-256 digest: {}", frame_digest))?;

            if let Some(message) = &frame.message {
                use proto::als::data_frame::Message;
                match message {
                    Message::InstantiateObject(obj) => {
                        let target_desc = format_instantiate_object_target(&obj.target);
                        writer.writeln(&format!("      Message: InstantiateObject"))?;
                        writer.writeln(&format!("        Object ID: {}", obj.object_id))?;
                        writer.writeln(&format!(
                            "        Owner ID: {:?}",
                            String::from_utf8_lossy(&obj.owner_id)
                        ))?;
                        writer.writeln(&format!(
                            "        Prefab Name: {:?}",
                            String::from_utf8_lossy(&obj.prefab_name)
                        ))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !obj.init_data.is_empty() {
                            writer.writeln(&format!(
                                "        Init Data: {} bytes",
                                obj.init_data.len()
                            ))?;
                            let init_data_hex = obj
                                .init_data
                                .iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!(
                                "        Init Data (first {} bytes): {}",
                                32.min(obj.init_data.len()),
                                init_data_hex
                            ))?;
                        }
                    }
                    Message::UpdateObject(obj) => {
                        let target_desc = format_update_object_target(&obj.target);
                        writer.writeln(&format!("      Message: UpdateObject"))?;
                        writer.writeln(&format!("        Object ID: {}", obj.object_id))?;
                        writer.writeln(&format!("        Method: {}", obj.method))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !obj.payload.is_empty() {
                            writer.writeln(&format!(
                                "        Payload: {} bytes",
                                obj.payload.len()
                            ))?;
                            let payload_hex = obj
                                .payload
                                .iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!(
                                "        Payload (first {} bytes): {}",
                                32.min(obj.payload.len()),
                                payload_hex
                            ))?;
                        }
                    }
                    Message::DestroyObject(obj) => {
                        writer.writeln(&format!(
                            "      Message: DestroyObject (object_id: {})",
                            obj.object_id
                        ))?;
                    }
                    Message::Room(room) => {
                        writer.writeln(&format!(
                            "      Message: Room (id: {:?}, started: {}, ended: {})",
                            String::from_utf8_lossy(&room.id),
                            room.started_at,
                            room.ended_at
                        ))?;
                    }
                    Message::AuthorizeResponse(resp) => {
                        writer.writeln(&format!(
                            "      Message: AuthorizeResponse (player_id: {:?}, role: {})",
                            String::from_utf8_lossy(&resp.player_id),
                            resp.role
                        ))?;
                    }
                    Message::JoinRoomResponse(resp) => {
                        if let Some(room) = &resp.room {
                            writer.writeln(&format!(
                                "      Message: JoinRoomResponse (room_id: {:?}, joined_at: {})",
                                String::from_utf8_lossy(&room.id),
                                resp.joined_at
                            ))?;
                        } else {
                            writer.writeln(&format!(
                                "      Message: JoinRoomResponse (joined_at: {})",
                                resp.joined_at
                            ))?;
                        }
                    }
                }
            } else {
                writer.writeln("      No message in frame")?;
            }
        }
    } else {
        writer.writeln("  No frames")?;
    }

    Ok(())
}

macro_rules! if_some {
    ($var:ident in $expr:expr, $block:block) => {
        if let Some(ref mut $var) = $expr {
            $block
        }
    };
}

pub struct ProtoPacketReader {
    reader: BufReader<File>,
}

impl ProtoPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    pub fn read_packets(&mut self) -> Result<Vec<PacketInfo>> {
        self.read_packets_with_limit(8)
    }

    pub fn read_packets_with_limit(&mut self, limit: usize) -> Result<Vec<PacketInfo>> {
        self.read_packets_with_limit_and_writer(limit, None)
    }

    pub fn read_packets_with_limit_and_writer(
        &mut self,
        limit: usize,
        mut writer: Option<&mut OutputWriter>,
    ) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        let mut packet_count = 0;
        let mut stats = PacketAnalysisStats::default();
        loop {
            match self.read_packet() {
                Ok(packet) => {
                    packet_count += 1;

                    // Update statistics
                    stats.update_from_packet(&packet.data_pack);
                    stats.update_from_raw_data(&packet.raw_data);
                    if_some!(w in writer, {
                        format_packet_unified(
                            w,
                            packet_count,
                            packet.len(),
                            Some(packet.timestamp),
                            Some(&packet.data_pack),
                            &packet.to_vec(),
                            &packet.protobuf_to_vec(),
                            "Standard protobuf format (int16 length + int8 unused + int64 timestamp + protobuf data)"
                        )?;
                    });
                    packets.push(packet);

                    // Check if we've reached the limit
                    if packet_count >= limit {
                        if_some!(w in writer, {
                            w.writeln(&format!("Reached requested packet limit of {} packets", limit))?;
                        });
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();

                    // Check for various EOF-related conditions
                    let is_eof = error_msg.contains("eof")
                        || error_msg.contains("unexpectedeof")
                        || error_msg.contains("failed to fill whole buffer")
                        || error_msg.contains("failed to read packet length")
                        || error_msg.contains("failed to read unused byte")
                        || error_msg.contains("failed to read timestamp")
                        || error_msg.contains("failed to read protobuf data");

                    // Also check the error chain for IO errors
                    let mut current_error: &dyn std::error::Error = e.as_ref();
                    let mut has_io_error = false;
                    while let Some(source) = current_error.source() {
                        if let Some(io_err) = source.downcast_ref::<std::io::Error>() {
                            if matches!(io_err.kind(), std::io::ErrorKind::UnexpectedEof) {
                                has_io_error = true;
                                break;
                            }
                        }
                        current_error = source;
                    }

                    if is_eof || has_io_error {
                        if_some!(w in writer, {
                            w.writeln(&format!("Reached end of file after {} packets (requested: {})", packets.len(), limit))?;
                        });
                        break;
                    }

                    // Write detailed error information
                    if_some!(w in writer, {
                        w.writeln(&format!("=== PACKET PARSING ERROR #{} ===", packet_count + 1))?;
                        w.writeln(&format!("Error: {}", e))?;
                    // Try to show current file position and nearby bytes for debugging
                        if let Ok(current_pos) = self.reader.stream_position() {
                            w.writeln(&format!("Current file position: 0x{:x} ({})", current_pos, current_pos))?;

                            // Try to read some bytes around the current position for debugging
                            let mut debug_buffer = [0u8; 32];
                            if current_pos >= 16 {
                                if self
                                    .reader
                                    .seek(std::io::SeekFrom::Start(current_pos - 16))
                                    .is_ok()
                                {
                                    if let Ok(bytes_read) = self.reader.read(&mut debug_buffer) {
                                        let debug_hex = debug_buffer[..bytes_read]
                                            .iter()
                                            .enumerate()
                                            .map(|(i, b)| {
                                                if i == 16 {
                                                    format!("[{:02x}]", b)
                                                }
                                                // Mark current position
                                                else {
                                                    format!("{:02x}", b)
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                            .join(" ");
                                        w.writeln(&format!("Context bytes (16 before + 16 after): {}", debug_hex))?;
                                    }
                                }
                            }
                        }
                    w.writeln("=================================")?;
                    });
                    return Err(e.context(format!("Failed at packet #{}", packet_count + 1)));
                }
            }
        }
        if_some!(w in writer, {
            format_statistics(w, &stats)?;
        });

        Ok(packets)
    }

    fn read_packet(&mut self) -> Result<PacketInfo> {
        let length = self
            .read_u16_be()
            .with_context(|| "Failed to read packet length")?;

        if length < 9 {
            return Err(anyhow!(
                "Invalid packet length: {}, must be at least 9",
                length
            ));
        }

        let unused = self
            .read_u8()
            .with_context(|| "Failed to read unused byte")?;
        if unused != 0x01 {
            return Err(anyhow!(
                "Invalid unused byte: expected 0x01, got 0x{:02x}",
                unused
            ));
        }

        let timestamp_micros = self
            .read_u64_be()
            .with_context(|| "Failed to read timestamp")?;
        let timestamp =
            DateTime::from_timestamp_micros(timestamp_micros as i64).ok_or_else(|| {
                anyhow!(
                    "Invalid timestamp: {} (0x{:x})",
                    timestamp_micros,
                    timestamp_micros
                )
            })?;

        let data_length = length - 9;
        let mut data = vec![0u8; data_length as usize];
        self.reader
            .read_exact(&mut data)
            .with_context(|| format!("Failed to read protobuf data of length {}", data_length))?;

        let data_pack = DataPack::decode(data.as_slice()).map_err(|e| {
            anyhow!(
                "Failed to decode protobuf data (length: {}, timestamp: {} / 0x{:x}): {}",
                data_length,
                timestamp_micros,
                timestamp_micros,
                e
            )
        })?;

        Ok(PacketInfo {
            timestamp,
            data_pack,
            raw_data: data,
        })
    }

    fn read_two_packets(&mut self) -> Result<(PacketInfo, PacketInfo)> {
        let first_packet = self.read_packet()?;
        let second_packet = self.read_packet()?;
        Ok((first_packet, second_packet))
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

enum MixedPacketReaderState {
    Packet,
    Timestamp,
}

pub struct MixedPacketReader {
    reader: BufReader<File>,
    state: MixedPacketReaderState,
}

impl MixedPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            state: MixedPacketReaderState::Packet,
        }
    }

    pub fn read_mixed_packets(&mut self) -> Result<Vec<MixedPacketInfo>> {
        self.read_mixed_packets_with_limit(std::usize::MAX)
    }

    pub fn read_mixed_packets_with_limit(&mut self, limit: usize) -> Result<Vec<MixedPacketInfo>> {
        self.read_mixed_packets_with_limit_and_writer(limit, None, None, None)
            .map(|(packets, _)| packets)
    }

    pub fn read_mixed_packets_with_limit_and_writer(
        &mut self,
        limit: usize,
        mut writer: Option<&mut OutputWriter>,
        data_start_time: Option<String>,
        data_end_time: Option<String>,
    ) -> Result<(Vec<MixedPacketInfo>, bool)> {
        let mut packets = Vec::new();
        let mut packet_count = 0;
        let mut stats = PacketAnalysisStats::default();
        let mut end_flag = false;
        let start_time = data_start_time.map_or(None, |f| {
            chrono::DateTime::parse_from_rfc3339(&f)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        });
        let end_time = data_end_time.map_or(None, |f| {
            chrono::DateTime::parse_from_rfc3339(&f)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc))
        });
        loop {
            match self.read_two_mixed_packets() {
                // treat second_packet as timestamp packet
                Ok((first_packet, second_packet)) => {
                    packet_count += 2;
                    // Check if the packet timestamp is outside the specified range
                    if let Some(ts) = second_packet.timestamp {
                        if let Some(start) = start_time {
                            if ts < start {
                                continue; // Skip this packet
                            }
                        }
                        if let Some(end) = end_time {
                            if ts > end {
                                end_flag = true;
                                if_some!(w in writer, {
                                    w.writeln(&format!("Reached end time limit at packet #{}: {}", packet_count, ts))?;
                                });
                                break; // Stop reading further packets
                            }
                        }
                    }
                    let two_packets = vec![first_packet, second_packet];
                    for packet in two_packets {
                        // Update statistics if we have a data pack
                        if let Some(data_pack) = &packet.data_pack {
                            stats.update_from_packet(data_pack);
                        }
                        stats.update_from_raw_data(&packet.raw_data);

                        let (format_desc, length, timestamp) = match &packet.format {
                            MixedPacketFormat::ProtobufFormat { length, unused } => (
                                format!(
                                    "Mixed protobuf format (int16 length {} + int8 unused 0x{:02x} + protobuf data)",
                                    length, unused
                                ),
                                *length,
                                None,
                            ),
                            MixedPacketFormat::TimestampFormat { length, timestamp } => (
                                format!(
                                    "Mixed timestamp format (int8 length {} + int64 timestamp)",
                                    length
                                ),
                                *length as u16,
                                Some(*timestamp),
                            ),
                        };
                        if_some!(w in writer, {
                            format_packet_unified(
                                w,
                                packet_count,
                                length,
                                timestamp,
                                packet.data_pack.as_ref(),
                                &packet.raw_data,
                                &vec![],
                                &format_desc
                            )?;
                        });
                        packets.push(packet);
                    }
                    // Check if we've reached the limit
                    if packet_count >= limit {
                        if_some!(w in writer, {
                            w.writeln(&format!("Reached requested packet limit of {} packets", limit))?;
                        });
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();

                    // Check for various EOF-related conditions
                    let is_eof = error_msg.contains("eof")
                        || error_msg.contains("unexpectedeof")
                        || error_msg.contains("failed to fill whole buffer")
                        || error_msg.contains("failed to read first byte")
                        || error_msg.contains("failed to read second byte")
                        || error_msg.contains("unexpected end of file");

                    // Also check the error chain for IO errors
                    let mut current_error: &dyn std::error::Error = e.as_ref();
                    let mut has_io_error = false;
                    while let Some(source) = current_error.source() {
                        if let Some(io_err) = source.downcast_ref::<std::io::Error>() {
                            if matches!(io_err.kind(), std::io::ErrorKind::UnexpectedEof) {
                                has_io_error = true;
                                break;
                            }
                        }
                        current_error = source;
                    }

                    if is_eof || has_io_error {
                        if_some!(w in writer, {
                            w.writeln(&format!("Reached end of file after {} packets (requested: {})", packets.len(), limit))?;
                        });
                        break;
                    }

                    if_some!(w in writer, {
                        // Write detailed error information
                        w.writeln(&format!("=== MIXED PACKET PARSING ERROR #{} ===", packet_count + 1))?;
                        w.writeln(&format!("Error: {}", e))?;

                    // Try to show current file position and nearby bytes for debugging
                    if let Ok(current_pos) = self.reader.stream_position() {
                        w.writeln(&format!("Current file position: 0x{:x} ({})", current_pos, current_pos))?;

                        // Try to read some bytes around the current position for debugging
                        let mut debug_buffer = [0u8; 64]; // More bytes for mixed format
                        if current_pos >= 32 {
                            if self.reader.seek(std::io::SeekFrom::Start(current_pos - 32)).is_ok() {
                                if let Ok(bytes_read) = self.reader.read(&mut debug_buffer) {
                                    let debug_hex = debug_buffer[..bytes_read]
                                        .iter()
                                        .enumerate()
                                        .map(|(i, b)| {
                                            if i == 32 { format!("[{:02x}]", b) } // Mark current position
                                            else { format!("{:02x}", b) }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    w.writeln(&format!("Context bytes (32 before + 32 after): {}", debug_hex))?;
                                }
                            }
                        } else {
                            // If we're near the beginning, just show from start
                            if self.reader.seek(std::io::SeekFrom::Start(0)).is_ok() {
                                if let Ok(bytes_read) = self.reader.read(&mut debug_buffer) {
                                    let debug_hex = debug_buffer[..bytes_read]
                                        .iter()
                                        .enumerate()
                                        .map(|(i, b)| {
                                            if i as u64 == current_pos { format!("[{:02x}]", b) }
                                            else { format!("{:02x}", b) }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    w.writeln(&format!("Context bytes from file start: {}", debug_hex))?;
                                }
                            }
                        }
                    }

                    w.writeln("======================================")?;
                    });
                    return Err(e.context(format!("Failed at mixed packet #{}", packet_count + 1)));
                }
            }
        }

        if_some!(w in writer, {
            // Display statistics at the end
            format_statistics(w, &stats)?;
        });
        Ok((packets, end_flag))
    }

    fn read_mixed_packet(&mut self) -> Result<MixedPacketInfo> {
        // Try to peek first byte to determine format
        let mut peek_buf = [0u8; 1];
        self.reader
            .read_exact(&mut peek_buf)
            .with_context(|| "Failed to read first byte")?;

        // Read the second byte to help determine the format
        let mut second_buf = [0u8; 1];
        self.reader
            .read_exact(&mut second_buf)
            .with_context(|| "Failed to read second byte")?;
        let length = u16::from_be_bytes([peek_buf[0], second_buf[0]]);
        match self.state {
            MixedPacketReaderState::Packet => {
                // If we are in packet state, we expect a protobuf format
                if length < 3 {
                    return Err(anyhow!(
                        "Invalid mixed packet length: {}, must be at least 3",
                        length
                    ));
                }
                let mut unused_buf = [0u8; 1];
                self.reader
                    .read_exact(&mut unused_buf)
                    .with_context(|| "Failed to read unused byte")?;
                self.read_protobuf_format_packet(length, unused_buf[0])
            }
            MixedPacketReaderState::Timestamp => {
                // If we are in timestamp state, we expect a timestamp format
                let mut micro_timestamp_buf = [0u8; 8];
                self.reader
                    .read_exact(&mut micro_timestamp_buf)
                    .with_context(|| "Failed to read micro timestamp")?;
                let timestamp = chrono::DateTime::from_timestamp_micros(i64::from_be_bytes(
                    micro_timestamp_buf,
                ))
                .with_context(|| "Failed to convert micro timestamp")?;
                self.state = MixedPacketReaderState::Packet;
                Ok(MixedPacketInfo {
                    format: MixedPacketFormat::TimestampFormat { length, timestamp },
                    timestamp: Some(timestamp),
                    data_pack: None,
                    raw_data: micro_timestamp_buf.to_vec(),
                })
            }
        }
    }

    fn read_two_mixed_packets(&mut self) -> Result<(MixedPacketInfo, MixedPacketInfo)> {
        let first_packet = self.read_mixed_packet()?;
        let second_packet = self.read_mixed_packet()?;
        Ok((first_packet, second_packet))
    }

    fn read_protobuf_format_packet(&mut self, length: u16, unused: u8) -> Result<MixedPacketInfo> {
        if length < 3 {
            return Err(anyhow!(
                "Invalid protobuf packet length: {}, must be at least 3",
                length
            ));
        }

        let data_length = length - 1; // unused(1)
        let mut data = vec![0u8; data_length as usize];
        self.reader
            .read_exact(&mut data)
            .with_context(|| format!("Failed to read protobuf data of length {}", data_length))?;

        // Try to parse as protobuf
        let data_pack = match DataPack::decode(data.as_slice()) {
            Ok(pack) => Some(pack),
            Err(e) => {
                println!("  Warning: Failed to decode protobuf data: {}", e);
                None
            }
        };
        self.state = MixedPacketReaderState::Timestamp;
        Ok(MixedPacketInfo {
            format: MixedPacketFormat::ProtobufFormat { length, unused },
            timestamp: None,
            data_pack,
            raw_data: data,
        })
    }
}

pub fn format_packet_info(packet: &PacketInfo) -> String {
    let mut output = String::new();

    output.push_str(&format!("=== Packet ===\n"));
    output.push_str(&format!("Length: {} bytes\n", packet.len()));
    output.push_str(&format!(
        "Timestamp: {} ({})\n",
        packet.timestamp.format("%Y-%m-%d %H:%M:%S%.6f UTC"),
        packet.timestamp.timestamp_micros()
    ));

    output.push_str(&format!("Data Pack:\n"));
    output.push_str(&format_data_pack(&packet.data_pack, 1));

    output
}

fn format_data_pack(data_pack: &DataPack, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    let mut output = String::new();

    if let Some(control) = &data_pack.control {
        output.push_str(&format!("{}Control: ", indent));
        match control {
            proto::als::data_pack::Control::Data(data) => {
                output.push_str(&format!("Data({})\n", data));
            }
            proto::als::data_pack::Control::Pong(pong) => {
                output.push_str(&format!("Pong({})\n", pong));
            }
            proto::als::data_pack::Control::SegmentStartedAt(timestamp) => {
                output.push_str(&format!("SegmentStartedAt({})\n", timestamp));
            }
            proto::als::data_pack::Control::CacheEnded(ended) => {
                output.push_str(&format!("CacheEnded({})\n", ended));
            }
        }
    }

    if !data_pack.frames.is_empty() {
        output.push_str(&format!("{}Frames ({}):\n", indent, data_pack.frames.len()));
        for (i, frame) in data_pack.frames.iter().enumerate() {
            output.push_str(&format!("{}  Frame {}:\n", indent, i));
            output.push_str(&format_data_frame(frame, indent_level + 2));
        }
    }

    output
}

fn format_data_frame(frame: &proto::als::DataFrame, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    let mut output = String::new();

    if let Some(message) = &frame.message {
        use proto::als::data_frame::Message;

        output.push_str(&format!("{}Message: ", indent));
        match message {
            Message::InstantiateObject(obj) => {
                let target_desc = format_instantiate_object_target(&obj.target);
                output.push_str(&format!(
                    "InstantiateObject(object_id: {}, owner: {:?}, prefab: {:?}, target: {})\n",
                    obj.object_id,
                    String::from_utf8_lossy(&obj.owner_id),
                    String::from_utf8_lossy(&obj.prefab_name),
                    target_desc
                ));
            }
            Message::UpdateObject(obj) => {
                let target_desc = format_update_object_target(&obj.target);
                output.push_str(&format!(
                    "UpdateObject(object_id: {}, method: {}, target: {})\n",
                    obj.object_id, obj.method, target_desc
                ));
            }
            Message::DestroyObject(obj) => {
                output.push_str(&format!("DestroyObject(object_id: {})\n", obj.object_id));
            }
            Message::Room(room) => {
                output.push_str(&format!(
                    "Room(id: {:?}, started: {}, ended: {})\n",
                    String::from_utf8_lossy(&room.id),
                    room.started_at,
                    room.ended_at
                ));
            }
            Message::AuthorizeResponse(resp) => {
                output.push_str(&format!(
                    "AuthorizeResponse(player_id: {:?}, role: {})\n",
                    String::from_utf8_lossy(&resp.player_id),
                    resp.role
                ));
            }
            Message::JoinRoomResponse(resp) => {
                if let Some(room) = &resp.room {
                    output.push_str(&format!(
                        "JoinRoomResponse(room_id: {:?}, joined_at: {})\n",
                        String::from_utf8_lossy(&room.id),
                        resp.joined_at
                    ));
                } else {
                    output.push_str(&format!(
                        "JoinRoomResponse(joined_at: {})\n",
                        resp.joined_at
                    ));
                }
            }
        }
    }

    output
}

pub struct OutputWriter {
    writer: Box<dyn Write>,
}

impl OutputWriter {
    pub fn new(output_path: Option<&str>) -> Result<Self> {
        let writer: Box<dyn Write> = match output_path {
            Some(path) => Box::new(
                File::create(path)
                    .with_context(|| format!("Failed to create output file: {}", path))?,
            ),
            None => Box::new(std::io::stdout()),
        };

        Ok(Self { writer })
    }

    pub fn writeln(&mut self, content: &str) -> Result<()> {
        writeln!(self.writer, "{}", content).with_context(|| "Failed to write to output")?;
        Ok(())
    }

    pub fn write(&mut self, content: &str) -> Result<()> {
        write!(self.writer, "{}", content).with_context(|| "Failed to write to output")?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer
            .flush()
            .with_context(|| "Failed to flush output")?;
        Ok(())
    }
}

pub fn analyze_binary_file_with_output_and_count(
    file_path: &str,
    output_path: Option<&str>,
    packet_count: usize,
    file_count_limit: usize,
    file_size_limit: usize,
    data_start_time: Option<String>,
    data_end_time: Option<String>,
) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(file_path);
    let mut writer = OutputWriter::new(output_path)?;

    if path.is_dir() {
        // Directory processing: analyze all files sorted by creation time
        let mut entries: Vec<_> = fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {}", file_path))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_path = entry.path();

                // Only process files (not subdirectories)
                if !file_path.is_file() {
                    return None;
                }

                // Get file metadata for creation time
                let metadata = file_path.metadata().ok()?;
                let created = metadata.created().or_else(|_| metadata.modified()).ok()?;

                Some((file_path, created))
            })
            .collect();

        // Sort by creation time (oldest first)
        entries.sort_by_key(|(_, created)| *created);

        if entries.is_empty() {
            return Err(anyhow!("No files found in directory: {}", file_path));
        }

        if entries.len() > file_count_limit {
            writer.writeln(&format!(
                "Warning: Directory contains {} files, exceeding the limit of {}. Only processing the first {} files.",
                entries.len(),
                file_count_limit,
                file_count_limit
            ))?;
            entries.truncate(file_count_limit);
        }

        // Write header for directory processing
        writer.writeln(&format!("=== ALS Standard Analysis Results ==="))?;
        writer.writeln(&format!("Directory: {}", file_path))?;
        writer.writeln(&format!("Packet Count per File: {}", packet_count))?;
        writer.writeln(&format!("Total Files: {}", entries.len()))?;
        writer.writeln(&format!(
            "Started at: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ))?;
        writer.writeln("=======================================")?;
        writer.writeln("")?;

        // Combined statistics across all files
        let mut combined_stats = PacketAnalysisStats::default();
        let mut files_processed = 0;
        let mut total_files_with_packets = 0;

        // Process each file
        for (index, (file_path, _created_time)) in entries.iter().enumerate() {
            let file_path_str = file_path.to_string_lossy();

            writer.writeln(&format!(
                "--- File {}/{}: {} ---",
                index + 1,
                entries.len(),
                file_path_str
            ))?;
            writer.flush()?; // Ensure immediate output

            match analyze_single_standard_file(
                &file_path_str,
                packet_count,
                &mut writer,
                data_start_time.clone(),
                data_end_time.clone(),
            ) {
                Ok((stats, end_flag)) => {
                    combined_stats.merge_with(stats);
                    total_files_with_packets += 1;
                    if end_flag {
                        writer.writeln("End time reached, stopping further processing.")?;
                        break;
                    }
                }
                Err(e) => {
                    writer.writeln(&format!("❌ Error analyzing file: {}", e))?;
                }
            }

            files_processed += 1;
            writer.writeln("")?; // Separator between files
            writer.flush()?; // Ensure immediate output

            // Check file size limit
            let metadata = file_path
                .metadata()
                .with_context(|| format!("Failed to read metadata for file: {}", file_path_str))?;
            if metadata.len() as usize > file_size_limit {
                writer.writeln(&format!(
                    "Warning: File size {} bytes exceeds the limit of {} bytes. Stopping further processing.",
                    metadata.len(),
                    file_size_limit
                ))?;
                break;
            }
        }

        // Write summary statistics
        writer.writeln("=======================================")?;
        writer.writeln("=== BATCH ANALYSIS SUMMARY ===")?;
        writer.writeln(&format!("Files processed: {}", files_processed))?;
        writer.writeln(&format!(
            "Files with valid packets: {}",
            total_files_with_packets
        ))?;
        writer.writeln("")?;

        if combined_stats.total_packets > 0 {
            format_statistics(&mut writer, &combined_stats)?;
        } else {
            writer.writeln("No valid packets found across all files.")?;
        }

        writer.writeln(&format!(
            "Completed at: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ))?;
        writer.writeln("=======================================")?;
        writer.flush()?;
    } else {
        // Single file processing (original logic)
        let mut file =
            File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
        let metadata = file
            .metadata()
            .with_context(|| format!("Failed to read file metadata: {}", file_path))?;

        writer.writeln(&format!("Analyzing binary file: {}", file_path))?;
        writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
        writer.writeln(&format!("Analyzing first {} packets", packet_count))?;

        // Debug: show first 32 bytes of the file
        let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
        file.read_exact(&mut debug_buffer)?;
        writer.writeln(&format!(
            "First {} bytes (hex): {}",
            debug_buffer.len(),
            debug_buffer
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        ))?;

        // Reset file position
        file.seek(std::io::SeekFrom::Start(0))?;

        writer.writeln("===========================================")?;
        writer.writeln("")?;

        let mut reader = ProtoPacketReader::new(file);

        let packets =
            match reader.read_packets_with_limit_and_writer(packet_count, Some(&mut writer)) {
                Ok(packets) => packets,
                Err(e) => {
                    return Err(anyhow!(
                        "Failed to read packets from file '{}': {}",
                        file_path,
                        e
                    ));
                }
            };

        writer.writeln(&format!("Total packets found: {}", packets.len()))?;
        writer.flush()?;
    }

    Ok(())
}

fn analyze_single_standard_file(
    file_path: &str,
    packet_count: usize,
    writer: &mut OutputWriter,
    data_start_time: Option<String>,
    data_end_time: Option<String>,
) -> Result<(PacketAnalysisStats, bool)> {
    let mut end_flag = false;
    let file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;

    let metadata = file
        .metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;

    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;

    let mut reader = ProtoPacketReader::new(file);
    let packets = reader.read_packets_with_limit_and_writer(packet_count, Some(writer))?;

    // Calculate statistics from standard packets
    let mut stats = PacketAnalysisStats::default();
    let start_time = data_start_time.map_or(None, |t| {
        DateTime::parse_from_rfc3339(&t)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });
    let end_time = data_end_time.map_or(None, |t| {
        DateTime::parse_from_rfc3339(&t)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    });
    for packet in &packets {
        // Check time range filtering
        if let Some(start) = start_time {
            if packet.timestamp < start {
                continue; // Skip packets before start time
            }
        }
        if let Some(end) = end_time {
            if packet.timestamp > end {
                end_flag = true; // Indicate we reached the end time
                break; // Stop processing further packets
            }
        }
        stats.update_from_packet(&packet.data_pack);
        stats.update_from_raw_data(&packet.raw_data);
    }
    Ok((stats, end_flag))
}

pub fn analyze_mixed_binary_file_with_output_and_count(
    file_path: &str,
    output_path: Option<&str>,
    packet_count: usize,
    file_count_limit: usize,
    file_size_limit: usize,
    data_start_time: Option<String>,
    data_end_time: Option<String>,
) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(file_path);
    let mut writer = OutputWriter::new(output_path)?;

    if path.is_dir() {
        // Directory processing: analyze all files sorted by creation time
        let mut entries: Vec<_> = fs::read_dir(path)
            .with_context(|| format!("Failed to read directory: {}", file_path))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let file_path = entry.path();

                // Only process files (not subdirectories)
                if !file_path.is_file() {
                    return None;
                }

                // Get file metadata for creation time
                let metadata = file_path.metadata().ok()?;
                let created = metadata.created().or_else(|_| metadata.modified()).ok()?;

                Some((file_path, created))
            })
            .collect();

        // Sort by creation time (oldest first)
        entries.sort_by_key(|(_, created)| *created);

        if entries.is_empty() {
            return Err(anyhow!("No files found in directory: {}", file_path));
        }

        if entries.len() > file_count_limit {
            writer.writeln(&format!(
                "Warning: Directory contains {} files, exceeding the limit of {}. Only processing the first {} files.",
                entries.len(),
                file_count_limit,
                file_count_limit
            ))?;
            entries.truncate(file_count_limit);
        }

        // Write header for directory processing
        writer.writeln(&format!("=== ALS Mixed Analysis Results ==="))?;
        writer.writeln(&format!("Directory: {}", file_path))?;
        writer.writeln(&format!("Packet Count per File: {}", packet_count))?;
        writer.writeln(&format!("Total Files: {}", entries.len()))?;
        writer.writeln(&format!(
            "Started at: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ))?;
        writer.writeln("=======================================")?;
        writer.writeln("")?;

        // Combined statistics across all files
        let mut combined_stats = PacketAnalysisStats::default();
        let mut files_processed = 0;
        let mut total_files_with_packets = 0;
        let mut total_protobuf_count = 0;
        let mut total_timestamp_count = 0;

        // Process each file
        for (index, (file_path, _created_time)) in entries.iter().enumerate() {
            let file_path_str = file_path.to_string_lossy();

            writer.writeln(&format!(
                "--- File {}/{}: {} ---",
                index + 1,
                entries.len(),
                file_path_str
            ))?;
            writer.flush()?; // Ensure immediate output

            match analyze_single_mixed_file(
                &file_path_str,
                packet_count,
                &mut writer,
                data_start_time.clone(),
                data_end_time.clone(),
            ) {
                Ok((stats, protobuf_count, timestamp_count, end_flag)) => {
                    combined_stats.merge_with(stats);
                    total_protobuf_count += protobuf_count;
                    total_timestamp_count += timestamp_count;
                    total_files_with_packets += 1;

                    if end_flag {
                        writer.writeln("End time reached, stopping further processing.")?;
                        break;
                    }
                }
                Err(e) => {
                    writer.writeln(&format!("❌ Error analyzing file: {}", e))?;
                }
            }

            files_processed += 1;
            writer.writeln("")?; // Separator between files
            writer.flush()?; // Ensure immediate output

            // Check file size limit
            let metadata = file_path
                .metadata()
                .with_context(|| format!("Failed to read metadata for file: {}", file_path_str))?;
            if metadata.len() as usize > file_size_limit {
                writer.writeln(&format!(
                    "Warning: File size {} bytes exceeds the limit of {} bytes. Stopping further processing.",
                    metadata.len(),
                    file_size_limit
                ))?;
                break;
            }
        }

        // Write summary statistics
        writer.writeln("=======================================")?;
        writer.writeln("=== BATCH ANALYSIS SUMMARY ===")?;
        writer.writeln(&format!("Files processed: {}", files_processed))?;
        writer.writeln(&format!(
            "Files with valid packets: {}",
            total_files_with_packets
        ))?;
        writer.writeln("")?;

        writer.writeln("Combined Format Breakdown:")?;
        writer.writeln(&format!(
            "  Total Protobuf format packets: {}",
            total_protobuf_count
        ))?;
        writer.writeln(&format!(
            "  Total Timestamp format packets: {}",
            total_timestamp_count
        ))?;
        writer.writeln("")?;

        if combined_stats.total_packets > 0 {
            format_statistics(&mut writer, &combined_stats)?;
        } else {
            writer.writeln("No valid packets found across all files.")?;
        }

        writer.writeln(&format!(
            "Completed at: {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ))?;
        writer.writeln("=======================================")?;
        writer.flush()?;
    } else {
        // Single file processing (original logic)
        let mut file =
            File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;
        let metadata = file
            .metadata()
            .with_context(|| format!("Failed to read file metadata: {}", file_path))?;

        writer.writeln(&format!(
            "Analyzing mixed format binary file: {}",
            file_path
        ))?;
        writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
        writer.writeln(&format!("Analyzing first {} packets", packet_count))?;

        // Debug: show first 32 bytes of the file
        let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
        file.read_exact(&mut debug_buffer)?;
        writer.writeln(&format!(
            "First {} bytes (hex): {}",
            debug_buffer.len(),
            debug_buffer
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        ))?;

        // Reset file position
        file.seek(std::io::SeekFrom::Start(0))?;

        writer.writeln("===========================================")?;
        writer.writeln("")?;

        let mut reader = MixedPacketReader::new(file);

        let (packets, _end_flag) = match reader.read_mixed_packets_with_limit_and_writer(
            packet_count,
            Some(&mut writer),
            data_start_time,
            data_end_time,
        ) {
            Ok(packets) => packets,
            Err(e) => {
                return Err(anyhow!(
                    "Failed to read mixed packets from file '{}': {}",
                    file_path,
                    e
                ));
            }
        };

        writer.writeln(&format!("Total mixed packets found: {}", packets.len()))?;

        // Format breakdown summary
        let mut protobuf_count = 0;
        let mut timestamp_count = 0;

        for packet in &packets {
            match packet.format {
                MixedPacketFormat::ProtobufFormat { .. } => protobuf_count += 1,
                MixedPacketFormat::TimestampFormat { .. } => timestamp_count += 1,
            }
        }

        writer.writeln("")?;
        writer.writeln("Format Breakdown:")?;
        writer.writeln(&format!("  Protobuf format packets: {}", protobuf_count))?;
        writer.writeln(&format!("  Timestamp format packets: {}", timestamp_count))?;
        writer.flush()?;
    }

    Ok(())
}

fn analyze_single_mixed_file(
    file_path: &str,
    packet_count: usize,
    writer: &mut OutputWriter,
    data_start_time: Option<String>,
    data_end_time: Option<String>,
) -> Result<(PacketAnalysisStats, u32, u32, bool)> {
    let file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;

    let metadata = file
        .metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;

    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;

    let mut reader = MixedPacketReader::new(file);
    let (packets, end_flag) = reader.read_mixed_packets_with_limit_and_writer(
        packet_count,
        Some(writer),
        data_start_time,
        data_end_time,
    )?;

    // Calculate statistics from mixed packets
    let mut stats = PacketAnalysisStats::default();
    let mut protobuf_count = 0;
    let mut timestamp_count = 0;
    for packet in &packets {
        if let Some(data_pack) = &packet.data_pack {
            stats.update_from_packet(data_pack);
        }
        stats.update_from_raw_data(&packet.raw_data);

        // Count format types
        match packet.format {
            MixedPacketFormat::ProtobufFormat { .. } => protobuf_count += 1,
            MixedPacketFormat::TimestampFormat { .. } => timestamp_count += 1,
        }
    }

    Ok((stats, protobuf_count, timestamp_count, end_flag))
}
