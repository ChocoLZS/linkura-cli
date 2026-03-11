use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(ItemStoreApi);

impl<'a> ItemStoreApi<'a> {
    // POST /v1/item_store/get_list
    post!(
        get_list,
        "/item_store/get_list",
        crate::model::ItemStoreGetListResponse
    );
}




