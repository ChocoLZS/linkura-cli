use super::super::{ParseError, extract};
use super::primitives::{
    parse_memorypack_f32, parse_memorypack_f64, parse_memorypack_i32, parse_memorypack_i64,
    parse_memorypack_string_with_len, parse_memorypack_vector3,
};

pub(crate) fn parse_virtual_camera_sync_parameter(
    payload: &[u8],
) -> Result<extract::VirtualCameraSyncParameter, ParseError> {
    if payload.len() < 24 {
        return Err(ParseError::InvalidPayload {
            expected: 24,
            actual: payload.len(),
        });
    }

    Ok(extract::VirtualCameraSyncParameter {
        compressed_position: parse_memorypack_i64(&payload[0..8])?,
        compressed_rotation: parse_memorypack_i64(&payload[8..16])?,
        sync_time: parse_memorypack_f64(&payload[16..24])?,
    })
}

pub(crate) fn parse_move_handy_camera_packet(
    payload: &[u8],
) -> Result<extract::MoveHandyCameraPacket, ParseError> {
    if payload.len() < 36 {
        return Err(ParseError::InvalidPayload {
            expected: 36,
            actual: payload.len(),
        });
    }

    Ok(extract::MoveHandyCameraPacket {
        position: parse_memorypack_vector3(&payload[0..12])?,
        rotation: super::primitives::parse_memorypack_quaternion(&payload[12..28])?,
        view_size: parse_memorypack_f32(&payload[28..32])?,
        is_orthographic: payload[32] != 0,
    })
}

pub(crate) fn parse_switcher_packet(payload: &[u8]) -> Result<extract::SwitcherPacket, ParseError> {
    if payload.len() < 36 {
        return Err(ParseError::InvalidPayload {
            expected: 36,
            actual: payload.len(),
        });
    }

    Ok(extract::SwitcherPacket {
        program_pass_camera_type: parse_memorypack_i32(&payload[0..4])?,
        program_pass_camera_id: parse_memorypack_i32(&payload[4..8])?,
        preset_pass_camera_type: parse_memorypack_i32(&payload[8..12])?,
        preset_pass_camera_id: parse_memorypack_i32(&payload[12..16])?,
        program_pass_alpha: parse_memorypack_f32(&payload[16..20])?,
        fade_time: parse_memorypack_f32(&payload[20..24])?,
        timeline_default_camera_type: parse_memorypack_i32(&payload[24..28])?,
        timeline_default_camera_id: parse_memorypack_i32(&payload[28..32])?,
        should_sync_timeline_default_pos: payload[32] != 0,
    })
}

pub(crate) fn parse_zoom_command(payload: &[u8]) -> Result<extract::ZoomCommand, ParseError> {
    if payload.len() < 8 {
        return Err(ParseError::InvalidPayload {
            expected: 8,
            actual: payload.len(),
        });
    }

    Ok(extract::ZoomCommand {
        camera_id: parse_memorypack_i32(&payload[0..4])?,
        zoom: parse_memorypack_f32(&payload[4..8])?,
    })
}

pub(crate) fn parse_fixed_camera_packet(payload: &[u8]) -> Result<extract::FixedCameraPacket, ParseError> {
    if payload.len() < 5 {
        return Err(ParseError::InvalidPayload {
            expected: 5,
            actual: payload.len(),
        });
    }

    // Observed MemoryPack layout:
    // [memberCount:u8][Name:string][Position:Vector3][RotationEuler:Vector3][ViewSize:f32][IsOrthographic:bool][Index?:i32]
    let member_count = payload[0];
    let (name, name_bytes) = parse_memorypack_string_with_len(&payload[1..])?;

    let mut cursor = 1 + name_bytes;
    if cursor + 12 + 12 + 4 + 1 > payload.len() {
        return Err(ParseError::InvalidPayload {
            expected: cursor + 12 + 12 + 4 + 1,
            actual: payload.len(),
        });
    }

    let position = parse_memorypack_vector3(&payload[cursor..cursor + 12])?;
    cursor += 12;
    let rotation = parse_memorypack_vector3(&payload[cursor..cursor + 12])?;
    cursor += 12;
    let view_size = parse_memorypack_f32(&payload[cursor..cursor + 4])?;
    cursor += 4;
    let is_orthographic = payload[cursor] != 0;
    cursor += 1;

    let index = if cursor + 4 <= payload.len() {
        Some(parse_memorypack_i32(&payload[cursor..cursor + 4])?)
    } else {
        None
    };

    Ok(extract::FixedCameraPacket {
        member_count,
        name,
        position,
        rotation,
        view_size,
        is_orthographic,
        index,
    })
}
