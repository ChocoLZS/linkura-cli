use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(AccountApi);

impl<'a> AccountApi<'a> {
    // POST /v1/account/connect
    define_post_method!(
        connect,
        "/account/connect",
        crate::model::AccountConnectRequest
    );

    // POST /v1/account/delete
    define_post_method!(delete, "/account/delete");

    // POST /v1/account/delete_connect_data
    define_post_method!(
        delete_connect_data,
        "/account/delete_connect_data",
        crate::model::AccountDeleteConnectDataRequest
    );

    // POST /v1/account/get_connect_data
    define_post_method!(get_connect_data, "/account/get_connect_data");

    // POST /v1/account/get_connect_user
    define_post_method!(
        get_connect_user,
        "/account/get_connect_user",
        crate::model::AccountGetConnectUserRequest
    );

    // POST /v1/account/set_connect_data
    define_post_method!(
        set_connect_data,
        "/account/set_connect_data",
        crate::model::AccountSetConnectDataRequest
    );
}
