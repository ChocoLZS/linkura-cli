use super::define::{InstantiateObject, UpdateObject};

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

mod init_specs;
mod parsers;
mod prefab_router;
mod update_handlers;

use init_specs::{property_name_for_kind_opt, summarize_init_property_value_by_kind};
use parsers::misc::parse_character_model_initial_data_v0;
use parsers::primitives::parse_memorypack_i32;
use prefab_router::is_costume_prefab;

pub use prefab_router::{
    PrefabKind, detect_prefab_kind, normalize_prefab_name, parse_update_payload_text, prefab_name,
};

pub trait UpdateObjectExt {
    /// DateTimeReceiver payload
    fn try_parse_date_time(&self) -> Result<extract::DateTimeConvert, ParseError>;
    fn try_parse_cover_image(&self) -> Result<extract::CoverImageReceiver, ParseError>;
    fn try_parse_scene_prop_manipulator(&self)
    -> Result<extract::ScenePropManipulator, ParseError>;
    fn try_parse_foot_shadow_manipulator(
        &self,
    ) -> Result<extract::FootShadowManipulator, ParseError>;
    fn try_parse_expression_communicator(
        &self,
    ) -> Result<extract::ExpressionCommunicator, ParseError>;
    fn try_parse_lip_communicator(&self) -> Result<extract::LipCommunicator, ParseError>;
    fn try_parse_pose_communicator(&self) -> Result<extract::PoseCommunicator, ParseError>;
    fn try_parse_visible_communicator(&self) -> Result<extract::VisibleCommunicator, ParseError>;
    fn try_parse_finger_leap_communicator(
        &self,
    ) -> Result<extract::FingerLeapCommunicator, ParseError>;
    fn try_parse_character_item_manipulator(
        &self,
    ) -> Result<extract::CharacterItemManipulator, ParseError>;
    fn try_parse_virtual_camera_container(
        &self,
    ) -> Result<extract::VirtualCameraContainer, ParseError>;
    fn try_parse_cameraman(&self) -> Result<extract::CameramanReceiver, ParseError>;
    fn try_parse_motion_communicator(&self) -> Result<extract::MotionCommunicator, ParseError>;
    fn try_parse_switch_receiver(&self) -> Result<extract::SwitchReceiver, ParseError>;
    fn try_parse_music_broadcaster(&self) -> Result<extract::MusicBroadcaster, ParseError>;
}

pub trait InstantiateObjectExt {
    fn try_parse_init_data(
        &self,
        prefab_name: &str,
    ) -> Result<extract::InstantiateInitData, ParseError>;
}

impl UpdateObjectExt for UpdateObject {
    fn try_parse_date_time(&self) -> Result<extract::DateTimeConvert, ParseError> {
        update_handlers::try_parse_date_time(self)
    }

    /// 鐢?ida-pro-mcp 鐪嬩笅鏉ワ紝0x4C5BE1C 鏈韩鍙槸 CoverImageCommandFormatter.Serialize 鐨勫３锛岀湡姝ｅ簭鍒楀寲閫昏緫鍦ㄥ畠璺宠浆鍒?    /// 鐨?0x4C5BB68 (Inspix.CoverImageCommand$$Serialize)銆?    ///
    /// 缁撹鏄畠鎶?CoverImageCommand 搴忓垪鍖栨垚 MemoryPack 浜岃繘鍒讹紝瀛楁椤哄簭鏄細
    ///
    /// 1. 鍏堝啓瀵硅薄鎴愬憳鏁?2锛? 瀛楄妭锛屽€?0x02锛屾潵鑷?sub_4C5CE2C(writer, 2)锛?    /// 2. 鍐欏瓧绗︿覆 CoverImageName
    /// - 绌轰覆 -> 00 00 00 00锛坕nt32 0锛?    /// - 闈炵┖ -> 鍐欓暱搴﹀拰鍐呭锛堝父瑙佹槸 UTF-16锛涗篃鏀寔 UTF-8 璺緞锛?    ///
    /// 3. 鍐?SyncTime 涓?double 鍘熷 8 瀛楄妭锛圛EEE754锛屽皬绔級
    ///
    /// 绀轰緥锛圕overImageName = "A", SyncTime = 1.5锛孶TF-16 璺緞锛夛細
    ///
    /// 02
    /// 01 00 00 00
    /// 41 00
    /// 00 00 00 00 00 00 F8 3F
    ///
    /// 鍚箟灏辨槸锛歮emberCount=2锛屽瓧绗︿覆 "A"锛岀劧鍚?double(1.5)銆?
    fn try_parse_cover_image(&self) -> Result<extract::CoverImageReceiver, ParseError> {
        update_handlers::try_parse_cover_image(self)
    }

    fn try_parse_scene_prop_manipulator(
        &self,
    ) -> Result<extract::ScenePropManipulator, ParseError> {
        update_handlers::try_parse_scene_prop_manipulator(self)
    }

