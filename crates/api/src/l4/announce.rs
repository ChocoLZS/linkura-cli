use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(AnnounceApi);

impl<'a> AnnounceApi<'a> {
    // POST /v1/announce/detail
    define_post_method!(
        detail,
        "/announce/detail",
        crate::model::AnnounceDetailRequest
    );

    // POST /v1/announce/list
    define_post_method!(list, "/announce/list", crate::model::AnnounceListRequest);
}
