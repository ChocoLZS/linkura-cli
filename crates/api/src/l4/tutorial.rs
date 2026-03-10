use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(TutorialApi);

impl<'a> TutorialApi<'a> {
    // POST /v1/tutorial/set_step
    define_post_method!(
        set_step,
        "/tutorial/set_step",
        crate::model::TutorialSetStepRequest
    );
}
