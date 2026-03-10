use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(GpPrizeExchangeApi);

impl<'a> GpPrizeExchangeApi<'a> {
    // POST /v1/gp_prize_exchange/get_list
    define_post_method!(
        get_list,
        "/gp_prize_exchange/get_list",
        crate::model::GpPrizeExchangeGetListRequest
    );

    // POST /v1/gp_prize_exchange/set_purchase
    define_post_method!(
        set_purchase,
        "/gp_prize_exchange/set_purchase",
        crate::model::GpPrizeExchangeSetPurchaseRequest
    );
}
