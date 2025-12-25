use std::fmt::Display;

use chrono::{DateTime, Utc};

use super::define::UpdateObject;

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

#[derive(Debug, Clone)]
pub struct DateTimeConvert {
    pub date_time: DateTime<Utc>, // ticks in c#
    pub sync_time: f64,
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

impl Default for DateTimeConvert {
    fn default() -> Self {
        DateTimeConvert {
            date_time: Utc::now(),
            sync_time: 0.0,
        }
    }
}

pub trait UpdateObjectExt {
    /// DateTimeReceiver payload
    fn try_parse_date_time(&self) -> Result<DateTimeConvert, ParseError>;
}

impl UpdateObjectExt for UpdateObject {
    fn try_parse_date_time(&self) -> Result<DateTimeConvert, ParseError> {
        // 16 bytes: 8 bytes for ticks (u64) + 8 bytes for sync_time (f64)
        if self.payload.len() != 16 {
            return Err(ParseError::InvalidPayload {
                expected: 16,
                actual: self.payload.len(),
            });
        }

        // 小端序解析
        let date_ticks = self.payload[0..8]
            .try_into()
            .map(u64::from_le_bytes)
            .map_err(|_| ParseError::InvalidPayload {
                expected: 8,
                actual: self.payload[0..8].len(),
            })?;
        let sync_time_seconds = self.payload[8..16]
            .try_into()
            .map(f64::from_le_bytes)
            .map_err(|_| ParseError::InvalidPayload {
                expected: 8,
                actual: self.payload[8..16].len(),
            })?;

        // .NET DateTime Ticks 转换
        // Ticks从 0001-01-01 00:00:00 开始，每tick = 100纳秒
        // 提取实际的ticks值（去除高2位的DateTimeKind标志）
        let actual_ticks = date_ticks & 0x3FFFFFFFFFFFFFFF;

        // .NET epoch: 0001-01-01 00:00:00
        // 转换为 Unix 时间戳（1970-01-01 00:00:00）
        // 从 0001-01-01 到 1970-01-01 的 ticks
        const TICKS_TO_UNIX_EPOCH: i64 = 621355968000000000; // 从0001-01-01到1970-01-01的ticks数
        const TICKS_PER_SECOND: i64 = 10000000; // 每秒的ticks数 (1秒 = 10,000,000个100纳秒)

        // 计算从Unix epoch开始的秒数和纳秒
        let unix_seconds = ((actual_ticks as i64) - TICKS_TO_UNIX_EPOCH) / TICKS_PER_SECOND;
        let unix_nanos = (((actual_ticks as i64) - TICKS_TO_UNIX_EPOCH) % TICKS_PER_SECOND) * 100;

        // 创建 DateTime<Utc>
        // Asia/Tokyo (JST) is UTC+9
        // 数据是东九区(UTC+9)时间,需要减去9小时转换为UTC
        const JST_OFFSET_SECONDS: i64 = 9 * 3600; // 东九区偏移量(秒)
        let utc_seconds = unix_seconds - JST_OFFSET_SECONDS;
        let date_time =
            DateTime::from_timestamp(utc_seconds, unix_nanos as u32).unwrap_or_else(|| Utc::now());

        // SyncTime 从秒转换为小时
        let sync_time_hours = sync_time_seconds / 3600.0;

        Ok(DateTimeConvert {
            date_time,
            sync_time: sync_time_hours,
        })
    }
}
