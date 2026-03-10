use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(FesliveApi);

impl<'a> FesliveApi<'a> {
    // POST /v1/feslive/circle_chat/comment
    define_post_method!(
        circle_chat_comment,
        "/feslive/circle_chat/comment",
        crate::model::FesliveCircleChatCommentRequest
    );

    // POST /v1/feslive/clap
    define_post_method!(clap, "/feslive/clap", crate::model::FesliveClapRequest);

    // POST /v1/feslive/comment
    define_post_method!(
        comment,
        "/feslive/comment",
        crate::model::FesliveCommentRequest
    );

    // POST /v1/feslive/connect_token
    define_post_method!(
        connect_token,
        "/feslive/connect_token",
        crate::model::FesliveConnectTokenRequest
    );

    // POST /v1/feslive/enquete
    define_post_method!(
        enquete,
        "/feslive/enquete",
        crate::model::FesliveEnqueteRequest
    );

    // POST /v1/feslive/enter
    define_post_method!(enter, "/feslive/enter", crate::model::FesliveEnterRequest);

    // POST /v1/feslive/get_gift_shop_list
    define_post_method!(
        get_gift_shop_list,
        "/feslive/get_gift_shop_list",
        crate::model::FesliveGetGiftShopListRequest
    );

    // POST /v1/feslive/get_list
    define_post_method!(get_list, "/feslive/get_list");

    // POST /v1/feslive/gift
    define_post_method!(gift, "/feslive/gift", crate::model::FesliveGiftRequest);

    // POST /v1/feslive/gift_pt_rankings
    define_post_method!(
        gift_pt_rankings,
        "/feslive/gift_pt_rankings",
        crate::model::FesliveGiftPtRankingsRequest
    );

    // POST /v1/feslive/grand_prix_ranking
    define_post_method!(
        grand_prix_ranking,
        "/feslive/grand_prix_ranking",
        crate::model::FesliveGrandPrixRankingRequest
    );

    // POST /v1/feslive/live_announcement
    define_post_method!(
        live_announcement,
        "/feslive/live_announcement",
        crate::model::FesliveLiveAnnouncementRequest
    );

    // POST /v1/feslive/live_info
    define_post_method!(
        live_info,
        "/feslive/live_info",
        crate::model::FesliveLiveInfoRequest
    );

    // POST /v1/feslive/live_subinfo
    define_post_method!(
        live_subinfo,
        "/feslive/live_subinfo",
        crate::model::FesliveLiveSubinfoRequest
    );

    // POST /v1/feslive/lobby
    define_post_method!(lobby, "/feslive/lobby", crate::model::FesliveLobbyRequest);

    // POST /v1/feslive/lobby_announcement
    define_post_method!(
        lobby_announcement,
        "/feslive/lobby_announcement",
        crate::model::FesliveLobbyAnnouncementRequest
    );

    // POST /v1/feslive/lobby_comment
    define_post_method!(
        lobby_comment,
        "/feslive/lobby_comment",
        crate::model::FesliveLobbyCommentRequest
    );

    // POST /v1/feslive/lobby_gift
    define_post_method!(
        lobby_gift,
        "/feslive/lobby_gift",
        crate::model::FesliveLobbyGiftRequest
    );

    // POST /v1/feslive/lobby_info
    define_post_method!(
        lobby_info,
        "/feslive/lobby_info",
        crate::model::FesliveLobbyInfoRequest
    );

    // POST /v1/feslive/lobby_message_card
    define_post_method!(
        lobby_message_card,
        "/feslive/lobby_message_card",
        crate::model::FesliveLobbyMessageCardRequest
    );

    // POST /v1/feslive/lobby_subinfo
    define_post_method!(
        lobby_subinfo,
        "/feslive/lobby_subinfo",
        crate::model::FesliveLobbySubinfoRequest
    );

    // POST /v1/feslive/message_card
    define_post_method!(
        message_card,
        "/feslive/message_card",
        crate::model::FesliveMessageCardRequest
    );

    // POST /v1/feslive/out_quest_live_ranking
    define_post_method!(
        out_quest_live_ranking,
        "/feslive/out_quest_live_ranking",
        crate::model::FesliveOutQuestLiveRankingRequest
    );

    // POST /v1/feslive/penlight_color
    define_post_method!(
        penlight_color,
        "/feslive/penlight_color",
        crate::model::FeslivePenlightColorRequest
    );

    // POST /v1/feslive/prize
    define_post_method!(prize, "/feslive/prize", crate::model::FeslivePrizeRequest);

    // POST /v1/feslive/set_camera
    define_post_method!(
        set_camera,
        "/feslive/set_camera",
        crate::model::FesliveSetCameraRequest
    );

    // POST /v1/feslive/set_flower_stand
    define_post_method!(
        set_flower_stand,
        "/feslive/set_flower_stand",
        crate::model::FesliveSetFlowerStandRequest
    );

    // POST /v1/feslive/set_prize
    define_post_method!(
        set_prize,
        "/feslive/set_prize",
        crate::model::FesliveSetPrizeRequest
    );
}
