use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(FesliveApi);

impl<'a> FesliveApi<'a> {
    // POST /v1/feslive/circle_chat/comment
    post!(
        circle_chat_comment,
        "/feslive/circle_chat/comment",
        crate::model::FesliveCircleChatCommentRequest,
        crate::model::FesliveCircleChatCommentResponse
    );

    // POST /v1/feslive/clap
    post!(
        clap,
        "/feslive/clap",
        crate::model::FesliveClapRequest,
        serde_json::Value
    );

    // POST /v1/feslive/comment
    post!(
        comment,
        "/feslive/comment",
        crate::model::FesliveCommentRequest,
        crate::model::FesliveCommentResponse
    );

    // POST /v1/feslive/connect_token
    post!(
        connect_token,
        "/feslive/connect_token",
        crate::model::LiveConnectTokenRequest,
        crate::model::LiveConnectTokenResponse
    );

    // POST /v1/feslive/enquete
    post!(
        enquete,
        "/feslive/enquete",
        crate::model::FesliveEnqueteRequest,
        serde_json::Value
    );

    // POST /v1/feslive/enter
    post!(
        enter,
        "/feslive/enter",
        crate::model::FesliveEnterRequest,
        crate::model::FesliveEnterResponse
    );

    // POST /v1/feslive/get_gift_shop_list
    post!(
        get_gift_shop_list,
        "/feslive/get_gift_shop_list",
        crate::model::FesliveGetGiftShopListRequest,
        crate::model::FesliveGetGiftShopListResponse
    );

    // POST /v1/feslive/get_list
    post!(
        get_list,
        "/feslive/get_list",
        crate::model::FesliveGetListResponse
    );

    // POST /v1/feslive/gift
    post!(
        gift,
        "/feslive/gift",
        crate::model::FesliveGiftRequest,
        crate::model::FesliveGiftResponse
    );

    // POST /v1/feslive/gift_pt_rankings
    post_params!(
        gift_pt_rankings,
        "/feslive/gift_pt_rankings",
        crate::model::FesliveGiftPtRankingsResponse,
        live_id: String,
    );

    // POST /v1/feslive/grand_prix_ranking
    post!(
        grand_prix_ranking,
        "/feslive/grand_prix_ranking",
        crate::model::FesliveGrandPrixRankingRequest,
        crate::model::FesliveGrandPrixRankingResponse
    );

    // POST /v1/feslive/live_announcement
    post_params!(
        live_announcement,
        "/feslive/live_announcement",
        crate::model::FesliveLiveAnnouncementResponse,
        live_id: String,
    );

    // POST /v1/feslive/live_info
    post_params!(
        live_info,
        "/feslive/live_info",
        crate::model::FesliveLiveInfoResponse,
        live_id: String,
        offset_timeline_id: String,
        offset_circle_chat_id: String,
    );

    // POST /v1/feslive/live_subinfo
    post_params!(
        live_subinfo,
        "/feslive/live_subinfo",
        crate::model::FesliveLiveSubinfoResponse,
        live_id: String,
    );

    // POST /v1/feslive/lobby
    post!(
        lobby,
        "/feslive/lobby",
        crate::model::FesliveLobbyRequest,
        crate::model::FesliveLobbyResponse
    );

    // POST /v1/feslive/lobby_announcement
    post_params!(
        lobby_announcement,
        "/feslive/lobby_announcement",
        crate::model::FesliveLobbyAnnouncementResponse,
        live_id: String,
    );

    // POST /v1/feslive/lobby_comment
    post!(
        lobby_comment,
        "/feslive/lobby_comment",
        crate::model::FesliveCommentRequest,
        crate::model::FesliveCommentResponse
    );

    // POST /v1/feslive/lobby_gift
    post!(
        lobby_gift,
        "/feslive/lobby_gift",
        crate::model::FesliveGiftRequest,
        crate::model::FesliveGiftResponse
    );

    // POST /v1/feslive/lobby_info
    post_params!(
        lobby_info,
        "/feslive/lobby_info",
        crate::model::FesliveLobbyInfoResponse,
        live_id: String,
        offset_timeline_id: String,
        offset_circle_chat_id: String,
    );

    // POST /v1/feslive/lobby_message_card
    post!(
        lobby_message_card,
        "/feslive/lobby_message_card",
        crate::model::FesliveMessageCardRequest,
        crate::model::FesliveMessageCardResponse
    );

    // POST /v1/feslive/lobby_subinfo
    post_params!(
        lobby_subinfo,
        "/feslive/lobby_subinfo",
        crate::model::FesliveLobbySubinfoResponse,
        live_id: String,
    );

    // POST /v1/feslive/message_card
    post!(
        message_card,
        "/feslive/message_card",
        crate::model::FesliveMessageCardRequest,
        crate::model::FesliveMessageCardResponse
    );

    // POST /v1/feslive/out_quest_live_ranking
    post!(
        out_quest_live_ranking,
        "/feslive/out_quest_live_ranking",
        crate::model::FesliveOutQuestLiveRankingRequest,
        crate::model::FesliveOutQuestLiveRankingResponse
    );

    // POST /v1/feslive/penlight_color
    post!(
        penlight_color,
        "/feslive/penlight_color",
        crate::model::FeslivePenlightColorRequest,
        serde_json::Value
    );

    // POST /v1/feslive/prize
    post_params!(
        prize,
        "/feslive/prize",
        crate::model::FeslivePrizeResponse,
        live_id: String,
        present_box_id: String,
    );

    // POST /v1/feslive/set_camera
    post!(
        set_camera,
        "/feslive/set_camera",
        crate::model::FesliveSetCameraRequest,
        crate::model::FesliveSetCameraResponse
    );

    // POST /v1/feslive/set_flower_stand
    post!(
        set_flower_stand,
        "/feslive/set_flower_stand",
        crate::model::SetFlowerStandRequest,
        crate::model::SetFlowerStandResponse
    );

    // POST /v1/feslive/set_prize
    post!(
        set_prize,
        "/feslive/set_prize",
        crate::model::FesliveSetPrizeRequest,
        serde_json::Value
    );
}





