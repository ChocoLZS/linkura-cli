use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(GpPrizeExchangeApi);

impl<'a> GpPrizeExchangeApi<'a> {
    // POST /v1/gp_prize_exchange/get_list
    post!(
        get_list,
        "/gp_prize_exchange/get_list",
        crate::model::GpPrizeExchangeGetListResponse
    );

    // POST /v1/gp_prize_exchange/set_purchase
    post!(
        set_purchase,
        "/gp_prize_exchange/set_purchase",
        crate::model::GpPrizeExchangeSetPurchaseRequest,
        crate::model::GpPrizeExchangeSetPurchaseResponse
    );
}




