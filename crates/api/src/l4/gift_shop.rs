use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(GiftShopApi);

impl<'a> GiftShopApi<'a> {
    // POST /v1/gift_shop/set_purchase
    post!(
        set_purchase,
        "/gift_shop/set_purchase",
        crate::model::GiftShopSetPurchaseRequest,
        crate::model::GiftShopSetPurchaseResponse
    );
}