    fn try_parse_foot_shadow_manipulator(
        &self,
    ) -> Result<extract::FootShadowManipulator, ParseError> {
        update_handlers::try_parse_foot_shadow_manipulator(self)
    }

    fn try_parse_expression_communicator(
        &self,
    ) -> Result<extract::ExpressionCommunicator, ParseError> {
        update_handlers::try_parse_expression_communicator(self)
    }

    fn try_parse_lip_communicator(&self) -> Result<extract::LipCommunicator, ParseError> {
        update_handlers::try_parse_lip_communicator(self)
    }

    fn try_parse_pose_communicator(&self) -> Result<extract::PoseCommunicator, ParseError> {
        update_handlers::try_parse_pose_communicator(self)
    }

    fn try_parse_visible_communicator(&self) -> Result<extract::VisibleCommunicator, ParseError> {
        update_handlers::try_parse_visible_communicator(self)
    }

    fn try_parse_finger_leap_communicator(
        &self,
    ) -> Result<extract::FingerLeapCommunicator, ParseError> {
        update_handlers::try_parse_finger_leap_communicator(self)
    }

    fn try_parse_character_item_manipulator(
        &self,
    ) -> Result<extract::CharacterItemManipulator, ParseError> {
        update_handlers::try_parse_character_item_manipulator(self)
    }

    fn try_parse_virtual_camera_container(
        &self,
    ) -> Result<extract::VirtualCameraContainer, ParseError> {
        update_handlers::try_parse_virtual_camera_container(self)
    }

    fn try_parse_cameraman(&self) -> Result<extract::CameramanReceiver, ParseError> {
        update_handlers::try_parse_cameraman(self)
    }

    fn try_parse_motion_communicator(&self) -> Result<extract::MotionCommunicator, ParseError> {
        update_handlers::try_parse_motion_communicator(self)
    }

    fn try_parse_switch_receiver(&self) -> Result<extract::SwitchReceiver, ParseError> {
        update_handlers::try_parse_switch_receiver(self)
    }

    fn try_parse_music_broadcaster(&self) -> Result<extract::MusicBroadcaster, ParseError> {
        update_handlers::try_parse_music_broadcaster(self)
    }
}

impl InstantiateObjectExt for InstantiateObject {
    fn try_parse_init_data(
        &self,
        prefab_name: &str,
    ) -> Result<extract::InstantiateInitData, ParseError> {
        parse_instantiate_init_data(prefab_name, &self.init_data)
    }
}

fn parse_instantiate_init_data(
    prefab_name: &str,
    init_data: &[u8],
) -> Result<extract::InstantiateInitData, ParseError> {
    let prefab_kind = detect_prefab_kind(prefab_name);
    let costume_prefab = is_costume_prefab(prefab_name);

    let mut parsed = extract::InstantiateInitData {
        total_len: init_data.len(),
        ..Default::default()
    };

    if init_data.is_empty() {
        parsed.note = Some("empty init_data".to_string());
        return Ok(parsed);
    }

    if init_data.len() < 9 {
        return Err(ParseError::InvalidPayload {
            expected: 9,
            actual: init_data.len(),
        });
    }

    let marker = init_data[0];
    let version = parse_memorypack_i32(&init_data[1..5])?;
    let declared_body_len = parse_memorypack_i32(&init_data[5..9])?;
    if declared_body_len < 0 {
        return Err(ParseError::InvalidPayload {
            expected: 0,
            actual: init_data.len(),
        });
    }

    let declared_body_len = declared_body_len as usize;
    let body = &init_data[9..];
    parsed.marker = Some(marker);
    parsed.version = Some(version);
    parsed.declared_body_len = Some(declared_body_len);

    if declared_body_len != body.len() {
        parsed.note = Some(format!(
            "declared_body_len={} but actual_body_len={}",
            declared_body_len,
            body.len()
        ));
    }

    // Legacy envelope observed on costume prefabs.
    if marker == 2 && version == 0 {
        parse_legacy_init_data_v0(costume_prefab, body, &mut parsed);
        return Ok(parsed);
    }

    // Observed dominant shape: marker=2, version=1,
    // body = [tag=1][property_count:i32][repeated rpc_id:u8 + len:i32 + payload]
    if marker == 2 && version == 1 {
        if body.is_empty() {
            return Ok(parsed);
        }

        if body.len() < 5 {
            parsed.note = Some("body too short for property list".to_string());
            parsed.raw_preview = Some(hex_preview(body, 32));
            return Ok(parsed);
        }

        let body_tag = body[0];
        parsed.body_tag = Some(body_tag);
        if body_tag != 1 {
            parsed.note = Some(format!("unsupported body_tag={}", body_tag));
            parsed.raw_preview = Some(hex_preview(body, 48));
            return Ok(parsed);
        }

        let property_count = parse_memorypack_i32(&body[1..5])?;
        if property_count < 0 {
            parsed.note = Some(format!("invalid property_count={}", property_count));
            parsed.raw_preview = Some(hex_preview(body, 48));
            return Ok(parsed);
        }
        parsed.property_count = Some(property_count as usize);

        let mut cursor = 5usize;
        for _ in 0..property_count {
            if cursor + 5 > body.len() {
                parsed.note = Some("property header truncated".to_string());
                break;
            }

            let rpc_id = body[cursor];
            cursor += 1;

            let payload_len = parse_memorypack_i32(&body[cursor..cursor + 4])?;
            cursor += 4;
            if payload_len < 0 {
                parsed.note = Some(format!(
                    "invalid payload_len={} for rpc={}",
                    payload_len, rpc_id
                ));
                break;
            }
            let payload_len = payload_len as usize;
            if cursor + payload_len > body.len() {
                parsed.note = Some(format!(
                    "payload out of range for rpc={}, len={}, remaining={}",
                    rpc_id,
                    payload_len,
                    body.len().saturating_sub(cursor)
                ));
                break;
            }

            let payload = &body[cursor..cursor + payload_len];
            cursor += payload_len;

            let property_name = property_name_for_kind_opt(prefab_kind, rpc_id)
                .map(ToString::to_string)
                .unwrap_or_else(|| format!("Rpc{}", rpc_id));
            let value_summary =
                summarize_init_property_value_by_kind(prefab_kind, rpc_id, payload);

            parsed.properties.push(extract::InstantiateProperty {
                rpc_id,
                property_name,
                payload_len,
                value_summary,
            });
        }

        if cursor < body.len() {
            parsed.raw_preview = Some(hex_preview(&body[cursor..], 48));
        }

        return Ok(parsed);
    }

    // fallback for alternative init_data layouts (e.g., avatar costume object init blob)
    parsed.note = Some(format!(
        "unsupported envelope marker/version = {}/{}",
        marker, version
    ));
    parsed.raw_preview = Some(hex_preview(body, 64));
    Ok(parsed)
}

