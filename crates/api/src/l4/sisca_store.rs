use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(SiscaStoreApi);

impl<'a> SiscaStoreApi<'a> {
    // POST /v1/sisca_store/get_list
    define_post_method!(
        get_list,
        "/sisca_store/get_list",
        crate::model::SiscaStoreGetListRequest
    );

    // POST /v1/sisca_store/set_purchase
    define_post_method!(
        set_purchase,
        "/sisca_store/set_purchase",
        crate::model::SiscaStoreSetPurchaseRequest
    );
}
