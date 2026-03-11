use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(AccountApi);

impl<'a> AccountApi<'a> {
    // POST /v1/account/connect
    post!(
        connect,
        "/account/connect",
        crate::model::ConnectRequest,
        crate::model::ConnectResponse
    );

    // POST /v1/account/delete
    post!(
        delete,
        "/account/delete",
        serde_json::Value
    );

    // POST /v1/account/delete_connect_data
    post!(
        delete_connect_data,
        "/account/delete_connect_data",
        crate::model::DeleteConnectDataRequest,
        crate::model::DeleteConnectDataResponse
    );

    // POST /v1/account/get_connect_data
    post!(
        get_connect_data,
        "/account/get_connect_data",
        crate::model::GetConnectDataResponse
    );

    // POST /v1/account/get_connect_user
    post!(
        get_connect_user,
        "/account/get_connect_user",
        crate::model::GetConnectUserRequest,
        crate::model::GetConnectUserResponse
    );

    // POST /v1/account/set_connect_data
    post!(
        set_connect_data,
        "/account/set_connect_data",
        crate::model::SetConnectDataRequest,
        crate::model::SetConnectDataResponse
    );
}




