use super::super::{ParseError, extract};
use super::primitives::{
    parse_memorypack_bool, parse_memorypack_f32, parse_memorypack_f64, parse_memorypack_i32,
    parse_memorypack_quaternion, parse_memorypack_string, parse_memorypack_vector3,
};

pub(crate) fn parse_scene_prop_by_shape(
    method: i32,
    payload: &[u8],
) -> Result<extract::ScenePropManipulator, ParseError> {
    if let Ok(value) = parse_memorypack_bool(payload) {
        if payload.len() == 1 {
            return Ok(extract::ScenePropManipulator::from_is_visible(method, value));
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
                return Ok(extract::ScenePropManipulator::from_animation_trigger(
                    method, value,
                ));
            }
        }
    }

    if let Ok(value) = parse_memorypack_i32(payload) {
        if payload.len() == 4 {
            return Ok(extract::ScenePropManipulator::from_prop_id(method, value));
        }
    }

    if let Ok(value) = parse_memorypack_vector3(payload) {
        if payload.len() == 12 {
            return Ok(extract::ScenePropManipulator::from_world_position(method, value));
        }
    }

    if let Ok(value) = parse_memorypack_quaternion(payload) {
        if payload.len() == 16 {
            return Ok(extract::ScenePropManipulator::from_world_rotation(method, value));
        }
    }

    Err(ParseError::UnknownMethod(method))
}

pub(crate) fn parse_foot_shadow_activate_command(
    payload: &[u8],
) -> Result<extract::FootShadowActivateCommand, ParseError> {
    if payload.len() < 16 {
        return Err(ParseError::InvalidPayload {
            expected: 16,
            actual: payload.len(),
        });
    }

    Ok(extract::FootShadowActivateCommand {
        is_active: payload[0] != 0,
        sync_time: f64::from_le_bytes(payload[8..16].try_into().map_err(|_| {
            ParseError::InvalidPayload {
                expected: 16,
                actual: payload.len(),
            }
        })?),
    })
}

pub(crate) fn parse_foot_shadow_by_shape(
    method: i32,
    payload: &[u8],
) -> Result<extract::FootShadowManipulator, ParseError> {
    if let Ok(value) = parse_foot_shadow_activate_command(payload) {
        if payload.len() == 16 {
            return Ok(extract::FootShadowManipulator::from_activate_command(
                method, value,
            ));
        }
    }

    if let Ok(value) = parse_memorypack_i32(payload) {
        if payload.len() == 4 {
            return Ok(extract::FootShadowManipulator::from_character_id(method, value));
        }
    }

    if let Ok(value) = parse_memorypack_bool(payload) {
        if payload.len() == 1 {
            return Ok(extract::FootShadowManipulator::from_is_active_only(
                method, value,
            ));
        }
    }

    Err(ParseError::UnknownMethod(method))
}

pub(crate) fn parse_expression_data(payload: &[u8]) -> Result<extract::ExpressionData, ParseError> {
    if payload.len() < 24 {
        return Err(ParseError::InvalidPayload {
            expected: 24,
            actual: payload.len(),
        });
    }

    Ok(extract::ExpressionData {
        next_id: parse_memorypack_i32(&payload[0..4])?,
        last_id: parse_memorypack_i32(&payload[4..8])?,
        weight: parse_memorypack_f32(&payload[8..12])?,
        is_switch_control: payload[12] != 0,
        send_time_sec: parse_memorypack_f64(&payload[16..24])?,
    })
}

pub(crate) fn parse_lip_sync_data(payload: &[u8]) -> Result<extract::LipSyncData, ParseError> {
    if payload.len() < 32 {
        return Err(ParseError::InvalidPayload {
            expected: 32,
            actual: payload.len(),
        });
    }

    Ok(extract::LipSyncData {
        mouth_a: parse_memorypack_f32(&payload[0..4])?,
        mouth_i: parse_memorypack_f32(&payload[4..8])?,
        mouth_u: parse_memorypack_f32(&payload[8..12])?,
        mouth_e: parse_memorypack_f32(&payload[12..16])?,
        mouth_o: parse_memorypack_f32(&payload[16..20])?,
        mouth_close: parse_memorypack_f32(&payload[20..24])?,
        send_time_sec: parse_memorypack_f64(&payload[24..32])?,
    })
}

pub(crate) fn parse_pose_packet(payload: &[u8]) -> Result<extract::PosePacket, ParseError> {
    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    Ok(extract::PosePacket {
        pose_id: parse_memorypack_i32(&payload[0..4])?,
        is_fixed: payload[4] != 0,
    })
}

pub(crate) fn parse_is_visible_packet(payload: &[u8]) -> Result<extract::IsVisiblePacket, ParseError> {
    if payload.len() < 16 {
        return Err(ParseError::InvalidPayload {
            expected: 16,
            actual: payload.len(),
        });
    }

    Ok(extract::IsVisiblePacket {
        is_visible: payload[0] != 0,
        sync_time: parse_memorypack_f64(&payload[8..16])?,
    })
}
