use chrono::{DateTime, Utc};

use crate::als::proto::{proto::als::{
    UpdateObject
}};

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

pub trait UpdateObjectExt {
    /// DateTimeReceiver payload
    fn try_parse_date_time(&self) -> Result<DateTimeConvert, ParseError>;
}

impl UpdateObjectExt for UpdateObject {
    fn try_parse_date_time(&self) -> Result<DateTimeConvert, ParseError> {
        // 16 bytes
        if self.payload.len() != 16 {
            return Err(ParseError::InvalidPayload { expected: 16, actual: self.payload.len() });
        }
        Ok(DateTimeConvert {
            date_time: chrono::Utc::now(), // TODO: implement actual parsing logic
            sync_time: 0.0, // TODO: implement actual parsing logic
        })
    }
}
