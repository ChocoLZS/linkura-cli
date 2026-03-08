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

pub mod prefab_name {
    pub const DATE_TIME_RECEIVER: &str = "TimedAsset/DateTimeReceiver";
    pub const MUSIC_BROADCASTER: &str = "VoiceObject/MusicBroadcaster";
    pub const COVER_IMAGE_RECEIVER: &str = "CoverImageReceiver";
    pub const SCENE_PROP_MANIPULATOR: &str = "Prefabs/ScenePropManipulator";
}
pub trait UpdateObjectExt {
    /// DateTimeReceiver payload
    fn try_parse_date_time(&self) -> Result<extract::DateTimeConvert, ParseError>;
    fn try_parse_cover_image(&self) -> Result<extract::CoverImageReceiver, ParseError>;
    fn try_parse_scene_prop_manipulator(&self)
    -> Result<extract::ScenePropManipulator, ParseError>;
}

impl UpdateObjectExt for UpdateObject {
    fn try_parse_date_time(&self) -> Result<extract::DateTimeConvert, ParseError> {
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

        Ok(extract::DateTimeConvert {
            date_time,
            sync_time: sync_time_hours,
        })
    }

    /// 用 ida-pro-mcp 看下来，0x4C5BE1C 本身只是 CoverImageCommandFormatter.Serialize 的壳，真正序列化逻辑在它跳转到
    /// 的 0x4C5BB68 (Inspix.CoverImageCommand$$Serialize)。
    ///
    /// 结论是它把 CoverImageCommand 序列化成 MemoryPack 二进制，字段顺序是：
    ///
    /// 1. 先写对象成员数 2（1 字节，值 0x02，来自 sub_4C5CE2C(writer, 2)）
    /// 2. 写字符串 CoverImageName
    /// - 空串 -> 00 00 00 00（int32 0）
    /// - 非空 -> 写长度和内容（常见是 UTF-16；也支持 UTF-8 路径）
    ///
    /// 3. 写 SyncTime 为 double 原始 8 字节（IEEE754，小端）
    ///
    /// 示例（CoverImageName = "A", SyncTime = 1.5，UTF-16 路径）：
    ///
    /// 02
    /// 01 00 00 00
    /// 41 00
    /// 00 00 00 00 00 00 F8 3F
    ///
    /// 含义就是：memberCount=2，字符串 "A"，然后 double(1.5)。
    fn try_parse_cover_image(&self) -> Result<extract::CoverImageReceiver, ParseError> {
        if self.payload.len() != 47 {
            // 1 + 4 + 4 + len(name) + 8(sync time)
            return Err(ParseError::InvalidPayload {
                expected: 47,
                actual: self.payload.len(),
            });
        }
        let len = u32::from_le_bytes(self.payload[5..9].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: self.payload[5..9].len(),
            }
        })?);
        let cover_image_name =
            String::from_utf8_lossy(&self.payload[9..9 + len as usize]).to_string();
        // last 8 bytes
        let sync_time_seconds = f64::from_le_bytes(
            self.payload[9 + len as usize..9 + len as usize + 8]
                .try_into()
                .map_err(|_| ParseError::InvalidPayload {
                    expected: 8,
                    actual: self.payload[9 + len as usize..9 + len as usize + 8].len(),
                })?,
        );
        let sync_time_hours = sync_time_seconds / 3600.0;
        // Implementation for parsing cover image payload
        // This is a placeholder - replace with actual parsing logic
        Ok(extract::CoverImageReceiver {
            cover_image_name,
            sync_time: sync_time_hours,
        })
    }

    fn try_parse_scene_prop_manipulator(
        &self,
    ) -> Result<extract::ScenePropManipulator, ParseError> {
        const METHOD_PROP_ID: i32 = 0;
        const METHOD_WORLD_POSITION: i32 = 1;
        const METHOD_WORLD_ROTATION: i32 = 2;
        const METHOD_IS_VISIBLE: i32 = 3;
        const METHOD_ANIMATION_TRIGGER: i32 = 10;

        match self.method {
            METHOD_PROP_ID => {
                let prop_id = parse_memorypack_i32(&self.payload)?;
                Ok(extract::ScenePropManipulator {
                    method: self.method,
                    prop_id: Some(prop_id),
                    world_position: None,
                    world_rotation: None,
                    is_visible: None,
                    animation_trigger: None,
                })
            }
            METHOD_WORLD_POSITION => {
                let world_position = parse_memorypack_vector3(&self.payload)?;
                Ok(extract::ScenePropManipulator {
                    method: self.method,
                    prop_id: None,
                    world_position: Some(world_position),
                    world_rotation: None,
                    is_visible: None,
                    animation_trigger: None,
                })
            }
            METHOD_WORLD_ROTATION => {
                let world_rotation = parse_memorypack_quaternion(&self.payload)?;
                Ok(extract::ScenePropManipulator {
                    method: self.method,
                    prop_id: None,
                    world_position: None,
                    world_rotation: Some(world_rotation),
                    is_visible: None,
                    animation_trigger: None,
                })
            }
            METHOD_IS_VISIBLE => {
                let is_visible = parse_memorypack_bool(&self.payload)?;
                Ok(extract::ScenePropManipulator {
                    method: self.method,
                    prop_id: None,
                    world_position: None,
                    world_rotation: None,
                    is_visible: Some(is_visible),
                    animation_trigger: None,
                })
            }
            METHOD_ANIMATION_TRIGGER => {
                let animation_trigger = parse_memorypack_string(&self.payload)?;
                Ok(extract::ScenePropManipulator {
                    method: self.method,
                    prop_id: None,
                    world_position: None,
                    world_rotation: None,
                    is_visible: None,
                    animation_trigger: Some(animation_trigger),
                })
            }
            _ => parse_scene_prop_by_shape(self.method, &self.payload),
        }
    }
}

