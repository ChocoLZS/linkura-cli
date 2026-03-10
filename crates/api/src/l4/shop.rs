use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ShopApi);

impl<'a> ShopApi<'a> {
    // POST /v1/shop/check_purchase
    define_post_method!(
        check_purchase,
        "/shop/check_purchase",
        crate::model::ShopCheckPurchaseRequest
    );

    // POST /v1/shop/get_list
    define_post_method!(get_list, "/shop/get_list", crate::model::ShopGetListRequest);
}
