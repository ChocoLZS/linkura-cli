use anyhow::{anyhow, Result, Context};
use chrono::{DateTime, Utc};
use prost::Message;
use std::fs::File;
use std::io::{BufReader, Read, Seek, Write};

pub mod proto {
    pub mod alstromeria {
        include!(concat!(env!("OUT_DIR"), "/als.rs"));
    }
}

use proto::alstromeria::DataPack;
use prost::bytes::Buf;

#[derive(Debug)]
pub struct PacketInfo {
    pub length: u16,
    pub timestamp: DateTime<Utc>,
    pub data_pack: DataPack,
    pub raw_data: Vec<u8>,
}

#[derive(Debug)]
pub struct MixedPacketInfo {
    pub format: MixedPacketFormat,
    pub timestamp: Option<DateTime<Utc>>,
    pub data_pack: Option<DataPack>,
    pub raw_data: Vec<u8>,
}

#[derive(Debug)]
pub enum MixedPacketFormat {
    // Format 1: int16 length (big endian) + int8 unused (0x00) + byte[length-3] protobuf data
    ProtobufFormat { length: u16, unused: u8 },
    // Format 2: int8 length + int64 timestamp  
    TimestampFormat { length: u8, timestamp: DateTime<Utc> },
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
    pub close_count: u32,
    pub ping_count: u32,
    pub pong_count: u32,
    pub segment_started_at_count: u32,
    pub cache_ended_count: u32,
    pub total_control_messages: u32,
}

#[derive(Debug, Default, Clone)]
pub struct FrameMessageStats {
    pub authorize_request_count: u32,
    pub create_room_request_count: u32,
    pub join_room_request_count: u32,
    pub instantiate_object_request_count: u32,
    pub update_object_request_count: u32,
    pub destroy_object_request_count: u32,
    pub leave_room_request_count: u32,
    pub delete_room_request_count: u32,
    pub debug_message_count: u32,
    pub nop_count: u32,
    pub instantiate_object_count: u32,
    pub update_object_count: u32,
    pub destroy_object_count: u32,
    pub room_count: u32,
    pub authorize_response_count: u32,
    pub create_room_response_count: u32,
    pub join_room_response_count: u32,
    pub leave_room_response_count: u32,
    pub delete_room_response_count: u32,
    pub live_status_count: u32,
    pub pod_close_count: u32,
    pub update_operator_source_count: u32,
    pub live_room_count: u32,
    pub live_player_status_count: u32,
    pub error_code_count: u32,
    pub error_message_count: u32,
    pub total_frame_messages: u32,
}

