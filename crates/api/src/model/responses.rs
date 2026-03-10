use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// Auto-generated from crates/api/docs/openapi_v1_path_mapping.csv
// Selected high-traffic responses are strongly typed; others remain transparent wrappers.
macro_rules! define_response_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Deserialize, Serialize, Default)]
        #[serde(rename_all = "snake_case")]
        pub struct $name {
            #[serde(flatten)]
            pub extra: Map<String, Value>,
        }
    };
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountConnectResponse {
    pub player_id: Option<String>,
    pub device_specific_id: Option<String>,
    pub session_token: Option<String>,
    pub player_name: Option<String>,
    pub player_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountDeleteResponse {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountDeleteConnectDataResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountConnectData {
    pub d_account_connect_datas_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.Provider
    pub provider: Option<i32>,
    pub update_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountGetConnectDataResponse {
    pub account_connect_data_list: Option<Vec<AccountConnectData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountGetConnectUserResponse {
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    pub player_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountSetConnectDataResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserLoginResponse {
    #[serde(rename = "type")]
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub type_field: Option<i32>,
    pub session_token: Option<String>,
    pub is_tutorial: Option<bool>,
    pub is_term_update: Option<bool>,
    pub is_login_bonus_receive: Option<bool>,
    pub push_device_token: Option<String>,
    pub sisca_product_id_list: Option<Vec<String>>,
    pub membership_product_id_list: Option<Vec<String>>,
    pub item_store_product_id_list: Option<Vec<String>>,
    pub tutorials_status_list: Option<Vec<TutorialsStatusInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetHomeResponse {
    pub favorite_channel_list: Option<Vec<FavoriteChannel>>,
    pub live_archive_list: Option<Vec<LiveInfo>>,
    pub trailer_archive_list: Option<Vec<LiveInfo>>,
    pub recommend_archive_list: Option<Vec<LiveInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetArchiveListResponse {
    pub archive_list: Option<Vec<LiveInfo>>,
    pub filterable_characters: Option<Vec<FilterableCharacter>>,
    pub sortable_fields: Option<Vec<SortableField>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnnounceDetailResponse {
    pub m_announces_id: Option<String>,
    pub m_announces_title: Option<String>,
    pub m_banners_id: Option<String>,
    pub resource_file_name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.AnnounceType
    pub announce_type: Option<i32>,
    pub contents: Option<String>,
    /// dnSpy type: System.DateTime
    pub start_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnnounceListItem {
    pub m_announces_id: Option<String>,
    pub m_announces_title: Option<String>,
    pub m_banners_id: Option<String>,
    pub resource_file_name: Option<String>,
    pub description: Option<String>,
    /// dnSpy type: System.DateTime
    pub pickup_start_time: Option<String>,
    /// dnSpy type: System.DateTime
    pub pickup_end_time: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.AnnounceType
    pub announce_type: Option<i32>,
    /// dnSpy type: System.DateTime
    pub start_time: Option<String>,
    pub priority: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnnounceListResponse {
    pub announce_list: Option<Vec<AnnounceListItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveChannel {
    pub live_channels_id: Option<i32>,
    pub name: Option<String>,
    pub is_favorite: Option<bool>,
    pub is_watched: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveLiveInfo {
    pub archives_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.LiveType
    pub live_type: Option<i32>,
    pub live_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub ticket_list: Option<Vec<TicketInfo>>,
    pub trailer_start_time: Option<String>,
    pub trailer_end_time: Option<String>,
    pub live_start_time: Option<String>,
    pub live_end_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub external_link: Option<String>,
    pub thumbnail_image_url: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTicketRank
    pub ticket_rank: Option<i32>,
    pub character_list: Option<Vec<LiveInfoCharacterListInner>>,
    pub total_playing_time_second: Option<i32>,
    pub is_publish_video_url: Option<bool>,
    pub is_extra_started: Option<bool>,
    pub has_extra: Option<bool>,
    pub has_extra_admission: Option<bool>,
    pub earned_star_count: Option<i32>,
    pub gift_stars_threshold_for_extra_admission: Option<i32>,
    pub video_url: Option<String>,
    pub archive_video_size_in_byte: Option<i64>,
    pub is_scheduled_start_time_visible: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSeasonListItem {
    pub season_id: Option<i32>,
    pub archive_count: Option<i64>,
    pub is_watched: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveLiveTimeline {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub animation: Option<LiveTimelineAnimation>,
    #[serde(rename = "type")]
    pub type_field: Option<i32>,
    /// dnSpy: Org.OpenAPITools.Model.LiveTimelineSenderType
    pub sender_type: Option<i32>,
    pub user_player_id: Option<String>,
    pub user_name: Option<String>,
    pub submit_time: Option<String>,
    pub body: Option<String>,
    pub text_size: Option<i32>,
    pub is_text_bold: Option<bool>,
    pub item_id: Option<i32>,
    pub amount: Option<i32>,
    pub gift_pt: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub display_user_icon: Option<bool>,
    pub asset_icon_parts_info: Option<String>,
    pub image_size: Option<i32>,
    pub play_time_second: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveConnectTokenResponse {
    pub operator_token: Option<String>,
    pub audience_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveConnectTokenResponse {
    pub operator_token: Option<String>,
    pub audience_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnterResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub circle_chats: Option<Vec<LiveCircleChat>>,
    pub room: Option<MrsRoomRoom>,
    pub announcements: Option<Vec<FesliveAnnouncement>>,
    pub live_location_id: Option<i32>,
    pub costume_ids: Option<Vec<i32>>,
    pub timeline_ids: Option<Vec<i64>>,
    pub content_code: Option<i32>,
    pub clap_info: Option<FesliveClap>,
    pub penlight_colors: Option<Vec<FeslivePenlightColor>>,
    pub characters: Option<Vec<FesliveCharacter>>,
    pub viewer_count: Option<i32>,
    pub user_gift_pt: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_season_fan_level: Option<i32>,
    pub camera_focus_character_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCameraType
    pub selectable_camera_types: Option<Vec<i32>>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCameraType
    pub current_camera_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTicketRank
    pub ticket_rank: Option<i32>,
    pub is_belong_circle: Option<bool>,
    pub is_exhibition: Option<bool>,
    pub has_extra: Option<bool>,
    pub scheduled_start_time: Option<String>,
    pub emoji_list: Option<Vec<i32>>,
    pub membership_list: Option<Vec<i32>>,
    pub hls: Option<HlsLive>,
    pub user_profile: Option<LiveUserProfile>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnterResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub room: Option<MrsRoomRoom>,
    pub live_location_id: Option<i32>,
    pub costume_ids: Option<Vec<i32>>,
    pub timeline_ids: Option<Vec<i64>>,
    pub content_code: Option<i32>,
    pub user_gift_pt: Option<i32>,
    pub total_gift_pt: Option<String>,
    pub viewer_count: Option<i32>,
    pub scheduled_start_time: Option<String>,
    pub is_live: Option<bool>,
    pub is_horizontal: Option<bool>,
    pub characters: Option<Vec<WithliveEnterCharacter>>,
    pub max_season_fan_level: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveDoorStatus
    pub door_status: Option<i32>,
    pub has_extra: Option<bool>,
    pub gift_star_reward_segments: Option<Vec<i32>>,
    pub gift_stars_threshold_for_extra_admission: Option<i32>,
    pub has_admission: Option<bool>,
    pub has_extra_admission: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
    pub is_belong_circle: Option<bool>,
    pub bgm_cue_name: Option<String>,
    pub bgm_acb_path: Option<String>,
    pub emoji_list: Option<Vec<i32>>,
    pub membership_list: Option<Vec<i32>>,
    pub hls: Option<HlsLive>,
    pub enquetes: Option<Vec<WithliveEnquete>>,
    pub latest_enquete_id: Option<String>,
    pub user_profile: Option<LiveUserProfile>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GiftBonusInfo {
    pub character_id: Option<i32>,
    pub favorite_rank: Option<i64>,
    pub order: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GiftShopItemInfo {
    pub shop_item_id: Option<i32>,
    pub item_id: Option<i32>,
    pub name: Option<String>,
    pub order_id: Option<i32>,
    pub description: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_quantity: Option<i32>,
    pub is_paid_sisca_only: Option<bool>,
    pub price: Option<i32>,
    pub rarity: Option<i64>,
    pub character_id: Option<i32>,
    pub effect_value: Option<i64>,
    pub own_num: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub is_sale: Option<bool>,
    pub gift_pt: Option<i32>,
    pub max_favorite_rank: Option<i64>,
    pub gift_bonus_info: Option<Vec<GiftBonusInfo>>,
    pub item_datas_id: Option<String>,
    pub item_get_date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MessageCardShopInfo {
    pub shop_item_id: Option<i32>,
    pub item_id: Option<i32>,
    pub name: Option<String>,
    pub order_id: Option<i32>,
    pub description: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_quantity: Option<i32>,
    pub is_paid_sisca_only: Option<bool>,
    pub price: Option<i32>,
    pub rarity: Option<i64>,
    pub character_id: Option<i32>,
    pub effect_value: Option<i64>,
    pub own_num: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub is_sale: Option<bool>,
    pub gift_pt: Option<i32>,
    pub limit_characters_num: Option<i64>,
    pub item_datas_id: Option<String>,
    pub item_get_date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveInfo {
    pub live_id: Option<String>,
    pub feslives_no: Option<i32>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveHoldType
    pub status: Option<i32>,
    pub open_time: Option<String>,
    pub scheduled_start_time: Option<String>,
    pub start_time: Option<String>,
    pub extra_start_time: Option<String>,
    pub end_time: Option<String>,
    pub close_time: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.FesliveStreamingType
    pub streaming_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveInfo {
    pub live_id: Option<String>,
    pub withlives_no: Option<i32>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveHoldType
    pub status: Option<i32>,
    pub open_time: Option<String>,
    pub scheduled_start_time: Option<String>,
    pub start_time: Option<String>,
    pub extra_start_time: Option<String>,
    pub end_time: Option<String>,
    pub close_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveTimelineAnimation {
    pub icon_normal_twinkle: Option<bool>,
    pub icon_strong_twinkle: Option<bool>,
    pub gift_box_falling: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveTimeline {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub animation: Option<LiveTimelineAnimation>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTimelineType
    #[serde(rename = "type")]
    pub type_field: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTimelineSenderType
    pub sender_type: Option<i32>,
    pub user_player_id: Option<String>,
    pub user_name: Option<String>,
    pub submit_time: Option<String>,
    pub body: Option<String>,
    pub text_size: Option<i32>,
    pub is_text_bold: Option<bool>,
    pub item_id: Option<i32>,
    pub amount: Option<i32>,
    pub gift_pt: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub display_user_icon: Option<bool>,
    pub asset_icon_parts_info: Option<String>,
    pub image_size: Option<i32>,
    pub play_time_second: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveCircleChat {
    pub circle_chat_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCircleChatType
    #[serde(rename = "type")]
    pub type_field: Option<i32>,
    pub user_player_id: Option<String>,
    pub user_name: Option<String>,
    pub submit_time: Option<String>,
    pub body: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePenlightColor {
    pub color_id: Option<String>,
    pub color_code: Option<String>,
    pub character_id: Option<i32>,
    pub rate: Option<i32>,
    #[serde(rename = "default")]
    pub default_field: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveClapNormal {
    pub step: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveClapMission {
    pub step: Option<i32>,
    pub rate_to_next: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveClap {
    pub normal: Option<FesliveClapNormal>,
    pub mission: Option<FesliveClapMission>,
    pub is_mission_ongoing: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveFlowerStand {
    pub grand_prix_rank: Option<i32>,
    pub circle_id: Option<String>,
    pub circle_name: Option<String>,
    pub circle_icon_parts_info: Option<String>,
    pub search_guild_key: Option<String>,
    pub flower_stand_type_id: Option<i32>,
    pub flower_stand_color1_id: Option<i32>,
    pub flower_stand_color2_id: Option<i32>,
    pub flower_stand_idol_picture_id: Option<i32>,
    pub flower_stand_message: Option<String>,
    pub is_editable: Option<bool>,
    pub last_update_unixtime: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveAnnouncement {
    pub announcement_id: Option<String>,
    pub body: Option<String>,
    pub duration: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleRankingInfo {
    pub name: Option<String>,
    pub rank: Option<i32>,
    pub month: Option<i64>,
    pub point: Option<i64>,
    pub member_ranking_list: Option<Vec<MemberRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleInfo {
    pub search_guild_key: Option<String>,
    pub guild_id: Option<String>,
    pub guild_name: Option<String>,
    pub leader_player_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildRank
    pub guild_rank: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildPolicyAction
    pub guild_policy_action: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildPolicyRecruiting
    pub guild_policy_recruiting: Option<i32>,
    pub guild_comment: Option<String>,
    pub guild_member_num: Option<i64>,
    pub guild_icon_parts_info: Option<String>,
    pub d_chat_rooms_id: Option<String>,
    pub feslive_rank_message: Option<String>,
    pub d_guild_invitations_id: Option<String>,
    pub circle_invitations_id: Option<String>,
    pub circle_ranking_info: Option<CircleRankingInfo>,
    pub is_join_guild_ranking: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveCircleMember {
    pub user_player_id: Option<String>,
    pub user_name: Option<String>,
    pub user_icon_parts_info: Option<String>,
    pub online: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveCharacter {
    pub character_id: Option<i32>,
    pub is_visible: Option<bool>,
    pub is_guest: Option<bool>,
    pub season_fan_level: Option<i32>,
    pub season_exp_per_next: Option<f32>,
    pub member_fan_level: Option<i32>,
    pub member_exp_per_next: Option<f32>,
    pub order: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyLiveGrandPrixTicketInfo {
    pub is_join_grand_prix: Option<bool>,
    pub image_url: Option<String>,
    pub addressee: Option<String>,
    pub circle_rank: Option<i32>,
    pub seat_rank: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyLiveGrandPrixInfo {
    pub flavor_text: Option<String>,
    pub footnote: Option<String>,
    pub ticket_info: Option<FesliveLobbyLiveGrandPrixTicketInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyTicketInfo {
    pub image_url: Option<String>,
    pub addressee: Option<String>,
    pub ticket_rank: Option<String>,
    pub user_rank: Option<i32>,
    pub has_user_rank: Option<bool>,
    pub flavor_text: Option<String>,
    pub footnote: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HlsLive {
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveUserProfile {
    pub name: Option<String>,
    pub icon_parts_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MrsRoomRoom {
    pub room_id: Option<i32>,
    pub ip_addr: Option<String>,
    pub port: Option<i32>,
    pub player_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnqueteOption {
    pub option_id: Option<String>,
    pub option_letter: Option<String>,
    pub text: Option<String>,
    pub rate: Option<i32>,
    pub is_correct: Option<bool>,
    pub is_max_rate: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnquete {
    pub enquete_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.WithliveEnqueteStatus
    pub status: Option<i32>,
    pub question: Option<String>,
    pub selected_option_id: Option<String>,
    pub options: Option<Vec<WithliveEnqueteOption>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGetGiftShopListResponse {
    pub gift_shop_item_info: Option<Vec<GiftShopItemInfo>>,
    pub message_card_shop_info: Option<Vec<MessageCardShopInfo>>,
    pub gift_box_borders: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGetListResponse {
    pub feslive_info: Option<Vec<FesliveInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveInfoResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub circle_chats: Option<Vec<LiveCircleChat>>,
    pub has_admission: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveDoorStatus
    pub door_status: Option<i32>,
    pub penlight_colors: Option<Vec<FeslivePenlightColor>>,
    pub clap: Option<FesliveClap>,
    pub polling_interval: Option<i32>,
    pub is_trouble: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveSubinfoResponse {
    pub viewer_count: Option<i32>,
    pub circle_members: Option<Vec<LiveCircleMember>>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub circle_chats: Option<Vec<LiveCircleChat>>,
    pub flower_stands: Option<Vec<FesliveFlowerStand>>,
    pub announcements: Option<Vec<FesliveAnnouncement>>,
    pub live_location_id: Option<i32>,
    pub costume_ids: Option<Vec<i32>>,
    pub timeline_ids: Option<Vec<i64>>,
    pub content_code: Option<i32>,
    pub circle_info: Option<CircleInfo>,
    pub circle_members: Option<Vec<LiveCircleMember>>,
    /// dnSpy type: System.DateTime
    pub scheduled_start_time: Option<String>,
    /// dnSpy type: System.DateTime
    pub close_time: Option<String>,
    pub characters: Option<Vec<FesliveCharacter>>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveDoorStatus
    pub door_status: Option<i32>,
    pub has_admission: Option<bool>,
    pub user_gift_pt: Option<i32>,
    pub live_grand_prix_info: Option<FesliveLobbyLiveGrandPrixInfo>,
    pub ticket_info: Option<FesliveLobbyTicketInfo>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_season_fan_level: Option<i32>,
    pub is_belong_circle: Option<bool>,
    pub is_exhibition: Option<bool>,
    pub has_extra: Option<bool>,
    pub has_ticket: Option<bool>,
    pub is_first_ticket_view: Option<bool>,
    pub background_image_url: Option<String>,
    pub bgm_cue_name: Option<String>,
    pub bgm_acb_path: Option<String>,
    pub emoji_list: Option<Vec<i32>>,
    pub membership_list: Option<Vec<i32>>,
    pub hls: Option<HlsLive>,
    /// dnSpy enum: Org.OpenAPITools.Model.FesliveStreamingType
    pub streaming_type: Option<i32>,
    pub is_scheduled_start_time_visible: Option<bool>,
    pub user_profile: Option<LiveUserProfile>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyInfoResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub circle_chats: Option<Vec<LiveCircleChat>>,
    /// dnSpy type: System.DateTime
    pub hall_open_time: Option<String>,
    pub has_admission: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveDoorStatus
    pub door_status: Option<i32>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbySubinfoResponse {
    pub flower_stands: Option<Vec<FesliveFlowerStand>>,
    pub viewer_count: Option<i32>,
    pub polling_interval: Option<i32>,
    pub circle_members: Option<Vec<LiveCircleMember>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGetGiftShopListResponse {
    pub gift_shop_item_info: Option<Vec<GiftShopItemInfo>>,
    pub message_card_shop_info: Option<Vec<MessageCardShopInfo>>,
    pub gift_box_borders: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGetListResponse {
    pub withlive_info: Option<Vec<WithliveInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveLiveInfoResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub room: Option<MrsRoomRoom>,
    /// dnSpy type: System.DateTime
    pub hall_open_time: Option<String>,
    pub has_admission: Option<bool>,
    pub has_extra_admission: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveDoorStatus
    pub door_status: Option<i32>,
    pub polling_interval: Option<i32>,
    pub hls: Option<HlsLive>,
    pub active_enquete_id: Option<String>,
    pub enquetes: Option<Vec<WithliveEnquete>>,
    pub latest_enquete_id: Option<String>,
    pub is_trouble: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveLiveSubinfoResponse {
    pub total_gift_pt: Option<String>,
    pub viewer_count: Option<i32>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetFesArchiveDataResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub gift_pt_rankings: Option<Vec<LiveGiftPtRanking>>,
    pub live_location_id: Option<i32>,
    pub costume_ids: Option<Vec<i32>>,
    pub timeline_ids: Option<Vec<i64>>,
    pub archive_url: Option<String>,
    pub video_url: Option<String>,
    pub chapters: Option<Vec<ArchiveFesliveChapter>>,
    pub has_extra_admission: Option<bool>,
    pub current_play_time_second: Option<i32>,
    pub total_play_time_second: Option<i32>,
    pub unixtime_live_rec_started: Option<i64>,
    pub live_start_time: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_code: Option<i32>,
    pub viewer_count: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub user_gift_pt: Option<i32>,
    pub user_gift_pt_ranking: Option<i32>,
    pub total_gift_pt: Option<String>,
    pub characters: Option<Vec<LiveArchiveCharacter>>,
    pub enquetes: Option<Vec<FesArchiveEnqueteWindow>>,
    pub camera_focus_character_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCameraType
    pub selectable_camera_types: Option<Vec<i32>>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCameraType
    pub current_camera_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTicketRank
    pub ticket_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetWithArchiveDataResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub gift_pt_rankings: Option<Vec<LiveGiftPtRanking>>,
    pub live_location_id: Option<i32>,
    pub costume_ids: Option<Vec<i32>>,
    pub timeline_ids: Option<Vec<i64>>,
    pub archive_url: Option<String>,
    pub video_url: Option<String>,
    pub chapters: Option<Vec<ArchiveWithliveChapter>>,
    pub has_extra: Option<bool>,
    pub gift_star_reward_segments: Option<Vec<i32>>,
    pub gift_stars_threshold_for_extra_admission: Option<i32>,
    pub has_extra_admission: Option<bool>,
    pub current_play_time_second: Option<i32>,
    pub total_play_time_second_without_extra: Option<i32>,
    pub total_play_time_second_including_extra: Option<i32>,
    pub unixtime_live_rec_started: Option<i64>,
    pub live_start_time: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_code: Option<i32>,
    pub viewer_count: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub user_gift_pt_without_archive: Option<i32>,
    pub user_gift_pt_including_archive: Option<i32>,
    pub user_gift_pt_ranking: Option<i32>,
    pub total_gift_pt: Option<String>,
    pub characters: Option<Vec<LiveArchiveCharacter>>,
    pub is_archive_published: Option<bool>,
    pub is_horizontal: Option<bool>,
    pub enquetes: Option<Vec<WithliveEnquete>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetChannelListResponse {
    pub channel_list: Option<Vec<ArchiveChannel>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetChannelMovieListResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub banner_image_url: Option<String>,
    pub icon_asset_bundle: Option<String>,
    pub icon_resource_file_name: Option<String>,
    pub is_favorite: Option<bool>,
    pub live_archive_list: Option<Vec<ArchiveLiveInfo>>,
    pub trailer_archive_list: Option<Vec<ArchiveLiveInfo>>,
    pub archive_list: Option<Vec<ArchiveLiveInfo>>,
    pub is_watched: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetFesTimelineDataResponse {
    pub timelines: Option<Vec<ArchiveLiveTimeline>>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetSeasonListResponse {
    pub season_list: Option<Vec<ArchiveSeasonListItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetCancelRecommendChannelResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetFesCameraResponse {
    /// dnSpy: Org.OpenAPITools.Model.LiveCameraType
    pub camera_type: Option<i32>,
    pub focus_character_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetPurchaseTicketResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetRecommendChannelResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveGiftResponse {
    pub user_gift_pt: Option<i32>,
    pub total_user_gift_pt: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveInfoResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub polling_interval: Option<i32>,
    pub has_extra_admission: Option<bool>,
    pub is_archive_published: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithlivePrizeResponse {
    pub user_gift_pt: Option<i32>,
    pub user_gift_pt_without_archive: Option<i32>,
    pub star_info: Option<ArchiveWithlivePrizeStarInfo>,
    pub characters: Option<Vec<ArchiveWithlivePrizeCharacter>>,
    pub season_id: Option<i32>,
    pub season_fanlevel_point_stock: Option<i32>,
    pub season_fanlevel_point_stock_limit: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveStarsResponse {
    /// dnSpy: ArchiveWithliveStarsPost returns ApiResponse<object>; no stable fields confirmed.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TutorialsStatusInfo {
    pub tutorial_id: Option<i32>,
    pub is_complete: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FavoriteChannel {
    pub live_channels_id: Option<i32>,
    pub name: Option<String>,
    pub is_watched: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TicketInfo {
    pub item_id: Option<i32>,
    pub shop_item_id: Option<i32>,
    pub is_paid_sisca_only: Option<bool>,
    pub price: Option<i32>,
    pub order_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTicketRank
    pub ticket_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveInfoCharacterListInner {
    pub character_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveInfo {
    pub archives_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveType
    pub live_type: Option<i32>,
    pub live_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub ticket_list: Option<Vec<TicketInfo>>,
    pub trailer_start_time: Option<String>,
    pub trailer_end_time: Option<String>,
    pub live_start_time: Option<String>,
    pub live_end_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub external_link: Option<String>,
    pub thumbnail_image_url: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveTicketRank
    pub ticket_rank: Option<i32>,
    pub character_list: Option<Vec<LiveInfoCharacterListInner>>,
    pub total_playing_time_second: Option<i32>,
    pub is_publish_video_url: Option<bool>,
    pub is_extra_started: Option<bool>,
    pub has_extra: Option<bool>,
    pub has_extra_admission: Option<bool>,
    pub earned_star_count: Option<i32>,
    pub gift_stars_threshold_for_extra_admission: Option<i32>,
    pub video_url: Option<String>,
    pub archive_video_size_in_byte: Option<i64>,
    pub is_scheduled_start_time_visible: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FilterableCharacter {
    pub character_id: Option<i32>,
    pub label: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SortableField {
    pub key: Option<String>,
    pub label: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnterCharacter {
    pub character_id: Option<i32>,
    pub is_guest: Option<bool>,
    pub season_fan_level: Option<i32>,
    pub season_exp_per_next: Option<f32>,
    pub member_fan_level: Option<i32>,
    pub member_exp_per_next: Option<f32>,
    pub order: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MemberRanking {
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub rank: Option<i32>,
    pub point: Option<i64>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveGiftPtRanking {
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_icon_parts_info: Option<String>,
    pub ranking: Option<i32>,
    pub gift_pt: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveFesliveChapter {
    pub name: Option<String>,
    pub play_time_second: Option<i32>,
    pub is_extra: Option<bool>,
    pub is_available: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveChapter {
    pub name: Option<String>,
    pub play_time_second: Option<i32>,
    pub is_extra: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LiveArchiveCharacter {
    pub character_id: Option<i32>,
    pub is_guest: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesArchiveEnqueteWindow {
    pub start_time_second: Option<i32>,
    pub end_time_second: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithlivePrizeStarInfo {
    pub star_num: Option<i32>,
    pub member_exp_per_star: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeMemberFanLevelInfo {
    pub before_level: Option<i64>,
    pub before_exp: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithlivePrizeCharacter {
    pub character_id: Option<i32>,
    pub member_fan_level_info: Option<WithlivePrizeMemberFanLevelInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LivePrize {
    pub item_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveCircleChatCommentResponse {
    pub circle_chat_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveCommentResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGiftPtRankingsResponse {
    pub gift_pt_rankings: Option<Vec<LiveGiftPtRanking>>,
    pub user_gift_pt: Option<i32>,
    pub user_gift_pt_ranking: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub total_gift_pt: Option<String>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGiftResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub animation: Option<LiveTimelineAnimation>,
    pub user_gift_pt: Option<i32>,
    pub total_user_gift_pt: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub display_user_icon: Option<bool>,
    pub image_size: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGrandPrixRankingBorder {
    pub range_text: Option<String>,
    pub start_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixPointRanking {
    pub id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub icon_parts_info: Option<String>,
    pub rank: Option<i32>,
    pub point: Option<i64>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGrandPrixRankingResponse {
    pub borders: Option<Vec<FesliveGrandPrixRankingBorder>>,
    pub user_rank: Option<i32>,
    pub ranking: Option<Vec<GrandPrixPointRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnqueteQuestionOption {
    pub text: Option<String>,
    pub value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnqueteQuestion {
    pub enquete_id: Option<String>,
    pub expire_time: Option<String>,
    pub text: Option<String>,
    pub options: Option<Vec<FesliveEnqueteQuestionOption>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnqueteResultAnswer {
    pub text: Option<String>,
    pub rate: Option<i32>,
    pub is_owned: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnqueteResult {
    pub enquete_id: Option<String>,
    pub text: Option<String>,
    pub answers: Option<Vec<FesliveEnqueteResultAnswer>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveAnnouncementEnquete {
    pub question: Option<FesliveEnqueteQuestion>,
    pub result: Option<FesliveEnqueteResult>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveAnnouncementResponse {
    pub announcement: Option<FesliveAnnouncement>,
    pub enquete: Option<FesliveLiveAnnouncementEnquete>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyAnnouncementResponse {
    pub announcement: Option<FesliveAnnouncement>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveMessageCardResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub user_gift_pt: Option<i32>,
    pub total_user_gift_pt: Option<i32>,
    pub message: Option<String>,
    pub text_size: Option<i32>,
    pub is_text_bold: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveOutQuestLiveRanking {
    pub id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub icon_parts_info: Option<String>,
    pub rank: Option<i32>,
    pub point: Option<i64>,
    pub is_mute_player: Option<bool>,
    pub is_self: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveOutQuestLiveRankingResponse {
    pub borders: Option<Vec<i32>>,
    pub my_rank: Option<i32>,
    pub rankings: Option<Vec<FesliveOutQuestLiveRanking>>,
    pub higher_target_rank: Option<i32>,
    pub lower_target_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePrizeSeasonFanLevelInfo {
    pub in_season: Option<bool>,
    pub max_level: Option<i64>,
    pub before_level: Option<i64>,
    pub before_exp_rate: Option<f32>,
    pub after_level: Option<i64>,
    pub after_exp_rate: Option<f32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePrizeMemberFanLevelInfo {
    pub before_level: Option<i64>,
    pub before_exp_rate: Option<f32>,
    pub after_level: Option<i64>,
    pub after_exp_rate: Option<f32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePrizeCharacter {
    pub character_id: Option<i32>,
    pub season_fan_level_info: Option<FeslivePrizeSeasonFanLevelInfo>,
    pub member_fan_level_info: Option<FeslivePrizeMemberFanLevelInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePrizeResponse {
    pub seat_rank_name: Option<String>,
    pub seat_rank_prizes: Option<Vec<LivePrize>>,
    pub mission_prizes: Option<Vec<LivePrize>>,
    pub common_prizes: Option<Vec<LivePrize>>,
    pub characters: Option<Vec<FeslivePrizeCharacter>>,
    pub season_id: Option<i32>,
    pub is_exhibition: Option<bool>,
    pub is_display_ranking: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveSetCameraResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.LiveCameraType
    pub camera_type: Option<i32>,
    pub focus_character_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SetFlowerStandResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveCommentResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnqueteAnswerResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.WithliveEnqueteStatus
    pub status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGiftPtRankingsResponse {
    pub gift_pt_rankings: Option<Vec<LiveGiftPtRanking>>,
    pub user_gift_pt: Option<i32>,
    pub user_gift_pt_ranking: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub total_gift_pt: Option<String>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGiftResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub animation: Option<LiveTimelineAnimation>,
    pub user_gift_pt: Option<i32>,
    pub total_user_gift_pt: Option<i32>,
    pub user_icon_parts_info: Option<String>,
    pub display_user_icon: Option<bool>,
    pub image_size: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveMessageCardResponse {
    pub timeline_id: Option<String>,
    pub name_color: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub user_gift_pt: Option<i32>,
    pub total_user_gift_pt: Option<i32>,
    pub message: Option<String>,
    pub text_size: Option<i32>,
    pub is_text_bold: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeStarInfo {
    pub star_num: Option<i32>,
    pub member_exp_per_star: Option<i64>,
    pub season_exp_per_star: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeSeasonFanLevelInfo {
    pub in_season: Option<bool>,
    pub max_level: Option<i64>,
    pub before_level: Option<i64>,
    pub before_exp: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeCharacter {
    pub character_id: Option<i32>,
    pub season_fan_level_info: Option<WithlivePrizeSeasonFanLevelInfo>,
    pub member_fan_level_info: Option<WithlivePrizeMemberFanLevelInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeResponse {
    pub items: Option<Vec<LivePrize>>,
    pub user_gift_pt: Option<i32>,
    pub star_info: Option<WithlivePrizeStarInfo>,
    pub characters: Option<Vec<WithlivePrizeCharacter>>,
    pub season_id: Option<i32>,
    pub season_fanlevel_point_stock: Option<i32>,
    pub season_fanlevel_point_stock_limit: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveResultRankingBorder {
    pub range_text: Option<String>,
    pub start_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveResultRankingItem {
    pub rank: Option<i32>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub user_icon_parts_info: Option<String>,
    pub gift_pt: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveResultRankingResponse {
    pub borders: Option<Vec<WithliveResultRankingBorder>>,
    pub user_rank: Option<i32>,
    pub ranking: Option<Vec<WithliveResultRankingItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveSetStarResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleMember {
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    pub player_level: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.PositionType
    pub position_type: Option<i32>,
    pub profile_card_parts_info: Option<String>,
    pub profile_icon_parts_info: Option<String>,
    /// dnSpy type: System.DateTime
    pub last_login_date_time: Option<String>,
    pub is_voice_chat_ban: Option<bool>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct InviteJoinInfo {
    pub d_guild_invitations_id: Option<String>,
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    pub player_level: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_card_parts_info: Option<String>,
    pub profile_icon_parts_info: Option<String>,
    /// dnSpy type: System.DateTime
    pub last_login_date_time: Option<String>,
    /// dnSpy type: System.Nullable<System.DateTime>
    pub create_time: Option<String>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct InvitationsInfo {
    pub d_guild_invitations_id: Option<String>,
    pub search_guild_key: Option<String>,
    pub guild_name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildRank
    pub guild_rank: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildPolicyAction
    pub guild_policy_action: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GuildPolicyRecruiting
    pub guild_policy_recruiting: Option<i32>,
    pub guild_member_num: Option<i64>,
    pub guild_icon_parts_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleChatLogList {
    pub d_chat_datas_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ChatType
    pub chat_type: Option<i32>,
    pub player_id: Option<String>,
    pub comment: Option<String>,
    pub order_id: Option<i64>,
    /// dnSpy type: System.Nullable<System.DateTime>
    pub create_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CommenterInfoList {
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemRequestResult {
    pub item_id: Option<i32>,
    pub num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetChatLogListResponse {
    pub circle_chat_log_list: Option<Vec<CircleChatLogList>>,
    pub commenter_info_list: Option<Vec<CommenterInfoList>>,
    pub polling_interval: Option<i64>,
    pub latest_chat_order_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetCircleTopInfoResponse {
    pub circle_info: Option<CircleInfo>,
    pub circle_member_list: Option<Vec<CircleMember>>,
    pub circle_ranking_info: Option<CircleRankingInfo>,
    pub item_request_result: Option<ItemRequestResult>,
    pub emoji_list: Option<Vec<i32>>,
    pub membership_list: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetDetailResponse {
    pub circle_info: Option<CircleInfo>,
    pub circle_member_list: Option<Vec<CircleMember>>,
    pub circle_ranking_info: Option<CircleRankingInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetInfoResponse {
    pub circle_info: Option<CircleInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetInviteAndJoinInfoResponse {
    pub invite_info_list: Option<Vec<InviteJoinInfo>>,
    pub join_info_list: Option<Vec<InviteJoinInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetInviteListResponse {
    pub invitations_list: Option<Vec<InvitationsInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetListResponse {
    pub circle_info_list: Option<Vec<CircleInfo>>,
    pub page_num: Option<i64>,
    pub total_page: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetApproveInviteResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetApproveJoinResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCancelInviteResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCancelJoinResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetChatMessageResponse {
    pub chat_comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCreateResponse {
    pub circle_info: Option<CircleInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetDismissalResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetDissolutionResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetDonationResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetExpulsionResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetInviteResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetItemRequestResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetJoinResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetOutResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetPositionResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetRejectInviteResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetRejectJoinResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetSettingResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetTransferLeaderResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CommonGetHeaderAnnounsResponse {
    pub d_header_announcements_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.CommonHeaderAnnounsType
    pub announce_type: Option<i32>,
    pub text: Option<String>,
    pub transition_scene: Option<String>,
    pub transition_value: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetExchangeCardHavingListResponse {
    pub exchange_card_list: Option<Vec<GachaExchangeCardHavingInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetExchangeCardListResponse {
    pub exchange_card_list: Option<Vec<GachaExchangeCard>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetGuaranteePointListResponse {
    pub exchange_item_list: Option<Vec<GachaGuaranteePointExchangeItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetHistoryResponse {
    pub gacha_result_history_list: Option<Vec<GachaResultHistory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetLotteryChanceResponse {
    pub gacha_prob_list: Option<Vec<GachaProb>>,
    pub gacha_bonus_prob_list: Option<Vec<GachaBonusProb>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetSeriesListResponse {
    pub gacha_series_list: Option<Vec<GachaSeries>>,
    pub convert_exchange_point_list: Option<Vec<ConvertExchangePoint>>,
    pub select_ticket_series_list: Option<Vec<SelectTicketSeries>>,
    pub expired_limited_gacha_ticket_id_confirm_list: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetGuaranteePointExchangeResponse {
    pub receipt_items: Option<Vec<GachaReceiptItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetPrizeReceiveResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetPurchaseResponse {
    pub gacha_result_list: Option<Vec<GachaResult>>,
    pub gacha_guarantee_point: Option<GachaGuaranteePoint>,
    pub gacha_bonus_result_list: Option<Vec<GachaBonusResult>>,
    pub user_single_ticket_num: Option<i32>,
    pub user_consective_ticket_num: Option<i32>,
    pub gacha_campaign_info_list: Option<Vec<GachaCampaignInfo>>,
    pub gacha_series_list: Option<Vec<GachaSeries>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetSelectTicketExchangeResponse {
    pub receipt_items: Option<Vec<GachaReceiptItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeGetCustomSettingResponse {
    pub sticker_info_list: Option<Vec<i32>>,
    pub user_card_data_list: Option<Vec<i32>>,
    pub profile_info: Option<ProfileInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeGetHomeResponse {
    pub plan_list: Option<Vec<Plan>>,
    pub is_login_bonus: Option<bool>,
    pub profile_info: Option<ProfileInfo>,
    pub is_finish_tutorial: Option<bool>,
    pub is_first_home: Option<bool>,
    pub membership_update_confirm_list: Option<Vec<String>>,
    pub membership_pending_list: Option<Vec<String>>,
    pub quest_quit_info: Option<HomeQuestQuitInfo>,
    pub finished_simple_tutorial_list: Option<Vec<i64>>,
    pub beginner_mission_status: Option<BeginnerMissionStatus>,
    pub is_not_watched_adv_from_current_season: Option<bool>,
    pub has_daily_ticket: Option<bool>,
    pub has_grand_prix_playable_count: Option<bool>,
    pub latest_news_id: Option<String>,
    pub highlighted_badge_info: Option<HomeHighlightedBadgeInfo>,
    pub expired_limited_gacha_ticket_id_confirm_list: Option<Vec<i32>>,
    pub standard_quest_areas_id: Option<i32>,
    pub standard_quest_stages_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeGetLoginBonusResponse {
    pub login_bonus_list: Option<Vec<LoginBonusPeriod>>,
    pub event_login_bonus_list: Option<Vec<LoginBonusPeriod>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeGetWallpaperSettingResponse {
    pub current_home_wall_paper_settings_id: Option<String>,
    pub wall_paper_setting_list: Option<Vec<WallpaperSetting>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeSetClockSettingResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeSetCurrentWallpaperSettingResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeSetShowRetireResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HomeSetWallpaperSettingResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeGetLimitBreakMaterialConvertListResponse {
    pub convert_limit_break_material_list: Option<Vec<ItemExchangeConvertLimitBreakMaterialInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeGetListResponse {
    pub item_exchange_list: Option<Vec<ItemExchangeInfo>>,
    pub is_convertible_limit_break_material: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeSetLimitBreakMaterialConvertResponse {
    pub get_item_list: Option<Vec<ItemExchangeGetItemInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeSetPurchaseResponse {
    pub user_product_item_num: Option<i64>,
    pub user_material_item_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemStoreGetListResponse {
    pub bundle_id: Option<String>,
    pub item_store_list: Option<Vec<ItemStore>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopGetBirthdayResponseBirthday {
    /// dnSpy type: System.DateTime
    pub birthday: Option<String>,
    pub age: Option<i64>,
    pub buyable: Option<bool>,
    pub total_amounts: Option<i64>,
    pub balance: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopGetBirthdayResponse {
    pub birthday: Option<JewelShopGetBirthdayResponseBirthday>,
    pub has_sisca_charge_pending_receipt: Option<bool>,
    pub has_item_store_pending_receipt: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopGetListResponse {
    pub bundle_id: Option<String>,
    pub jewel_shop_item_info: Option<Vec<JewelShopItemInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopGetMembershipListResponse {
    pub bundle_id: Option<String>,
    pub membership_list: Option<Vec<MembershipListInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopSetBirthdayResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopSetPurchaseResponse {
    pub result: Option<bool>,
    pub payment_items_id: Option<i32>,
    pub name: Option<String>,
    pub product_id: Option<String>,
    pub tier: Option<i64>,
    pub paid_jewel: Option<i64>,
    pub free_jewel: Option<i64>,
    pub membership_status: Option<MembershipStatus>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PetalExchangeGetListResponse {
    pub petal_coin_num: Option<i64>,
    pub character_petal_exchange_list: Option<Vec<CharacterPetalExchangeInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PetalExchangeSetPurchaseResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxGetHistoryResponse {
    pub total_count: Option<i32>,
    pub page: Option<i32>,
    pub items: Option<Vec<PresentItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxGetListResponse {
    pub total_count: Option<i32>,
    pub page: Option<i32>,
    pub items: Option<Vec<PresentItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxItemDetailResponse {
    pub item: Option<PresentItem>,
    pub num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxSetItemAllResponse {
    pub result: Option<i32>,
    pub receipt_success_items: Option<Vec<PresentItem>>,
    pub receipt_failed_items: Option<Vec<PresentItem>>,
    pub receipt_exclude_items: Option<Vec<PresentItem>>,
    pub grade_up_data: Option<GradeUpData>,
    pub is_not_received_grade_reward: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxSetItemResponse {
    pub result: Option<i32>,
    pub receipt_success_item: Option<PresentItem>,
    pub receipt_failed_item: Option<PresentItem>,
    pub grade_up_data: Option<GradeUpData>,
    pub is_not_received_grade_reward: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UseFanLevelPointStock {
    pub characters_id: Option<i32>,
    pub use_stock: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifyRhythmGameDeckCard {
    pub d_card_datas_id: Option<String>,
    pub slot_no: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifyRhythmGameDeck {
    pub deck_no: Option<i32>,
    pub deck_card_list: Option<Vec<ModifyRhythmGameDeckCard>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetDeckData {
    pub slot_no: Option<i32>,
    pub d_card_datas_id: Option<String>,
    pub is_center: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetPositionData {
    pub series_deck_card_datas_id: Option<String>,
    pub slot_no: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct NotesResultInfo {
    pub order: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.NotesJudgementType
    pub judgement_result: Option<i32>,
    pub timing_msec: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGamePlayData {
    pub skill_use_count: Option<i32>,
    pub center_skill_use_count: Option<i32>,
    pub max_voltage_level: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixDeckCardInfo {
    pub series_deck_card_datas_id: Option<String>,
    pub d_card_datas_id: Option<String>,
    pub slot_no: Option<i32>,
    pub is_center: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameReward {
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub reward_type: Option<i32>,
    pub reward_item_id: Option<i32>,
    pub reward_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MusicMasteryLevelResultInfo {
    pub music_id: Option<i32>,
    pub music_mastery_exp_before: Option<i64>,
    pub music_mastery_exp_after: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.MusicOwnStatus
    pub own_status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameClassMissionInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameConditionType
    pub condition_type: Option<i32>,
    pub progress_num: Option<i64>,
    pub received_order: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MusicScore {
    /// dnSpy enum: Org.OpenAPITools.Model.MusicScoreDifficulty
    pub difficulty: Option<i32>,
    pub technical_score: Option<i32>,
    pub best_combo: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub combo_achievement_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.MusicScoreClearLamp
    pub clear_lamp: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameMusic {
    pub music_id: Option<i32>,
    pub music_scores: Option<Vec<MusicScore>>,
    pub high_score: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub high_score_achievement_status: Option<i32>,
    pub music_mastery_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameItemInfo {
    pub d_item_datas_id: Option<String>,
    pub item_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MusicScoreMissionInfo {
    pub high_score_before: Option<i64>,
    pub high_score_after: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub high_score_achievement_status_before: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub high_score_achievement_status_after: Option<i32>,
    pub max_combo_before: Option<i32>,
    pub max_combo_after: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub combo_achievement_status_before: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameAchievementStatus
    pub combo_achievement_status_after: Option<i32>,
    pub technical_score_before: Option<i32>,
    pub technical_score_after: Option<i32>,
    pub reward_list: Option<Vec<RhythmGameReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ClassMissionProgressInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameConditionType
    pub condition_type: Option<i32>,
    pub progress_before: Option<i64>,
    pub progress_after: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixPointBonus {
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixPointBonusType
    pub target_type: Option<i32>,
    pub target_detail: Option<i32>,
    pub target_num: Option<i32>,
    pub bonus_value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixPointInfo {
    pub point: Option<i64>,
    pub point_bonus_list: Option<Vec<GrandPrixPointBonus>>,
    pub before_point: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixCardUsedInfo {
    pub card_series_id: Option<i32>,
    pub used_rate: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSeries {
    pub grand_prix_rhythm_game_series_id: Option<i32>,
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixDeckCardInfo>>,
    pub best_score: Option<i64>,
    pub current_daily_point: Option<i64>,
    pub is_locked: Option<bool>,
    pub top_used_list: Option<Vec<RhythmGameGrandPrixCardUsedInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixPersonalInfo {
    pub personal_total_point: Option<i64>,
    pub personal_ranking: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixCircleInfo {
    pub search_guild_key: Option<String>,
    pub guild_name: Option<String>,
    pub guild_total_point: Option<i64>,
    pub guild_ranking: Option<i32>,
    pub within_guild_ranking: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixRankingScoreInfo {
    pub ranking_score: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.GameContentsType
    pub contents_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixGetPoint {
    pub quest_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixGetPointType
    pub get_point_type: Option<i32>,
    pub point: Option<i64>,
    /// dnSpy type: System.DateTime
    pub get_point_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SelectTicketExchangeInfo {
    pub select_ticket_series_id: Option<i32>,
    pub user_select_ticket_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ShopInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.ShopType
    pub shop_type: Option<i32>,
    pub name: Option<String>,
    pub order_id: Option<i32>,
    pub is_maintenance: Option<bool>,
    pub is_exist_products: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SiscaStoreItemInfo {
    pub sisca_store_datas_id: Option<i32>,
    pub own_num: Option<i64>,
    pub buyable_num: Option<i64>,
    pub is_new_item: Option<bool>,
    pub is_sale: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StepUpBeginnerMission {
    pub mission_id: Option<i32>,
    pub achieved_num: Option<i64>,
    pub is_achieved: Option<bool>,
    pub is_received: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StickerExchangeInfo {
    pub sticker_exchanges_id: Option<i32>,
    pub is_having: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Mute {
    pub player_id: Option<String>,
    pub player_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MyDesignInfo {
    pub d_profile_my_designs_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.MyDesignType
    pub my_design_type: Option<i32>,
    pub my_design_parts_info: Option<String>,
    pub my_design_name: Option<String>,
    pub is_setting: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfilePointHistory {
    /// dnSpy type: System.DateTime
    pub date: Option<String>,
    pub message: Option<String>,
    pub point: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSeasonPointHistory {
    pub season_id: Option<i32>,
    pub point_history_list: Option<Vec<ProfilePointHistory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileEarnProgressInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.FanLevelPointEarnType
    pub earn_type: Option<i32>,
    pub total_point: Option<i64>,
    pub season_point_history_list: Option<Vec<ProfileSeasonPointHistory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetFanLevelInfoResponseFanLevelInfoListInner {
    pub character_id: Option<i32>,
    pub d_season_fan_level: Option<i64>,
    pub d_season_fan_experience: Option<i64>,
    pub member_fan_level: Option<i64>,
    pub member_fan_experience: Option<i64>,
    pub earn_progress_list: Option<Vec<ProfileEarnProgressInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileDeleteMyDesignResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetFanLevelInfoResponse {
    pub fan_level_info_list: Option<Vec<ProfileGetFanLevelInfoResponseFanLevelInfoListInner>>,
    pub season_id: Option<i32>,
    pub season_fanlevel_point_stock: Option<i32>,
    pub season_fanlevel_point_stock_limit: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetInfoResponse {
    pub profile_info: Option<ProfileInfo>,
    pub is_own: Option<bool>,
    pub is_mute: Option<bool>,
    pub is_report: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.ProfileFriendRequestStatus
    pub friend_request_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ProfileCircleInviteStatus
    pub circle_invite_status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetMuteListResponse {
    pub mute_list: Option<Vec<Mute>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetMyDesignCardListResponse {
    pub my_design_list: Option<Vec<MyDesignInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetMyDesignIconListResponse {
    pub my_design_list: Option<Vec<MyDesignInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetMyDesignIconResponse {
    pub profile_icon_parts_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetProfileCardResponse {
    pub profile_card_parts_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetProfileIconResponse {
    pub profile_icon_parts_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetBirthdayResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetCommentResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMuteCancelResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMuteResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignCardResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignIconResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignNameResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetNameResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetProfileCardResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetProfileIconResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetReportResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SeasonFanLevelInfoList {
    pub characters_id: Option<i32>,
    pub d_season_fan_level: Option<i64>,
    pub d_season_fan_experience: Option<i64>,
    pub grade: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileUseFanlevelPointStocksResponse {
    pub season_fan_level_info_list: Option<Vec<SeasonFanLevelInfoList>>,
    pub season_fanlevel_point_stock: Option<i32>,
    pub season_fanlevel_point_stock_limit: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterApproveTermsResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.ApproveTermsType
    pub r#type: Option<i32>,
    pub player_id: Option<String>,
    pub device_specific_id: Option<String>,
    pub session_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterGetTermsResponse {
    pub terms: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSetNewUserResponse {
    pub player_id: Option<String>,
    pub device_specific_id: Option<String>,
    pub session_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSetUserDataResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetCenterResponse {
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixDeckCardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetDeckResponse {
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixDeckCardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetPositionResponse {
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixDeckCardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetFinishLiveResponse {
    pub music_score_mission_result: Option<MusicScoreMissionInfo>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameLiveClearStatus
    pub clear_status: Option<i32>,
    pub class_mission_progress_list: Option<Vec<ClassMissionProgressInfo>>,
    pub rhythm_game_star_total_count: Option<i32>,
    pub grand_prix_point_info: Option<GrandPrixPointInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixTopResponse {
    pub series_list: Option<Vec<RhythmGameGrandPrixSeries>>,
    pub ranking: Option<i32>,
    pub ranking_score: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixRankDisplayType
    pub rank_display_type: Option<i32>,
    pub daily_total_grand_prix_point: Option<i64>,
    pub personal_ranking_info: Option<RhythmGameGrandPrixPersonalInfo>,
    pub circle_ranking_info: Option<RhythmGameGrandPrixCircleInfo>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixStatus
    pub status: Option<i32>,
    pub predicated_ranking_score_info: Option<GrandPrixRankingScoreInfo>,
    pub get_points: Option<Vec<GrandPrixGetPoint>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameLiveSetFinishResponse {
    pub music_mastery_level_result: Option<MusicMasteryLevelResultInfo>,
    pub drop_reward_list: Option<Vec<RhythmGameReward>>,
    pub music_score_mission_result: Option<MusicScoreMissionInfo>,
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameLiveClearStatus
    pub clear_status: Option<i32>,
    pub class_mission_progress_list: Option<Vec<ClassMissionProgressInfo>>,
    pub rhythm_game_star_total_count: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.CampaignType
    pub applied_campaign_types: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameReceiveClassMissionResponse {
    pub received_class_data_ids: Option<Vec<i32>>,
    pub class_mission_info: Option<RhythmGameClassMissionInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameReceiveTotalMissionResponse {
    pub received_total_mission_ids: Option<Vec<i32>>,
    pub received_total_mission_order: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameSetMusicResponse {
    pub music_info: Option<RhythmGameMusic>,
    pub consumed_item: Option<RhythmGameItemInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    /// dnSpy type: System.DateTime
    pub start_time: Option<String>,
    /// dnSpy type: System.DateTime
    pub end_time: Option<String>,
    pub video_url: Option<String>,
    pub lobby_image_url: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveEnterResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub live_info: Option<WebviewLiveInfo>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveLiveInfoResponse {
    pub timelines: Option<Vec<LiveTimeline>>,
    pub live_info: Option<WebviewLiveInfo>,
    pub polling_interval: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveLoginResponse {
    pub session_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SelectTicketExchangeGetListResponse {
    pub select_ticket_exchange_list: Option<Vec<SelectTicketExchangeInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ShopGetListResponse {
    pub shop_list: Option<Vec<ShopInfo>>,
    pub is_convertible_limit_break_material: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SiscaStoreGetListResponse {
    pub item_list: Option<Vec<SiscaStoreItemInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SiscaStoreSetPurchaseResponse {
    pub sisca_info: Option<SiscaInfo>,
    pub user_item_info: Option<UserItemInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StepUpBeginnerMissionGetListResponse {
    pub step_up_mission_list: Option<Vec<StepUpBeginnerMission>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StepUpBeginnerMissionSetRewardResponse {
    pub received_reward_list: Option<Vec<RhythmGameReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StickerExchangeGetListResponse {
    pub sticker_point_num: Option<i64>,
    pub sticker_exchange_list: Option<Vec<StickerExchangeInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StickerExchangeSetPurchaseResponse {
    pub sticker_point_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TutorialSetStepResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendInfo {
    pub d_friends_id: Option<String>,
    pub player_id: Option<String>,
    pub name: Option<String>,
    pub player_level: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub comment: Option<String>,
    pub profile_icon_parts_info: Option<String>,
    pub last_login: Option<String>,
    pub friend_num: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.ProfileFriendRequestStatus
    pub friend_request_status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendRequestInfo {
    pub d_friend_requests_id: Option<String>,
    pub player_id: Option<String>,
    pub name: Option<String>,
    pub player_level: Option<i64>,
    pub comment: Option<String>,
    pub profile_icon_parts_info: Option<String>,
    pub last_login: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendRequestInfoList {
    pub waiting_list: Option<Vec<FriendRequestInfo>>,
    pub requesting_list: Option<Vec<FriendRequestInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendGetListResponse {
    pub friend_info_list: Option<Vec<FriendInfo>>,
    pub max_friend_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendGetRequestListResponse {
    pub friend_request_info_data: Option<FriendRequestInfoList>,
    pub max_friend_waiting_count: Option<i64>,
    pub max_friend_requesting_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchNameResponse {
    pub friend_info_list: Option<Vec<FriendInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchPlayerIdResponse {
    pub friend_info: Option<FriendInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchRecommendResponse {
    pub friend_info_list: Option<Vec<FriendInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetCommonResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendUpdateRequestViewHistoryResponse {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SiscaInfo {
    pub free: Option<i32>,
    pub paid: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserItemInfo {
    pub user_item_id: Option<String>,
    pub item_id: Option<i32>,
    pub item_num: Option<i32>,
    pub limit_date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemGetQuest {
    pub quest_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_type: Option<i32>,
    pub quest_name: Option<String>,
    pub is_transition: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserItem {
    pub user_item_id: Option<String>,
    pub item_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.Rarity
    pub rarity: Option<i32>,
    pub item_num: Option<i32>,
    /// dnSpy type: System.DateTime
    pub limit_date_time: Option<String>,
    pub resource_file_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemCategoryList {
    /// dnSpy enum: Org.OpenAPITools.Model.ItemCategory
    pub item_category: Option<i32>,
    pub user_item_list: Option<Vec<UserItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelHistory {
    /// dnSpy type: System.DateTime
    pub date: Option<String>,
    pub usage: Option<String>,
    pub jewel_free: Option<i64>,
    pub jewel_paid: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserItemGetDetailResponse {
    pub d_item_datas_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub item_num: Option<i32>,
    pub quest_list: Option<Vec<ItemGetQuest>>,
    /// dnSpy type: System.DateTime
    pub limit_date_time: Option<String>,
    pub transition_scene: Option<String>,
    pub resource_file_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserItemsGetListResponse {
    pub item_category_list: Option<Vec<ItemCategoryList>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserJewelGetHistoryResponse {
    pub jewel_free: Option<i64>,
    pub jewel_paid: Option<i64>,
    pub jewel_histories: Option<Vec<JewelHistory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserSetContentsReleaseEffectHistoryResponse {
    pub contents_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ContentsReleaseEffectType
    pub status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionInfo {
    pub mission_id: Option<i32>,
    pub achieved_num: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.BeginnerMissionStatus
    pub status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionCategory {
    /// dnSpy enum: Org.OpenAPITools.Model.MissionType
    pub mission_type: Option<i32>,
    pub is_received: Option<bool>,
    pub mission_list: Option<Vec<BeginnerMissionInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionGetListResponse {
    pub mission_category_list: Option<Vec<BeginnerMissionCategory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionSetRewardAllResponse {
    pub mission_id_list: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MissionRewardInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.MissionRewardType
    pub reward_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_id: Option<i32>,
    pub reward_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MissionInfo {
    pub mission_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub condition_achieve_num: Option<i64>,
    pub achieved_num: Option<i64>,
    pub transition_contents_id: Option<i32>,
    pub is_achieved: Option<bool>,
    pub reward_list: Option<Vec<MissionRewardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MissionAchieveRewardInfo {
    pub achieved_num: Option<i64>,
    pub target_num: Option<i64>,
    pub is_achieved: Option<bool>,
    pub reward_list: Option<Vec<MissionRewardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MissionGroupInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.MissionType
    pub mission_type: Option<i32>,
    pub series_name: Option<String>,
    pub series_description: Option<String>,
    /// dnSpy type: System.DateTime
    pub start_date: Option<String>,
    /// dnSpy type: System.DateTime
    pub end_date: Option<String>,
    pub mission_list: Option<Vec<MissionInfo>>,
    pub achieve_reward_list: Option<Vec<MissionAchieveRewardInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CommonMission {
    pub school_stage_star: Option<i32>,
    pub rhythm_game_star: Option<i32>,
    pub received_order: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MissionGetListResponse {
    pub mission_list: Option<Vec<MissionGroupInfo>>,
    pub common_mission_info: Option<CommonMission>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserDailyTicketInfo {
    pub num: Option<i64>,
    pub max: Option<i64>,
    /// dnSpy type: System.DateTime
    pub next_reset_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StageReward {
    pub item_id: Option<i32>,
    pub is_received: Option<bool>,
    pub reward_type: Option<i64>,
    pub item_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StageMusic {
    pub m_musics_id: Option<i32>,
    pub is_enable: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Stage {
    pub stage_id: Option<i32>,
    pub clear_status: Option<i64>,
    pub is_lock: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetRecoveryChallengeCountResponse {
    pub challenge_count: Option<i64>,
    pub recovery_count: Option<i64>,
    pub max_recovery_count: Option<i64>,
    pub price: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetReleaseTicketResponse {
    pub items_id: Option<i32>,
    pub item_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageDataResponse {
    pub stage_id: Option<i32>,
    pub clear_status: Option<i64>,
    pub best_love_music_id: Option<i32>,
    pub user_daily_ticket_info: Option<UserDailyTicketInfo>,
    pub stage_reward_list: Option<Vec<StageReward>>,
    pub music_list: Option<Vec<StageMusic>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageListResponse {
    pub user_daily_ticket_info: Option<UserDailyTicketInfo>,
    pub stage_list: Option<Vec<Stage>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageSelectResponse {
    pub user_daily_ticket_info: Option<UserDailyTicketInfo>,
    pub temporary_release_daily_quest_series_id_list: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyRecoveryChallengeCountResponse {
    pub challenge_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailySetReleaseResponse {
    pub result: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DreamLiveCondition {
    pub condition_text: Option<String>,
    pub achieve_count: Option<i64>,
    pub is_release: Option<bool>,
    pub is_display_progress_num: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DreamLiveInfo {
    pub dream_quest_series_id: Option<i32>,
    pub dream_quest_stages_id: Option<i32>,
    pub clear_times: Option<i64>,
    pub is_clear: Option<bool>,
    pub is_get_card: Option<bool>,
    pub limit_break_times: Option<i64>,
    pub condition_list: Option<Vec<DreamLiveCondition>>,
    pub music_list: Option<Vec<StageMusic>>,
    pub dream_live_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamGetMemberSelectResponse {
    pub quest_lives: Option<Vec<DreamLiveInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamGetResultResponse {
    pub stage_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub is_clear: Option<bool>,
    pub result_love: Option<i64>,
    pub get_card_id: Option<i32>,
    pub music_id: Option<i32>,
    pub play_report: Option<String>,
    pub first_clear_flag: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamNotifyMemberReleaseConfirmResponse {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamSetCardResponse {
    pub card_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MusicMasteryBonus {
    /// dnSpy enum: Org.OpenAPITools.Model.MusicMasteryBonusId
    pub bonus_id: Option<i32>,
    pub level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CharacterBonus {
    pub character_id: Option<i32>,
    pub music_mastery_bonus: Option<i64>,
    pub love_correction_value: Option<i64>,
    pub music_mastery_bonus_list: Option<Vec<MusicMasteryBonus>>,
    pub season_fan_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CardParameters {
    pub smile: Option<i64>,
    pub pure: Option<i64>,
    pub cool: Option<i64>,
    pub mental: Option<i64>,
    pub beat_point: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SkillData {
    /// dnSpy enum: Org.OpenAPITools.Model.SkillType
    pub skill_type: Option<i32>,
    pub card_skill_series_id: Option<i32>,
    pub skill_level: Option<i64>,
    pub max_skill_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameSkillData {
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameSkillType
    pub rhythm_game_skill_type: Option<i32>,
    pub skill_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserCardData {
    pub d_card_datas_id: Option<String>,
    pub card_datas_id: Option<i32>,
    pub card_name: Option<String>,
    pub style_level: Option<i64>,
    pub max_style_level: Option<i64>,
    pub limit_break_times: Option<i64>,
    pub max_limit_break_times: Option<i64>,
    pub card_parameters: Option<CardParameters>,
    pub skill_list: Option<Vec<SkillData>>,
    pub character_id: Option<i32>,
    pub generations_id: Option<i32>,
    pub series_type: Option<i64>,
    pub card_sort_order: Option<i64>,
    pub character_bonus: Option<CharacterBonus>,
    pub is_evolve_possible: Option<bool>,
    pub is_evolve_max: Option<bool>,
    pub member_fan_level: Option<i64>,
    pub is_limit_break: Option<bool>,
    pub is_style_level_up: Option<bool>,
    pub rhythm_game_skill_list: Option<Vec<RhythmGameSkillData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DeckCards {
    pub d_deck_cards_id: Option<String>,
    pub d_card_datas_id: Option<String>,
    pub grade_card_bonus_value: Option<i32>,
    pub grade_card_bonus_limit_up: Option<i32>,
    pub slot_no: Option<i64>,
    pub side_style1_d_card_datas_id: Option<String>,
    pub side_style1_grade_card_bonus_value: Option<i32>,
    pub side_style1_grade_card_bonus_limit_up: Option<i32>,
    pub side_style2_d_card_datas_id: Option<String>,
    pub side_style2_grade_card_bonus_value: Option<i32>,
    pub side_style2_grade_card_bonus_limit_up: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Deck {
    pub d_deck_datas_id: Option<String>,
    pub deck_name: Option<String>,
    pub deck_no: Option<i64>,
    pub generations_id: Option<i32>,
    pub ace_card: Option<String>,
    pub deck_cards_list: Option<Vec<DeckCards>>,
    pub is_change_deck_cards: Option<bool>,
    pub grade_bonus_value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RentalDeckCards {
    pub rental_deck_card_id: Option<i32>,
    pub grade_card_bonus_value: Option<i32>,
    pub grade_card_bonus_limit_up: Option<i32>,
    pub side_style1_grade_card_bonus_value: Option<i32>,
    pub side_style1_grade_card_bonus_limit_up: Option<i32>,
    pub side_style2_grade_card_bonus_value: Option<i32>,
    pub side_style2_grade_card_bonus_limit_up: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RentalDeck {
    pub rental_deck_id: Option<i32>,
    pub rental_deck_cards_list: Option<Vec<RentalDeckCards>>,
    pub grade_bonus_value: Option<i32>,
    pub is_released: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RentalCardData {
    pub rental_card_datas_id: Option<i64>,
    pub card_parameters: Option<CardParameters>,
    pub character_bonus: Option<CharacterBonus>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SectionSkill {
    pub section_no: Option<i32>,
    pub section_skills_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetLiveInfoFanLevelInfo {
    pub character_id: Option<i32>,
    pub member_fan_level: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Area {
    pub area_id: Option<i32>,
    pub is_lock: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserStamina {
    pub stamina_now: Option<i64>,
    pub stamina_max: Option<i64>,
    /// dnSpy type: System.DateTime
    pub stamina_recovery_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Reward {
    pub item_id: Option<i32>,
    pub item_num: Option<i64>,
    pub reward_type: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeReward {
    pub grade_rewards_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradeRewardStatus
    pub reward_status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeDetail {
    pub grade_num: Option<i32>,
    pub season_grade_id: Option<i32>,
    pub season_grade_point: Option<i64>,
    pub total_grade_point: Option<i64>,
    pub season_grade_point_rank: Option<i32>,
    pub is_not_received_grade_reward: Option<bool>,
    pub grade_rewards: Option<Vec<GradeReward>>,
    /// dnSpy enum: Org.OpenAPITools.Model.SeasonGradeStatus
    pub status: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuestClearStatusInfo {
    pub stages_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub is_lock: Option<bool>,
    pub star_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetAreaSelectResponse {
    pub name: Option<String>,
    pub area_list: Option<Vec<Area>>,
    pub user_stamina: Option<UserStamina>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveStaminaRecoveryItem {
    pub d_item_datas_id: Option<String>,
    pub item_id: Option<i32>,
    pub item_num: Option<i64>,
    /// dnSpy type: System.DateTime
    pub limit_date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StaminaUseJewelSetting {
    pub jewel_st_recovery_count_limit: Option<i64>,
    pub price: Option<i64>,
    pub effect_value: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestTopResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub standard_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub daily_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub music_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub grade_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub grade_challenge_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub dream_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub grand_prix_status: Option<i32>,
    pub grand_prix_id: Option<i32>,
    /// dnSpy type: System.DateTime
    pub grand_prix_start_datetime: Option<String>,
    /// dnSpy type: System.DateTime
    pub grand_prix_end_datetime: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub raid_event_status: Option<i32>,
    pub raid_event_id: Option<i32>,
    /// dnSpy type: System.DateTime
    pub raid_event_start_datetime: Option<String>,
    /// dnSpy type: System.DateTime
    pub raid_event_end_datetime: Option<String>,
    pub standard_area_select: Option<GetAreaSelectResponse>,
    pub dream_live_member_select: Option<Vec<DreamLiveInfo>>,
    pub grade_detail: Option<GradeDetail>,
    pub open_campaign_id_list: Option<Vec<i32>>,
    pub grade_challenge_season_id: Option<i32>,
    /// dnSpy type: System.DateTime
    pub grade_challenge_season_start_datetime: Option<String>,
    /// dnSpy type: System.DateTime
    pub grade_challenge_season_end_datetime: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestListResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub standard_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub daily_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub music_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub grade_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub dream_live_status: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub grand_prix_status: Option<i32>,
    pub grand_prix_id: Option<i32>,
    /// dnSpy type: System.DateTime
    pub grand_prix_start_datetime: Option<String>,
    /// dnSpy type: System.DateTime
    pub grand_prix_end_datetime: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveStatusFlagType
    pub raid_event_status: Option<i32>,
    pub raid_event_id: Option<i32>,
    /// dnSpy type: System.DateTime
    pub raid_event_start_datetime: Option<String>,
    /// dnSpy type: System.DateTime
    pub raid_event_end_datetime: Option<String>,
    pub user_stamina: Option<UserStamina>,
    pub open_campaign_id_list: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestClearStatusListResponse {
    pub quest_clear_status_list: Option<Vec<QuestClearStatusInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetLiveSettingResponse {
    pub quest_live_type: Option<i64>,
    pub stage_id: Option<i32>,
    pub deck_data: Option<Vec<Deck>>,
    pub best_love_deck: Option<Deck>,
    pub best_love_musics_id: Option<i32>,
    pub user_stamina: Option<UserStamina>,
    pub music_list: Option<Vec<StageMusic>>,
    pub user_card_data_list: Option<Vec<UserCardData>>,
    pub rental_deck_data: Option<Vec<RentalDeck>>,
    pub rental_card_data_list: Option<Vec<RentalCardData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetPlayReportResponse {
    pub quest_live_type: Option<i64>,
    pub play_report: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetResultResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub stage_id: Option<i32>,
    pub quest_result: Option<bool>,
    pub result_love: Option<i64>,
    pub best_love: Option<i64>,
    pub before_best_love: Option<i64>,
    pub add_style_point: Option<i64>,
    pub is_challenge_mode: Option<bool>,
    pub music_id: Option<i32>,
    pub reward_list: Option<Vec<Reward>>,
    pub play_report: Option<String>,
    pub mastary_level_before: Option<i64>,
    pub mastary_level_after: Option<i64>,
    pub mastary_level_experience: Option<i64>,
    pub mastary_level_total_experience_before: Option<i64>,
    pub first_clear_flag: Option<bool>,
    pub first_complete_clear_flag: Option<bool>,
    pub is_limit_over_style_point: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetStaminaRecoveryInfoResponse {
    pub user_stamina: Option<UserStamina>,
    pub item_list: Option<Vec<OutQuestLiveStaminaRecoveryItem>>,
    pub use_jewel_settings: Option<Vec<StaminaUseJewelSetting>>,
    pub jewel_st_recovery_count: Option<i64>,
    pub stamina_limit: Option<i64>,
    pub max_st_recovery_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveRecoveryStaminaResponse {
    pub user_stamina: Option<UserStamina>,
    pub stamina_present: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveSetLiveSettingResponse {
    pub result: Option<bool>,
    pub quest_live_id: Option<String>,
    pub quest_live_type: Option<i64>,
    pub quest_id: Option<i32>,
    pub is_challenge_mode: Option<bool>,
    pub music_id: Option<i32>,
    pub deck_data: Option<Deck>,
    pub rental_deck_data: Option<RentalDeck>,
    pub character_bonus: Option<CharacterBonus>,
    pub section_skill_list: Option<Vec<SectionSkill>>,
    pub init_hand_data: Option<String>,
    pub grand_prix_retry_count: Option<i32>,
    pub grand_prix_is_rehearsal: Option<bool>,
    pub grand_prix_id: Option<i32>,
    pub grade_retry_count: Option<i32>,
    pub grade_add_skill_list: Option<Vec<i64>>,
    pub playable_count: Option<i32>,
    pub play_count: Option<i32>,
    pub fan_level_info_list: Option<Vec<GetLiveInfoFanLevelInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeUpData {
    pub before_grade_num: Option<i32>,
    pub after_grade_num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeQuestReward {
    pub grade_quest_rewards_id: Option<i32>,
    pub is_received: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeQuestSeriesInfo {
    pub grade_quest_series_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradePlayStatus
    pub play_status: Option<i32>,
    pub is_lock: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradeClearStatus
    pub clear_status: Option<i32>,
    pub reward_list: Option<Vec<GradeQuestReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeSeasonInfo {
    pub grade_quest_season_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradePlayStatus
    pub play_status: Option<i32>,
    pub is_lock: Option<bool>,
    pub clear_num: Option<i32>,
    pub quest_series_list: Option<Vec<GradeQuestSeriesInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeLiveInfo {
    pub quest_id: Option<String>,
    pub current_rank: Option<i64>,
    pub is_lock: Option<bool>,
    pub is_try: Option<bool>,
    pub is_complete: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeLiveNormalStageInfo {
    pub stage_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.LiveClearStatus
    pub clear_status: Option<i32>,
    pub is_lock: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeLiveBossStageInfo {
    pub stage_id: Option<String>,
    pub is_lock: Option<bool>,
    pub star_num: Option<i64>,
    pub stage_reward_list: Option<Vec<StageReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeLiveRankInfo {
    pub rank: Option<i64>,
    pub rank_name: Option<String>,
    pub is_clear: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PointBonusInfo {
    /// dnSpy enum: Org.OpenAPITools.Model.GradePointBonusType
    pub bonus_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradePointBonusTargetDetailType
    pub target_detail: Option<i32>,
    pub target_num: Option<i64>,
    pub bonus_value: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradePointRanking {
    pub player_id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub rank: Option<i32>,
    pub point: Option<i64>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeQuestSquare {
    pub grade_quest_square_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradeQuestSquareStatus
    pub status: Option<i32>,
    pub live_point: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeGoalClearData {
    pub character_id: Option<i32>,
    pub current_square_id: Option<i32>,
    pub quest_square_list: Option<Vec<GradeQuestSquare>>,
    pub clear_grade_quest_rewards_id_list: Option<Vec<i32>>,
    pub reward_list: Option<Vec<GradeQuestReward>>,
    pub grade_up_data: Option<GradeUpData>,
    pub is_not_received_grade_reward: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetLiveListResponse {
    pub grade_live_list: Option<Vec<GradeLiveInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetQuestListResponse {
    pub quest_season_list: Option<Vec<GradeSeasonInfo>>,
    pub point_bonus_list: Option<Vec<PointBonusInfo>>,
    pub is_update_grade_live: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetRankingListResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.SeasonGradeStatus
    pub status: Option<i32>,
    pub my_rank: Option<i32>,
    pub point_rankings: Option<Vec<GradePointRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetRankListResponse {
    pub quest_id: Option<String>,
    pub rank_list: Option<Vec<GradeLiveRankInfo>>,
    pub current_rank: Option<i64>,
    pub next_rank: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetResultResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub quest_stage_id: Option<i32>,
    pub quest_result: Option<bool>,
    pub result_love: Option<i64>,
    pub play_report: Option<String>,
    pub add_live_point: Option<i64>,
    pub before_live_point: Option<i64>,
    pub point_bonus_list: Option<Vec<PointBonusInfo>>,
    pub action_point: Option<i32>,
    pub goal_clear_data: Option<GradeGoalClearData>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetStageDataResponse {
    pub live_point: Option<i64>,
    pub music_list: Option<Vec<StageMusic>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetStageListResponse {
    pub normal_stage_list: Option<Vec<GradeLiveNormalStageInfo>>,
    pub boss_stage: Option<GradeLiveBossStageInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestActionResponse {
    pub action_point: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradeQuestSquareType
    pub square_type: Option<i32>,
    pub add_skill_id_list: Option<Vec<i64>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestAddSkillResponse {
    pub action_point: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestRetireResponse {
    pub clear_grade_quest_rewards_id_list: Option<Vec<i32>>,
    pub live_point: Option<i64>,
    pub reward_list: Option<Vec<GradeQuestReward>>,
    pub grade_up_data: Option<GradeUpData>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestStartResponse {
    pub character_id: Option<i32>,
    pub current_square_id: Option<i32>,
    pub action_point: Option<i32>,
    pub quest_square_list: Option<Vec<GradeQuestSquare>>,
    pub active_add_skill_id_list: Option<Vec<i64>>,
    pub lot_add_skill_id_list: Option<Vec<i64>>,
    pub reward_list: Option<Vec<GradeQuestReward>>,
    pub point_bonus_list: Option<Vec<PointBonusInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetRewardResponse {
    pub grade_rewards: Option<Vec<GradeReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeChallengeStageRewardInfo {
    pub grade_chal_quest_stages_rewards_id: Option<i64>,
    pub is_received: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeChallengeStageInfo {
    pub quest_stage_id: Option<i32>,
    pub high_score: Option<i64>,
    pub reward_list: Option<Vec<GradeChallengeStageRewardInfo>>,
    pub music_list: Option<Vec<StageMusic>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetQuestListResponse {
    pub stage_list: Option<Vec<GradeChallengeStageInfo>>,
    pub received_total_score_rewards_id: Option<i32>,
    pub season_grade_point: Option<i64>,
    pub is_update_grade_live: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GradeChallengeRanking {
    pub player_id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub rank: Option<i32>,
    pub high_score: Option<i64>,
    pub is_mute_player: Option<bool>,
    pub play_report: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetRankingListResponse {
    pub rankings: Option<Vec<GradeChallengeRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetResultResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub quest_stage_id: Option<i32>,
    pub quest_result: Option<bool>,
    pub result_love: Option<i64>,
    pub add_high_score: Option<i64>,
    pub total_high_score: Option<i64>,
    pub add_grade_point: Option<i64>,
    pub season_grade_point: Option<i64>,
    pub reward_list: Option<Vec<GradeChallengeStageRewardInfo>>,
    pub play_report: Option<String>,
    pub grade_up_data: Option<GradeUpData>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixGetPointHistory {
    pub player_id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub point: Option<i64>,
    /// dnSpy type: System.DateTime
    pub play_time: Option<String>,
    pub music_id: Option<i32>,
    pub stage_id: Option<i32>,
    pub play_report: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixStageHighScoreRanking {
    pub player_id: Option<String>,
    pub name: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.UserType
    pub user_type: Option<i32>,
    pub profile_icon_parts_info: Option<String>,
    pub rank: Option<i64>,
    pub high_score: Option<i64>,
    pub is_mute_player: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixCircleHighScoreRanking {
    pub stage_id: Option<i32>,
    pub stage_high_score_rankings: Option<Vec<GrandPrixStageHighScoreRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixStage {
    pub stage_id: Option<i32>,
    pub clear_status: Option<i64>,
    pub is_lock: Option<bool>,
    pub playable_count: Option<i32>,
    pub play_count: Option<i32>,
    pub play_count_bonus_value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixStageSelect {
    pub quest_id: Option<i32>,
    pub daily_stage_id: Option<i32>,
    pub daily_point: Option<i64>,
    pub daily_music_id: Option<i32>,
    pub daily_rank: Option<i32>,
    pub high_score: Option<i64>,
    pub high_score_rank: Option<i32>,
    pub playable_count: Option<i32>,
    pub play_count: Option<i32>,
    pub play_count_bonus_value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixStageRankingInfo {
    pub quest_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixRankDisplayType
    pub rank_display_type: Option<i32>,
    pub rank: Option<i32>,
    pub point: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixRewardData {
    pub grand_prix_reward_datas_id: Option<i64>,
    /// dnSpy enum: Org.OpenAPITools.Model.ItemType
    pub item_type: Option<i32>,
    pub item_id: Option<i32>,
    pub num: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrandPrixReward {
    pub grand_prix_rewards_id: Option<i32>,
    pub min_target_num: Option<i32>,
    pub max_target_num: Option<i32>,
    pub reward_datas: Option<Vec<GrandPrixRewardData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetHistoryResponse {
    pub get_point_histories: Option<Vec<GrandPrixGetPointHistory>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetRankingListResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixStatus
    pub status: Option<i32>,
    pub my_rank: Option<i32>,
    pub lowest_rank: Option<i32>,
    pub point_rankings: Option<Vec<GrandPrixPointRanking>>,
    pub circle_high_score_rankings: Option<Vec<GrandPrixCircleHighScoreRanking>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetResultResponse {
    /// dnSpy enum: Org.OpenAPITools.Model.QuestType
    pub quest_live_type: Option<i32>,
    pub stage_id: Option<i32>,
    pub quest_result: Option<bool>,
    pub result_love: Option<i64>,
    pub best_love: Option<i64>,
    pub before_best_love: Option<i64>,
    pub add_style_point: Option<i64>,
    pub is_challenge_mode: Option<bool>,
    pub music_id: Option<i32>,
    pub reward_list: Option<Vec<Reward>>,
    pub play_report: Option<String>,
    pub mastary_level_before: Option<i64>,
    pub mastary_level_after: Option<i64>,
    pub mastary_level_experience: Option<i64>,
    pub mastary_level_total_experience_before: Option<i64>,
    pub first_clear_flag: Option<bool>,
    pub first_complete_clear_flag: Option<bool>,
    pub is_limit_over_style_point: Option<bool>,
    pub daily_score: Option<i64>,
    pub before_daily_score: Option<i64>,
    pub is_rehearsal: Option<bool>,
    pub is_guarantee: Option<bool>,
    pub point_bonus_list: Option<Vec<GrandPrixPointBonus>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetStageDataResponse {
    pub stage_name: Option<String>,
    pub clear_status: Option<i64>,
    pub stage_reward_list: Option<Vec<StageReward>>,
    pub playable_count: Option<i32>,
    pub play_count: Option<i32>,
    pub play_count_bonus_value: Option<i32>,
    pub skip_guarantee_point: Option<i64>,
    pub music_list: Option<Vec<StageMusic>>,
    pub is_today_playable: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetStageListResponse {
    pub stage_list: Option<Vec<GrandPrixStage>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetStageSelectResponse {
    pub daily_total_point: Option<i64>,
    pub best_point: Option<i64>,
    pub stages: Option<Vec<GrandPrixStageSelect>>,
    pub stage_ranking_info_list: Option<Vec<GrandPrixStageRankingInfo>>,
    pub predicated_ranking_score_info: Option<GrandPrixRankingScoreInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGrandPrixGetTopInfoResponse {
    /// dnSpy type: System.DateTime
    pub start_date: Option<String>,
    /// dnSpy type: System.DateTime
    pub end_date: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixType
    pub grand_prix_type: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GrandPrixStatus
    pub status: Option<i32>,
    pub search_guild_key: Option<String>,
    pub guild_name: Option<String>,
    pub guild_total_point: Option<i64>,
    pub guild_ranking: Option<i32>,
    pub within_guild_ranking: Option<i32>,
    pub personal_total_point: Option<i64>,
    pub personal_ranking: Option<i32>,
    pub get_points: Option<Vec<GrandPrixGetPoint>>,
    pub personal_rewards: Option<Vec<GrandPrixReward>>,
    pub circle_rewards: Option<Vec<GrandPrixReward>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventDropItemData {
    pub box_datas_id: Option<i32>,
    pub box_item_datas_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct LimitRewardReceivedCount {
    pub id: Option<i64>,
    pub count: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventGetTopInfoResponse {
    pub additional_box_progress_point: Option<i32>,
    pub special_reward_progress_point: Option<i32>,
    pub now_drop_box_count: Option<i32>,
    pub received_special_reward_ids: Option<Vec<i32>>,
    pub limit_reward_received_count: Option<Vec<LimitRewardReceivedCount>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventSetDropBoxResponse {
    pub additional_box_progress_point: Option<i32>,
    pub special_reward_progress_point: Option<i32>,
    pub now_drop_box_count: Option<i32>,
    pub received_box_item_datas: Option<Vec<BoxEventDropItemData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventSetSpecialRewardResponse {
    pub received_special_reward_ids: Option<Vec<i32>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GiftShopSetPurchaseResponse {
    pub sisca_info: Option<SiscaInfo>,
    pub user_item_info: Option<UserItemInfo>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserTicketInfo {
    pub item_id: Option<i32>,
    pub user_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GpPrizeExchangeInfo {
    pub gp_prize_exchanges_id: Option<i32>,
    pub is_having: Option<bool>,
    pub remaining_exchange_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GpPrizeExchangeGetListResponse {
    pub user_ticket_list: Option<Vec<UserTicketInfo>>,
    pub gp_prize_exchange_list: Option<Vec<GpPrizeExchangeInfo>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GpPrizeExchangeSetPurchaseResponse {
    pub gp_prize_ticket_id: Option<i32>,
    pub user_ticket_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

pub type FesliveLobbyCommentResponse = FesliveCommentResponse;
pub type FesliveLobbyGiftResponse = FesliveGiftResponse;
pub type FesliveLobbyMessageCardResponse = FesliveMessageCardResponse;
pub type FesliveSetFlowerStandResponse = SetFlowerStandResponse;
pub type WithliveEnquetesAnswerResponse = WithliveEnqueteAnswerResponse;
pub type FriendSetApprovalRequestAllResponse = FriendSetCommonResponse;
pub type FriendSetApprovalRequestResponse = FriendSetCommonResponse;
pub type FriendSetBreakOffResponse = FriendSetCommonResponse;
pub type FriendSetRefuseRequestAllResponse = FriendSetCommonResponse;
pub type FriendSetRequestCancelResponse = FriendSetCommonResponse;
pub type FriendSetRequestResponse = FriendSetCommonResponse;

// AccountApi

// ActivityRecordApi
define_response_type!(ActivityRecordGetTopResponse);
define_response_type!(ActivityRecordNotifyMonthDisplayedResponse);
define_response_type!(ActivityRecordPlayAdvDataResponse);
define_response_type!(ActivityRecordSetWatchableStatusResponse);

// AnnounceApi
// AnnounceDetailResponse / AnnounceListResponse are strongly typed above.

// ArchiveApi

// MissionApi
// BeginnerMissionGetListResponse is strongly typed above.
define_response_type!(BeginnerMissionSetBannerRewardResponse);
// BeginnerMissionSetRewardAllResponse is strongly typed above.
define_response_type!(BeginnerMissionSetRewardResponse);

// BoxEventApi
// BoxEventGetTopInfoResponse/BoxEventSetDropBoxResponse are strongly typed above.
// BoxEventSetSpecialRewardResponse is strongly typed above.

// CircleApi
// Circle responses are strongly typed above.

// CollectionApi
define_response_type!(CollectionGetCharacterInfoResponse);
define_response_type!(CollectionGetEmojiListResponse);
define_response_type!(CollectionGetGallaryListResponse);
define_response_type!(CollectionGetMusicListResponse);
define_response_type!(CollectionGetStickerListResponse);
define_response_type!(CollectionSetGallaryDataResponse);
define_response_type!(CollectionSetMusicPlayResponse);

// CommonApi
// CommonGetHeaderAnnounsResponse is strongly typed above.

// FesliveApi
define_response_type!(FesliveClapResponse);
define_response_type!(FesliveEnqueteResponse);
define_response_type!(FeslivePenlightColorResponse);
define_response_type!(FesliveSetPrizeResponse);

// FriendApi

// GachaApi or GachaWebApi
define_response_type!(GachaConfirmTicketExpiredTimeResponse);
// GachaGetExchangeCardHavingListResponse/GachaGetExchangeCardListResponse are strongly typed above.
// GachaGetGuaranteePointListResponse/GachaGetHistoryResponse are strongly typed above.
// GachaGetLotteryChanceResponse/GachaGetSeriesListResponse are strongly typed above.
// GachaSetGuaranteePointExchangeResponse/GachaSetPrizeReceiveResponse are strongly typed above.
// GachaSetPurchaseResponse/GachaSetSelectTicketExchangeResponse are strongly typed above.

// GiftShopApi

// GpPrizeExchangeApi

// HomeApi
// HomeGetCustomSettingResponse/HomeGetHomeResponse are strongly typed above.
// HomeGetLoginBonusResponse/HomeGetWallpaperSettingResponse are strongly typed above.
define_response_type!(HomeNotifyWallpaperSettingResponse);
// HomeSetClockSettingResponse/HomeSetCurrentWallpaperSettingResponse are strongly typed above.
// HomeSetShowRetireResponse/HomeSetWallpaperSettingResponse are strongly typed above.

// InQuestLiveApi
define_response_type!(InQuestLiveGetLiveInfoResponse);
define_response_type!(InQuestLiveSetFinishResponse);
define_response_type!(InQuestLiveSetRetireResponse);
define_response_type!(InQuestLiveSetStartResponse);
define_response_type!(InQuestLiveSkipQuestResponse);

// ItemExchangeApi
// ItemExchangeGetLimitBreakMaterialConvertListResponse is strongly typed above.
define_response_type!(ItemExchangeGetListNewResponse);
// ItemExchangeGetListResponse is strongly typed above.
// ItemExchangeSetLimitBreakMaterialConvertResponse/ItemExchangeSetPurchaseResponse are strongly typed above.

// ItemStoreApi
// ItemStoreGetListResponse is strongly typed above.

// JewelShopApi
// JewelShopGetBirthdayResponse/JewelShopGetListResponse are strongly typed above.
// JewelShopGetMembershipListResponse/JewelShopSetBirthdayResponse are strongly typed above.
define_response_type!(JewelShopSetMembershipPurchaseResponse);
// JewelShopSetPurchaseResponse is strongly typed above.

// MissionApi
// MissionGetListResponse is strongly typed above.
define_response_type!(MissionReceiveCommonMissionRewardResponse);

// OutQuestLiveDailyApi
// OutQuestLiveDailyGetRecoveryChallengeCountResponse is strongly typed above.
// OutQuestLiveDailyGetReleaseTicketResponse is strongly typed above.
// OutQuestLiveDailyGetStageDataResponse is strongly typed above.
// OutQuestLiveDailyGetStageListResponse is strongly typed above.
// OutQuestLiveDailyGetStageSelectResponse is strongly typed above.
// OutQuestLiveDailyRecoveryChallengeCountResponse is strongly typed above.
// OutQuestLiveDailySetReleaseResponse is strongly typed above.

// OutQuestLiveDreamApi
// OutQuestLiveDreamGetMemberSelectResponse is strongly typed above.
// OutQuestLiveDreamGetResultResponse is strongly typed above.
// OutQuestLiveDreamNotifyMemberReleaseConfirmResponse is strongly typed above.
// OutQuestLiveDreamSetCardResponse is strongly typed above.

// OutQuestLiveApi
// OutQuestLiveGetLiveSettingResponse is strongly typed above.
// OutQuestLiveGetPlayReportResponse is strongly typed above.
// OutQuestLiveGetQuestClearStatusListResponse is strongly typed above.
// OutQuestLiveGetQuestListResponse is strongly typed above.
// OutQuestLiveGetQuestTopResponse is strongly typed above.
// OutQuestLiveGetResultResponse is strongly typed above.
// OutQuestLiveGetStaminaRecoveryInfoResponse is strongly typed above.

// OutQuestLiveGradeChallengeApi
// OutQuestLiveGradeChallengeGetQuestListResponse is strongly typed above.
// OutQuestLiveGradeChallengeGetRankingListResponse is strongly typed above.
// OutQuestLiveGradeChallengeGetResultResponse is strongly typed above.

// OutQuestLiveGradeApi
// OutQuestLiveGradeGetLiveListResponse is strongly typed above.
// OutQuestLiveGradeGetQuestListResponse is strongly typed above.
// OutQuestLiveGradeGetRankingListResponse is strongly typed above.
// OutQuestLiveGradeGetRankListResponse is strongly typed above.
// OutQuestLiveGradeGetResultResponse is strongly typed above.
// OutQuestLiveGradeGetStageDataResponse is strongly typed above.
// OutQuestLiveGradeGetStageListResponse is strongly typed above.
// OutQuestLiveGradeSetQuestActionResponse is strongly typed above.
// OutQuestLiveGradeSetQuestAddSkillResponse is strongly typed above.
// OutQuestLiveGradeSetQuestRetireResponse is strongly typed above.
// OutQuestLiveGradeSetQuestStartResponse is strongly typed above.
// OutQuestLiveGradeSetRewardResponse is strongly typed above.

// OutQuestLiveGrandPrixApi
// OutQuestLiveGrandPrixGetHistoryResponse is strongly typed above.
// OutQuestLiveGrandPrixGetRankingListResponse is strongly typed above.
// OutQuestLiveGrandPrixGetResultResponse is strongly typed above.
// OutQuestLiveGrandPrixGetStageDataResponse is strongly typed above.
// OutQuestLiveGrandPrixGetStageListResponse is strongly typed above.
// OutQuestLiveGrandPrixGetStageSelectResponse is strongly typed above.
// OutQuestLiveGrandPrixGetTopInfoResponse is strongly typed above.

// OutQuestLiveMusicLearningApi
define_response_type!(OutQuestLiveMusicLearningGetMusicSelectResponse);
define_response_type!(OutQuestLiveMusicLearningGetResultResponse);
define_response_type!(OutQuestLiveMusicLearningSetMusicResponse);

// OutQuestLiveRaidEventApi
define_response_type!(OutQuestLiveRaidEventGetResultResponse);
define_response_type!(OutQuestLiveRaidEventGetStageDataResponse);
define_response_type!(OutQuestLiveRaidEventGetStageListResponse);
define_response_type!(OutQuestLiveRaidEventGetStaminaRecoveryInfoResponse);
define_response_type!(OutQuestLiveRaidEventGetTopInfoResponse);
define_response_type!(OutQuestLiveRaidEventRecoveryStaminaResponse);
define_response_type!(OutQuestLiveRaidEventSetJoinMessageResponse);
define_response_type!(OutQuestLiveRaidEventSetRewardResponse);

// OutQuestLiveApi
// OutQuestLiveRecoveryStaminaResponse is strongly typed above.
// OutQuestLiveSetLiveSettingResponse is strongly typed above.
define_response_type!(OutQuestLiveStandardGetAreaSelectResponse);
define_response_type!(OutQuestLiveStandardGetStageDataResponse);
define_response_type!(OutQuestLiveStandardGetStageSelectResponse);
define_response_type!(OutQuestLiveStandardSetAreaSelectViewHistResponse);

// PetalExchangeApi
// PetalExchangeGetListResponse/PetalExchangeSetPurchaseResponse are strongly typed above.

// PresentBoxApi
// PresentBoxGetHistoryResponse/PresentBoxGetListResponse are strongly typed above.
// PresentBoxItemDetailResponse/PresentBoxSetItemAllResponse are strongly typed above.
// PresentBoxSetItemResponse is strongly typed above.

// ProfileApi
// ProfileDeleteMyDesignResponse/ProfileGetFanLevelInfoResponse/ProfileGetInfoResponse are strongly typed above.
// ProfileGetMuteListResponse/ProfileGetMyDesignCardListResponse/ProfileGetMyDesignIconListResponse are strongly typed above.
// ProfileGetMyDesignIconResponse/ProfileGetProfileCardResponse/ProfileGetProfileIconResponse are strongly typed above.
// ProfileSetBirthdayResponse/ProfileSetCommentResponse/ProfileSetMuteCancelResponse are strongly typed above.
// ProfileSetMuteResponse/ProfileSetMyDesignCardResponse/ProfileSetMyDesignIconResponse are strongly typed above.
// ProfileSetMyDesignNameResponse/ProfileSetNameResponse/ProfileSetProfileCardResponse are strongly typed above.
// ProfileSetProfileIconResponse/ProfileSetReportResponse/ProfileUseFanlevelPointStocksResponse are strongly typed above.

// RegisterApi
// RegisterApproveTermsResponse/RegisterGetTermsResponse are strongly typed above.
define_response_type!(RegisterSetApproveTermsResponse);
// RegisterSetNewUserResponse/RegisterSetUserDataResponse are strongly typed above.

// RhythmGameDeckApi
define_response_type!(RhythmGameDeckModifyDeckListResponse);
define_response_type!(RhythmGameDeckSetDeckNameResponse);
define_response_type!(RhythmGameDeckSetResetDeckResponse);

// RhythmGameGrandPrixApi
// RhythmGameGrandPrixSetCenterResponse/RhythmGameGrandPrixSetDeckResponse are strongly typed above.
// RhythmGameGrandPrixSetFinishLiveResponse is strongly typed above.
// RhythmGameGrandPrixSetPositionResponse is strongly typed above.
define_response_type!(RhythmGameGrandPrixSetResetResponse);
define_response_type!(RhythmGameGrandPrixSetRetireLiveResponse);
define_response_type!(RhythmGameGrandPrixSetStartLiveResponse);
// RhythmGameGrandPrixTopResponse is strongly typed above.

// RhythmGameApi
define_response_type!(RhythmGameHomeResponse);

// RhythmGameLiveApi
// RhythmGameLiveSetFinishResponse is strongly typed above.
define_response_type!(RhythmGameLiveSetRetireResponse);
define_response_type!(RhythmGameLiveSetStartResponse);

// RhythmGameApi
// RhythmGameReceiveClassMissionResponse/RhythmGameReceiveTotalMissionResponse are strongly typed above.
// RhythmGameSetMusicResponse is strongly typed above.

// SelectTicketExchangeApi
// SelectTicketExchangeGetListResponse is strongly typed above.

// SerialCodeApi
define_response_type!(SerialCodeSetExchangeResponse);

// ShopApi
define_response_type!(ShopCheckPurchaseResponse);
// ShopGetListResponse is strongly typed above.

// SiscaStoreApi
// SiscaStoreGetListResponse/SiscaStoreSetPurchaseResponse are strongly typed above.

// MissionApi
// StepUpBeginnerMissionGetListResponse/StepUpBeginnerMissionSetRewardResponse are strongly typed above.

// StickerExchangeApi
// StickerExchangeGetListResponse/StickerExchangeSetPurchaseResponse are strongly typed above.

// TutorialApi
// TutorialSetStepResponse is strongly typed above.

// UserApi
define_response_type!(UserCardCheckEvolutionResponse);
define_response_type!(UserCardCheckLimitBreakResponse);
define_response_type!(UserCardCheckSkillLevelUpResponse);
define_response_type!(UserCardCheckStyleLevelUpResponse);
define_response_type!(UserCardEvolutionResponse);
define_response_type!(UserCardGetDetailRentalResponse);
define_response_type!(UserCardGetDetailResponse);
define_response_type!(UserCardGetListResponse);
define_response_type!(UserCardLimitBreakResponse);
define_response_type!(UserCardRhythmGameSkillLevelUpResponse);
define_response_type!(UserCardSkillLevelUpResponse);
define_response_type!(UserCardStyleLevelUpResponse);
define_response_type!(UserDeckGetCardListResponse);
define_response_type!(UserDeckGetListResponse);
define_response_type!(UserDeckModifyDeckListResponse);
define_response_type!(UserDeckNotifyAutoDeckResponse);
define_response_type!(UserDeckRemoveSideStyleResponse);
define_response_type!(UserDeckSetCopyDeckResponse);
define_response_type!(UserDeckSetDeckResponse);
define_response_type!(UserDeckSetDeleteDeckResponse);
define_response_type!(UserDeckSetPositionResponse);
define_response_type!(UserDeckSetSideStyleResponse);
// UserItemGetDetailResponse/UserItemsGetListResponse are strongly typed above.
// UserJewelGetHistoryResponse is strongly typed above.
define_response_type!(UserPushDeviceResponse);
define_response_type!(UserPushDevicesResponse);
// UserSetContentsReleaseEffectHistoryResponse is strongly typed above.
define_response_type!(UserSetSimpleTutorialFinishResponse);

// WebviewApi
define_response_type!(WebviewGachaGetDetailResponse);
define_response_type!(WebviewGachaGetSelectCardListResponse);

// WebviewLiveApi
// WebviewLiveEnterResponse/WebviewLiveLiveInfoResponse/WebviewLiveLoginResponse are strongly typed above.

// WebviewApi
define_response_type!(WebviewSchoolIdolConnectPostGetThemeListResponse);
define_response_type!(WebviewShopGetMembershipPerkDetailResponse);

// WithliveApi

// FriendApi

// GachaApi or GachaWebApi

// GiftShopApi

// GpPrizeExchangeApi

// HomeApi

// InQuestLiveApi

// ItemExchangeApi

// ItemStoreApi

// JewelShopApi

// MissionApi

// OutQuestLiveDailyApi

// OutQuestLiveDreamApi

// OutQuestLiveApi

// OutQuestLiveGradeChallengeApi

// OutQuestLiveGradeApi

// OutQuestLiveGrandPrixApi

// OutQuestLiveMusicLearningApi

// OutQuestLiveRaidEventApi

// OutQuestLiveApi

// PetalExchangeApi

// PresentBoxApi

// ProfileApi

// RegisterApi

// RhythmGameDeckApi

// RhythmGameGrandPrixApi

// RhythmGameApi

// RhythmGameLiveApi

// RhythmGameApi

// SelectTicketExchangeApi

// SerialCodeApi

// ShopApi

// SiscaStoreApi

// MissionApi

// StickerExchangeApi

// TutorialApi

// UserApi

// WebviewApi

// WebviewLiveApi

// WebviewApi

// WithliveApi

// FriendApi

// GachaApi or GachaWebApi

// GiftShopApi

// GpPrizeExchangeApi

// HomeApi

// InQuestLiveApi

// ItemExchangeApi

// ItemStoreApi

// JewelShopApi

// MissionApi

// OutQuestLiveDailyApi

// OutQuestLiveDreamApi

// OutQuestLiveApi

// OutQuestLiveGradeChallengeApi

// OutQuestLiveGradeApi

// OutQuestLiveGrandPrixApi

// OutQuestLiveMusicLearningApi

// OutQuestLiveRaidEventApi

// OutQuestLiveApi

// PetalExchangeApi

// PresentBoxApi

// ProfileApi

// RegisterApi

// RhythmGameDeckApi

// RhythmGameGrandPrixApi

// RhythmGameApi

// RhythmGameLiveApi

// RhythmGameApi

// SelectTicketExchangeApi

// SerialCodeApi

// ShopApi

// SiscaStoreApi

// MissionApi

// StickerExchangeApi

// TutorialApi

// UserApi

// WebviewApi

// WebviewLiveApi

// WebviewApi

// WithliveApi
