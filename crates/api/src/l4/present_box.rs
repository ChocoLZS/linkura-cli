use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(PresentBoxApi);

impl<'a> PresentBoxApi<'a> {
    // POST /v1/present_box/get_history
    define_post_method!(
        get_history,
        "/present_box/get_history",
        crate::model::PresentBoxGetHistoryRequest
    );

    // POST /v1/present_box/get_list
    define_post_method!(
        get_list,
        "/present_box/get_list",
        crate::model::PresentBoxGetListRequest
    );

    // POST /v1/present_box/item_detail
    define_post_method!(
        item_detail,
        "/present_box/item_detail",
        crate::model::PresentBoxItemDetailRequest
    );

    // POST /v1/present_box/set_item
    define_post_method!(
        set_item,
        "/present_box/set_item",
        crate::model::PresentBoxSetItemRequest
    );

    // POST /v1/present_box/set_item_all
    define_post_method!(
        set_item_all,
        "/present_box/set_item_all",
        crate::model::PresentBoxSetItemAllRequest
    );
}
