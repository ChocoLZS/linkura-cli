//! Presentation layer - responsible for formatting and output
//! All formatting logic in one place

use anyhow::{Context, Result};
use prost::Message;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

use super::analyzer::PacketStats;
use super::define::{DataPack, data_frame, data_pack, instantiate_object, update_object};
use crate::als::proto::PacketInfo;
use crate::als::proto::define::UpdateObject;
use crate::als::proto::extension::UpdateObjectExt;

impl Display for instantiate_object::Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            instantiate_object::Target::CurrentPlayer(_) => write!(f, "CurrentPlayer"),
            instantiate_object::Target::RoomAll(room) => write!(
                f,
                "RoomAll(room_id: {})",
                String::from_utf8_lossy(room.room_id.as_slice())
            ),
            instantiate_object::Target::PlayerId(player) => write!(
                f,
                "PlayerId(player_id: {})",
                String::from_utf8_lossy(player)
            ),
        }
    }
}
impl Display for update_object::Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            update_object::Target::CurrentPlayer(_) => write!(f, "CurrentPlayer"),
            update_object::Target::RoomAll(room) => write!(
                f,
                "RoomAll(room_id: {})",
                String::from_utf8_lossy(room.room_id.as_slice())
            ),
            update_object::Target::PlayerId(player) => write!(
                f,
                "PlayerId(player_id: {})",
                String::from_utf8_lossy(player)
            ),
        }
    }
}
/// Output writer abstraction
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

/// Packet formatter
pub struct PacketFormatter<'a> {
    objects_map: &'a mut HashMap<i32, String>, // object_id to prefab_name mapping
}

impl<'a> PacketFormatter<'a> {
    /// Create a new PacketFormatter
    pub fn new(objects_map: &'a mut HashMap<i32, String>) -> Self {
        Self { objects_map }
    }

    /// Format a single packet with full details
    pub fn format_packet(
        &mut self,
        writer: &mut OutputWriter,
        packet_number: usize,
        packet: &PacketInfo,
    ) -> Result<()> {
        writer.writeln(&format!(
            "=== Packet #{}: {} bytes ===",
            packet_number,
            packet.len()
        ))?;

        let timestamp_micros = packet.timestamp.timestamp_micros() as u64;
        writer.writeln(&format!(
            "  Timestamp: {} ({} / 0x{:x})",
            packet.timestamp.format("%Y-%m-%d %H:%M:%S%.6f UTC"),
            timestamp_micros,
            timestamp_micros
        ))?;

        // Show protobuf digest
        let protobuf_digest = super::calculate_digest(&packet.raw_data);
        writer.writeln(&format!("  Protobuf SHA-256: {}", protobuf_digest))?;

        // Show first bytes in hex
        let debug_len = 32.min(packet.raw_data.len());
        writer.writeln(&format!(
            "  Raw data (first {} bytes): {}",
            debug_len,
            hex_string(&packet.raw_data[..debug_len])
        ))?;

        self.format_data_pack(writer, &packet.data_pack)?;
        writer.writeln("")?;

        Ok(())
    }

    /// Format DataPack details
    fn format_data_pack(&mut self, writer: &mut OutputWriter, data_pack: &DataPack) -> Result<()> {
        // Control message
        if let Some(control) = &data_pack.control {
            writer.writeln("  Control message:")?;
            match control {
                data_pack::Control::Data(data) => {
                    writer.writeln(&format!("    Type: Data, Value: {}", data))?;
                }
                data_pack::Control::Pong(pong) => {
                    writer.writeln(&format!("    Type: Pong, Value: {}", pong))?;
                }
                data_pack::Control::SegmentStartedAt(ts) => {
                    writer.writeln(&format!("    Type: SegmentStartedAt, Timestamp: {}", ts))?;
                }
                data_pack::Control::CacheEnded(ended) => {
                    writer.writeln(&format!("    Type: CacheEnded, Value: {}", ended))?;
                }
            }
        } else {
            writer.writeln("  No control message")?;
        }

        // Frames
        if !data_pack.frames.is_empty() {
            writer.writeln(&format!("  Frames ({}):", data_pack.frames.len()))?;
            for (i, frame) in data_pack.frames.iter().enumerate() {
                writer.writeln(&format!("    Frame #{}:", i + 1))?;
                // print sha-256 for frame
                let frame_digest = super::calculate_digest(&frame.encode_to_vec());
                writer.writeln(&format!("      SHA-256: {}", frame_digest))?;
                if let Some(message) = &frame.message {
                    self.format_frame_message(writer, message)?;
                } else {
                    writer.writeln("      No message in frame")?;
                }
            }
        } else {
            writer.writeln("  No frames")?;
        }

        Ok(())
    }

