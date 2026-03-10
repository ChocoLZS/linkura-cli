use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(PetalExchangeApi);

impl<'a> PetalExchangeApi<'a> {
    // POST /v1/petal_exchange/get_list
    define_post_method!(
        get_list,
        "/petal_exchange/get_list",
        crate::model::PetalExchangeGetListRequest
    );

    // POST /v1/petal_exchange/set_purchase
    define_post_method!(
        set_purchase,
        "/petal_exchange/set_purchase",
        crate::model::PetalExchangeSetPurchaseRequest
    );
}
