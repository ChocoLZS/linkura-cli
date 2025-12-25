//! Analysis layer - responsible for statistics and data processing
//! No I/O operations, pure business logic

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::define::{DataFrame, data_frame, data_pack};
use crate::als::proto::PacketInfo;

/// Main analyzer for packet statistics
#[derive(Debug, Default, Clone)]
pub struct PacketAnalyzer {
    stats: PacketStats,
}

impl PacketAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a single packet
    pub fn analyze_packet(&mut self, packet: &PacketInfo) {
        self.stats.total_packets += 1;

        // Analyze data pack
        if let Some(control) = &packet.data_pack.control {
            self.stats.packets_with_control += 1;
            self.analyze_control(control);
        }

        if !packet.data_pack.frames.is_empty() {
            self.stats.packets_with_frames += 1;
            self.stats.total_frames += packet.data_pack.frames.len() as u32;
            for frame in &packet.data_pack.frames {
                self.analyze_frame(frame);
            }
        }

        // Analyze unknown fields
        self.analyze_unknown_fields(&packet.raw_data);
    }

    /// Analyze multiple packets
    pub fn analyze_batch(&mut self, packets: &[PacketInfo]) {
        for packet in packets {
            self.analyze_packet(packet);
        }
    }

    /// Get current statistics
    pub fn stats(&self) -> &PacketStats {
        &self.stats
    }

    /// Merge another analyzer's stats into this one
    pub fn merge(&mut self, other: &PacketAnalyzer) {
        self.stats.merge(&other.stats);
    }

    fn analyze_control(&mut self, control: &data_pack::Control) {
        match control {
            data_pack::Control::Data(_) => self.stats.control.data_count += 1,
            data_pack::Control::Pong(_) => self.stats.control.pong_count += 1,
            data_pack::Control::SegmentStartedAt(_) => {
                self.stats.control.segment_started_at_count += 1
            }
            data_pack::Control::CacheEnded(_) => self.stats.control.cache_ended_count += 1,
        }
        self.stats.control.total += 1;
    }

    fn analyze_frame(&mut self, frame: &DataFrame) {
        if let Some(message) = &frame.message {
            match message {
                data_frame::Message::InstantiateObject(_) => {
                    self.stats.frames.instantiate_object_count += 1
                }
                data_frame::Message::UpdateObject(_) => self.stats.frames.update_object_count += 1,
                data_frame::Message::DestroyObject(_) => {
                    self.stats.frames.destroy_object_count += 1
                }
                data_frame::Message::Room(_) => self.stats.frames.room_count += 1,
                data_frame::Message::AuthorizeResponse(_) => {
                    self.stats.frames.authorize_response_count += 1
                }
                data_frame::Message::JoinRoomResponse(_) => {
                    self.stats.frames.join_room_response_count += 1
                }
            }
            self.stats.frames.total += 1;
        }
    }

    fn analyze_unknown_fields(&mut self, raw_data: &[u8]) {
        let fields = parse_protobuf_fields(raw_data);
        for field in fields {
            if !is_known_field_number(field.field_number) {
                *self
                    .stats
                    .unknown_fields
                    .entry(field.field_number)
                    .or_insert(0) += 1;
            }
        }
    }
}

/// Complete packet statistics
#[derive(Debug, Default, Clone)]
pub struct PacketStats {
    pub total_packets: u32,
    pub packets_with_control: u32,
    pub packets_with_frames: u32,
    pub total_frames: u32,
    pub control: ControlStats,
    pub frames: FrameStats,
    pub unknown_fields: HashMap<u32, u32>,
}

