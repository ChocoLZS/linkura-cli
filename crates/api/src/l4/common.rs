use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(CommonApi);

impl<'a> CommonApi<'a> {
    // POST /v1/common/get_header_announs
    post!(
        get_header_announs,
        "/common/get_header_announs",
        crate::model::CommonGetHeaderAnnounsResponse
    );
}