    /// Format frame message
    fn format_frame_message(
        &mut self,
        writer: &mut OutputWriter,
        message: &data_frame::Message,
    ) -> Result<()> {
        use data_frame::Message;

        match message {
            Message::InstantiateObject(obj) => {
                let object_id = obj.object_id;
                let prefab_name = String::from_utf8_lossy(&obj.prefab_name).to_string();
                self.objects_map.insert(object_id, prefab_name.clone());
                writer.writeln("      Type: InstantiateObject")?;
                writer.writeln(&format!("        Object ID: {}", object_id))?;
                writer.writeln(&format!(
                    "        Owner ID: {:?}",
                    String::from_utf8_lossy(&obj.owner_id)
                ))?;
                writer.writeln(&format!("        Prefab: {:?}", prefab_name))?;
                if let Some(target) = &obj.target {
                    writer.writeln(&format!("        Target: {}", target))?;
                }
                writer.writeln(&format!(
                    "        Init data size: {} bytes",
                    obj.init_data.len()
                ))?;
                // print first 32 bytes
                let debug_len = 32.min(obj.init_data.len());
                writer.writeln(&format!(
                    "        Init data (first {} bytes): {}",
                    debug_len,
                    hex_string(&obj.init_data[..debug_len])
                ))?;
            }
            Message::UpdateObject(obj) => {
                let object_id = obj.object_id;
                writer.writeln("      Type: UpdateObject")?;
                writer.writeln(&format!("        Object ID: {}", object_id))?;
                writer.writeln(&format!("        Method: {}", obj.method))?;
                if let Some(target) = &obj.target {
                    writer.writeln(&format!("        Target: {}", target))?;
                }
                // Try to parse specific payload with prefab_name
                // with struture or router
                if let Some(prefab_name) = self.objects_map.get(&object_id) {
                    let analyzer = UpdateObjectPayloadAnalyzer::new(prefab_name, obj);
                    writer.writeln(&format!("        Prefab: {}", prefab_name))?;
                    writer.writeln(&format!("        Parsed Payload: {}", analyzer))?;
                } else {
                    writer.writeln("        <unknown prefab>")?;
                }
                writer.writeln(&format!(
                    "        Payload size: {} bytes",
                    obj.payload.len()
                ))?;
                let debug_len = 32.min(obj.payload.len());
                writer.writeln(&format!(
                    "        Payload (first {} bytes): {}",
                    debug_len,
                    hex_string(&obj.payload[..debug_len])
                ))?;
            }
            Message::DestroyObject(obj) => {
                writer.writeln("      Type: DestroyObject")?;
                writer.writeln(&format!("        Object ID: {}", obj.object_id))?;
            }
            Message::Room(room) => {
                writer.writeln("      Type: Room")?;
                writer.writeln(&format!(
                    "        ID: {:?}",
                    String::from_utf8_lossy(&room.id)
                ))?;
                writer.writeln(&format!("        Started: {}", room.started_at))?;
                writer.writeln(&format!("        Ended: {}", room.ended_at))?;
            }
            Message::AuthorizeResponse(resp) => {
                writer.writeln("      Type: AuthorizeResponse")?;
                writer.writeln(&format!(
                    "        Player ID: {:?}",
                    String::from_utf8_lossy(&resp.player_id)
                ))?;
                writer.writeln(&format!("        Role: {}", resp.role))?;
            }
            Message::JoinRoomResponse(resp) => {
                writer.writeln("      Type: JoinRoomResponse")?;
                writer.writeln(&format!("        Joined at: {}", resp.joined_at))?;
            }
        }

        Ok(())
    }
}

/// Statistics formatter
pub struct StatsFormatter;

