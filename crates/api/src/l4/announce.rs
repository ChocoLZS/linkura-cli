use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(AnnounceApi);

impl<'a> AnnounceApi<'a> {
    // POST /v1/announce/detail
    post!(
        detail,
        "/announce/detail",
        crate::model::AnnounceDetailRequest,
        crate::model::AnnounceDetailResponse
    );

    // POST /v1/announce/list
    post!(
        list,
        "/announce/list",
        crate::model::AnnounceListRequest,
        crate::model::AnnounceListResponse
    );
}




