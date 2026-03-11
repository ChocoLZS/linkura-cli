use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(PresentBoxApi);

impl<'a> PresentBoxApi<'a> {
    // POST /v1/present_box/get_history
    post!(
        get_history,
        "/present_box/get_history",
        crate::model::PresentBoxGetHistoryRequest,
        crate::model::PresentBoxGetHistoryResponse
    );

    // POST /v1/present_box/get_list
    post!(
        get_list,
        "/present_box/get_list",
        crate::model::PresentBoxGetListRequest,
        crate::model::PresentBoxGetListResponse
    );

    // POST /v1/present_box/item_detail
    post!(
        item_detail,
        "/present_box/item_detail",
        crate::model::PresentBoxItemDetailRequest,
        crate::model::PresentBoxItemDetailResponse
    );

    // POST /v1/present_box/set_item
    post!(
        set_item,
        "/present_box/set_item",
        crate::model::PresentBoxSetItemRequest,
        crate::model::PresentBoxSetItemResponse
    );

    // POST /v1/present_box/set_item_all
    post!(
        set_item_all,
        "/present_box/set_item_all",
        crate::model::PresentBoxSetItemAllRequest,
        crate::model::PresentBoxSetItemAllResponse
    );
}




