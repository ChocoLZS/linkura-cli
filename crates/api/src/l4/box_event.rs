use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(BoxEventApi);

impl<'a> BoxEventApi<'a> {
    // POST /v1/box_event/get_top_info
    define_post_method!(
        get_top_info,
        "/box_event/get_top_info",
        crate::model::BoxEventGetTopInfoRequest
    );

    // POST /v1/box_event/set_drop_box
    define_post_method!(
        set_drop_box,
        "/box_event/set_drop_box",
        crate::model::BoxEventSetDropBoxRequest
    );

    // POST /v1/box_event/set_special_reward
    define_post_method!(
        set_special_reward,
        "/box_event/set_special_reward",
        crate::model::BoxEventSetSpecialRewardRequest
    );
}
