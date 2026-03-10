use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ActivityRecordApi);

impl<'a> ActivityRecordApi<'a> {
    // POST /v1/activity_record/get_top
    define_post_method!(
        get_top,
        "/activity_record/get_top",
        crate::model::ActivityRecordGetTopRequest
    );

    // POST /v1/activity_record/notify_month_displayed
    define_post_method!(
        notify_month_displayed,
        "/activity_record/notify_month_displayed",
        crate::model::ActivityRecordNotifyMonthDisplayedRequest
    );

    // POST /v1/activity_record/play_adv_data
    define_post_method!(
        play_adv_data,
        "/activity_record/play_adv_data",
        crate::model::ActivityRecordPlayAdvDataRequest
    );

    // POST /v1/activity_record/set_watchable_status
    define_post_method!(
        set_watchable_status,
        "/activity_record/set_watchable_status",
        crate::model::ActivityRecordSetWatchableStatusRequest
    );
}
