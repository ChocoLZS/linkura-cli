use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(RegisterApi);

impl<'a> RegisterApi<'a> {
    // POST /v1/register/approve_terms
    define_post_method!(
        approve_terms,
        "/register/approve_terms",
        crate::model::RegisterApproveTermsRequest
    );

    // POST /v1/register/get_terms
    define_post_method!(
        get_terms,
        "/register/get_terms",
        crate::model::RegisterGetTermsRequest
    );

    // POST /v1/register/set_approve_terms
    define_post_method!(
        set_approve_terms,
        "/register/set_approve_terms",
        crate::model::RegisterSetApproveTermsRequest
    );

    // POST /v1/register/set_new_user
    define_post_method!(
        set_new_user,
        "/register/set_new_user",
        crate::model::RegisterSetNewUserRequest
    );

    // POST /v1/register/set_user_data
    define_post_method!(
        set_user_data,
        "/register/set_user_data",
        crate::model::RegisterSetUserDataRequest
    );
}
