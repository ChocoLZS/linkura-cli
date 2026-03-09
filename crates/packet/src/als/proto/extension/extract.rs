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
pub struct InstantiateProperty {
    pub rpc_id: u8,
    pub property_name: String,
    pub payload_len: usize,
    pub value_summary: String,
}

impl Display for InstantiateProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{}:{} len={} {}",
            self.rpc_id, self.property_name, self.payload_len, self.value_summary
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct InstantiateInitData {
    pub total_len: usize,
    pub marker: Option<u8>,
    pub version: Option<i32>,
    pub declared_body_len: Option<usize>,
    pub body_tag: Option<u8>,
    pub property_count: Option<usize>,
    pub properties: Vec<InstantiateProperty>,
    pub note: Option<String>,
    pub raw_preview: Option<String>,
}

impl Display for InstantiateInitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InitData(len={}", self.total_len)?;
        if let Some(marker) = self.marker {
            write!(f, ", marker={}", marker)?;
        }
        if let Some(version) = self.version {
            write!(f, ", version={}", version)?;
        }
        if let Some(body_len) = self.declared_body_len {
            write!(f, ", body_len={}", body_len)?;
        }
        if let Some(tag) = self.body_tag {
            write!(f, ", body_tag={}", tag)?;
        }
        if let Some(count) = self.property_count {
            write!(f, ", properties={}", count)?;
        }
        write!(f, ")")?;

        if !self.properties.is_empty() {
            let joined = self
                .properties
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ");
            write!(f, " [{}]", joined)?;
        }

        if let Some(note) = &self.note {
            write!(f, " note={}", note)?;
        }
        if let Some(raw) = &self.raw_preview {
            write!(f, " raw={}", raw)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExpressionData {
    pub next_id: i32,
    pub last_id: i32,
    pub weight: f32,
    pub is_switch_control: bool,
    pub send_time_sec: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ExpressionCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub expression_data: Option<ExpressionData>,
}

impl Display for ExpressionCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "ExpressionCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(data) = self.expression_data {
            return write!(
                f,
                "ExpressionCommunicator(method={}): NextId={}, LastId={}, Weight={:.6}, IsSwitchControl={}, SendTimeSec={:.6}",
                self.method,
                data.next_id,
                data.last_id,
                data.weight,
                data.is_switch_control,
                data.send_time_sec
            );
        }
        write!(f, "ExpressionCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LipSyncData {
    pub mouth_a: f32,
    pub mouth_i: f32,
    pub mouth_u: f32,
    pub mouth_e: f32,
    pub mouth_o: f32,
    pub mouth_close: f32,
    pub send_time_sec: f64,
}

#[derive(Debug, Clone, Default)]
pub struct LipCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub lip_sync_data: Option<LipSyncData>,
}

impl Display for LipCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "LipCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(data) = self.lip_sync_data {
            return write!(
                f,
                "LipCommunicator(method={}): A={:.6}, I={:.6}, U={:.6}, E={:.6}, O={:.6}, Close={:.6}, SendTimeSec={:.6}",
                self.method,
                data.mouth_a,
                data.mouth_i,
                data.mouth_u,
                data.mouth_e,
                data.mouth_o,
                data.mouth_close,
                data.send_time_sec
            );
        }
        write!(f, "LipCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PosePacket {
    pub pose_id: i32,
    pub is_fixed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PoseCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub pose_packet: Option<PosePacket>,
}

impl Display for PoseCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "PoseCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(packet) = self.pose_packet {
            return write!(
                f,
                "PoseCommunicator(method={}): PoseId={}, IsFixed={}",
                self.method, packet.pose_id, packet.is_fixed
            );
        }
        write!(f, "PoseCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IsVisiblePacket {
    pub is_visible: bool,
    pub sync_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct VisibleCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub visible_packet: Option<IsVisiblePacket>,
}

impl Display for VisibleCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "VisibleCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(packet) = self.visible_packet {
            return write!(
                f,
                "VisibleCommunicator(method={}): IsVisible={}, SyncTime={:.6}",
                self.method, packet.is_visible, packet.sync_time
            );
        }
        write!(f, "VisibleCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FingerLeapCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub leap_rate: Option<f32>,
}

