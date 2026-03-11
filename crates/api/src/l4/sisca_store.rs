use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(SiscaStoreApi);

impl<'a> SiscaStoreApi<'a> {
    // POST /v1/sisca_store/get_list
    post!(
        get_list,
        "/sisca_store/get_list",
        crate::model::SiscaStoreGetListResponse
    );

    // POST /v1/sisca_store/set_purchase
    post!(
        set_purchase,
        "/sisca_store/set_purchase",
        crate::model::SiscaStoreSetPurchaseRequest,
        crate::model::SiscaStoreSetPurchaseResponse
    );
}




