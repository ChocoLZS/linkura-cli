use super::{PrefabKind, hex_preview};
use super::parsers::camera::{
    parse_fixed_camera_packet, parse_move_handy_camera_packet, parse_switcher_packet,
    parse_virtual_camera_sync_parameter, parse_zoom_command,
};
use super::parsers::character::{
    parse_expression_data, parse_foot_shadow_activate_command, parse_is_visible_packet,
    parse_lip_sync_data, parse_pose_packet,
};
use super::parsers::misc::parse_motion_packet_and_time;
use super::parsers::primitives::{
    parse_memorypack_bool, parse_memorypack_f32, parse_memorypack_f64, parse_memorypack_i32,
    parse_memorypack_i64, parse_memorypack_quaternion, parse_memorypack_string,
    parse_memorypack_u8, parse_memorypack_vector3,
};

type InitSummaryFn = fn(&[u8]) -> Option<String>;

#[derive(Clone, Copy)]
struct InitPropertySpec {
    name: &'static str,
    summarize: InitSummaryFn,
}

fn init_property_spec(kind: PrefabKind, rpc_id: u8) -> Option<InitPropertySpec> {
    use PrefabKind::*;

    let spec = match (kind, rpc_id) {
        (CharacterPositionCommunicator, 0)
        | (CharacterFocusableCommunicator, 0)
        | (MagicaControlCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (ScenePropManipulator, 0) => InitPropertySpec {
            name: "PropId",
            summarize: summarize_i32,
        },
        (ScenePropManipulator, 1) => InitPropertySpec {
            name: "WorldPosition",
            summarize: summarize_vector3,
        },
        (ScenePropManipulator, 2) => InitPropertySpec {
            name: "WorldRotation",
            summarize: summarize_quaternion,
        },
        (ScenePropManipulator, 3) => InitPropertySpec {
            name: "IsVisible",
            summarize: summarize_bool,
        },
        (ScenePropManipulator, 10) => InitPropertySpec {
            name: "AnimationTrigger",
            summarize: summarize_string,
        },
        (FootShadowManipulator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (FootShadowManipulator, 1) => InitPropertySpec {
            name: "ActivateCommand",
            summarize: summarize_activate_command,
        },
        (ExpressionCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (ExpressionCommunicator, 1) => InitPropertySpec {
            name: "ExpressionData",
            summarize: summarize_expression_data,
        },
        (LipCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (LipCommunicator, 1) => InitPropertySpec {
            name: "LipSyncData",
            summarize: summarize_lip_sync_data,
        },
        (PoseCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (PoseCommunicator, 1) => InitPropertySpec {
            name: "PosePacket",
            summarize: summarize_pose_packet,
        },
        (VisibleCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (VisibleCommunicator, 1) => InitPropertySpec {
            name: "IsVisiblePacket",
            summarize: summarize_visible_packet,
        },
        (FingerLeapCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (FingerLeapCommunicator, 1) => InitPropertySpec {
            name: "LeapRate",
            summarize: summarize_f32_short,
        },
        (CharacterItemManipulator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (CharacterItemManipulator, 1) => InitPropertySpec {
            name: "ItemId",
            summarize: summarize_i32,
        },
        (CharacterItemManipulator, 2) => InitPropertySpec {
            name: "IsVisible",
            summarize: summarize_bool,
        },
        (VirtualCameraContainer, 0) => InitPropertySpec {
            name: "CompressedTransform",
            summarize: summarize_virtual_camera_sync,
        },
        (VirtualCameraContainer, 1) => InitPropertySpec {
            name: "Fov",
            summarize: summarize_u8,
        },
        (CameraMan, 0) => InitPropertySpec {
            name: "MoveHandyCamera",
            summarize: summarize_move_camera,
        },
        (MotionCommunicator, 0) => InitPropertySpec {
            name: "CharacterId",
            summarize: summarize_i32,
        },
        (MotionCommunicator, 1) => InitPropertySpec {
            name: "MotionPacketAndTime",
            summarize: summarize_motion_packet,
        },
        (FixedCamera, 0) => InitPropertySpec {
            name: "CreateFixedCameraPacket",
            summarize: summarize_fixed_camera,
        },
        (SwitchReceiver, 0) => InitPropertySpec {
            name: "SwitchCommand",
            summarize: summarize_switcher,
        },
        (SwitchReceiver, 1) => InitPropertySpec {
            name: "ZoomCommand",
            summarize: summarize_zoom,
        },
        (DateTimeReceiver, 0) => InitPropertySpec {
            name: "DateTimeTicks",
            summarize: summarize_i64,
        },
        _ => return None,
    };

    Some(spec)
}

pub(super) fn property_name_for_kind_opt(kind: Option<PrefabKind>, rpc_id: u8) -> Option<&'static str> {
    kind.and_then(|k| init_property_spec(k, rpc_id).map(|s| s.name))
}

pub(super) fn summarize_init_property_value_by_kind(
    kind: Option<PrefabKind>,
    rpc_id: u8,
    payload: &[u8],
) -> String {
    if let Some(value) = kind
        .and_then(|k| init_property_spec(k, rpc_id))
        .and_then(|spec| (spec.summarize)(payload))
    {
        return value;
    }

    summarize_init_property_value_fallback(payload)
}

fn summarize_init_property_value_fallback(payload: &[u8]) -> String {
    if let Ok(v) = parse_memorypack_bool(payload) {
        if payload.len() == 1 {
            return format!("bool({})", v);
        }
    }
    if let Ok(v) = parse_memorypack_i32(payload) {
        if payload.len() == 4 {
            return format!("i32({})", v);
        }
    }
    if let Ok(v) = parse_memorypack_f32(payload) {
        if payload.len() == 4 {
            return format!("f32({:.6})", v);
        }
    }
    if let Ok(v) = parse_memorypack_f64(payload) {
        if payload.len() == 8 {
            return format!("f64({:.6})", v);
        }
    }
    if let Ok(v) = parse_memorypack_vector3(payload) {
        if payload.len() == 12 {
            return format!("Vector3({:.3}, {:.3}, {:.3})", v.x, v.y, v.z);
        }
    }
    if let Ok(v) = parse_memorypack_quaternion(payload) {
        if payload.len() == 16 {
            return format!("Quaternion({:.3}, {:.3}, {:.3}, {:.3})", v.x, v.y, v.z, v.w);
        }
    }
    if let Ok(v) = parse_memorypack_string(payload) {
        return format!("string({})", v);
    }

    format!("raw({})", hex_preview(payload, 32))
}

fn summarize_i32(payload: &[u8]) -> Option<String> {
    parse_memorypack_i32(payload).ok().map(|v| format!("i32({})", v))
}

fn summarize_i64(payload: &[u8]) -> Option<String> {
    parse_memorypack_i64(payload).ok().map(|v| format!("i64({})", v))
}

fn summarize_bool(payload: &[u8]) -> Option<String> {
    parse_memorypack_bool(payload).ok().map(|v| format!("bool({})", v))
}

fn summarize_string(payload: &[u8]) -> Option<String> {
    parse_memorypack_string(payload)
        .ok()
        .map(|v| format!("string({})", v))
}

fn summarize_f32_short(payload: &[u8]) -> Option<String> {
    parse_memorypack_f32(payload).ok().map(|v| format!("f32({:.3})", v))
}

fn summarize_u8(payload: &[u8]) -> Option<String> {
    parse_memorypack_u8(payload).ok().map(|v| format!("u8({})", v))
}

fn summarize_vector3(payload: &[u8]) -> Option<String> {
    parse_memorypack_vector3(payload)
        .ok()
        .map(|v| format!("Vector3({:.3}, {:.3}, {:.3})", v.x, v.y, v.z))
}

fn summarize_quaternion(payload: &[u8]) -> Option<String> {
    parse_memorypack_quaternion(payload)
        .ok()
        .map(|v| format!("Quaternion({:.3}, {:.3}, {:.3}, {:.3})", v.x, v.y, v.z, v.w))
}

fn summarize_activate_command(payload: &[u8]) -> Option<String> {
    parse_foot_shadow_activate_command(payload)
        .ok()
        .map(|v| format!("activate(is_active={}, sync_time={:.3})", v.is_active, v.sync_time))
}

fn summarize_expression_data(payload: &[u8]) -> Option<String> {
    parse_expression_data(payload).ok().map(|v| {
        format!(
            "ExpressionData(next={}, last={}, weight={:.3}, switch={}, t={:.3})",
            v.next_id, v.last_id, v.weight, v.is_switch_control, v.send_time_sec
        )
    })
}

fn summarize_lip_sync_data(payload: &[u8]) -> Option<String> {
    parse_lip_sync_data(payload).ok().map(|v| {
        format!(
            "LipSyncData(A={:.3}, I={:.3}, U={:.3}, E={:.3}, O={:.3}, C={:.3}, t={:.3})",
            v.mouth_a, v.mouth_i, v.mouth_u, v.mouth_e, v.mouth_o, v.mouth_close, v.send_time_sec
        )
    })
}

fn summarize_pose_packet(payload: &[u8]) -> Option<String> {
    parse_pose_packet(payload)
        .ok()
        .map(|v| format!("PosePacket(id={}, fixed={})", v.pose_id, v.is_fixed))
}

fn summarize_visible_packet(payload: &[u8]) -> Option<String> {
    parse_is_visible_packet(payload)
        .ok()
        .map(|v| format!("VisiblePacket(visible={}, t={:.3})", v.is_visible, v.sync_time))
}

fn summarize_virtual_camera_sync(payload: &[u8]) -> Option<String> {
    parse_virtual_camera_sync_parameter(payload).ok().map(|v| {
        format!(
            "VirtualCameraSync(pos={}, rot={}, t={:.3})",
            v.compressed_position, v.compressed_rotation, v.sync_time
        )
    })
}

fn summarize_move_camera(payload: &[u8]) -> Option<String> {
    parse_move_handy_camera_packet(payload).ok().map(|v| {
        format!(
            "MoveCamera(pos=({:.2},{:.2},{:.2}), view={:.2}, ortho={})",
            v.position.x, v.position.y, v.position.z, v.view_size, v.is_orthographic
        )
    })
}

fn summarize_motion_packet(payload: &[u8]) -> Option<String> {
    parse_motion_packet_and_time(payload)
        .ok()
        .map(|v| format!("Motion(bytes={}, t={:.3})", v.motion_len, v.time_sec))
}

fn summarize_switcher(payload: &[u8]) -> Option<String> {
    parse_switcher_packet(payload).ok().map(|v| {
        format!(
            "Switcher(program={}#{}, preset={}#{})",
            v.program_pass_camera_type,
            v.program_pass_camera_id,
            v.preset_pass_camera_type,
            v.preset_pass_camera_id
        )
    })
}

fn summarize_zoom(payload: &[u8]) -> Option<String> {
    parse_zoom_command(payload)
        .ok()
        .map(|v| format!("Zoom(camera_id={}, zoom={:.3})", v.camera_id, v.zoom))
}

fn summarize_fixed_camera(payload: &[u8]) -> Option<String> {
    parse_fixed_camera_packet(payload).ok().map(|v| {
        let base = format!(
            "FixedCamera(name={}, pos=({:.3},{:.3},{:.3}), rot=({:.3},{:.3},{:.3}), view={:.3}, ortho={})",
            v.name,
            v.position.x,
            v.position.y,
            v.position.z,
            v.rotation.x,
            v.rotation.y,
            v.rotation.z,
            v.view_size,
            v.is_orthographic
        );
        if let Some(index) = v.index {
            format!("{}, index={}", base, index)
        } else {
            base
        }
    })
}
