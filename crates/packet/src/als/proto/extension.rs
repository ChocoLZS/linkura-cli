use std::convert::TryFrom;
use std::fmt::Display;

use chrono::{DateTime, Utc};

use super::define::UpdateObject;

// Constants for .NET DateTime Ticks conversion
const TICKS_TO_UNIX_EPOCH: i64 = 621355968000000000;
const TICKS_PER_SECOND: i64 = 10_000_000;
const JST_OFFSET_SECONDS: i64 = 9 * 3600;
const PAYLOAD_SIZE: usize = 16;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown method: {0}")]
    UnknownMethod(i32),

    #[error("Invalid payload: expected at least {expected} bytes, got {actual}")]
    InvalidPayload { expected: usize, actual: usize },

    #[error("Missing target field")]
    MissingTarget,

    #[error("Failed to decode protobuf message")]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error("Invalid object ID: {0}")]
    InvalidObjectId(i32),
}

/// A trait that bundles constraints for packet conversion, display, and default values.
pub trait CommandPacket: Default + Display + for<'a> TryFrom<&'a [u8], Error = ParseError> {}

// --- DateTimeConvert ---

#[derive(Debug, Clone)]
pub struct DateTimeConvert {
    pub date_time: DateTime<Utc>, // ticks in c#
    pub sync_time: f64,
}

impl CommandPacket for DateTimeConvert {}

impl Default for DateTimeConvert {
    fn default() -> Self {
        DateTimeConvert {
            date_time: Utc::now(),
            sync_time: 0.0,
        }
    }
}

impl Display for DateTimeConvert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_seconds = self.sync_time * 3600.0;
        let hours = (total_seconds / 3600.0) as i32;
        let minutes = ((total_seconds % 3600.0) / 60.0) as i32;
        let seconds = total_seconds % 60.0;

        let sync_duration = chrono::Duration::milliseconds((total_seconds * 1000.0) as i64);
        let start_time = self.date_time - sync_duration;

        write!(
            f,
            "DateTime: {}, SyncTime: {}h{}m{:.2}s, StartTime: {}",
            self.date_time, hours, minutes, seconds, start_time
        )
    }
}

impl TryFrom<&[u8]> for DateTimeConvert {
    type Error = ParseError;

    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != PAYLOAD_SIZE {
            return Err(ParseError::InvalidPayload {
                expected: PAYLOAD_SIZE,
                actual: payload.len(),
            });
        }

        // Safety: Length check ensures slices are valid
        let date_ticks = u64::from_le_bytes(payload[0..8].try_into().unwrap()) & 0x3FFFFFFFFFFFFFFF;
        let sync_time_seconds = f64::from_le_bytes(payload[8..16].try_into().unwrap());

        // .NET DateTime Ticks conversion
        let unix_seconds = ((date_ticks as i64) - TICKS_TO_UNIX_EPOCH) / TICKS_PER_SECOND - JST_OFFSET_SECONDS;
        let unix_nanos = (((date_ticks as i64) - TICKS_TO_UNIX_EPOCH) % TICKS_PER_SECOND) * 100;

        let date_time = DateTime::from_timestamp(unix_seconds, unix_nanos as u32)
            .unwrap_or_else(Utc::now);

        Ok(DateTimeConvert {
            date_time,
            sync_time: sync_time_seconds / 3600.0, // Convert seconds to hours
        })
    }
}

// --- TimelineCommandPacket ---

#[derive(Debug, Clone, Default)]
pub struct TimelineCommandPacket {
    pub timeline_id: i64,
    pub start_time_sec: f64
}

impl CommandPacket for TimelineCommandPacket {}

impl Display for TimelineCommandPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hour = (self.start_time_sec / 3600.0) as i32;
        let minute = ((self.start_time_sec % 3600.0) / 60.0) as i32;
        let second = self.start_time_sec % 60.0;
        write!(
            f,
            "TimelineCommandPacket {{ timeline_id: {}, start_time_sec: {:.2}, which is {}h{}m{:.2}s }}",
            self.timeline_id, self.start_time_sec, hour, minute, second
        )
    }
}

impl TryFrom<&[u8]> for TimelineCommandPacket {
    type Error = ParseError;

    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != PAYLOAD_SIZE {
            return Err(ParseError::InvalidPayload {
                expected: PAYLOAD_SIZE,
                actual: payload.len(),
            });
        }

        // Safety: Length check ensures slices are valid
        let timeline_id = i64::from_le_bytes(payload[0..8].try_into().unwrap());
        let start_time_sec = f64::from_le_bytes(payload[8..16].try_into().unwrap());

        Ok(TimelineCommandPacket {
            timeline_id,
            start_time_sec,
        })
    }
}

// --- UpdateObject Extension ---

pub trait UpdateObjectExt {
    /// Generic method to parse payload into any type that implements Packet
    fn try_parse_as<T: CommandPacket>(&self) -> Result<T, ParseError>;
}

impl UpdateObjectExt for UpdateObject {
    fn try_parse_as<T: CommandPacket>(&self) -> Result<T, ParseError> {
        T::try_from(&self.payload)
    }
}