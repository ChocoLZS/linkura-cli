use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(BoxEventApi);

impl<'a> BoxEventApi<'a> {
    // POST /v1/box_event/get_top_info
    post_params!(
        get_top_info,
        "/box_event/get_top_info",
        crate::model::BoxEventGetTopInfoResponse,
        box_event_series_id: Option<i32>,
    );

    // POST /v1/box_event/set_drop_box
    post!(
        set_drop_box,
        "/box_event/set_drop_box",
        crate::model::BoxEventSetDropBoxRequest,
        crate::model::BoxEventSetDropBoxResponse
    );

    // POST /v1/box_event/set_special_reward
    post!(
        set_special_reward,
        "/box_event/set_special_reward",
        crate::model::BoxEventSetSpecialRewardRequest,
        crate::model::BoxEventSetSpecialRewardResponse
    );
}





