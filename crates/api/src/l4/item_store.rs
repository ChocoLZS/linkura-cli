use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ItemStoreApi);

impl<'a> ItemStoreApi<'a> {
    // POST /v1/item_store/get_list
    define_post_method!(
        get_list,
        "/item_store/get_list",
        crate::model::ItemStoreGetListRequest
    );
}
