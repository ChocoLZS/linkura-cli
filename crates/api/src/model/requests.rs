use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountConnectRequest {
    /// dnSpy: Org.OpenAPITools.Model.Provider
    pub provider: Option<i32>,
    pub player_id: Option<String>,
    pub id_token: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.PlatformType
    pub platform_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountDeleteRequest {
    /// dnSpy: Org.OpenAPITools.Api.AccountApi::AccountDelete() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountDeleteConnectDataRequest {
    /// dnSpy: Org.OpenAPITools.Model.Provider
    pub provider: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountGetConnectUserRequest {
    /// dnSpy: Org.OpenAPITools.Model.Provider
    pub provider: Option<i32>,
    pub player_id: Option<String>,
    pub id_token: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.PlatformType
    pub platform_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountSetConnectDataRequest {
    /// dnSpy: Org.OpenAPITools.Model.Provider
    pub provider: Option<i32>,
    pub id_token: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserLoginRequest {
    pub player_id: Option<String>,
    pub device_specific_id: Option<String>,
    #[serde(rename = "version", alias = "_Version")]
    pub version: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetHomeRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetArchiveListRequest {
    pub season_id: Option<i32>,
    /// dnSpy: Org.OpenAPITools.Model.FilterLiveType
    pub live_type: Option<i32>,
    /// dnSpy: Org.OpenAPITools.Model.FilterHasWithliveExtra
    pub has_withlive_extra: Option<i32>,
    /// dnSpy: Org.OpenAPITools.Model.FilterHasWithliveExtraAdmission
    pub has_withlive_extra_admission: Option<i32>,
    pub characters: Option<Vec<Value>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order: Option<String>,
    pub sort: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnnounceDetailRequest {
    pub announce_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnnounceListRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.AnnounceType
    pub announce_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetDetailRequest {
    pub search_guild_key: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetInfoRequest {
    pub search_guild_key: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetInviteAndJoinInfoRequest {
    pub search_guild_key: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleGetListRequest {
    pub search_conditions: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetApproveInviteRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetApproveJoinRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCancelInviteRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCancelJoinRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetChatMessageRequest {
    pub guild_chat_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetCreateRequest {
    pub guild_setting: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetDismissalRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetDonationRequest {
    pub donation_item_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetExpulsionRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetInviteRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetItemRequestRequest {
    pub request_item_info: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetJoinRequest {
    pub search_guild_key: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetPositionRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.PositionType
    pub position_type: Option<i32>,
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetRejectInviteRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetRejectJoinRequest {
    pub d_guild_invitations_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetSettingRequest {
    pub search_guild_key: Option<String>,
    pub guild_setting: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CircleSetTransferLeaderRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveConnectTokenRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveConnectTokenRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnterRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnterRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetFesArchiveDataRequest {
    pub archives_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetWithArchiveDataRequest {
    pub archives_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccountGetConnectDataRequest {
    /// dnSpy: Org.OpenAPITools.Api.AccountApi::AccountGetConnectData() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetChannelListRequest {
    /// dnSpy: Org.OpenAPITools.Api.ArchiveApi::ArchiveGetChannelList() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetChannelMovieListRequest {
    pub live_channels_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetFesTimelineDataRequest {
    pub archives_id: Option<String>,
    pub play_time_second: Option<i32>,
    pub timeline_unixtime: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveGetSeasonListRequest {
    /// dnSpy: Org.OpenAPITools.Api.ArchiveApi::ArchiveGetSeasonList() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetCancelRecommendChannelRequest {
    pub live_channels_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetFesCameraRequest {
    pub live_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.LiveCameraType
    pub camera_type: Option<i32>,
    pub focus_character_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetPurchaseTicketRequest {
    pub feslive_archives_id: Option<String>,
    pub shop_item_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveSetRecommendChannelRequest {
    pub live_channels_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveGiftRequest {
    pub live_id: Option<String>,
    pub user_item_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub use_own_amount: Option<i32>,
    pub purchase_amount: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveInfoRequest {
    pub live_id: Option<String>,
    pub play_time_second: Option<i32>,
    pub timeline_unixtime: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithlivePrizeRequest {
    pub present_box_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveWithliveStarsRequest {
    pub present_box_id: Option<String>,
    pub set_star_json: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGetGiftShopListRequest {
    /// dnSpy property: MFeslivesId
    pub m_feslives_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGetListRequest {
    /// dnSpy: FesliveGetListWithHttpInfo() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveInfoRequest {
    pub live_id: Option<String>,
    pub offset_timeline_id: Option<String>,
    pub offset_circle_chat_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveSubinfoRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyInfoRequest {
    pub live_id: Option<String>,
    pub offset_timeline_id: Option<String>,
    pub offset_circle_chat_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbySubinfoRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGetGiftShopListRequest {
    /// dnSpy property: MWithlivesId
    pub m_withlives_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGetListRequest {
    /// dnSpy: WithliveGetListWithHttpInfo() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveLiveInfoRequest {
    pub live_id: Option<String>,
    pub offset_timeline_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveLiveSubinfoRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveCircleChatCommentRequest {
    pub live_id: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveClapRequest {
    pub live_id: Option<String>,
    pub count: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveCommentRequest {
    pub live_id: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveEnqueteRequest {
    pub live_id: Option<String>,
    pub enquete_id: Option<String>,
    pub value: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGiftPtRankingsRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGiftRequest {
    pub live_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub user_item_id: Option<String>,
    pub use_own_amount: Option<i32>,
    pub purchase_amount: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveGrandPrixRankingRequest {
    pub live_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.LiveResultGetRankType
    pub get_rank_type: Option<i32>,
    pub target_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLiveAnnouncementRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyAnnouncementRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyCommentRequest {
    pub live_id: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyGiftRequest {
    pub live_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub user_item_id: Option<String>,
    pub use_own_amount: Option<i32>,
    pub purchase_amount: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveLobbyMessageCardRequest {
    pub live_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub user_item_id: Option<String>,
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveMessageCardRequest {
    pub live_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub user_item_id: Option<String>,
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveOutQuestLiveRankingRequest {
    pub live_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.FesliveGetRankType
    pub get_rank_type: Option<i32>,
    pub target_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePenlightColorRequest {
    pub live_id: Option<String>,
    pub before_color_ids: Option<Vec<String>>,
    pub after_color_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FeslivePrizeRequest {
    pub live_id: Option<String>,
    pub present_box_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveSetCameraRequest {
    pub live_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.LiveCameraType
    pub camera_type: Option<i32>,
    pub focus_character_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveSetFlowerStandRequest {
    pub live_id: Option<String>,
    pub flower_stand_type_id: Option<i32>,
    pub flower_stand_color1_id: Option<i32>,
    pub flower_stand_color2_id: Option<i32>,
    pub flower_stand_idol_picture_id: Option<i32>,
    pub flower_stand_message: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FesliveSetPrizeRequest {
    pub live_id: Option<String>,
    pub present_box_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveCommentRequest {
    pub live_id: Option<String>,
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveEnquetesAnswerRequest {
    pub live_id: Option<String>,
    pub enquete_id: Option<String>,
    pub option_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGiftPtRankingsRequest {
    pub live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveGiftRequest {
    pub live_id: Option<String>,
    pub user_item_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub use_own_amount: Option<i32>,
    pub purchase_amount: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveMessageCardRequest {
    pub live_id: Option<String>,
    pub shop_item_id: Option<i32>,
    pub user_item_id: Option<String>,
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithlivePrizeRequest {
    pub live_id: Option<String>,
    pub present_box_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveResultRankingRequest {
    pub live_id: Option<String>,
    /// dnSpy: Org.OpenAPITools.Model.LiveResultGetRankType
    pub get_rank_type: Option<i32>,
    pub target_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WithliveSetStarRequest {
    pub present_box_id: Option<String>,
    pub set_star_json: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendGetListRequest {
    pub desc_order: Option<bool>,
    /// dnSpy enum: Org.OpenAPITools.Model.FriendOrderType
    pub order_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendGetRequestListRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchNameRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchPlayerIdRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSearchRecommendRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetApprovalRequestAllRequest {
    pub d_friend_requests_id_list: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetApprovalRequestRequest {
    pub d_friend_requests_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetBreakOffRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetRefuseRequestAllRequest {
    pub d_friend_requests_id_list: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetRequestCancelRequest {
    pub d_friend_requests_id_list: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendSetRequestRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FriendUpdateRequestViewHistoryRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GiftShopSetPurchaseRequest {
    pub shop_item_id: Option<i32>,
    pub number: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GpPrizeExchangeGetListRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GpPrizeExchangeSetPurchaseRequest {
    pub gp_prize_exchanges_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionSetBannerRewardRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.MissionType
    pub mission_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionSetRewardAllRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.MissionType
    pub mission_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeginnerMissionSetRewardRequest {
    pub mission_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventGetTopInfoRequest {
    pub box_event_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventSetDropBoxRequest {
    pub box_event_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoxEventSetSpecialRewardRequest {
    pub box_event_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaConfirmTicketExpiredTimeRequest {
    pub gacha_ticket_id: Option<i32>,
    pub is_confirm_disabled: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetExchangeCardHavingListRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.GachaExchangeType
    pub exchange_type: Option<i32>,
    pub exchange_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetExchangeCardListRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.GachaExchangeType
    pub exchange_type: Option<i32>,
    pub exchange_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetGuaranteePointListRequest {
    pub exchange_point_id: Option<i32>,
    pub gacha_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaGetLotteryChanceRequest {
    pub gacha_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetGuaranteePointExchangeRequest {
    pub exchange_point_rate_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetPrizeReceiveRequest {
    pub gacha_series_id: Option<i32>,
    pub gacha_prize_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetPurchaseRequest {
    pub gacha_series_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GachaSetPurchaseType
    pub gacha_set_purchase_type: Option<i32>,
    pub campaign_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GachaSetSelectTicketExchangeRequest {
    pub select_ticket_exchange_rate_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeGetListRequest {
    pub category_list_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ItemExchangeSetPurchaseRequest {
    pub item_exchanges_id: Option<i32>,
    pub num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopSetBirthdayRequest {
    /// dnSpy type: System.DateTime
    pub birthday: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct JewelShopSetPurchaseRequest {
    pub product_id: Option<String>,
    pub receipt: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ShopType
    pub shop_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PetalExchangeSetPurchaseRequest {
    pub card_series_id: Option<i32>,
    pub num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxGetHistoryRequest {
    pub page: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxGetListRequest {
    pub page: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxItemDetailRequest {
    pub d_present_boxes_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxSetItemAllRequest {
    pub page: Option<i32>,
    pub is_receive_with_expire: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PresentBoxSetItemRequest {
    pub d_present_boxes_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileDeleteMyDesignRequest {
    pub d_profile_my_designs_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetFanLevelInfoRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetInfoRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileGetMyDesignIconRequest {
    pub d_profile_my_designs_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetBirthdayRequest {
    /// dnSpy type: System.DateTime
    pub birthday: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetCommentRequest {
    pub comment: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMuteCancelRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMuteRequest {
    pub player_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignCardRequest {
    pub d_profile_my_designs_id: Option<String>,
    pub profile_card_parts_info: Option<String>,
    pub my_design_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignIconRequest {
    pub d_profile_my_designs_id: Option<String>,
    pub profile_icon_parts_info: Option<String>,
    pub my_design_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetMyDesignNameRequest {
    pub d_profile_my_designs_id: Option<String>,
    pub my_design_name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetNameRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetProfileCardRequest {
    pub d_profile_my_designs_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetProfileIconRequest {
    pub d_profile_my_designs_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileSetReportRequest {
    pub player_id: Option<String>,
    /// dnSpy enum: Org.OpenAPITools.Model.ReportType
    pub report_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProfileUseFanlevelPointStocksRequest {
    pub use_stock_list: Option<Vec<UseFanLevelPointStock>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterApproveTermsRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.PlatformType
    pub platform_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSetNewUserRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.PlatformType
    pub platform_type: Option<i32>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSetUserDataRequest {
    pub name: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameDeckModifyDeckListRequest {
    pub modify_deck_list: Option<Vec<ModifyRhythmGameDeck>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameDeckSetDeckNameRequest {
    pub name: Option<String>,
    pub deck_no: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetCenterRequest {
    pub grand_prix_rhythm_game_series_id: Option<i32>,
    pub slot_no: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetDeckRequest {
    pub grand_prix_rhythm_game_series_id: Option<i32>,
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixSetDeckData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetFinishLiveRequest {
    pub score: Option<i64>,
    pub technical_score: Option<i32>,
    pub notes_result_list: Option<Vec<NotesResultInfo>>,
    pub remain_mental: Option<i32>,
    pub play_data: Option<RhythmGamePlayData>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetPositionRequest {
    pub grand_prix_rhythm_game_series_id: Option<i32>,
    pub deck_card_list: Option<Vec<RhythmGameGrandPrixSetPositionData>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetResetRequest {
    pub grand_prix_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixSetStartLiveRequest {
    pub grand_prix_rhythm_game_series_id: Option<i32>,
    pub music_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.MusicScoreDifficulty
    pub music_score_difficulty: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameGrandPrixTopRequest {
    pub grand_prix_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameLiveSetFinishRequest {
    pub score: Option<i64>,
    pub technical_score: Option<i32>,
    pub notes_result_list: Option<Vec<NotesResultInfo>>,
    pub remain_mental: Option<i32>,
    pub play_data: Option<RhythmGamePlayData>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameLiveSetStartRequest {
    pub music_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.MusicScoreDifficulty
    pub music_score_difficulty: Option<i32>,
    pub use_slot_no: Option<i32>,
    pub consume_stamina_magnification: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameReceiveClassMissionRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.RhythmGameConditionType
    pub target_condition_type: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RhythmGameSetMusicRequest {
    pub music_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ShopCheckPurchaseRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.ShopType
    pub shop_type: Option<i32>,
    pub product_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SiscaStoreSetPurchaseRequest {
    pub sisca_store_datas_id: Option<i32>,
    pub purchase_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StepUpBeginnerMissionSetRewardRequest {
    pub mission_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StickerExchangeSetPurchaseRequest {
    pub sticker_exchanges_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TutorialSetStepRequest {
    pub tutorial_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserItemGetDetailRequest {
    pub d_item_datas_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserSetContentsReleaseEffectHistoryRequest {
    pub contents_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserSetSimpleTutorialFinishRequest {
    pub contents_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetRecoveryChallengeCountRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetReleaseTicketRequest {
    pub daily_quest_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageDataRequest {
    pub stage_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageListRequest {
    pub quest_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyGetStageSelectRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailyRecoveryChallengeCountRequest {
    pub recovery_count: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDailySetReleaseRequest {
    pub daily_quest_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamGetMemberSelectRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamGetResultRequest {
    pub quest_live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamNotifyMemberReleaseConfirmRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveDreamSetCardRequest {
    pub dream_quest_stages_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct StaminaRecoveryItem {
    pub d_item_datas_id: Option<String>,
    pub item_id: Option<i32>,
    pub item_num: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetLiveSettingRequest {
    pub stage_id: Option<i32>,
    pub is_challenge_mode: Option<bool>,
    pub quest_live_type: Option<i64>,
    pub is_rehearsal: Option<bool>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetPlayReportRequest {
    pub quest_live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestClearStatusListRequest {
    pub items_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestListRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetQuestTopRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetResultRequest {
    pub quest_live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGetStaminaRecoveryInfoRequest {
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveRecoveryStaminaRequest {
    /// dnSpy enum: Org.OpenAPITools.Model.UseRecoveryType
    pub use_recovery_type: Option<i32>,
    pub recovery_count: Option<i64>,
    pub use_item_list: Option<Vec<StaminaRecoveryItem>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveSetLiveSettingRequest {
    pub stage_id: Option<i32>,
    pub deck_id: Option<String>,
    pub deck_data: Option<Deck>,
    pub music_id: Option<i32>,
    pub is_challenge_mode: Option<bool>,
    pub quest_live_type: Option<i64>,
    pub character_voices_id: Option<i32>,
    pub is_rehearsal: Option<bool>,
    pub resource_value: Option<i32>,
    pub rental_deck_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetQuestListRequest {
    pub grade_chal_season_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetRankingListRequest {
    pub grade_chal_season_id: Option<i32>,
    pub grade_chal_quest_stages_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeChallengeGetResultRequest {
    pub quest_live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetLiveListRequest {
    /// dnSpy: Org.OpenAPITools.Api.OutQuestLiveGradeApi::OutQuestLiveGradeGetLiveList() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetQuestListRequest {
    /// dnSpy: Org.OpenAPITools.Api.OutQuestLiveGradeApi::OutQuestLiveGradeGetQuestList() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetRankingListRequest {
    pub season_grade_id: Option<i32>,
    /// dnSpy enum: Org.OpenAPITools.Model.GradeGetRankType
    pub get_rank_type: Option<i32>,
    pub target_rank: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetRankListRequest {
    pub quest_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetResultRequest {
    pub quest_live_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetStageDataRequest {
    pub grade_quest_square_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeGetStageListRequest {
    pub quest_id: Option<String>,
    pub rank: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestActionRequest {
    pub grade_quest_square_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestAddSkillRequest {
    pub grade_quest_square_id: Option<i32>,
    pub grade_add_skills_id: Option<i64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestRetireRequest {
    pub grade_quest_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetQuestStartRequest {
    pub grade_quest_series_id: Option<i32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OutQuestLiveGradeSetRewardRequest {
    /// dnSpy: Org.OpenAPITools.Api.OutQuestLiveGradeApi::OutQuestLiveGradeSetReward() has no request body fields
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveEnterRequest {
    pub live_id: Option<String>,
    pub offset_timeline_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveLiveInfoRequest {
    pub live_id: Option<String>,
    pub offset_timeline_id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WebviewLiveLoginRequest {
    pub player_id: Option<String>,
    pub password: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// Auto-generated from crates/api/src/l4/* path signatures.
macro_rules! define_request_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Deserialize, Serialize, Default)]
        #[serde(rename_all = "snake_case")]
        pub struct $name {
            #[serde(flatten)]
            pub extra: Map<String, Value>,
        }

        impl From<Value> for $name {
            fn from(value: Value) -> Self {
                match value {
                    Value::Object(extra) => Self { extra },
                    _ => Self::default(),
                }
            }
        }
    };
}

define_request_type!(ActivityRecordGetTopRequest);
define_request_type!(ActivityRecordNotifyMonthDisplayedRequest);
define_request_type!(ActivityRecordPlayAdvDataRequest);
define_request_type!(ActivityRecordSetWatchableStatusRequest);
define_request_type!(BeginnerMissionGetListRequest);
define_request_type!(CircleGetChatLogListRequest);
define_request_type!(CircleGetCircleTopInfoRequest);
// CircleGetDetailRequest/CircleGetInfoRequest/CircleGetInviteAndJoinInfoRequest are strongly typed above.
define_request_type!(CircleGetInviteListRequest);
// CircleGetListRequest/CircleSetApproveInviteRequest/CircleSetApproveJoinRequest are strongly typed above.
// CircleSetCancelInviteRequest/CircleSetCancelJoinRequest/CircleSetChatMessageRequest are strongly typed above.
// CircleSetCreateRequest/CircleSetDismissalRequest are strongly typed above.
define_request_type!(CircleSetDissolutionRequest);
// CircleSetDonationRequest/CircleSetExpulsionRequest/CircleSetInviteRequest are strongly typed above.
// CircleSetItemRequestRequest/CircleSetJoinRequest are strongly typed above.
define_request_type!(CircleSetOutRequest);
// CircleSetPositionRequest/CircleSetRejectInviteRequest/CircleSetRejectJoinRequest are strongly typed above.
// CircleSetSettingRequest/CircleSetTransferLeaderRequest are strongly typed above.
define_request_type!(CollectionGetCharacterInfoRequest);
define_request_type!(CollectionGetEmojiListRequest);
define_request_type!(CollectionGetGallaryListRequest);
define_request_type!(CollectionGetMusicListRequest);
define_request_type!(CollectionGetStickerListRequest);
define_request_type!(CollectionSetGallaryDataRequest);
define_request_type!(CollectionSetMusicPlayRequest);
define_request_type!(CommonGetHeaderAnnounsRequest);
// GachaConfirmTicketExpiredTimeRequest/GachaGetExchangeCardHavingListRequest are strongly typed above.
// GachaGetExchangeCardListRequest/GachaGetGuaranteePointListRequest are strongly typed above.
define_request_type!(GachaGetHistoryRequest);
// GachaGetLotteryChanceRequest is strongly typed above.
define_request_type!(GachaGetSeriesListRequest);
// GachaSetGuaranteePointExchangeRequest/GachaSetPrizeReceiveRequest are strongly typed above.
// GachaSetPurchaseRequest/GachaSetSelectTicketExchangeRequest are strongly typed above.
define_request_type!(HomeGetCustomSettingRequest);
define_request_type!(HomeGetHomeRequest);
define_request_type!(HomeGetLoginBonusRequest);
define_request_type!(HomeGetWallpaperSettingRequest);
define_request_type!(HomeNotifyWallpaperSettingRequest);
define_request_type!(HomeSetClockSettingRequest);
define_request_type!(HomeSetCurrentWallpaperSettingRequest);
define_request_type!(HomeSetShowRetireRequest);
define_request_type!(HomeSetWallpaperSettingRequest);
define_request_type!(InQuestLiveGetLiveInfoRequest);
define_request_type!(InQuestLiveSetFinishRequest);
define_request_type!(InQuestLiveSetRetireRequest);
define_request_type!(InQuestLiveSetStartRequest);
define_request_type!(InQuestLiveSkipQuestRequest);
define_request_type!(ItemExchangeGetLimitBreakMaterialConvertListRequest);
define_request_type!(ItemExchangeGetListNewRequest);
// ItemExchangeGetListRequest is strongly typed above.
define_request_type!(ItemExchangeSetLimitBreakMaterialConvertRequest);
// ItemExchangeSetPurchaseRequest is strongly typed above.
define_request_type!(ItemStoreGetListRequest);
define_request_type!(JewelShopGetBirthdayRequest);
define_request_type!(JewelShopGetListRequest);
define_request_type!(JewelShopGetMembershipListRequest);
// JewelShopSetBirthdayRequest is strongly typed above.
define_request_type!(JewelShopSetMembershipPurchaseRequest);
// JewelShopSetPurchaseRequest is strongly typed above.
define_request_type!(MissionGetListRequest);
define_request_type!(MissionReceiveCommonMissionRewardRequest);
// OutQuestLiveDailyGetRecoveryChallengeCountRequest is strongly typed above.
// OutQuestLiveDailyGetReleaseTicketRequest is strongly typed above.
// OutQuestLiveDailyGetStageDataRequest is strongly typed above.
// OutQuestLiveDailyGetStageListRequest is strongly typed above.
// OutQuestLiveDailyGetStageSelectRequest is strongly typed above.
// OutQuestLiveDailyRecoveryChallengeCountRequest is strongly typed above.
// OutQuestLiveDailySetReleaseRequest is strongly typed above.
// OutQuestLiveDreamGetMemberSelectRequest is strongly typed above.
// OutQuestLiveDreamGetResultRequest is strongly typed above.
// OutQuestLiveDreamNotifyMemberReleaseConfirmRequest is strongly typed above.
// OutQuestLiveDreamSetCardRequest is strongly typed above.
// OutQuestLiveGetLiveSettingRequest is strongly typed above.
// OutQuestLiveGetPlayReportRequest is strongly typed above.
// OutQuestLiveGetQuestClearStatusListRequest is strongly typed above.
// OutQuestLiveGetQuestListRequest is strongly typed above.
// OutQuestLiveGetQuestTopRequest is strongly typed above.
// OutQuestLiveGetResultRequest is strongly typed above.
// OutQuestLiveGetStaminaRecoveryInfoRequest is strongly typed above.
// OutQuestLiveGradeChallengeGetQuestListRequest is strongly typed above.
// OutQuestLiveGradeChallengeGetRankingListRequest is strongly typed above.
// OutQuestLiveGradeChallengeGetResultRequest is strongly typed above.
// OutQuestLiveGradeGetLiveListRequest is strongly typed above.
// OutQuestLiveGradeGetQuestListRequest is strongly typed above.
// OutQuestLiveGradeGetRankingListRequest is strongly typed above.
// OutQuestLiveGradeGetRankListRequest is strongly typed above.
// OutQuestLiveGradeGetResultRequest is strongly typed above.
// OutQuestLiveGradeGetStageDataRequest is strongly typed above.
// OutQuestLiveGradeGetStageListRequest is strongly typed above.
// OutQuestLiveGradeSetQuestActionRequest is strongly typed above.
// OutQuestLiveGradeSetQuestAddSkillRequest is strongly typed above.
// OutQuestLiveGradeSetQuestRetireRequest is strongly typed above.
// OutQuestLiveGradeSetQuestStartRequest is strongly typed above.
// OutQuestLiveGradeSetRewardRequest is strongly typed above.
define_request_type!(OutQuestLiveGrandPrixGetHistoryRequest);
define_request_type!(OutQuestLiveGrandPrixGetRankingListRequest);
define_request_type!(OutQuestLiveGrandPrixGetResultRequest);
define_request_type!(OutQuestLiveGrandPrixGetStageDataRequest);
define_request_type!(OutQuestLiveGrandPrixGetStageListRequest);
define_request_type!(OutQuestLiveGrandPrixGetStageSelectRequest);
define_request_type!(OutQuestLiveGrandPrixGetTopInfoRequest);
define_request_type!(OutQuestLiveMusicLearningGetMusicSelectRequest);
define_request_type!(OutQuestLiveMusicLearningGetResultRequest);
define_request_type!(OutQuestLiveMusicLearningSetMusicRequest);
define_request_type!(OutQuestLiveRaidEventGetResultRequest);
define_request_type!(OutQuestLiveRaidEventGetStageDataRequest);
define_request_type!(OutQuestLiveRaidEventGetStageListRequest);
define_request_type!(OutQuestLiveRaidEventGetStaminaRecoveryInfoRequest);
define_request_type!(OutQuestLiveRaidEventGetTopInfoRequest);
define_request_type!(OutQuestLiveRaidEventRecoveryStaminaRequest);
define_request_type!(OutQuestLiveRaidEventSetJoinMessageRequest);
define_request_type!(OutQuestLiveRaidEventSetRewardRequest);
// OutQuestLiveRecoveryStaminaRequest is strongly typed above.
// OutQuestLiveSetLiveSettingRequest is strongly typed above.
define_request_type!(OutQuestLiveStandardGetAreaSelectRequest);
define_request_type!(OutQuestLiveStandardGetStageDataRequest);
define_request_type!(OutQuestLiveStandardGetStageSelectRequest);
define_request_type!(OutQuestLiveStandardSetAreaSelectViewHistRequest);
define_request_type!(PetalExchangeGetListRequest);
// PetalExchangeSetPurchaseRequest is strongly typed above.
// PresentBoxGetHistoryRequest/PresentBoxGetListRequest are strongly typed above.
// PresentBoxItemDetailRequest/PresentBoxSetItemAllRequest are strongly typed above.
// PresentBoxSetItemRequest is strongly typed above.
// ProfileDeleteMyDesignRequest/ProfileGetFanLevelInfoRequest/ProfileGetInfoRequest are strongly typed above.
define_request_type!(ProfileGetMuteListRequest);
define_request_type!(ProfileGetMyDesignCardListRequest);
define_request_type!(ProfileGetMyDesignIconListRequest);
// ProfileGetMyDesignIconRequest is strongly typed above.
define_request_type!(ProfileGetProfileCardRequest);
define_request_type!(ProfileGetProfileIconRequest);
// ProfileSetBirthdayRequest/ProfileSetCommentRequest/ProfileSetMuteCancelRequest are strongly typed above.
// ProfileSetMuteRequest/ProfileSetMyDesignCardRequest/ProfileSetMyDesignIconRequest are strongly typed above.
// ProfileSetMyDesignNameRequest/ProfileSetNameRequest/ProfileSetProfileCardRequest are strongly typed above.
// ProfileSetProfileIconRequest/ProfileSetReportRequest/ProfileUseFanlevelPointStocksRequest are strongly typed above.
// RegisterApproveTermsRequest is strongly typed above.
define_request_type!(RegisterGetTermsRequest);
define_request_type!(RegisterSetApproveTermsRequest);
// RegisterSetNewUserRequest/RegisterSetUserDataRequest are strongly typed above.
// RhythmGameDeckModifyDeckListRequest/RhythmGameDeckSetDeckNameRequest are strongly typed above.
define_request_type!(RhythmGameDeckSetResetDeckRequest);
// RhythmGameGrandPrixSetCenterRequest/RhythmGameGrandPrixSetDeckRequest are strongly typed above.
// RhythmGameGrandPrixSetFinishLiveRequest/RhythmGameGrandPrixSetPositionRequest are strongly typed above.
// RhythmGameGrandPrixSetResetRequest is strongly typed above.
define_request_type!(RhythmGameGrandPrixSetRetireLiveRequest);
// RhythmGameGrandPrixSetStartLiveRequest/RhythmGameGrandPrixTopRequest are strongly typed above.
define_request_type!(RhythmGameHomeRequest);
// RhythmGameLiveSetFinishRequest is strongly typed above.
define_request_type!(RhythmGameLiveSetRetireRequest);
// RhythmGameLiveSetStartRequest is strongly typed above.
// RhythmGameReceiveClassMissionRequest is strongly typed above.
define_request_type!(RhythmGameReceiveTotalMissionRequest);
// RhythmGameSetMusicRequest is strongly typed above.
define_request_type!(SelectTicketExchangeGetListRequest);
define_request_type!(SerialCodeSetExchangeRequest);
// ShopCheckPurchaseRequest is strongly typed above.
define_request_type!(ShopGetListRequest);
define_request_type!(SiscaStoreGetListRequest);
// SiscaStoreSetPurchaseRequest is strongly typed above.
define_request_type!(StepUpBeginnerMissionGetListRequest);
// StepUpBeginnerMissionSetRewardRequest is strongly typed above.
define_request_type!(StickerExchangeGetListRequest);
// StickerExchangeSetPurchaseRequest is strongly typed above.
// TutorialSetStepRequest is strongly typed above.
define_request_type!(UserCardCheckEvolutionRequest);
define_request_type!(UserCardCheckLimitBreakRequest);
define_request_type!(UserCardCheckSkillLevelUpRequest);
define_request_type!(UserCardCheckStyleLevelUpRequest);
define_request_type!(UserCardEvolutionRequest);
define_request_type!(UserCardGetDetailRentalRequest);
define_request_type!(UserCardGetDetailRequest);
define_request_type!(UserCardGetListRequest);
define_request_type!(UserCardLimitBreakRequest);
define_request_type!(UserCardRhythmGameSkillLevelUpRequest);
define_request_type!(UserCardSkillLevelUpRequest);
define_request_type!(UserCardStyleLevelUpRequest);
define_request_type!(UserDeckGetCardListRequest);
define_request_type!(UserDeckGetListRequest);
define_request_type!(UserDeckModifyDeckListRequest);
define_request_type!(UserDeckNotifyAutoDeckRequest);
define_request_type!(UserDeckRemoveSideStyleRequest);
define_request_type!(UserDeckSetCopyDeckRequest);
define_request_type!(UserDeckSetDeckRequest);
define_request_type!(UserDeckSetDeleteDeckRequest);
define_request_type!(UserDeckSetPositionRequest);
define_request_type!(UserDeckSetSideStyleRequest);
// UserItemGetDetailRequest is strongly typed above.
define_request_type!(UserItemsGetListRequest);
define_request_type!(UserJewelGetHistoryRequest);
define_request_type!(UserPushDeviceRequest);
define_request_type!(UserPushDevicesRequest);
// UserSetContentsReleaseEffectHistoryRequest is strongly typed above.
// UserSetSimpleTutorialFinishRequest is strongly typed above.
define_request_type!(WebviewGachaGetDetailRequest);
define_request_type!(WebviewGachaGetSelectCardListRequest);
// WebviewLiveEnterRequest/WebviewLiveLiveInfoRequest/WebviewLiveLoginRequest are strongly typed above.
define_request_type!(WebviewSchoolIdolConnectPostGetThemeListRequest);
define_request_type!(WebviewShopGetMembershipPerkDetailRequest);
