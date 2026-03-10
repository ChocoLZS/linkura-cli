use chrono::{DateTime, Utc};

use super::parsers::camera::{
    parse_move_handy_camera_packet, parse_switcher_packet, parse_virtual_camera_sync_parameter,
    parse_zoom_command,
};
use super::parsers::character::{
    parse_expression_data, parse_foot_shadow_activate_command, parse_foot_shadow_by_shape,
    parse_is_visible_packet, parse_lip_sync_data, parse_pose_packet, parse_scene_prop_by_shape,
};
use super::parsers::misc::{parse_motion_packet_and_time, parse_music_broadcaster_payload};
use super::parsers::primitives::{
    parse_memorypack_bool, parse_memorypack_f32, parse_memorypack_f64, parse_memorypack_i32,
    parse_memorypack_quaternion, parse_memorypack_string, parse_memorypack_string_with_len,
    parse_memorypack_u8, parse_memorypack_vector3,
};
use super::{ParseError, UpdateObject, extract};

pub(crate) fn try_parse_date_time(obj: &UpdateObject) -> Result<extract::DateTimeConvert, ParseError> {
    if obj.payload.len() != 16 {
        return Err(ParseError::InvalidPayload {
            expected: 16,
            actual: obj.payload.len(),
        });
    }

    let date_ticks = u64::from_le_bytes(
        obj.payload[0..8]
            .try_into()
            .map_err(|_| ParseError::InvalidPayload {
                expected: 8,
                actual: obj.payload[0..8].len(),
            })?,
    );
    let sync_time_seconds = f64::from_le_bytes(
        obj.payload[8..16]
            .try_into()
            .map_err(|_| ParseError::InvalidPayload {
                expected: 8,
                actual: obj.payload[8..16].len(),
            })?,
    );

    let actual_ticks = date_ticks & 0x3FFF_FFFF_FFFF_FFFF;
    const TICKS_TO_UNIX_EPOCH: i64 = 621_355_968_000_000_000;
    const TICKS_PER_SECOND: i64 = 10_000_000;
    let unix_seconds = ((actual_ticks as i64) - TICKS_TO_UNIX_EPOCH) / TICKS_PER_SECOND;
    let unix_nanos = (((actual_ticks as i64) - TICKS_TO_UNIX_EPOCH) % TICKS_PER_SECOND) * 100;

    const JST_OFFSET_SECONDS: i64 = 9 * 3600;
    let utc_seconds = unix_seconds - JST_OFFSET_SECONDS;
    let date_time = DateTime::from_timestamp(utc_seconds, unix_nanos as u32).unwrap_or_else(Utc::now);

    Ok(extract::DateTimeConvert {
        date_time,
        sync_time: sync_time_seconds / 3600.0,
    })
}

pub(crate) fn try_parse_cover_image(
    obj: &UpdateObject,
) -> Result<extract::CoverImageReceiver, ParseError> {
    if obj.payload.len() < 13 {
        return Err(ParseError::InvalidPayload {
            expected: 13,
            actual: obj.payload.len(),
        });
    }

    let (_, name_len) = parse_memorypack_string_with_len(&obj.payload[1..])?;
    let sync_start = 1 + name_len;
    if sync_start + 8 > obj.payload.len() {
        return Err(ParseError::InvalidPayload {
            expected: sync_start + 8,
            actual: obj.payload.len(),
        });
    }

    let cover_image_name = parse_memorypack_string(&obj.payload[1..1 + name_len])?;
    let sync_time_seconds = parse_memorypack_f64(&obj.payload[sync_start..sync_start + 8])?;
    Ok(extract::CoverImageReceiver {
        cover_image_name,
        sync_time: sync_time_seconds / 3600.0,
    })
}

pub(crate) fn try_parse_scene_prop_manipulator(
    obj: &UpdateObject,
) -> Result<extract::ScenePropManipulator, ParseError> {
    const METHOD_PROP_ID: i32 = 0;
    const METHOD_WORLD_POSITION: i32 = 1;
    const METHOD_WORLD_ROTATION: i32 = 2;
    const METHOD_IS_VISIBLE: i32 = 3;
    const METHOD_ANIMATION_TRIGGER: i32 = 10;

    match obj.method {
        METHOD_PROP_ID => Ok(extract::ScenePropManipulator::from_prop_id(
            obj.method,
            parse_memorypack_i32(&obj.payload)?,
        )),
        METHOD_WORLD_POSITION => Ok(extract::ScenePropManipulator::from_world_position(
            obj.method,
            parse_memorypack_vector3(&obj.payload)?,
        )),
        METHOD_WORLD_ROTATION => Ok(extract::ScenePropManipulator::from_world_rotation(
            obj.method,
            parse_memorypack_quaternion(&obj.payload)?,
        )),
        METHOD_IS_VISIBLE => Ok(extract::ScenePropManipulator::from_is_visible(
            obj.method,
            parse_memorypack_bool(&obj.payload)?,
        )),
        METHOD_ANIMATION_TRIGGER => Ok(extract::ScenePropManipulator::from_animation_trigger(
            obj.method,
            parse_memorypack_string(&obj.payload)?,
        )),
        _ => parse_scene_prop_by_shape(obj.method, &obj.payload),
    }
}