impl StatsFormatter {
    pub fn format_stats(writer: &mut OutputWriter, stats: &PacketStats) -> Result<()> {
        writer.writeln("")?;
        writer.writeln("================== STATISTICS ==================")?;
        writer.writeln(&format!("Total packets: {}", stats.total_packets))?;
        writer.writeln(&format!(
            "Packets with control: {} ({:.1}%)",
            stats.packets_with_control,
            percentage(stats.packets_with_control, stats.total_packets)
        ))?;
        writer.writeln(&format!(
            "Packets with frames: {} ({:.1}%)",
            stats.packets_with_frames,
            percentage(stats.packets_with_frames, stats.total_packets)
        ))?;
        writer.writeln(&format!("Total frames: {}", stats.total_frames))?;
        writer.writeln("")?;

        // Control stats
        if stats.control.total > 0 {
            writer.writeln("Control Messages:")?;
            if stats.control.data_count > 0 {
                writer.writeln(&format!(
                    "  Data: {} ({:.1}%)",
                    stats.control.data_count,
                    percentage(stats.control.data_count, stats.control.total)
                ))?;
            }
            if stats.control.pong_count > 0 {
                writer.writeln(&format!(
                    "  Pong: {} ({:.1}%)",
                    stats.control.pong_count,
                    percentage(stats.control.pong_count, stats.control.total)
                ))?;
            }
            if stats.control.segment_started_at_count > 0 {
                writer.writeln(&format!(
                    "  SegmentStartedAt: {} ({:.1}%)",
                    stats.control.segment_started_at_count,
                    percentage(stats.control.segment_started_at_count, stats.control.total)
                ))?;
            }
            if stats.control.cache_ended_count > 0 {
                writer.writeln(&format!(
                    "  CacheEnded: {} ({:.1}%)",
                    stats.control.cache_ended_count,
                    percentage(stats.control.cache_ended_count, stats.control.total)
                ))?;
            }
            writer.writeln("")?;
        }

        // Frame stats
        if stats.frames.total > 0 {
            writer.writeln("Frame Messages:")?;

            if stats.frames.instantiate_object_count > 0 {
                writer.writeln(&format!(
                    "  InstantiateObject: {} ({:.1}%)",
                    stats.frames.instantiate_object_count,
                    percentage(stats.frames.instantiate_object_count, stats.frames.total)
                ))?;
            }
            if stats.frames.update_object_count > 0 {
                writer.writeln(&format!(
                    "  UpdateObject: {} ({:.1}%)",
                    stats.frames.update_object_count,
                    percentage(stats.frames.update_object_count, stats.frames.total)
                ))?;
            }
            if stats.frames.destroy_object_count > 0 {
                writer.writeln(&format!(
                    "  DestroyObject: {} ({:.1}%)",
                    stats.frames.destroy_object_count,
                    percentage(stats.frames.destroy_object_count, stats.frames.total)
                ))?;
            }
            if stats.frames.room_count > 0 {
                writer.writeln(&format!(
                    "  Room: {} ({:.1}%)",
                    stats.frames.room_count,
                    percentage(stats.frames.room_count, stats.frames.total)
                ))?;
            }
            if stats.frames.authorize_response_count > 0 {
                writer.writeln(&format!(
                    "  AuthorizeResponse: {} ({:.1}%)",
                    stats.frames.authorize_response_count,
                    percentage(stats.frames.authorize_response_count, stats.frames.total)
                ))?;
            }
            if stats.frames.join_room_response_count > 0 {
                writer.writeln(&format!(
                    "  JoinRoomResponse: {} ({:.1}%)",
                    stats.frames.join_room_response_count,
                    percentage(stats.frames.join_room_response_count, stats.frames.total)
                ))?;
            }
            writer.writeln("")?;
        }

        // Unknown fields
        if !stats.unknown_fields.is_empty() {
            writer.writeln("Unknown Fields:")?;
            let mut fields: Vec<_> = stats.unknown_fields.iter().collect();
            fields.sort_by_key(|(num, _)| *num);
            for (field_num, count) in fields {
                writer.writeln(&format!("  Field #{}: {} occurrences", field_num, count))?;
            }
            writer.writeln("")?;
        }

        writer.writeln("================================================")?;
        writer.writeln("")?;

        Ok(())
    }
}

struct UpdateObjectPayloadAnalyzer<'a> {
    prefab_name: &'a str,
    object: &'a UpdateObject,
}

impl<'a> UpdateObjectPayloadAnalyzer<'a> {
    pub fn new(prefab_name: &'a str, object: &'a UpdateObject) -> Self {
        Self {
            prefab_name,
            object,
        }
    }
}

impl Display for UpdateObjectPayloadAnalyzer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // endwith
        if self.prefab_name.ends_with("DateTimeReceiver") {
            write!(
                f,
                "{}",
                self.object.try_parse_date_time().unwrap_or_default()
            )
        }
        // do not parse
        else {
            write!(
                f,
                "<unparsed payload, length: {} bytes>",
                self.object.payload.len()
            )
        }
    }
}

// Helper functions
fn percentage(count: u32, total: u32) -> f64 {
    if total > 0 {
        count as f64 / total as f64 * 100.0
    } else {
        0.0
    }
}

fn hex_string(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