impl PacketStats {
    pub fn merge(&mut self, other: &PacketStats) {
        self.total_packets += other.total_packets;
        self.packets_with_control += other.packets_with_control;
        self.packets_with_frames += other.packets_with_frames;
        self.total_frames += other.total_frames;
        self.control.merge(&other.control);
        self.frames.merge(&other.frames);

        for (field_num, count) in &other.unknown_fields {
            *self.unknown_fields.entry(*field_num).or_insert(0) += count;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ControlStats {
    pub data_count: u32,
    pub pong_count: u32,
    pub segment_started_at_count: u32,
    pub cache_ended_count: u32,
    pub total: u32,
}

impl ControlStats {
    pub fn merge(&mut self, other: &ControlStats) {
        self.data_count += other.data_count;
        self.pong_count += other.pong_count;
        self.segment_started_at_count += other.segment_started_at_count;
        self.cache_ended_count += other.cache_ended_count;
        self.total += other.total;
    }
}

#[derive(Debug, Default, Clone)]
pub struct FrameStats {
    pub instantiate_object_count: u32,
    pub update_object_count: u32,
    pub destroy_object_count: u32,
    pub room_count: u32,
    pub authorize_response_count: u32,
    pub join_room_response_count: u32,
    pub total: u32,
}

impl FrameStats {
    pub fn merge(&mut self, other: &FrameStats) {
        self.instantiate_object_count += other.instantiate_object_count;
        self.update_object_count += other.update_object_count;
        self.destroy_object_count += other.destroy_object_count;
        self.room_count += other.room_count;
        self.authorize_response_count += other.authorize_response_count;
        self.join_room_response_count += other.join_room_response_count;
        self.total += other.total;
    }
}

/// Time-based packet filter
pub struct PacketFilter {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl PacketFilter {
    pub fn new(start: Option<String>, end: Option<String>) -> Self {
        Self {
            start_time: start.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
            end_time: end.and_then(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }),
        }
    }

    /// Check if a packet should be included
    pub fn should_include(&self, timestamp: &DateTime<Utc>) -> bool {
        if let Some(start) = &self.start_time {
            if timestamp < start {
                return false;
            }
        }
        if let Some(end) = &self.end_time {
            if timestamp > end {
                return false;
            }
        }
        true
    }

    /// Check if we've passed the end time
    pub fn is_past_end(&self, timestamp: &DateTime<Utc>) -> bool {
        if let Some(end) = &self.end_time {
            timestamp > end
        } else {
            false
        }
    }
}

// Helper structures for protobuf field parsing
#[derive(Debug)]
struct ProtobufField {
    field_number: u32,
    #[allow(unused)]
    wire_type: u8,
}

fn parse_protobuf_fields(data: &[u8]) -> Vec<ProtobufField> {
    let mut fields = Vec::new();
    let mut cursor = std::io::Cursor::new(data);

    while cursor.position() < data.len() as u64 {
        if let Ok(field) = parse_field(&mut cursor) {
            fields.push(field);
        } else {
            break;
        }
    }

    fields
}

fn parse_field(cursor: &mut std::io::Cursor<&[u8]>) -> anyhow::Result<ProtobufField> {
    use prost::bytes::Buf;

    let tag = read_varint(cursor)?;
    let field_number = (tag >> 3) as u32;
    let wire_type = (tag & 0x7) as u8;

    // Skip field data based on wire type
    match wire_type {
        0 => {
            read_varint(cursor)?;
        }
        1 => {
            if cursor.remaining() < 8 {
                return Err(anyhow::anyhow!("Not enough bytes"));
            }
            cursor.advance(8);
        }
        2 => {
            let len = read_varint(cursor)?;
            if cursor.remaining() < len as usize {
                return Err(anyhow::anyhow!("Not enough bytes"));
            }
            cursor.advance(len as usize);
        }
        5 => {
            if cursor.remaining() < 4 {
                return Err(anyhow::anyhow!("Not enough bytes"));
            }
            cursor.advance(4);
        }
        _ => return Err(anyhow::anyhow!("Unsupported wire type: {}", wire_type)),
    }

    Ok(ProtobufField {
        field_number,
        wire_type,
    })
}

fn read_varint(cursor: &mut std::io::Cursor<&[u8]>) -> anyhow::Result<u64> {
    use prost::bytes::Buf;

    let mut result = 0u64;
    let mut shift = 0;

    loop {
        if cursor.remaining() == 0 {
            return Err(anyhow::anyhow!("Unexpected end of data"));
        }

        let byte = cursor.get_u8();
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            break;
        }

        shift += 7;
        if shift >= 64 {
            return Err(anyhow::anyhow!("Varint too long"));
        }
    }

    Ok(result)
}

fn is_known_field_number(field_number: u32) -> bool {
    matches!(
        field_number,
        1 | 2 | 3 | 4 | 6 | 7 | 8 | 9 | 10 | 11 | 14 | 15 | 16 | 128 | 129 | 130 | 143 | 144 | 147
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_filter() {
        let filter = PacketFilter::new(
            Some("2025-01-01T00:00:00Z".to_string()),
            Some("2025-12-31T23:59:59Z".to_string()),
        );

        let ts = DateTime::parse_from_rfc3339("2025-06-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert!(filter.should_include(&ts));
    }

    #[test]
    fn test_stats_merge() {
        let mut stats1 = PacketStats::default();
        stats1.total_packets = 10;

        let mut stats2 = PacketStats::default();
        stats2.total_packets = 20;

        stats1.merge(&stats2);
        assert_eq!(stats1.total_packets, 30);
    }
}