fn hex_preview(payload: &[u8], limit: usize) -> String {
    let take = payload.len().min(limit);
    let mut s = payload[..take]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ");
    if payload.len() > take {
        s.push_str(" ...");
    }
    s
}

fn parse_legacy_init_data_v0(costume_prefab: bool, body: &[u8], parsed: &mut extract::InstantiateInitData) {
    if body.is_empty() {
        parsed.note = Some("legacy envelope version=0 with empty body".to_string());
        return;
    }

    if body.len() % 4 != 0 {
        parsed.note = Some("legacy envelope version=0 body is not 4-byte aligned".to_string());
        parsed.raw_preview = Some(hex_preview(body, 64));
        return;
    }

    if costume_prefab {
        if let Ok(model) = parse_character_model_initial_data_v0(body) {
            parsed.note =
                Some("legacy envelope version=0 parsed as CharacterModelInitialData".to_string());
            parsed.property_count = Some(4);
            parsed.properties.push(extract::InstantiateProperty {
                rpc_id: 0,
                property_name: "CharacterId".to_string(),
                payload_len: 4,
                value_summary: format!("i32({})", model.character_id),
            });
            parsed.properties.push(extract::InstantiateProperty {
                rpc_id: 1,
                property_name: "CostumeId".to_string(),
                payload_len: 4,
                value_summary: format!("i32({})", model.costume_id),
            });
            parsed.properties.push(extract::InstantiateProperty {
                rpc_id: 2,
                property_name: "Position".to_string(),
                payload_len: 12,
                value_summary: format!(
                    "Vector3({:.3}, {:.3}, {:.3})",
                    model.position.x, model.position.y, model.position.z
                ),
            });
            parsed.properties.push(extract::InstantiateProperty {
                rpc_id: 3,
                property_name: "Rotation".to_string(),
                payload_len: 16,
                value_summary: format!(
                    "Quaternion({:.3}, {:.3}, {:.3}, {:.3})",
                    model.rotation.x, model.rotation.y, model.rotation.z, model.rotation.w
                ),
            });
            return;
        }
    }

    parsed.note = Some("legacy envelope version=0 parsed as 4-byte words".to_string());

    let words = body.len() / 4;
    parsed.property_count = Some(words);
    for idx in 0..words {
        let start = idx * 4;
        let end = start + 4;
        let chunk = &body[start..end];
        let as_u32 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let as_i32 = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let as_f32 = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);

        let property_name = if idx == 0 && costume_prefab {
            "CharacterId".to_string()
        } else {
            format!("Word{}", idx)
        };

        parsed.properties.push(extract::InstantiateProperty {
            rpc_id: idx as u8,
            property_name,
            payload_len: 4,
            value_summary: format!("i32({}), u32({}), f32({:.6})", as_i32, as_u32, as_f32),
        });
    }
}

pub mod extract;



