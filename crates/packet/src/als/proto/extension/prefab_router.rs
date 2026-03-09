use super::{ParseError, UpdateObject, UpdateObjectExt};

pub mod prefab_name {
    pub const DATE_TIME_RECEIVER: &str = "TimedAsset/DateTimeReceiver";
    pub const MUSIC_BROADCASTER: &str = "VoiceObject/MusicBroadcaster";
    pub const COVER_IMAGE_RECEIVER: &str = "CoverImageReceiver";
    pub const SCENE_PROP_MANIPULATOR: &str = "Prefabs/ScenePropManipulator";
    pub const FOOT_SHADOW_MANIPULATOR: &str = "FootShadowManipulator";
    pub const EXPRESSION_COMMUNICATOR: &str = "ExpressionCommunicator";
    pub const LIP_COMMUNICATOR: &str = "LipCommunicator";
    pub const POSE_COMMUNICATOR: &str = "PoseCommunicator";
    pub const VISIBLE_COMMUNICATOR: &str = "VisibleCommunicator";
    pub const FINGER_LEAP_COMMUNICATOR: &str = "FingerLeapCommunicator";
    pub const CHARACTER_ITEM_MANIPULATOR: &str = "CharacterItemManipulator";
    pub const VIRTUAL_CAMERA_CONTAINER: &str = "VirtualCameraContainer";
    pub const CAMERA_MAN: &str = "Prefabs/Camera/Cameraman";
    pub const FIXED_CAMERA: &str = "Prefabs/Camera/FixedCamera";
    pub const MOTION_COMMUNICATOR: &str = "MotionCommunicator";
    pub const SWITCH_RECEIVER: &str = "SwitchReceiver";
    pub const CHARACTER_POSITION_COMMUNICATOR: &str = "CharacterPositionCommunicator";
    pub const CHARACTER_FOCUSABLE_COMMUNICATOR: &str = "CharacterFocusableCommunicator";
    pub const MAGICA_CONTROL_COMMUNICATOR: &str = "MagicaControlCommunicator";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefabKind {
    DateTimeReceiver,
    MusicBroadcaster,
    CoverImageReceiver,
    ScenePropManipulator,
    FootShadowManipulator,
    ExpressionCommunicator,
    LipCommunicator,
    PoseCommunicator,
    VisibleCommunicator,
    FingerLeapCommunicator,
    CharacterItemManipulator,
    VirtualCameraContainer,
    CameraMan,
    FixedCamera,
    MotionCommunicator,
    SwitchReceiver,
    CharacterPositionCommunicator,
    CharacterFocusableCommunicator,
    MagicaControlCommunicator,
}

const PREFAB_KIND_PATTERNS: &[(PrefabKind, &str)] = &[
    (
        PrefabKind::DateTimeReceiver,
        prefab_name::DATE_TIME_RECEIVER,
    ),
    (PrefabKind::MusicBroadcaster, prefab_name::MUSIC_BROADCASTER),
    (
        PrefabKind::CoverImageReceiver,
        prefab_name::COVER_IMAGE_RECEIVER,
    ),
    (
        PrefabKind::ScenePropManipulator,
        prefab_name::SCENE_PROP_MANIPULATOR,
    ),
    (
        PrefabKind::FootShadowManipulator,
        prefab_name::FOOT_SHADOW_MANIPULATOR,
    ),
    (
        PrefabKind::ExpressionCommunicator,
        prefab_name::EXPRESSION_COMMUNICATOR,
    ),
    (PrefabKind::LipCommunicator, prefab_name::LIP_COMMUNICATOR),
    (PrefabKind::PoseCommunicator, prefab_name::POSE_COMMUNICATOR),
    (
        PrefabKind::VisibleCommunicator,
        prefab_name::VISIBLE_COMMUNICATOR,
    ),
    (
        PrefabKind::FingerLeapCommunicator,
        prefab_name::FINGER_LEAP_COMMUNICATOR,
    ),
    (
        PrefabKind::CharacterItemManipulator,
        prefab_name::CHARACTER_ITEM_MANIPULATOR,
    ),
    (
        PrefabKind::VirtualCameraContainer,
        prefab_name::VIRTUAL_CAMERA_CONTAINER,
    ),
    (PrefabKind::CameraMan, prefab_name::CAMERA_MAN),
    (PrefabKind::FixedCamera, prefab_name::FIXED_CAMERA),
    (
        PrefabKind::MotionCommunicator,
        prefab_name::MOTION_COMMUNICATOR,
    ),
    (PrefabKind::SwitchReceiver, prefab_name::SWITCH_RECEIVER),
    (
        PrefabKind::CharacterPositionCommunicator,
        prefab_name::CHARACTER_POSITION_COMMUNICATOR,
    ),
    (
        PrefabKind::CharacterFocusableCommunicator,
        prefab_name::CHARACTER_FOCUSABLE_COMMUNICATOR,
    ),
    (
        PrefabKind::MagicaControlCommunicator,
        prefab_name::MAGICA_CONTROL_COMMUNICATOR,
    ),
];

pub fn detect_prefab_kind(prefab_name: &str) -> Option<PrefabKind> {
    let normalized = normalize_prefab_name(prefab_name);
    PREFAB_KIND_PATTERNS
        .iter()
        .find(|(_, needle)| normalized.contains(*needle))
        .map(|(kind, _)| *kind)
}

pub fn normalize_prefab_name(prefab_name: &str) -> &str {
    if let Some(index) = prefab_name.find("3d_costume_") {
        return &prefab_name[index..];
    }
    if let Some((_, needle)) = PREFAB_KIND_PATTERNS
        .iter()
        .find(|(_, needle)| prefab_name.contains(*needle))
    {
        if let Some(index) = prefab_name.find(needle) {
            return &prefab_name[index..];
        }
    }
    prefab_name
}

pub(super) fn is_costume_prefab(prefab_name: &str) -> bool {
    normalize_prefab_name(prefab_name).starts_with("3d_costume_")
}

type UpdateTextParserFn = fn(&UpdateObject) -> Result<String, ParseError>;

macro_rules! define_update_text_parser {
    ($fn_name:ident, $method:ident) => {
        fn $fn_name(object: &UpdateObject) -> Result<String, ParseError> {
            object.$method().map(|v| v.to_string())
        }
    };
}

define_update_text_parser!(parse_date_time_text, try_parse_date_time);
define_update_text_parser!(parse_music_broadcaster_text, try_parse_music_broadcaster);
define_update_text_parser!(parse_cover_image_text, try_parse_cover_image);
define_update_text_parser!(
    parse_scene_prop_manipulator_text,
    try_parse_scene_prop_manipulator
);
define_update_text_parser!(
    parse_foot_shadow_manipulator_text,
    try_parse_foot_shadow_manipulator
);
define_update_text_parser!(
    parse_expression_communicator_text,
    try_parse_expression_communicator
);
define_update_text_parser!(parse_lip_communicator_text, try_parse_lip_communicator);
define_update_text_parser!(parse_pose_communicator_text, try_parse_pose_communicator);
define_update_text_parser!(parse_visible_communicator_text, try_parse_visible_communicator);
define_update_text_parser!(
    parse_finger_leap_communicator_text,
    try_parse_finger_leap_communicator
);
define_update_text_parser!(
    parse_character_item_manipulator_text,
    try_parse_character_item_manipulator
);
define_update_text_parser!(
    parse_virtual_camera_container_text,
    try_parse_virtual_camera_container
);
define_update_text_parser!(parse_cameraman_text, try_parse_cameraman);
define_update_text_parser!(parse_motion_communicator_text, try_parse_motion_communicator);
define_update_text_parser!(parse_switch_receiver_text, try_parse_switch_receiver);

const UPDATE_TEXT_PARSERS: &[(PrefabKind, UpdateTextParserFn)] = &[
    (PrefabKind::DateTimeReceiver, parse_date_time_text),
    (PrefabKind::MusicBroadcaster, parse_music_broadcaster_text),
    (PrefabKind::CoverImageReceiver, parse_cover_image_text),
    (
        PrefabKind::ScenePropManipulator,
        parse_scene_prop_manipulator_text,
    ),
    (
        PrefabKind::FootShadowManipulator,
        parse_foot_shadow_manipulator_text,
    ),
    (
        PrefabKind::ExpressionCommunicator,
        parse_expression_communicator_text,
    ),
    (PrefabKind::LipCommunicator, parse_lip_communicator_text),
    (PrefabKind::PoseCommunicator, parse_pose_communicator_text),
    (PrefabKind::VisibleCommunicator, parse_visible_communicator_text),
    (
        PrefabKind::FingerLeapCommunicator,
        parse_finger_leap_communicator_text,
    ),
    (
        PrefabKind::CharacterItemManipulator,
        parse_character_item_manipulator_text,
    ),
    (
        PrefabKind::VirtualCameraContainer,
        parse_virtual_camera_container_text,
    ),
    (PrefabKind::CameraMan, parse_cameraman_text),
    (
        PrefabKind::MotionCommunicator,
        parse_motion_communicator_text,
    ),
    (PrefabKind::SwitchReceiver, parse_switch_receiver_text),
];

pub fn parse_update_payload_text(
    prefab_name: &str,
    object: &UpdateObject,
) -> Option<Result<String, ParseError>> {
    let kind = detect_prefab_kind(prefab_name)?;
    UPDATE_TEXT_PARSERS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, parser)| parser(object))
}
