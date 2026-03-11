use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(PetalExchangeApi);

impl<'a> PetalExchangeApi<'a> {
    // POST /v1/petal_exchange/get_list
    post!(
        get_list,
        "/petal_exchange/get_list",
        crate::model::PetalExchangeGetListResponse
    );

    // POST /v1/petal_exchange/set_purchase
    post!(
        set_purchase,
        "/petal_exchange/set_purchase",
        crate::model::PetalExchangeSetPurchaseRequest,
        crate::model::PetalExchangeSetPurchaseResponse
    );
}




