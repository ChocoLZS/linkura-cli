use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(RegisterApi);

impl<'a> RegisterApi<'a> {
    // POST /v1/register/approve_terms
    post!(
        approve_terms,
        "/register/approve_terms",
        crate::model::RegisterApproveTermsRequest,
        crate::model::RegisterApproveTermsResponse
    );

    // POST /v1/register/get_terms
    post!(
        get_terms,
        "/register/get_terms",
        crate::model::RegisterGetTermsResponse
    );

    // POST /v1/register/set_approve_terms
    post!(
        set_approve_terms,
        "/register/set_approve_terms",
        serde_json::Value
    );

    // POST /v1/register/set_new_user
    post!(
        set_new_user,
        "/register/set_new_user",
        crate::model::RegisterSetNewUserRequest,
        crate::model::RegisterSetNewUserResponse
    );

    // POST /v1/register/set_user_data
    post!(
        set_user_data,
        "/register/set_user_data",
        crate::model::RegisterSetUserDataRequest,
        crate::model::RegisterSetUserDataResponse
    );
}




