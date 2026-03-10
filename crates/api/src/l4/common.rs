use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(CommonApi);

impl<'a> CommonApi<'a> {
    // POST /v1/common/get_header_announs
    define_post_method!(
        get_header_announs,
        "/common/get_header_announs",
        crate::model::CommonGetHeaderAnnounsRequest
    );
}