fn parse_scene_prop_by_shape(
    method: i32,
    payload: &[u8],
) -> Result<extract::ScenePropManipulator, ParseError> {
    if let Ok(value) = parse_memorypack_bool(payload) {
        if payload.len() == 1 {
            return Ok(extract::ScenePropManipulator {
                method,
                prop_id: None,
                world_position: None,
                world_rotation: None,
                is_visible: Some(value),
                animation_trigger: None,
            });
        }
    }

    if payload.len() >= 4 {
        let marker = i32::from_le_bytes(payload[0..4].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            }
        })?);
        let looks_like_string = if marker == -1 || marker == 0 {
            payload.len() == 4
        } else if marker > 0 {
            payload.len() == 4 + (marker as usize * 2)
        } else if payload.len() >= 8 {
            payload.len() == 8 + (!marker) as usize
        } else {
            false
        };

        if looks_like_string {
            if let Ok(value) = parse_memorypack_string(payload) {
                return Ok(extract::ScenePropManipulator {
                    method,
                    prop_id: None,
                    world_position: None,
                    world_rotation: None,
                    is_visible: None,
                    animation_trigger: Some(value),
                });
            }
        }
    }

    if let Ok(value) = parse_memorypack_i32(payload) {
        if payload.len() == 4 {
            return Ok(extract::ScenePropManipulator {
                method,
                prop_id: Some(value),
                world_position: None,
                world_rotation: None,
                is_visible: None,
                animation_trigger: None,
            });
        }
    }

    if let Ok(value) = parse_memorypack_vector3(payload) {
        if payload.len() == 12 {
            return Ok(extract::ScenePropManipulator {
                method,
                prop_id: None,
                world_position: Some(value),
                world_rotation: None,
                is_visible: None,
                animation_trigger: None,
            });
        }
    }

    if let Ok(value) = parse_memorypack_quaternion(payload) {
        if payload.len() == 16 {
            return Ok(extract::ScenePropManipulator {
                method,
                prop_id: None,
                world_position: None,
                world_rotation: Some(value),
                is_visible: None,
                animation_trigger: None,
            });
        }
    }

    Err(ParseError::UnknownMethod(method))
}

fn parse_memorypack_i32(payload: &[u8]) -> Result<i32, ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    Ok(i32::from_le_bytes(payload[0..4].try_into().map_err(
        |_| ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        },
    )?))
}

fn parse_memorypack_vector3(payload: &[u8]) -> Result<extract::Vector3, ParseError> {
    if payload.len() < 12 {
        return Err(ParseError::InvalidPayload {
            expected: 12,
            actual: payload.len(),
        });
    }
    Ok(extract::Vector3 {
        x: f32::from_le_bytes(payload[0..4].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            }
        })?),
        y: f32::from_le_bytes(payload[4..8].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 8,
                actual: payload.len(),
            }
        })?),
        z: f32::from_le_bytes(payload[8..12].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 12,
                actual: payload.len(),
            }
        })?),
    })
}

