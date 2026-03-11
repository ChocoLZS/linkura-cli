use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(SerialCodeApi);

impl<'a> SerialCodeApi<'a> {
    // POST /v1/serial_code/set_exchange
    post!(
        set_exchange,
        "/serial_code/set_exchange",
        crate::model::SetExchangeRequest,
        crate::model::SetExchangeResponse
    );
}




