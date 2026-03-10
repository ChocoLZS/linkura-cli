use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(GachaApi);

impl<'a> GachaApi<'a> {
    // POST /v1/gacha/confirm_ticket_expired_time
    define_post_method!(
        confirm_ticket_expired_time,
        "/gacha/confirm_ticket_expired_time",
        crate::model::GachaConfirmTicketExpiredTimeRequest
    );

    // POST /v1/gacha/get_exchange_card_having_list
    define_post_method!(
        get_exchange_card_having_list,
        "/gacha/get_exchange_card_having_list",
        crate::model::GachaGetExchangeCardHavingListRequest
    );

    // POST /v1/gacha/get_guarantee_point_list
    define_post_method!(
        get_guarantee_point_list,
        "/gacha/get_guarantee_point_list",
        crate::model::GachaGetGuaranteePointListRequest
    );

    // POST /v1/gacha/get_history
    define_post_method!(
        get_history,
        "/gacha/get_history",
        crate::model::GachaGetHistoryRequest
    );

    // POST /v1/gacha/get_series_list
    define_post_method!(
        get_series_list,
        "/gacha/get_series_list",
        crate::model::GachaGetSeriesListRequest
    );

    // POST /v1/gacha/set_guarantee_point_exchange
    define_post_method!(
        set_guarantee_point_exchange,
        "/gacha/set_guarantee_point_exchange",
        crate::model::GachaSetGuaranteePointExchangeRequest
    );

    // POST /v1/gacha/set_prize_receive
    define_post_method!(
        set_prize_receive,
        "/gacha/set_prize_receive",
        crate::model::GachaSetPrizeReceiveRequest
    );

    // POST /v1/gacha/set_purchase
    define_post_method!(
        set_purchase,
        "/gacha/set_purchase",
        crate::model::GachaSetPurchaseRequest
    );

    // POST /v1/gacha/set_select_ticket_exchange
    define_post_method!(
        set_select_ticket_exchange,
        "/gacha/set_select_ticket_exchange",
        crate::model::GachaSetSelectTicketExchangeRequest
    );
}