fn parse_memorypack_quaternion(payload: &[u8]) -> Result<extract::Quaternion, ParseError> {
    if payload.len() < 16 {
        return Err(ParseError::InvalidPayload {
            expected: 16,
            actual: payload.len(),
        });
    }
    Ok(extract::Quaternion {
        x: f32::from_le_bytes(payload[0..4].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            }
        })?),
        y: f32::from_le_bytes(payload[4..8].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 8,
                actual: payload.len(),
            }
        })?),
        z: f32::from_le_bytes(payload[8..12].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 12,
                actual: payload.len(),
            }
        })?),
        w: f32::from_le_bytes(payload[12..16].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 16,
                actual: payload.len(),
            }
        })?),
    })
}

fn parse_memorypack_bool(payload: &[u8]) -> Result<bool, ParseError> {
    if payload.is_empty() {
        return Err(ParseError::InvalidPayload {
            expected: 1,
            actual: 0,
        });
    }
    Ok(payload[0] != 0)
}

fn parse_memorypack_string(payload: &[u8]) -> Result<String, ParseError> {
    if payload.len() < 4 {
        return Err(ParseError::InvalidPayload {
            expected: 4,
            actual: payload.len(),
        });
    }

    let marker =
        i32::from_le_bytes(
            payload[0..4]
                .try_into()
                .map_err(|_| ParseError::InvalidPayload {
                    expected: 4,
                    actual: payload.len(),
                })?,
        );

    if marker == -1 || marker == 0 {
        if payload.len() != 4 {
            return Err(ParseError::InvalidPayload {
                expected: 4,
                actual: payload.len(),
            });
        }
        return Ok(String::new());
    }

    if marker > 0 {
        let utf16_units = marker as usize;
        let bytes_len = utf16_units * 2;
        let expected = 4 + bytes_len;
        if payload.len() < expected {
            return Err(ParseError::InvalidPayload {
                expected,
                actual: payload.len(),
            });
        }

        let mut units = Vec::with_capacity(utf16_units);
        for chunk in payload[4..expected].chunks_exact(2) {
            units.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
        return Ok(String::from_utf16_lossy(&units));
    }

    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    let utf8_len = (!marker) as usize;
    let expected = 8 + utf8_len;
    if payload.len() < expected {
        return Err(ParseError::InvalidPayload {
            expected,
            actual: payload.len(),
        });
    }

    Ok(String::from_utf8_lossy(&payload[8..expected]).to_string())
}

pub mod extract {
    use std::fmt::Display;

    use chrono::{DateTime, Utc};

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

    pub struct CoverImageReceiver {
        pub cover_image_name: String,
        pub sync_time: f64,
    }

    impl Display for CoverImageReceiver {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "CoverImage: {}, SyncTime: {}h{}m{:.2}s",
                self.cover_image_name,
                self.sync_time as i32 / 3600,
                (self.sync_time as i32 % 3600) / 60,
                self.sync_time % 60.0
            )
        }
    }

    impl Default for CoverImageReceiver {
        fn default() -> Self {
            CoverImageReceiver {
                cover_image_name: String::new(),
                sync_time: 0.0,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct ScenePropManipulator {
        pub method: i32,
        pub prop_id: Option<i32>,
        pub world_position: Option<Vector3>,
        pub world_rotation: Option<Quaternion>,
        pub is_visible: Option<bool>,
        pub animation_trigger: Option<String>,
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Quaternion {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    impl Display for ScenePropManipulator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(value) = self.prop_id {
                return write!(
                    f,
                    "ScenePropManipulator(method={}): PropId={}",
                    self.method, value
                );
            }
            if let Some(value) = self.world_position {
                return write!(
                    f,
                    "ScenePropManipulator(method={}): WorldPosition=({:.6}, {:.6}, {:.6})",
                    self.method, value.x, value.y, value.z
                );
            }
            if let Some(value) = self.world_rotation {
                return write!(
                    f,
                    "ScenePropManipulator(method={}): WorldRotation=({:.6}, {:.6}, {:.6}, {:.6})",
                    self.method, value.x, value.y, value.z, value.w
                );
            }
            if let Some(value) = self.is_visible {
                return write!(
                    f,
                    "ScenePropManipulator(method={}): IsVisible={}",
                    self.method, value
                );
            }
            if let Some(trigger) = &self.animation_trigger {
                return write!(
                    f,
                    "ScenePropManipulator(method={}): AnimationTrigger={}",
                    self.method, trigger
                );
            }
            write!(f, "ScenePropManipulator(method={}): <empty>", self.method)
        }
    }
}