pub(crate) fn try_parse_foot_shadow_manipulator(
    obj: &UpdateObject,
) -> Result<extract::FootShadowManipulator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_ACTIVATE_COMMAND: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::FootShadowManipulator::from_character_id(
            obj.method,
            parse_memorypack_i32(&obj.payload)?,
        )),
        METHOD_ACTIVATE_COMMAND => Ok(extract::FootShadowManipulator::from_activate_command(
            obj.method,
            parse_foot_shadow_activate_command(&obj.payload)?,
        )),
        _ => parse_foot_shadow_by_shape(obj.method, &obj.payload),
    }
}

pub(crate) fn try_parse_expression_communicator(
    obj: &UpdateObject,
) -> Result<extract::ExpressionCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_EXPRESSION_DATA: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::ExpressionCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            expression_data: None,
        }),
        METHOD_EXPRESSION_DATA => Ok(extract::ExpressionCommunicator {
            method: obj.method,
            character_id: None,
            expression_data: Some(parse_expression_data(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 24 {
                Ok(extract::ExpressionCommunicator {
                    method: obj.method,
                    character_id: None,
                    expression_data: Some(parse_expression_data(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::ExpressionCommunicator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    expression_data: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_lip_communicator(
    obj: &UpdateObject,
) -> Result<extract::LipCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_LIP_SYNC_DATA: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::LipCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            lip_sync_data: None,
        }),
        METHOD_LIP_SYNC_DATA => Ok(extract::LipCommunicator {
            method: obj.method,
            character_id: None,
            lip_sync_data: Some(parse_lip_sync_data(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 32 {
                Ok(extract::LipCommunicator {
                    method: obj.method,
                    character_id: None,
                    lip_sync_data: Some(parse_lip_sync_data(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::LipCommunicator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    lip_sync_data: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_pose_communicator(
    obj: &UpdateObject,
) -> Result<extract::PoseCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_POSE_PACKET: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::PoseCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            pose_packet: None,
        }),
        METHOD_POSE_PACKET => Ok(extract::PoseCommunicator {
            method: obj.method,
            character_id: None,
            pose_packet: Some(parse_pose_packet(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 8 {
                Ok(extract::PoseCommunicator {
                    method: obj.method,
                    character_id: None,
                    pose_packet: Some(parse_pose_packet(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::PoseCommunicator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    pose_packet: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_visible_communicator(
    obj: &UpdateObject,
) -> Result<extract::VisibleCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_IS_VISIBLE_PACKET: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::VisibleCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            visible_packet: None,
        }),
        METHOD_IS_VISIBLE_PACKET => Ok(extract::VisibleCommunicator {
            method: obj.method,
            character_id: None,
            visible_packet: Some(parse_is_visible_packet(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 16 {
                Ok(extract::VisibleCommunicator {
                    method: obj.method,
                    character_id: None,
                    visible_packet: Some(parse_is_visible_packet(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::VisibleCommunicator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    visible_packet: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_finger_leap_communicator(
    obj: &UpdateObject,
) -> Result<extract::FingerLeapCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_LEAP_RATE: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::FingerLeapCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            leap_rate: None,
        }),
        METHOD_LEAP_RATE => Ok(extract::FingerLeapCommunicator {
            method: obj.method,
            character_id: None,
            leap_rate: Some(parse_memorypack_f32(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 4 {
                Ok(extract::FingerLeapCommunicator {
                    method: obj.method,
                    character_id: None,
                    leap_rate: Some(parse_memorypack_f32(&obj.payload)?),
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_character_item_manipulator(
    obj: &UpdateObject,
) -> Result<extract::CharacterItemManipulator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_ITEM_ID: i32 = 1;
    const METHOD_IS_VISIBLE: i32 = 2;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::CharacterItemManipulator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            item_id: None,
            is_visible: None,
        }),
        METHOD_ITEM_ID => Ok(extract::CharacterItemManipulator {
            method: obj.method,
            character_id: None,
            item_id: Some(parse_memorypack_i32(&obj.payload)?),
            is_visible: None,
        }),
        METHOD_IS_VISIBLE => Ok(extract::CharacterItemManipulator {
            method: obj.method,
            character_id: None,
            item_id: None,
            is_visible: Some(parse_memorypack_bool(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 1 {
                Ok(extract::CharacterItemManipulator {
                    method: obj.method,
                    character_id: None,
                    item_id: None,
                    is_visible: Some(parse_memorypack_bool(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::CharacterItemManipulator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    item_id: None,
                    is_visible: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_virtual_camera_container(
    obj: &UpdateObject,
) -> Result<extract::VirtualCameraContainer, ParseError> {
    const METHOD_COMPRESSED_TRANSFORM: i32 = 0;
    const METHOD_FOV: i32 = 1;

    match obj.method {
        METHOD_COMPRESSED_TRANSFORM => Ok(extract::VirtualCameraContainer {
            method: obj.method,
            sync_parameter: Some(parse_virtual_camera_sync_parameter(&obj.payload)?),
            fov: None,
        }),
        METHOD_FOV => Ok(extract::VirtualCameraContainer {
            method: obj.method,
            sync_parameter: None,
            fov: Some(parse_memorypack_u8(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 24 {
                Ok(extract::VirtualCameraContainer {
                    method: obj.method,
                    sync_parameter: Some(parse_virtual_camera_sync_parameter(&obj.payload)?),
                    fov: None,
                })
            } else if obj.payload.len() == 1 {
                Ok(extract::VirtualCameraContainer {
                    method: obj.method,
                    sync_parameter: None,
                    fov: Some(parse_memorypack_u8(&obj.payload)?),
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_cameraman(
    obj: &UpdateObject,
) -> Result<extract::CameramanReceiver, ParseError> {
    const METHOD_MOVE_HANDY_CAMERA: i32 = 0;

    match obj.method {
        METHOD_MOVE_HANDY_CAMERA => Ok(extract::CameramanReceiver {
            method: obj.method,
            move_packet: Some(parse_move_handy_camera_packet(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 36 {
                Ok(extract::CameramanReceiver {
                    method: obj.method,
                    move_packet: Some(parse_move_handy_camera_packet(&obj.payload)?),
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_motion_communicator(
    obj: &UpdateObject,
) -> Result<extract::MotionCommunicator, ParseError> {
    const METHOD_CHARACTER_ID: i32 = 0;
    const METHOD_MOTION_PACKET_AND_TIME: i32 = 1;

    match obj.method {
        METHOD_CHARACTER_ID => Ok(extract::MotionCommunicator {
            method: obj.method,
            character_id: Some(parse_memorypack_i32(&obj.payload)?),
            motion_packet: None,
        }),
        METHOD_MOTION_PACKET_AND_TIME => Ok(extract::MotionCommunicator {
            method: obj.method,
            character_id: None,
            motion_packet: Some(parse_motion_packet_and_time(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() >= 12 {
                Ok(extract::MotionCommunicator {
                    method: obj.method,
                    character_id: None,
                    motion_packet: Some(parse_motion_packet_and_time(&obj.payload)?),
                })
            } else if obj.payload.len() == 4 {
                Ok(extract::MotionCommunicator {
                    method: obj.method,
                    character_id: Some(parse_memorypack_i32(&obj.payload)?),
                    motion_packet: None,
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_switch_receiver(
    obj: &UpdateObject,
) -> Result<extract::SwitchReceiver, ParseError> {
    const METHOD_SWITCH_COMMAND: i32 = 0;
    const METHOD_ZOOM_COMMAND: i32 = 1;

    match obj.method {
        METHOD_SWITCH_COMMAND => Ok(extract::SwitchReceiver {
            method: obj.method,
            switch_packet: Some(parse_switcher_packet(&obj.payload)?),
            zoom_command: None,
        }),
        METHOD_ZOOM_COMMAND => Ok(extract::SwitchReceiver {
            method: obj.method,
            switch_packet: None,
            zoom_command: Some(parse_zoom_command(&obj.payload)?),
        }),
        _ => {
            if obj.payload.len() == 36 {
                Ok(extract::SwitchReceiver {
                    method: obj.method,
                    switch_packet: Some(parse_switcher_packet(&obj.payload)?),
                    zoom_command: None,
                })
            } else if obj.payload.len() == 8 {
                Ok(extract::SwitchReceiver {
                    method: obj.method,
                    switch_packet: None,
                    zoom_command: Some(parse_zoom_command(&obj.payload)?),
                })
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}

pub(crate) fn try_parse_music_broadcaster(
    obj: &UpdateObject,
) -> Result<extract::MusicBroadcaster, ParseError> {
    const METHOD_MUSIC_PACKET: i32 = 0;

    match obj.method {
        METHOD_MUSIC_PACKET => Ok(parse_music_broadcaster_payload(obj.method, &obj.payload)?),
        _ => {
            if obj.payload.len() >= 28 {
                Ok(parse_music_broadcaster_payload(obj.method, &obj.payload)?)
            } else {
                Err(ParseError::UnknownMethod(obj.method))
            }
        }
    }
}
