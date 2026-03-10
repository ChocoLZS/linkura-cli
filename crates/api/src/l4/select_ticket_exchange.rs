use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(SelectTicketExchangeApi);

impl<'a> SelectTicketExchangeApi<'a> {
    // POST /v1/select_ticket_exchange/get_list
    define_post_method!(
        get_list,
        "/select_ticket_exchange/get_list",
        crate::model::SelectTicketExchangeGetListRequest
    );
}
