use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(ActivityRecordApi);

impl<'a> ActivityRecordApi<'a> {
    // POST /v1/activity_record/get_top
    post!(
        get_top,
        "/activity_record/get_top",
        crate::model::GetTopResponse
    );

    // POST /v1/activity_record/notify_month_displayed
    post!(
        notify_month_displayed,
        "/activity_record/notify_month_displayed",
        crate::model::NotifyMonthDisplayedRequest,
        serde_json::Value
    );

    // POST /v1/activity_record/play_adv_data
    post!(
        play_adv_data,
        "/activity_record/play_adv_data",
        crate::model::PlayAdvDataRequest,
        crate::model::PlayAdvDataResponse
    );

    // POST /v1/activity_record/set_watchable_status
    post!(
        set_watchable_status,
        "/activity_record/set_watchable_status",
        crate::model::SetWatchableStatusRequest,
        crate::model::SetWatchableStatusResponse
    );
}




