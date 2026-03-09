use super::super::{ParseError, extract};
use super::primitives::{
    parse_memorypack_f32, parse_memorypack_f64, parse_memorypack_i32, parse_memorypack_u32,
};

pub(crate) fn parse_motion_packet_and_time(
    payload: &[u8],
) -> Result<extract::MotionPacketAndTime, ParseError> {
    if payload.len() < 12 {
        return Err(ParseError::InvalidPayload {
            expected: 12,
            actual: payload.len(),
        });
    }

    let motion_len = payload.len() - 8;
    let time_sec = parse_memorypack_f64(&payload[motion_len..])?;
    Ok(extract::MotionPacketAndTime {
        motion_len,
        time_sec,
    })
}

pub(crate) fn parse_music_broadcaster_payload(
    method: i32,
    payload: &[u8],
) -> Result<extract::MusicBroadcaster, ParseError> {
    if payload.len() < 28 {
        return Err(ParseError::InvalidPayload {
            expected: 28,
            actual: payload.len(),
        });
    }

    let header0 = parse_memorypack_u32(&payload[0..4])?;
    let header1 = parse_memorypack_u32(&payload[4..8])?;
    let header2 = parse_memorypack_u32(&payload[8..12])?;
    let header3 = parse_memorypack_u32(&payload[12..16])?;
    let sync_time = parse_memorypack_f64(&payload[16..24])?;
    let encoded_length = parse_memorypack_i32(&payload[24..28])?;
    if encoded_length < 0 {
        return Err(ParseError::InvalidPayload {
            expected: 0,
            actual: encoded_length as usize,
        });
    }
    let encoded_length = encoded_length as usize;
    let available = payload.len().saturating_sub(28);
    let encoded_available = encoded_length.min(available);

    Ok(extract::MusicBroadcaster {
        method,
        header0,
        header1,
        header2,
        header3,
        sync_time,
        encoded_length,
        encoded_available,
    })
}

pub(crate) fn parse_character_model_initial_data_v0(
    body: &[u8],
) -> Result<extract::CharacterModelInitialData, ParseError> {
    if body.len() != 36 {
        return Err(ParseError::InvalidPayload {
            expected: 36,
            actual: body.len(),
        });
    }

    Ok(extract::CharacterModelInitialData {
        character_id: parse_memorypack_i32(&body[0..4])?,
        costume_id: parse_memorypack_i32(&body[4..8])?,
        position: extract::Vector3 {
            x: parse_memorypack_f32(&body[8..12])?,
            y: parse_memorypack_f32(&body[12..16])?,
            z: parse_memorypack_f32(&body[16..20])?,
        },
        rotation: extract::Quaternion {
            x: parse_memorypack_f32(&body[20..24])?,
            y: parse_memorypack_f32(&body[24..28])?,
            z: parse_memorypack_f32(&body[28..32])?,
            w: parse_memorypack_f32(&body[32..36])?,
        },
    })
}
