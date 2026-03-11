use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(WithliveApi);

impl<'a> WithliveApi<'a> {
    // POST /v1/withlive/comment
    post!(
        comment,
        "/withlive/comment",
        crate::model::WithliveCommentRequest,
        crate::model::WithliveCommentResponse
    );

    // POST /v1/withlive/connect_token
    post!(
        connect_token,
        "/withlive/connect_token",
        crate::model::LiveConnectTokenRequest,
        crate::model::LiveConnectTokenResponse
    );

    // POST /v1/withlive/enquetes/answer
    post!(
        enquetes_answer,
        "/withlive/enquetes/answer",
        crate::model::WithliveEnqueteAnswerRequest,
        crate::model::WithliveEnqueteAnswerResponse
    );

    // POST /v1/withlive/enter
    post!(
        enter,
        "/withlive/enter",
        crate::model::WithliveEnterRequest,
        crate::model::WithliveEnterResponse
    );

    // POST /v1/withlive/get_gift_shop_list
    post!(
        get_gift_shop_list,
        "/withlive/get_gift_shop_list",
        crate::model::WithliveGetGiftShopListRequest,
        crate::model::WithliveGetGiftShopListResponse
    );

    // POST /v1/withlive/get_list
    post!(
        get_list,
        "/withlive/get_list",
        crate::model::WithliveGetListResponse
    );

    // POST /v1/withlive/gift
    post!(
        gift,
        "/withlive/gift",
        crate::model::WithliveGiftRequest,
        crate::model::WithliveGiftResponse
    );

    // POST /v1/withlive/gift_pt_rankings
    post_params!(
        gift_pt_rankings,
        "/withlive/gift_pt_rankings",
        crate::model::WithliveGiftPtRankingsResponse,
        live_id: String,
    );

    // POST /v1/withlive/live_info
    post_params!(
        live_info,
        "/withlive/live_info",
        crate::model::WithliveLiveInfoResponse,
        live_id: String,
        offset_timeline_id: String,
    );

    // POST /v1/withlive/live_subinfo
    post_params!(
        live_subinfo,
        "/withlive/live_subinfo",
        crate::model::WithliveLiveSubinfoResponse,
        live_id: String,
    );

    // POST /v1/withlive/message_card
    post!(
        message_card,
        "/withlive/message_card",
        crate::model::WithliveMessageCardRequest,
        crate::model::WithliveMessageCardResponse
    );

    // POST /v1/withlive/prize
    post_params!(
        prize,
        "/withlive/prize",
        crate::model::WithlivePrizeResponse,
        live_id: String,
        present_box_id: String,
    );

    // POST /v1/withlive/result_ranking
    post!(
        result_ranking,
        "/withlive/result_ranking",
        crate::model::WithliveResultRankingRequest,
        crate::model::WithliveResultRankingResponse
    );

    // POST /v1/withlive/set_star
    post!(
        set_star,
        "/withlive/set_star",
        crate::model::WithliveSetStarRequest,
        crate::model::WithliveSetStarResponse
    );
}