impl Display for FingerLeapCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "FingerLeapCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(leap_rate) = self.leap_rate {
            return write!(
                f,
                "FingerLeapCommunicator(method={}): LeapRate={:.6}",
                self.method, leap_rate
            );
        }
        write!(f, "FingerLeapCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CharacterItemManipulator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub item_id: Option<i32>,
    pub is_visible: Option<bool>,
}

impl Display for CharacterItemManipulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "CharacterItemManipulator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(item_id) = self.item_id {
            return write!(
                f,
                "CharacterItemManipulator(method={}): ItemId={}",
                self.method, item_id
            );
        }
        if let Some(is_visible) = self.is_visible {
            return write!(
                f,
                "CharacterItemManipulator(method={}): IsVisible={}",
                self.method, is_visible
            );
        }
        write!(
            f,
            "CharacterItemManipulator(method={}): <empty>",
            self.method
        )
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VirtualCameraSyncParameter {
    pub compressed_position: i64,
    pub compressed_rotation: i64,
    pub sync_time: f64,
}

#[derive(Debug, Clone, Default)]
pub struct VirtualCameraContainer {
    pub method: i32,
    pub sync_parameter: Option<VirtualCameraSyncParameter>,
    pub fov: Option<u8>,
}

impl Display for VirtualCameraContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(param) = self.sync_parameter {
            return write!(
                f,
                "VirtualCameraContainer(method={}): CompressedPosition={}, CompressedRotation={}, SyncTime={:.6}",
                self.method,
                param.compressed_position,
                param.compressed_rotation,
                param.sync_time
            );
        }
        if let Some(fov) = self.fov {
            return write!(
                f,
                "VirtualCameraContainer(method={}): Fov={}",
                self.method, fov
            );
        }
        write!(f, "VirtualCameraContainer(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MoveHandyCameraPacket {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub view_size: f32,
    pub is_orthographic: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FixedCameraPacket {
    pub member_count: u8,
    pub name: String,
    pub position: Vector3,
    pub rotation: Vector3,
    pub view_size: f32,
    pub is_orthographic: bool,
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CharacterModelInitialData {
    pub character_id: i32,
    pub costume_id: i32,
    pub position: Vector3,
    pub rotation: Quaternion,
}

#[derive(Debug, Clone, Default)]
pub struct CameramanReceiver {
    pub method: i32,
    pub move_packet: Option<MoveHandyCameraPacket>,
}

impl Display for CameramanReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(packet) = self.move_packet {
            return write!(
                f,
                "Cameraman(method={}): Position=({:.6}, {:.6}, {:.6}), Rotation=({:.6}, {:.6}, {:.6}, {:.6}), ViewSize={:.6}, IsOrthographic={}",
                self.method,
                packet.position.x,
                packet.position.y,
                packet.position.z,
                packet.rotation.x,
                packet.rotation.y,
                packet.rotation.z,
                packet.rotation.w,
                packet.view_size,
                packet.is_orthographic
            );
        }
        write!(f, "Cameraman(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MotionPacketAndTime {
    pub motion_len: usize,
    pub time_sec: f64,
}

#[derive(Debug, Clone, Default)]
pub struct MotionCommunicator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub motion_packet: Option<MotionPacketAndTime>,
}

impl Display for MotionCommunicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(character_id) = self.character_id {
            return write!(
                f,
                "MotionCommunicator(method={}): CharacterId={}",
                self.method, character_id
            );
        }
        if let Some(packet) = self.motion_packet {
            return write!(
                f,
                "MotionCommunicator(method={}): MotionBytes={}, TimeSec={:.6}",
                self.method, packet.motion_len, packet.time_sec
            );
        }
        write!(f, "MotionCommunicator(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SwitcherPacket {
    pub program_pass_camera_type: i32,
    pub program_pass_camera_id: i32,
    pub preset_pass_camera_type: i32,
    pub preset_pass_camera_id: i32,
    pub program_pass_alpha: f32,
    pub fade_time: f32,
    pub timeline_default_camera_type: i32,
    pub timeline_default_camera_id: i32,
    pub should_sync_timeline_default_pos: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ZoomCommand {
    pub camera_id: i32,
    pub zoom: f32,
}

#[derive(Debug, Clone, Default)]
pub struct SwitchReceiver {
    pub method: i32,
    pub switch_packet: Option<SwitcherPacket>,
    pub zoom_command: Option<ZoomCommand>,
}

impl Display for SwitchReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(packet) = self.switch_packet {
            return write!(
                f,
                "SwitchReceiver(method={}): ProgramType={}, ProgramId={}, PresetType={}, PresetId={}, Alpha={:.6}, FadeTime={:.6}, TimelineType={}, TimelineId={}, SyncTimelineDefault={}",
                self.method,
                packet.program_pass_camera_type,
                packet.program_pass_camera_id,
                packet.preset_pass_camera_type,
                packet.preset_pass_camera_id,
                packet.program_pass_alpha,
                packet.fade_time,
                packet.timeline_default_camera_type,
                packet.timeline_default_camera_id,
                packet.should_sync_timeline_default_pos
            );
        }
        if let Some(zoom) = self.zoom_command {
            return write!(
                f,
                "SwitchReceiver(method={}): ZoomCameraId={}, Zoom={:.6}",
                self.method, zoom.camera_id, zoom.zoom
            );
        }
        write!(f, "SwitchReceiver(method={}): <empty>", self.method)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MusicBroadcaster {
    pub method: i32,
    pub header0: u32,
    pub header1: u32,
    pub header2: u32,
    pub header3: u32,
    pub sync_time: f64,
    pub encoded_length: usize,
    pub encoded_available: usize,
}

impl Display for MusicBroadcaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MusicBroadcaster(method={}): Header=[0x{:08x}, 0x{:08x}, 0x{:08x}, 0x{:08x}], SyncTime={:.6}, EncodedLength={}, EncodedAvailable={}",
            self.method,
            self.header0,
            self.header1,
            self.header2,
            self.header3,
            self.sync_time,
            self.encoded_length,
            self.encoded_available
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct FootShadowManipulator {
    pub method: i32,
    pub character_id: Option<i32>,
    pub activate_command: Option<FootShadowActivateCommand>,
    pub is_active_only: Option<bool>,
}

impl FootShadowManipulator {
    pub(crate) fn from_character_id(method: i32, character_id: i32) -> Self {
        Self {
            method,
            character_id: Some(character_id),
            ..Self::default()
        }
    }

    pub(crate) fn from_activate_command(
        method: i32,
        activate_command: FootShadowActivateCommand,
    ) -> Self {
        Self {
            method,
            activate_command: Some(activate_command),
            ..Self::default()
        }
    }

    pub(crate) fn from_is_active_only(method: i32, is_active_only: bool) -> Self {
        Self {
            method,
            is_active_only: Some(is_active_only),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FootShadowActivateCommand {
    pub is_active: bool,
    pub sync_time: f64,
}

impl Display for FootShadowManipulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = self.character_id {
            return write!(
                f,
                "FootShadowManipulator(method={}): CharacterId={}",
                self.method, value
            );
        }

        if let Some(value) = self.activate_command {
            return write!(
                f,
                "FootShadowManipulator(method={}): IsActive={}, SyncTime={:.6}",
                self.method, value.is_active, value.sync_time
            );
        }

        if let Some(value) = self.is_active_only {
            return write!(
                f,
                "FootShadowManipulator(method={}): IsActiveOnly={}",
                self.method, value
            );
        }

        write!(f, "FootShadowManipulator(method={}): <empty>", self.method)
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

impl ScenePropManipulator {
    pub(crate) fn from_prop_id(method: i32, prop_id: i32) -> Self {
        Self {
            method,
            prop_id: Some(prop_id),
            ..Self::default()
        }
    }

    pub(crate) fn from_world_position(method: i32, world_position: Vector3) -> Self {
        Self {
            method,
            world_position: Some(world_position),
            ..Self::default()
        }
    }

    pub(crate) fn from_world_rotation(method: i32, world_rotation: Quaternion) -> Self {
        Self {
            method,
            world_rotation: Some(world_rotation),
            ..Self::default()
        }
    }

    pub(crate) fn from_is_visible(method: i32, is_visible: bool) -> Self {
        Self {
            method,
            is_visible: Some(is_visible),
            ..Self::default()
        }
    }

    pub(crate) fn from_animation_trigger(method: i32, animation_trigger: String) -> Self {
        Self {
            method,
            animation_trigger: Some(animation_trigger),
            ..Self::default()
        }
    }
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
