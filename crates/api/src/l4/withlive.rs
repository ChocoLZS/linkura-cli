use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(WithliveApi);

impl<'a> WithliveApi<'a> {
    // POST /v1/withlive/comment
    define_post_method!(
        comment,
        "/withlive/comment",
        crate::model::WithliveCommentRequest
    );

    // POST /v1/withlive/connect_token
    define_post_method!(
        connect_token,
        "/withlive/connect_token",
        crate::model::WithliveConnectTokenRequest
    );

    // POST /v1/withlive/enquetes/answer
    define_post_method!(
        enquetes_answer,
        "/withlive/enquetes/answer",
        crate::model::WithliveEnquetesAnswerRequest
    );

    // POST /v1/withlive/enter
    define_post_method!(enter, "/withlive/enter", crate::model::WithliveEnterRequest);

    // POST /v1/withlive/get_gift_shop_list
    define_post_method!(
        get_gift_shop_list,
        "/withlive/get_gift_shop_list",
        crate::model::WithliveGetGiftShopListRequest
    );

    // POST /v1/withlive/get_list
    define_post_method!(get_list, "/withlive/get_list");

    // POST /v1/withlive/gift
    define_post_method!(gift, "/withlive/gift", crate::model::WithliveGiftRequest);

    // POST /v1/withlive/gift_pt_rankings
    define_post_method!(
        gift_pt_rankings,
        "/withlive/gift_pt_rankings",
        crate::model::WithliveGiftPtRankingsRequest
    );

    // POST /v1/withlive/live_info
    define_post_method!(
        live_info,
        "/withlive/live_info",
        crate::model::WithliveLiveInfoRequest
    );

    // POST /v1/withlive/live_subinfo
    define_post_method!(
        live_subinfo,
        "/withlive/live_subinfo",
        crate::model::WithliveLiveSubinfoRequest
    );

    // POST /v1/withlive/message_card
    define_post_method!(
        message_card,
        "/withlive/message_card",
        crate::model::WithliveMessageCardRequest
    );

    // POST /v1/withlive/prize
    define_post_method!(prize, "/withlive/prize", crate::model::WithlivePrizeRequest);

    // POST /v1/withlive/result_ranking
    define_post_method!(
        result_ranking,
        "/withlive/result_ranking",
        crate::model::WithliveResultRankingRequest
    );

    // POST /v1/withlive/set_star
    define_post_method!(
        set_star,
        "/withlive/set_star",
        crate::model::WithliveSetStarRequest
    );
}
