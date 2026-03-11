use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(GachaApi);

impl<'a> GachaApi<'a> {
    // POST /v1/gacha/confirm_ticket_expired_time
    post!(
        confirm_ticket_expired_time,
        "/gacha/confirm_ticket_expired_time",
        crate::model::GachaConfirmTicketExpiredTimeRequest,
        serde_json::Value
    );

    // POST /v1/gacha/get_exchange_card_having_list
    post!(
        get_exchange_card_having_list,
        "/gacha/get_exchange_card_having_list",
        crate::model::GachaGetExchangeCardHavingListRequest,
        crate::model::GachaGetExchangeCardHavingListResponse
    );

    // POST /v1/gacha/get_guarantee_point_list
    post!(
        get_guarantee_point_list,
        "/gacha/get_guarantee_point_list",
        crate::model::GachaGetGuaranteePointListRequest,
        crate::model::GachaGetGuaranteePointListResponse
    );

    // POST /v1/gacha/get_history
    post!(
        get_history,
        "/gacha/get_history",
        crate::model::GachaGetHistoryResponse
    );

    // POST /v1/gacha/get_series_list
    post!(
        get_series_list,
        "/gacha/get_series_list",
        crate::model::GachaGetSeriesListResponse
    );

    // POST /v1/gacha/set_guarantee_point_exchange
    post!(
        set_guarantee_point_exchange,
        "/gacha/set_guarantee_point_exchange",
        crate::model::GachaSetGuaranteePointExchangeRequest,
        crate::model::GachaSetGuaranteePointExchangeResponse
    );

    // POST /v1/gacha/set_prize_receive
    post!(
        set_prize_receive,
        "/gacha/set_prize_receive",
        crate::model::GachaSetPrizeReceiveRequest,
        crate::model::GachaSetPrizeReceiveResponse
    );

    // POST /v1/gacha/set_purchase
    post!(
        set_purchase,
        "/gacha/set_purchase",
        crate::model::GachaSetPurchaseRequest,
        crate::model::GachaSetPurchaseResponse
    );

    // POST /v1/gacha/set_select_ticket_exchange
    post!(
        set_select_ticket_exchange,
        "/gacha/set_select_ticket_exchange",
        crate::model::GachaSetSelectTicketExchangeRequest,
        crate::model::GachaSetSelectTicketExchangeResponse
    );
}




