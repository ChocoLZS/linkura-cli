use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(ShopApi);

impl<'a> ShopApi<'a> {
    // POST /v1/shop/check_purchase
    post!(
        check_purchase,
        "/shop/check_purchase",
        crate::model::ShopCheckPurchaseRequest,
        serde_json::Value
    );

    // POST /v1/shop/get_list
    post!(
        get_list,
        "/shop/get_list",
        crate::model::ShopGetListResponse
    );
}