#[derive(Debug, Default, Clone)]
pub struct PacketAnalysisStats {
    pub total_packets: u32,
    pub packets_with_control: u32,
    pub packets_with_frames: u32,
    pub total_frames: u32,
    pub control_stats: ControlMessageStats,
    pub frame_stats: FrameMessageStats,
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
    let field_data_start = cursor.position();
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
fn format_target_type(target: &Option<proto::alstromeria::instantiate_object_request::Target>) -> String {
    match target {
        Some(proto::alstromeria::instantiate_object_request::Target::GlobalAll(_)) => "GlobalAll".to_string(),
        Some(proto::alstromeria::instantiate_object_request::Target::RoomAll(room)) => {
            format!("RoomAll (room_id: {:?})", String::from_utf8_lossy(&room.room_id))
        }
        Some(proto::alstromeria::instantiate_object_request::Target::RoomOthers(room)) => {
            format!("RoomOthers (room_id: {:?}, operator: {:?})", 
                String::from_utf8_lossy(&room.room_id),
                String::from_utf8_lossy(&room.operator_player_id))
        }
        Some(proto::alstromeria::instantiate_object_request::Target::RoomRole(role)) => {
            format!("RoomRole (room_id: {:?}, roles: {:?})", 
                String::from_utf8_lossy(&role.room_id), role.role)
        }
        None => "None".to_string(),
    }
}

fn format_target_type_with_player(target: &Option<proto::alstromeria::instantiate_object::Target>) -> String {
    match target {
        Some(proto::alstromeria::instantiate_object::Target::GlobalAll(_)) => "GlobalAll".to_string(),
        Some(proto::alstromeria::instantiate_object::Target::CurrentPlayer(_)) => "CurrentPlayer".to_string(),
        Some(proto::alstromeria::instantiate_object::Target::RoomAll(room)) => {
            format!("RoomAll (room_id: {:?})", String::from_utf8_lossy(&room.room_id))
        }
        Some(proto::alstromeria::instantiate_object::Target::RoomOthers(room)) => {
            format!("RoomOthers (room_id: {:?}, operator: {:?})", 
                String::from_utf8_lossy(&room.room_id),
                String::from_utf8_lossy(&room.operator_player_id))
        }
        Some(proto::alstromeria::instantiate_object::Target::RoomRole(role)) => {
            format!("RoomRole (room_id: {:?}, roles: {:?})", 
                String::from_utf8_lossy(&role.room_id), role.role)
        }
        Some(proto::alstromeria::instantiate_object::Target::PlayerId(player_id)) => {
            format!("PlayerId ({:?})", String::from_utf8_lossy(player_id))
        }
        None => "None".to_string(),
    }
}

fn format_update_object_request_target(target: &Option<proto::alstromeria::update_object_request::Target>) -> String {
    match target {
        Some(proto::alstromeria::update_object_request::Target::GlobalAll(_)) => "GlobalAll".to_string(),
        Some(proto::alstromeria::update_object_request::Target::RoomAll(room)) => {
            format!("RoomAll (room_id: {:?})", String::from_utf8_lossy(&room.room_id))
        }
        Some(proto::alstromeria::update_object_request::Target::RoomOthers(room)) => {
            format!("RoomOthers (room_id: {:?}, operator: {:?})", 
                String::from_utf8_lossy(&room.room_id),
                String::from_utf8_lossy(&room.operator_player_id))
        }
        Some(proto::alstromeria::update_object_request::Target::RoomRole(role)) => {
            format!("RoomRole (room_id: {:?}, roles: {:?})", 
                String::from_utf8_lossy(&role.room_id), role.role)
        }
        None => "None".to_string(),
    }
}

fn format_update_object_target(target: &Option<proto::alstromeria::update_object::Target>) -> String {
    match target {
        Some(proto::alstromeria::update_object::Target::GlobalAll(_)) => "GlobalAll".to_string(),
        Some(proto::alstromeria::update_object::Target::CurrentPlayer(_)) => "CurrentPlayer".to_string(),
        Some(proto::alstromeria::update_object::Target::RoomAll(room)) => {
            format!("RoomAll (room_id: {:?})", String::from_utf8_lossy(&room.room_id))
        }
        Some(proto::alstromeria::update_object::Target::RoomOthers(room)) => {
            format!("RoomOthers (room_id: {:?}, operator: {:?})", 
                String::from_utf8_lossy(&room.room_id),
                String::from_utf8_lossy(&room.operator_player_id))
        }
        Some(proto::alstromeria::update_object::Target::RoomRole(role)) => {
            format!("RoomRole (room_id: {:?}, roles: {:?})", 
                String::from_utf8_lossy(&role.room_id), role.role)
        }
        Some(proto::alstromeria::update_object::Target::PlayerId(player_id)) => {
            format!("PlayerId ({:?})", String::from_utf8_lossy(player_id))
        }
        None => "None".to_string(),
    }
}

impl ControlMessageStats {
    pub fn update_from_control(&mut self, control: &proto::alstromeria::data_pack::Control) {
        match control {
            proto::alstromeria::data_pack::Control::Data(_) => {
                self.data_count += 1;
            }
            proto::alstromeria::data_pack::Control::Close(_) => {
                self.close_count += 1;
            }
            proto::alstromeria::data_pack::Control::Ping(_) => {
                self.ping_count += 1;
            }
            proto::alstromeria::data_pack::Control::Pong(_) => {
                self.pong_count += 1;
            }
            proto::alstromeria::data_pack::Control::SegmentStartedAt(_) => {
                self.segment_started_at_count += 1;
            }
            proto::alstromeria::data_pack::Control::CacheEnded(_) => {
                self.cache_ended_count += 1;
            }
        }
        self.total_control_messages += 1;
    }
}

impl FrameMessageStats {
    pub fn update_from_frame(&mut self, frame: &proto::alstromeria::DataFrame) {
        if let Some(message) = &frame.message {
            use proto::alstromeria::data_frame::Message;
            match message {
                Message::AuthorizeRequest(_) => self.authorize_request_count += 1,
                Message::CreateRoomRequest(_) => self.create_room_request_count += 1,
                Message::JoinRoomRequest(_) => self.join_room_request_count += 1,
                Message::InstantiateObjectRequest(_) => self.instantiate_object_request_count += 1,
                Message::UpdateObjectRequest(_) => self.update_object_request_count += 1,
                Message::DestroyObjectRequest(_) => self.destroy_object_request_count += 1,
                Message::LeaveRoomRequest(_) => self.leave_room_request_count += 1,
                Message::DeleteRoomRequest(_) => self.delete_room_request_count += 1,
                Message::DebugMessage(_) => self.debug_message_count += 1,
                Message::Nop(_) => self.nop_count += 1,
                Message::InstantiateObject(_) => self.instantiate_object_count += 1,
                Message::UpdateObject(_) => self.update_object_count += 1,
                Message::DestroyObject(_) => self.destroy_object_count += 1,
                Message::Room(_) => self.room_count += 1,
                Message::AuthorizeResponse(_) => self.authorize_response_count += 1,
                Message::CreateRoomResponse(_) => self.create_room_response_count += 1,
                Message::JoinRoomResponse(_) => self.join_room_response_count += 1,
                Message::LeaveRoomResponse(_) => self.leave_room_response_count += 1,
                Message::DeleteRoomResponse(_) => self.delete_room_response_count += 1,
                Message::LiveStatus(_) => self.live_status_count += 1,
                Message::PodClose(_) => self.pod_close_count += 1,
                Message::UpdateOperatorSource(_) => self.update_operator_source_count += 1,
                Message::LiveRoom(_) => self.live_room_count += 1,
                Message::LivePlayerStatus(_) => self.live_player_status_count += 1,
                Message::ErrorCode(_) => self.error_code_count += 1,
                Message::ErrorMessage(_) => self.error_message_count += 1,
            }
            self.total_frame_messages += 1;
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
}

pub fn format_statistics(writer: &mut OutputWriter, stats: &PacketAnalysisStats) -> Result<()> {
    writer.writeln("")?;
    writer.writeln("================== STATISTICS ==================")?;
    writer.writeln(&format!("Total packets analyzed: {}", stats.total_packets))?;
    writer.writeln(&format!("Packets with control messages: {} ({:.1}%)", 
        stats.packets_with_control, 
        if stats.total_packets > 0 { stats.packets_with_control as f64 / stats.total_packets as f64 * 100.0 } else { 0.0 }))?;
    writer.writeln(&format!("Packets with frames: {} ({:.1}%)", 
        stats.packets_with_frames,
        if stats.total_packets > 0 { stats.packets_with_frames as f64 / stats.total_packets as f64 * 100.0 } else { 0.0 }))?;
    writer.writeln(&format!("Total frames: {}", stats.total_frames))?;
    writer.writeln("")?;
    
    // Control message statistics
    let control = &stats.control_stats;
    if control.total_control_messages > 0 {
        writer.writeln("Control Message Types:")?;
        if control.data_count > 0 {
            writer.writeln(&format!("  Data: {} ({:.1}%)", 
                control.data_count, 
                control.data_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        if control.close_count > 0 {
            writer.writeln(&format!("  Close: {} ({:.1}%)", 
                control.close_count, 
                control.close_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        if control.ping_count > 0 {
            writer.writeln(&format!("  Ping: {} ({:.1}%)", 
                control.ping_count, 
                control.ping_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        if control.pong_count > 0 {
            writer.writeln(&format!("  Pong: {} ({:.1}%)", 
                control.pong_count, 
                control.pong_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        if control.segment_started_at_count > 0 {
            writer.writeln(&format!("  SegmentStartedAt: {} ({:.1}%)", 
                control.segment_started_at_count, 
                control.segment_started_at_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        if control.cache_ended_count > 0 {
            writer.writeln(&format!("  CacheEnded: {} ({:.1}%)", 
                control.cache_ended_count, 
                control.cache_ended_count as f64 / control.total_control_messages as f64 * 100.0))?;
        }
        writer.writeln(&format!("  Total Control Messages: {}", control.total_control_messages))?;
        writer.writeln("")?;
    }
    
    // Frame message statistics
    let frame = &stats.frame_stats;
    if frame.total_frame_messages > 0 {
        writer.writeln("Frame Message Types:")?;
        
        // Request messages
        let request_count = frame.authorize_request_count + frame.create_room_request_count + 
            frame.join_room_request_count + frame.instantiate_object_request_count +
            frame.update_object_request_count + frame.destroy_object_request_count +
            frame.leave_room_request_count + frame.delete_room_request_count;
        
        if request_count > 0 {
            writer.writeln("  Request Messages:")?;
            if frame.authorize_request_count > 0 {
                writer.writeln(&format!("    AuthorizeRequest: {} ({:.1}%)", 
                    frame.authorize_request_count, 
                    frame.authorize_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.create_room_request_count > 0 {
                writer.writeln(&format!("    CreateRoomRequest: {} ({:.1}%)", 
                    frame.create_room_request_count, 
                    frame.create_room_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.join_room_request_count > 0 {
                writer.writeln(&format!("    JoinRoomRequest: {} ({:.1}%)", 
                    frame.join_room_request_count, 
                    frame.join_room_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.instantiate_object_request_count > 0 {
                writer.writeln(&format!("    InstantiateObjectRequest: {} ({:.1}%)", 
                    frame.instantiate_object_request_count, 
                    frame.instantiate_object_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.update_object_request_count > 0 {
                writer.writeln(&format!("    UpdateObjectRequest: {} ({:.1}%)", 
                    frame.update_object_request_count, 
                    frame.update_object_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.destroy_object_request_count > 0 {
                writer.writeln(&format!("    DestroyObjectRequest: {} ({:.1}%)", 
                    frame.destroy_object_request_count, 
                    frame.destroy_object_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.leave_room_request_count > 0 {
                writer.writeln(&format!("    LeaveRoomRequest: {} ({:.1}%)", 
                    frame.leave_room_request_count, 
                    frame.leave_room_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.delete_room_request_count > 0 {
                writer.writeln(&format!("    DeleteRoomRequest: {} ({:.1}%)", 
                    frame.delete_room_request_count, 
                    frame.delete_room_request_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
        }
        
        // Response messages
        let response_count = frame.authorize_response_count + frame.create_room_response_count + 
            frame.join_room_response_count + frame.leave_room_response_count + 
            frame.delete_room_response_count;
        
        if response_count > 0 {
            writer.writeln("  Response Messages:")?;
            if frame.authorize_response_count > 0 {
                writer.writeln(&format!("    AuthorizeResponse: {} ({:.1}%)", 
                    frame.authorize_response_count, 
                    frame.authorize_response_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.create_room_response_count > 0 {
                writer.writeln(&format!("    CreateRoomResponse: {} ({:.1}%)", 
                    frame.create_room_response_count, 
                    frame.create_room_response_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.join_room_response_count > 0 {
                writer.writeln(&format!("    JoinRoomResponse: {} ({:.1}%)", 
                    frame.join_room_response_count, 
                    frame.join_room_response_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.leave_room_response_count > 0 {
                writer.writeln(&format!("    LeaveRoomResponse: {} ({:.1}%)", 
                    frame.leave_room_response_count, 
                    frame.leave_room_response_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.delete_room_response_count > 0 {
                writer.writeln(&format!("    DeleteRoomResponse: {} ({:.1}%)", 
                    frame.delete_room_response_count, 
                    frame.delete_room_response_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
        }
        
        // Object-related messages
        let object_count = frame.instantiate_object_count + frame.update_object_count + frame.destroy_object_count;
        if object_count > 0 {
            writer.writeln("  Object Messages:")?;
            if frame.instantiate_object_count > 0 {
                writer.writeln(&format!("    InstantiateObject: {} ({:.1}%)", 
                    frame.instantiate_object_count, 
                    frame.instantiate_object_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.update_object_count > 0 {
                writer.writeln(&format!("    UpdateObject: {} ({:.1}%)", 
                    frame.update_object_count, 
                    frame.update_object_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.destroy_object_count > 0 {
                writer.writeln(&format!("    DestroyObject: {} ({:.1}%)", 
                    frame.destroy_object_count, 
                    frame.destroy_object_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
        }
        
        // Live-related messages
        let live_count = frame.live_status_count + frame.live_room_count + frame.live_player_status_count;
        if live_count > 0 {
            writer.writeln("  Live Messages:")?;
            if frame.live_status_count > 0 {
                writer.writeln(&format!("    LiveStatus: {} ({:.1}%)", 
                    frame.live_status_count, 
                    frame.live_status_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.live_room_count > 0 {
                writer.writeln(&format!("    LiveRoom: {} ({:.1}%)", 
                    frame.live_room_count, 
                    frame.live_room_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.live_player_status_count > 0 {
                writer.writeln(&format!("    LivePlayerStatus: {} ({:.1}%)", 
                    frame.live_player_status_count, 
                    frame.live_player_status_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
        }
        
        // Other messages
        if frame.room_count > 0 {
            writer.writeln(&format!("  Room: {} ({:.1}%)", 
                frame.room_count, 
                frame.room_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
        }
        if frame.debug_message_count > 0 {
            writer.writeln(&format!("  DebugMessage: {} ({:.1}%)", 
                frame.debug_message_count, 
                frame.debug_message_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
        }
        if frame.nop_count > 0 {
            writer.writeln(&format!("  Nop: {} ({:.1}%)", 
                frame.nop_count, 
                frame.nop_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
        }
        if frame.pod_close_count > 0 {
            writer.writeln(&format!("  PodClose: {} ({:.1}%)", 
                frame.pod_close_count, 
                frame.pod_close_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
        }
        if frame.update_operator_source_count > 0 {
            writer.writeln(&format!("  UpdateOperatorSource: {} ({:.1}%)", 
                frame.update_operator_source_count, 
                frame.update_operator_source_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
        }
        
        // Error messages
        let error_count = frame.error_code_count + frame.error_message_count;
        if error_count > 0 {
            writer.writeln("  Error Messages:")?;
            if frame.error_code_count > 0 {
                writer.writeln(&format!("    ErrorCode: {} ({:.1}%)", 
                    frame.error_code_count, 
                    frame.error_code_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
            if frame.error_message_count > 0 {
                writer.writeln(&format!("    ErrorMessage: {} ({:.1}%)", 
                    frame.error_message_count, 
                    frame.error_message_count as f64 / frame.total_frame_messages as f64 * 100.0))?;
            }
        }
        
        writer.writeln(&format!("  Total Frame Messages: {}", frame.total_frame_messages))?;
    }
    
    writer.writeln("================================================")?;
    writer.writeln("")?;
    Ok(())
}

// Unified formatting functions
pub fn format_packet_unified(writer: &mut OutputWriter, packet_number: usize, length: u16, timestamp: Option<DateTime<Utc>>, data_pack: Option<&DataPack>, raw_data: &[u8], format_type: &str) -> Result<()> {
    writer.writeln(&format!("=== Packet #{}: {} bytes ===", packet_number, length))?;
    writer.writeln(&format!("  Format: {}", format_type))?;
    
    if let Some(ts) = timestamp {
        let timestamp_micros = ts.timestamp_micros() as u64;
        writer.writeln(&format!("  Timestamp: {} ({} / 0x{:x})", 
            ts.format("%Y-%m-%d %H:%M:%S%.6f UTC"),
            timestamp_micros,
            timestamp_micros))?;
    }
    
    // Show protobuf field analysis for all packets containing protobuf data
    if !raw_data.is_empty() {
        writer.writeln(&format!("  Raw data length: {} bytes", raw_data.len()))?;
        
        // Show first 32 bytes in hex
        let debug_len = 32.min(raw_data.len());
        writer.writeln(&format!("  Raw data (first {} bytes): {}", debug_len,
            raw_data[..debug_len].iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")))?;
        
        // Try to parse protobuf fields
        let fields = parse_protobuf_fields(raw_data);
        if !fields.is_empty() {
            writer.writeln("  Protobuf Fields:")?;
            for field in &fields {
                writer.writeln(&format!("    Field #{}: {} (wire type: {}, {} bytes)", 
                    field.field_number, 
                    field.field_name,
                    field.wire_type,
                    field.raw_bytes.len()))?;
                
                // Show field bytes
                let field_hex = field.raw_bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
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
            proto::alstromeria::data_pack::Control::Data(data) => {
                writer.writeln(&format!("    Type: Data, Value: {}", data))?;
            }
            proto::alstromeria::data_pack::Control::Close(close) => {
                writer.writeln(&format!("    Type: Close, Cause: {}", close.cause))?;
            }
            proto::alstromeria::data_pack::Control::Ping(ping) => {
                writer.writeln(&format!("    Type: Ping, Value: {}", ping))?;
            }
            proto::alstromeria::data_pack::Control::Pong(pong) => {
                writer.writeln(&format!("    Type: Pong, Value: {}", pong))?;
            }
            proto::alstromeria::data_pack::Control::SegmentStartedAt(timestamp) => {
                writer.writeln(&format!("    Type: SegmentStartedAt, Timestamp: {}", timestamp))?;
            }
            proto::alstromeria::data_pack::Control::CacheEnded(ended) => {
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
            if let Some(message) = &frame.message {
                use proto::alstromeria::data_frame::Message;
                match message {
                    Message::AuthorizeRequest(req) => {
                        writer.writeln(&format!("      Message: AuthorizeRequest (token: {} bytes)", req.token.len()))?;
                    }
                    Message::CreateRoomRequest(req) => {
                        writer.writeln(&format!("      Message: CreateRoomRequest (room_id: {:?})", 
                            String::from_utf8_lossy(&req.room_id)))?;
                    }
                    Message::JoinRoomRequest(req) => {
                        writer.writeln(&format!("      Message: JoinRoomRequest (room_id: {:?}, methods: {:?})", 
                            String::from_utf8_lossy(&req.room_id), req.methods))?;
                    }
                    Message::InstantiateObjectRequest(req) => {
                        let target_desc = format_target_type(&req.target);
                        writer.writeln(&format!("      Message: InstantiateObjectRequest"))?;
                        writer.writeln(&format!("        Object ID: {}", req.object_id))?;
                        writer.writeln(&format!("        Prefab Name: {:?}", String::from_utf8_lossy(&req.prefab_name)))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !req.init_data.is_empty() {
                            writer.writeln(&format!("        Init Data: {} bytes", req.init_data.len()))?;
                            let init_data_hex = req.init_data.iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!("        Init Data (first {} bytes): {}", 
                                32.min(req.init_data.len()), init_data_hex))?;
                        }
                    }
                    Message::UpdateObjectRequest(req) => {
                        let target_desc = format_update_object_request_target(&req.target);
                        writer.writeln(&format!("      Message: UpdateObjectRequest"))?;
                        writer.writeln(&format!("        Object ID: {}", req.object_id))?;
                        writer.writeln(&format!("        Method: {}", req.method))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !req.payload.is_empty() {
                            writer.writeln(&format!("        Payload: {} bytes", req.payload.len()))?;
                            let payload_hex = req.payload.iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!("        Payload (first {} bytes): {}", 
                                32.min(req.payload.len()), payload_hex))?;
                        }
                    }
                    Message::DestroyObjectRequest(req) => {
                        writer.writeln(&format!("      Message: DestroyObjectRequest (object_id: {})", req.object_id))?;
                    }
                    Message::LeaveRoomRequest(_) => {
                        writer.writeln("      Message: LeaveRoomRequest")?;
                    }
                    Message::DeleteRoomRequest(req) => {
                        writer.writeln(&format!("      Message: DeleteRoomRequest (room_id: {:?})", 
                            String::from_utf8_lossy(&req.room_id)))?;
                    }
                    Message::DebugMessage(msg) => {
                        writer.writeln(&format!("      Message: DebugMessage (device_time: {}, text: {:?})", 
                            msg.device_time, String::from_utf8_lossy(&msg.text)))?;
                    }
                    Message::Nop(_) => {
                        writer.writeln("      Message: Nop")?;
                    }
                    Message::InstantiateObject(obj) => {
                        let target_desc = format_target_type_with_player(&obj.target);
                        writer.writeln(&format!("      Message: InstantiateObject"))?;
                        writer.writeln(&format!("        Object ID: {}", obj.object_id))?;
                        writer.writeln(&format!("        Owner ID: {:?}", String::from_utf8_lossy(&obj.owner_id)))?;
                        writer.writeln(&format!("        Prefab Name: {:?}", String::from_utf8_lossy(&obj.prefab_name)))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !obj.init_data.is_empty() {
                            writer.writeln(&format!("        Init Data: {} bytes", obj.init_data.len()))?;
                            let init_data_hex = obj.init_data.iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!("        Init Data (first {} bytes): {}", 
                                32.min(obj.init_data.len()), init_data_hex))?;
                        }
                    }
                    Message::UpdateObject(obj) => {
                        let target_desc = format_update_object_target(&obj.target);
                        writer.writeln(&format!("      Message: UpdateObject"))?;
                        writer.writeln(&format!("        Object ID: {}", obj.object_id))?;
                        writer.writeln(&format!("        Method: {}", obj.method))?;
                        writer.writeln(&format!("        Target: {}", target_desc))?;
                        if !obj.payload.is_empty() {
                            writer.writeln(&format!("        Payload: {} bytes", obj.payload.len()))?;
                            let payload_hex = obj.payload.iter()
                                .take(32)
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            writer.writeln(&format!("        Payload (first {} bytes): {}", 
                                32.min(obj.payload.len()), payload_hex))?;
                        }
                    }
                    Message::DestroyObject(obj) => {
                        writer.writeln(&format!("      Message: DestroyObject (object_id: {})", obj.object_id))?;
                    }
                    Message::Room(room) => {
                        writer.writeln(&format!("      Message: Room (id: {:?}, started: {}, ended: {})", 
                            String::from_utf8_lossy(&room.id), room.started_at, room.ended_at))?;
                    }
                    Message::AuthorizeResponse(resp) => {
                        writer.writeln(&format!("      Message: AuthorizeResponse (player_id: {:?}, role: {})", 
                            String::from_utf8_lossy(&resp.player_id), resp.role))?;
                    }
                    Message::CreateRoomResponse(resp) => {
                        if let Some(room) = &resp.room {
                            writer.writeln(&format!("      Message: CreateRoomResponse (room_id: {:?})", 
                                String::from_utf8_lossy(&room.id)))?;
                        } else {
                            writer.writeln("      Message: CreateRoomResponse (no room)")?;
                        }
                    }
                    Message::JoinRoomResponse(resp) => {
                        if let Some(room) = &resp.room {
                            writer.writeln(&format!("      Message: JoinRoomResponse (room_id: {:?}, joined_at: {})", 
                                String::from_utf8_lossy(&room.id), resp.joined_at))?;
                        } else {
                            writer.writeln(&format!("      Message: JoinRoomResponse (joined_at: {})", resp.joined_at))?;
                        }
                    }
                    Message::LeaveRoomResponse(_) => {
                        writer.writeln("      Message: LeaveRoomResponse")?;
                    }
                    Message::DeleteRoomResponse(_) => {
                        writer.writeln("      Message: DeleteRoomResponse")?;
                    }
                    Message::LiveStatus(live) => {
                        writer.writeln(&format!("      Message: LiveStatus (room_id: {:?}, status: {:?})", 
                            String::from_utf8_lossy(&live.room_id), live.status()))?;
                    }
                    Message::PodClose(close) => {
                        writer.writeln(&format!("      Message: PodClose (cause: {})", close.cause))?;
                    }
                    Message::UpdateOperatorSource(update) => {
                        writer.writeln(&format!("      Message: UpdateOperatorSource (urls: {:?})", 
                            update.src_urls.iter().map(|url| String::from_utf8_lossy(url)).collect::<Vec<_>>()))?;
                    }
                    Message::LiveRoom(room) => {
                        writer.writeln(&format!("      Message: LiveRoom (id: {:?})", 
                            String::from_utf8_lossy(&room.id)))?;
                    }
                    Message::LivePlayerStatus(status) => {
                        writer.writeln(&format!("      Message: LivePlayerStatus ({:?})", status))?;
                    }
                    Message::ErrorCode(code) => {
                        writer.writeln(&format!("      Message: ErrorCode ({})", code))?;
                    }
                    Message::ErrorMessage(msg) => {
                        writer.writeln(&format!("      Message: ErrorMessage ({:?})", String::from_utf8_lossy(msg)))?;
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
        let mut writer = OutputWriter::new(None)?;
        self.read_packets_with_limit_and_writer(&mut writer, limit)
    }
    
    pub fn read_packets_with_limit_and_writer(&mut self, writer: &mut OutputWriter, limit: usize) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        let mut packet_count = 0;
        let mut stats = PacketAnalysisStats::default();
        
        loop {
            match self.read_packet() {
                Ok(packet) => {
                    packet_count += 1;
                    
                    // Update statistics
                    stats.update_from_packet(&packet.data_pack);
                    
                    format_packet_unified(
                        writer,
                        packet_count, 
                        packet.length, 
                        Some(packet.timestamp),
                        Some(&packet.data_pack),
                        &packet.raw_data,
                        "Standard protobuf format (int16 length + int8 unused + int64 timestamp + protobuf data)"
                    )?;
                    
                    packets.push(packet);
                    
                    // Check if we've reached the limit
                    if packet_count >= limit {
                        writer.writeln(&format!("Reached requested packet limit of {} packets", limit))?;
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();
                    
                    // Check for various EOF-related conditions
                    let is_eof = error_msg.contains("eof") || 
                                error_msg.contains("unexpectedeof") || 
                                error_msg.contains("failed to fill whole buffer") || 
                                error_msg.contains("failed to read packet length") ||
                                error_msg.contains("failed to read unused byte") ||
                                error_msg.contains("failed to read timestamp") ||
                                error_msg.contains("failed to read protobuf data");
                    
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
                        writer.writeln(&format!("Reached end of file after {} packets (requested: {})", packets.len(), limit))?;
                        break;
                    }
                    
                    // Write detailed error information
                    writer.writeln(&format!("=== PACKET PARSING ERROR #{} ===", packet_count + 1))?;
                    writer.writeln(&format!("Error: {}", e))?;
                    
                    // Try to show current file position and nearby bytes for debugging
                    if let Ok(current_pos) = self.reader.stream_position() {
                        writer.writeln(&format!("Current file position: 0x{:x} ({})", current_pos, current_pos))?;
                        
                        // Try to read some bytes around the current position for debugging
                        let mut debug_buffer = [0u8; 32];
                        if current_pos >= 16 {
                            if self.reader.seek(std::io::SeekFrom::Start(current_pos - 16)).is_ok() {
                                if let Ok(bytes_read) = self.reader.read(&mut debug_buffer) {
                                    let debug_hex = debug_buffer[..bytes_read]
                                        .iter()
                                        .enumerate()
                                        .map(|(i, b)| {
                                            if i == 16 { format!("[{:02x}]", b) } // Mark current position
                                            else { format!("{:02x}", b) }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    writer.writeln(&format!("Context bytes (16 before + 16 after): {}", debug_hex))?;
                                }
                            }
                        }
                    }
                    
                    writer.writeln("=================================")?;
                    return Err(e.context(format!("Failed at packet #{}", packet_count + 1)));
                }
            }
        }
        
        // Display statistics at the end
        format_statistics(writer, &stats)?;
        
        Ok(packets)
    }

    fn read_packet(&mut self) -> Result<PacketInfo> {
        let length = self.read_u16_be()
            .with_context(|| "Failed to read packet length")?;
        
        if length < 9 {
            return Err(anyhow!("Invalid packet length: {}, must be at least 9", length));
        }

        let unused = self.read_u8()
            .with_context(|| "Failed to read unused byte")?;
        if unused != 0x01 {
            return Err(anyhow!("Invalid unused byte: expected 0x01, got 0x{:02x}", unused));
        }

        let timestamp_micros = self.read_u64_be()
            .with_context(|| "Failed to read timestamp")?;
        let timestamp = DateTime::from_timestamp_micros(timestamp_micros as i64)
            .ok_or_else(|| anyhow!("Invalid timestamp: {} (0x{:x})", timestamp_micros, timestamp_micros))?;

        let data_length = length - 9;
        let mut data = vec![0u8; data_length as usize];
        self.reader.read_exact(&mut data)
            .with_context(|| format!("Failed to read protobuf data of length {}", data_length))?;

        let data_pack = DataPack::decode(data.as_slice())
            .map_err(|e| {
                anyhow!("Failed to decode protobuf data (length: {}, timestamp: {} / 0x{:x}): {}", 
                    data_length, timestamp_micros, timestamp_micros, e)
            })?;

        Ok(PacketInfo {
            length,
            timestamp,
            data_pack,
            raw_data: data,
        })
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

pub struct MixedPacketReader {
    reader: BufReader<File>,
}

impl MixedPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    pub fn read_mixed_packets(&mut self) -> Result<Vec<MixedPacketInfo>> {
        self.read_mixed_packets_with_limit(8)
    }
    
    pub fn read_mixed_packets_with_limit(&mut self, limit: usize) -> Result<Vec<MixedPacketInfo>> {
        let mut writer = OutputWriter::new(None)?;
        self.read_mixed_packets_with_limit_and_writer(&mut writer, limit)
    }
    
    pub fn read_mixed_packets_with_limit_and_writer(&mut self, writer: &mut OutputWriter, limit: usize) -> Result<Vec<MixedPacketInfo>> {
        let mut packets = Vec::new();
        let mut packet_count = 0;
        let mut stats = PacketAnalysisStats::default();
        
        loop {
            match self.read_mixed_packet() {
                Ok(packet) => {
                    packet_count += 1;
                    
                    // Update statistics if we have a data pack
                    if let Some(data_pack) = &packet.data_pack {
                        stats.update_from_packet(data_pack);
                    }
                    
                    let (format_desc, length, timestamp) = match &packet.format {
                        MixedPacketFormat::ProtobufFormat { length, unused } => {
                            (format!("Mixed protobuf format (int16 length {} + int8 unused 0x{:02x} + protobuf data)", length, unused), *length, None)
                        }
                        MixedPacketFormat::TimestampFormat { length, timestamp } => {
                            (format!("Mixed timestamp format (int8 length {} + int64 timestamp)", length), *length as u16, Some(*timestamp))
                        }
                    };
                    
                    format_packet_unified(
                        writer,
                        packet_count, 
                        length, 
                        timestamp,
                        packet.data_pack.as_ref(),
                        &packet.raw_data,
                        &format_desc
                    )?;
                    
                    packets.push(packet);
                    
                    // Check if we've reached the limit
                    if packet_count >= limit {
                        writer.writeln(&format!("Reached requested packet limit of {} packets", limit))?;
                        break;
                    }
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();
                    
                    // Check for various EOF-related conditions
                    let is_eof = error_msg.contains("eof") || 
                                error_msg.contains("unexpectedeof") || 
                                error_msg.contains("failed to fill whole buffer") || 
                                error_msg.contains("failed to read first byte") ||
                                error_msg.contains("failed to read second byte") ||
                                error_msg.contains("unexpected end of file");
                    
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
                        writer.writeln(&format!("Reached end of file after {} packets (requested: {})", packets.len(), limit))?;
                        break;
                    }
                    
                    // Write detailed error information
                    writer.writeln(&format!("=== MIXED PACKET PARSING ERROR #{} ===", packet_count + 1))?;
                    writer.writeln(&format!("Error: {}", e))?;
                    
                    // Try to show current file position and nearby bytes for debugging
                    if let Ok(current_pos) = self.reader.stream_position() {
                        writer.writeln(&format!("Current file position: 0x{:x} ({})", current_pos, current_pos))?;
                        
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
                                    writer.writeln(&format!("Context bytes (32 before + 32 after): {}", debug_hex))?;
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
                                    writer.writeln(&format!("Context bytes from file start: {}", debug_hex))?;
                                }
                            }
                        }
                    }
                    
                    writer.writeln("======================================")?;
                    return Err(e.context(format!("Failed at mixed packet #{}", packet_count + 1)));
                }
            }
        }
        
        // Display statistics at the end
        format_statistics(writer, &stats)?;
        
        Ok(packets)
    }

    fn read_mixed_packet(&mut self) -> Result<MixedPacketInfo> {
        // Try to peek first byte to determine format
        let mut peek_buf = [0u8; 1];
        self.reader.read_exact(&mut peek_buf)
            .with_context(|| "Failed to read first byte")?;
        
        // Read the second byte to help determine the format
        let mut second_buf = [0u8; 1];
        self.reader.read_exact(&mut second_buf)
            .with_context(|| "Failed to read second byte")?;
        println!("First byte: 0x{:02x}, Second byte: 0x{:02x}", peek_buf[0], second_buf[0]);
        let first_two_bytes = u16::from_be_bytes([peek_buf[0], second_buf[0]]);
        
        // Heuristic: if the first two bytes as big-endian u16 form a reasonable length (> 3 and < 65536)
        // and the next byte is 0x00, it's likely the protobuf format
        let mut third_buf = [0u8; 1];
        match self.reader.read_exact(&mut third_buf) {
            Ok(_) => {
                if third_buf[0] == 0x00 && first_two_bytes >= 3 && first_two_bytes < 32768 {
                    // This looks like protobuf format: int16 length + int8 unused(0x00) + data
                    self.read_protobuf_format_packet(first_two_bytes, third_buf[0])
                } else {
                    // This looks like timestamp format: int8 length + int64 timestamp
                    // Reconstruct the data and read as timestamp format
                    let length = peek_buf[0];
                    
                    // We've already read 3 bytes, need 5 more for the int64 timestamp
                    let mut timestamp_buf = [0u8; 8];
                    timestamp_buf[0] = second_buf[0];
                    timestamp_buf[1] = third_buf[0];
                    self.reader.read_exact(&mut timestamp_buf[2..])?;
                    
                    let timestamp_micros = u64::from_be_bytes(timestamp_buf);
                    let timestamp = DateTime::from_timestamp_micros(timestamp_micros as i64)
                        .ok_or_else(|| anyhow!("Invalid timestamp: {}", timestamp_micros))?;
                    
                    let mut raw_data = Vec::new();
                    if length > 9 {
                        let remaining_length = length as usize - 9; // 1 byte length + 8 bytes timestamp
                        raw_data = vec![0u8; remaining_length];
                        self.reader.read_exact(&mut raw_data)?;
                    }
                    
                    Ok(MixedPacketInfo {
                        format: MixedPacketFormat::TimestampFormat { length, timestamp },
                        timestamp: Some(timestamp),
                        data_pack: None,
                        raw_data,
                    })
                }
            }
            Err(_) => {
                // Not enough data, probably end of file
                Err(anyhow!("Unexpected end of file"))
            }
        }
    }

    fn read_protobuf_format_packet(&mut self, length: u16, unused: u8) -> Result<MixedPacketInfo> {
        if length < 3 {
            return Err(anyhow!("Invalid protobuf packet length: {}, must be at least 3", length));
        }

        let data_length = length - 1; // unused(1)
        println!("Reading protobuf packet with length: {}, unused: 0x{:02x}", data_length, unused);
        let mut data = vec![0u8; data_length as usize];
        self.reader.read_exact(&mut data)
            .with_context(|| format!("Failed to read protobuf data of length {}", data_length))?;

        // Try to parse as protobuf
        let data_pack = match DataPack::decode(data.as_slice()) {
            Ok(pack) => Some(pack),
            Err(e) => {
                println!("  Warning: Failed to decode protobuf data: {}", e);
                None
            }
        };

        Ok(MixedPacketInfo {
            format: MixedPacketFormat::ProtobufFormat { length, unused },
            timestamp: None,
            data_pack,
            raw_data: data,
        })
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

pub fn format_packet_info(packet: &PacketInfo) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("=== Packet ===\n"));
    output.push_str(&format!("Length: {} bytes\n", packet.length));
    output.push_str(&format!("Timestamp: {} ({})\n", 
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
            proto::alstromeria::data_pack::Control::Data(data) => {
                output.push_str(&format!("Data({})\n", data));
            }
            proto::alstromeria::data_pack::Control::Close(close) => {
                output.push_str(&format!("Close(cause: {})\n", close.cause));
            }
            proto::alstromeria::data_pack::Control::Ping(ping) => {
                output.push_str(&format!("Ping({})\n", ping));
            }
            proto::alstromeria::data_pack::Control::Pong(pong) => {
                output.push_str(&format!("Pong({})\n", pong));
            }
            proto::alstromeria::data_pack::Control::SegmentStartedAt(timestamp) => {
                output.push_str(&format!("SegmentStartedAt({})\n", timestamp));
            }
            proto::alstromeria::data_pack::Control::CacheEnded(ended) => {
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

fn format_data_frame(frame: &proto::alstromeria::DataFrame, indent_level: usize) -> String {
    let indent = "  ".repeat(indent_level);
    let mut output = String::new();
    
    if let Some(message) = &frame.message {
        use proto::alstromeria::data_frame::Message;
        
        output.push_str(&format!("{}Message: ", indent));
        match message {
            Message::AuthorizeRequest(req) => {
                output.push_str(&format!("AuthorizeRequest(token: {} bytes)\n", req.token.len()));
            }
            Message::CreateRoomRequest(req) => {
                output.push_str(&format!("CreateRoomRequest(room_id: {:?})\n", 
                    String::from_utf8_lossy(&req.room_id)));
            }
            Message::JoinRoomRequest(req) => {
                output.push_str(&format!("JoinRoomRequest(room_id: {:?}, methods: {:?})\n", 
                    String::from_utf8_lossy(&req.room_id), req.methods));
            }
            Message::InstantiateObjectRequest(req) => {
                let target_desc = format_target_type(&req.target);
                output.push_str(&format!("InstantiateObjectRequest(object_id: {}, prefab: {:?}, target: {})\n", 
                    req.object_id, String::from_utf8_lossy(&req.prefab_name), target_desc));
            }
            Message::UpdateObjectRequest(req) => {
                let target_desc = format_update_object_request_target(&req.target);
                output.push_str(&format!("UpdateObjectRequest(object_id: {}, method: {}, target: {})\n", 
                    req.object_id, req.method, target_desc));
            }
            Message::DestroyObjectRequest(req) => {
                output.push_str(&format!("DestroyObjectRequest(object_id: {})\n", req.object_id));
            }
            Message::LeaveRoomRequest(_) => {
                output.push_str("LeaveRoomRequest\n");
            }
            Message::DeleteRoomRequest(req) => {
                output.push_str(&format!("DeleteRoomRequest(room_id: {:?})\n", 
                    String::from_utf8_lossy(&req.room_id)));
            }
            Message::DebugMessage(msg) => {
                output.push_str(&format!("DebugMessage(device_time: {}, text: {:?})\n", 
                    msg.device_time, String::from_utf8_lossy(&msg.text)));
            }
            Message::Nop(_) => {
                output.push_str("Nop\n");
            }
            Message::InstantiateObject(obj) => {
                let target_desc = format_target_type_with_player(&obj.target);
                output.push_str(&format!("InstantiateObject(object_id: {}, owner: {:?}, prefab: {:?}, target: {})\n", 
                    obj.object_id, String::from_utf8_lossy(&obj.owner_id), 
                    String::from_utf8_lossy(&obj.prefab_name), target_desc));
            }
            Message::UpdateObject(obj) => {
                let target_desc = format_update_object_target(&obj.target);
                output.push_str(&format!("UpdateObject(object_id: {}, method: {}, target: {})\n", 
                    obj.object_id, obj.method, target_desc));
            }
            Message::DestroyObject(obj) => {
                output.push_str(&format!("DestroyObject(object_id: {})\n", obj.object_id));
            }
            Message::Room(room) => {
                output.push_str(&format!("Room(id: {:?}, started: {}, ended: {})\n", 
                    String::from_utf8_lossy(&room.id), room.started_at, room.ended_at));
            }
            Message::AuthorizeResponse(resp) => {
                output.push_str(&format!("AuthorizeResponse(player_id: {:?}, role: {})\n", 
                    String::from_utf8_lossy(&resp.player_id), resp.role));
            }
            Message::CreateRoomResponse(resp) => {
                if let Some(room) = &resp.room {
                    output.push_str(&format!("CreateRoomResponse(room_id: {:?})\n", 
                        String::from_utf8_lossy(&room.id)));
                } else {
                    output.push_str("CreateRoomResponse(no room)\n");
                }
            }
            Message::JoinRoomResponse(resp) => {
                if let Some(room) = &resp.room {
                    output.push_str(&format!("JoinRoomResponse(room_id: {:?}, joined_at: {})\n", 
                        String::from_utf8_lossy(&room.id), resp.joined_at));
                } else {
                    output.push_str(&format!("JoinRoomResponse(joined_at: {})\n", resp.joined_at));
                }
            }
            Message::LeaveRoomResponse(_) => {
                output.push_str("LeaveRoomResponse\n");
            }
            Message::DeleteRoomResponse(_) => {
                output.push_str("DeleteRoomResponse\n");
            }
            Message::LiveStatus(live) => {
                output.push_str(&format!("LiveStatus(room_id: {:?}, status: {:?})\n", 
                    String::from_utf8_lossy(&live.room_id), live.status()));
            }
            Message::PodClose(close) => {
                output.push_str(&format!("PodClose(cause: {})\n", close.cause));
            }
            Message::UpdateOperatorSource(update) => {
                output.push_str(&format!("UpdateOperatorSource(urls: {:?})\n", 
                    update.src_urls.iter().map(|url| String::from_utf8_lossy(url)).collect::<Vec<_>>()));
            }
            Message::LiveRoom(room) => {
                output.push_str(&format!("LiveRoom(id: {:?})\n", 
                    String::from_utf8_lossy(&room.id)));
            }
            Message::LivePlayerStatus(status) => {
                output.push_str(&format!("LivePlayerStatus({:?})\n", status));
            }
            Message::ErrorCode(code) => {
                output.push_str(&format!("ErrorCode({})\n", code));
            }
            Message::ErrorMessage(msg) => {
                output.push_str(&format!("ErrorMessage({:?})\n", String::from_utf8_lossy(msg)));
            }
        }
    }
    
    output
}

pub fn analyze_binary_file(file_path: &str) -> Result<()> {
    analyze_binary_file_with_count(file_path, 8)
}

pub fn analyze_binary_file_with_count(file_path: &str, packet_count: usize) -> Result<()> {
    let mut writer = OutputWriter::new(None)?;
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let metadata = file.metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;
    
    writer.writeln(&format!("Analyzing binary file: {}", file_path))?;
    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
    writer.writeln(&format!("Analyzing first {} packets", packet_count))?;
    
    // Debug: show first 32 bytes of the file
    let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
    file.read_exact(&mut debug_buffer)?;
    writer.writeln(&format!("First {} bytes (hex): {}", debug_buffer.len(), 
        debug_buffer.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")))?;
    
    // Reset file position
    file.seek(std::io::SeekFrom::Start(0))?;
    
    writer.writeln("==========================================")?;
    writer.writeln("")?;
    
    let mut reader = ProtoPacketReader::new(file);
    
    let packets = match reader.read_packets_with_limit_and_writer(&mut writer, packet_count) {
        Ok(packets) => packets,
        Err(e) => {
            return Err(anyhow!("Failed to read packets from file '{}': {}", file_path, e));
        }
    };
    
    writer.writeln(&format!("Total packets found: {}", packets.len()))?;
    writer.flush()?;
    
    Ok(())
}

pub fn analyze_mixed_binary_file(file_path: &str) -> Result<()> {
    analyze_mixed_binary_file_with_count(file_path, 8)
}

pub fn analyze_mixed_binary_file_with_count(file_path: &str, packet_count: usize) -> Result<()> {
    let mut writer = OutputWriter::new(None)?;
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let metadata = file.metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;
    
    writer.writeln(&format!("Analyzing mixed format binary file: {}", file_path))?;
    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
    writer.writeln(&format!("Analyzing first {} packets", packet_count))?;
    
    // Debug: show first 32 bytes of the file
    let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
    file.read_exact(&mut debug_buffer)?;
    writer.writeln(&format!("First {} bytes (hex): {}", debug_buffer.len(), 
        debug_buffer.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")))?;
    
    // Reset file position
    file.seek(std::io::SeekFrom::Start(0))?;
    
    writer.writeln("==========================================")?;
    writer.writeln("")?;
    
    let mut reader = MixedPacketReader::new(file);
    
    let packets = match reader.read_mixed_packets_with_limit_and_writer(&mut writer, packet_count) {
        Ok(packets) => packets,
        Err(e) => {
            return Err(anyhow!("Failed to read mixed packets from file '{}': {}", file_path, e));
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
    
    Ok(())
}

pub struct OutputWriter {
    writer: Box<dyn Write>,
}

impl OutputWriter {
    pub fn new(output_path: Option<&str>) -> Result<Self> {
        let writer: Box<dyn Write> = match output_path {
            Some(path) => Box::new(File::create(path)
                .with_context(|| format!("Failed to create output file: {}", path))?),
            None => Box::new(std::io::stdout()),
        };
        
        Ok(Self { writer })
    }
    
    pub fn writeln(&mut self, content: &str) -> Result<()> {
        writeln!(self.writer, "{}", content)
            .with_context(|| "Failed to write to output")?;
        Ok(())
    }
    
    pub fn write(&mut self, content: &str) -> Result<()> {
        write!(self.writer, "{}", content)
            .with_context(|| "Failed to write to output")?;
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
            .with_context(|| "Failed to flush output")?;
        Ok(())
    }
}

pub fn analyze_binary_file_with_output(file_path: &str, output_path: Option<&str>) -> Result<()> {
    analyze_binary_file_with_output_and_count(file_path, output_path, 8)
}

pub fn analyze_binary_file_with_output_and_count(file_path: &str, output_path: Option<&str>, packet_count: usize) -> Result<()> {
    let mut writer = OutputWriter::new(output_path)?;
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let metadata = file.metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;
    
    writer.writeln(&format!("Analyzing binary file: {}", file_path))?;
    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
    writer.writeln(&format!("Analyzing first {} packets", packet_count))?;
    
    // Debug: show first 32 bytes of the file
    let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
    file.read_exact(&mut debug_buffer)?;
    writer.writeln(&format!("First {} bytes (hex): {}", debug_buffer.len(), 
        debug_buffer.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")))?;
    
    // Reset file position
    file.seek(std::io::SeekFrom::Start(0))?;
    
    writer.writeln("===========================================")?;
    writer.writeln("")?;
    
    let mut reader = ProtoPacketReader::new(file);
    
    let packets = match reader.read_packets_with_limit_and_writer(&mut writer, packet_count) {
        Ok(packets) => packets,
        Err(e) => {
            return Err(anyhow!("Failed to read packets from file '{}': {}", file_path, e));
        }
    };
    
    writer.writeln(&format!("Total packets found: {}", packets.len()))?;
    writer.flush()?;
    
    Ok(())
}

pub fn analyze_mixed_binary_file_with_output(file_path: &str, output_path: Option<&str>) -> Result<()> {
    analyze_mixed_binary_file_with_output_and_count(file_path, output_path, 8)
}

pub fn analyze_mixed_binary_file_with_output_and_count(file_path: &str, output_path: Option<&str>, packet_count: usize) -> Result<()> {
    let mut writer = OutputWriter::new(output_path)?;
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let metadata = file.metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;
    
    writer.writeln(&format!("Analyzing mixed format binary file: {}", file_path))?;
    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;
    writer.writeln(&format!("Analyzing first {} packets", packet_count))?;
    
    // Debug: show first 32 bytes of the file
    let mut debug_buffer = vec![0u8; 32.min(metadata.len() as usize)];
    file.read_exact(&mut debug_buffer)?;
    writer.writeln(&format!("First {} bytes (hex): {}", debug_buffer.len(), 
        debug_buffer.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")))?;
    
    // Reset file position
    file.seek(std::io::SeekFrom::Start(0))?;
    
    writer.writeln("===========================================")?;
    writer.writeln("")?;
    
    let mut reader = MixedPacketReader::new(file);
    
    let packets = match reader.read_mixed_packets_with_limit_and_writer(&mut writer, packet_count) {
        Ok(packets) => packets,
        Err(e) => {
            return Err(anyhow!("Failed to read mixed packets from file '{}': {}", file_path, e));
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
    
    Ok(())
}