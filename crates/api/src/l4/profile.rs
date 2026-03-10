use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ProfileApi);

impl<'a> ProfileApi<'a> {
    // POST /v1/profile/delete_my_design
    define_post_method!(
        delete_my_design,
        "/profile/delete_my_design",
        crate::model::ProfileDeleteMyDesignRequest
    );

    // POST /v1/profile/get_fan_level_info
    define_post_method!(
        get_fan_level_info,
        "/profile/get_fan_level_info",
        crate::model::ProfileGetFanLevelInfoRequest
    );

    // POST /v1/profile/get_info
    define_post_method!(
        get_info,
        "/profile/get_info",
        crate::model::ProfileGetInfoRequest
    );

    // POST /v1/profile/get_mute_list
    define_post_method!(
        get_mute_list,
        "/profile/get_mute_list",
        crate::model::ProfileGetMuteListRequest
    );

    // POST /v1/profile/get_my_design_card_list
    define_post_method!(
        get_my_design_card_list,
        "/profile/get_my_design_card_list",
        crate::model::ProfileGetMyDesignCardListRequest
    );

    // POST /v1/profile/get_my_design_icon
    define_post_method!(
        get_my_design_icon,
        "/profile/get_my_design_icon",
        crate::model::ProfileGetMyDesignIconRequest
    );

    // POST /v1/profile/get_my_design_icon_list
    define_post_method!(
        get_my_design_icon_list,
        "/profile/get_my_design_icon_list",
        crate::model::ProfileGetMyDesignIconListRequest
    );

    // POST /v1/profile/get_profile_card
    define_post_method!(
        get_profile_card,
        "/profile/get_profile_card",
        crate::model::ProfileGetProfileCardRequest
    );

    // POST /v1/profile/get_profile_icon
    define_post_method!(
        get_profile_icon,
        "/profile/get_profile_icon",
        crate::model::ProfileGetProfileIconRequest
    );

    // POST /v1/profile/set_birthday
    define_post_method!(
        set_birthday,
        "/profile/set_birthday",
        crate::model::ProfileSetBirthdayRequest
    );

    // POST /v1/profile/set_comment
    define_post_method!(
        set_comment,
        "/profile/set_comment",
        crate::model::ProfileSetCommentRequest
    );

    // POST /v1/profile/set_mute
    define_post_method!(
        set_mute,
        "/profile/set_mute",
        crate::model::ProfileSetMuteRequest
    );

    // POST /v1/profile/set_mute_cancel
    define_post_method!(
        set_mute_cancel,
        "/profile/set_mute_cancel",
        crate::model::ProfileSetMuteCancelRequest
    );

    // POST /v1/profile/set_my_design_card
    define_post_method!(
        set_my_design_card,
        "/profile/set_my_design_card",
        crate::model::ProfileSetMyDesignCardRequest
    );

    // POST /v1/profile/set_my_design_icon
    define_post_method!(
        set_my_design_icon,
        "/profile/set_my_design_icon",
        crate::model::ProfileSetMyDesignIconRequest
    );

    // POST /v1/profile/set_my_design_name
    define_post_method!(
        set_my_design_name,
        "/profile/set_my_design_name",
        crate::model::ProfileSetMyDesignNameRequest
    );

    // POST /v1/profile/set_name
    define_post_method!(
        set_name,
        "/profile/set_name",
        crate::model::ProfileSetNameRequest
    );

    // POST /v1/profile/set_profile_card
    define_post_method!(
        set_profile_card,
        "/profile/set_profile_card",
        crate::model::ProfileSetProfileCardRequest
    );

    // POST /v1/profile/set_profile_icon
    define_post_method!(
        set_profile_icon,
        "/profile/set_profile_icon",
        crate::model::ProfileSetProfileIconRequest
    );

    // POST /v1/profile/set_report
    define_post_method!(
        set_report,
        "/profile/set_report",
        crate::model::ProfileSetReportRequest
    );

    // POST /v1/profile/use_fanlevel_point_stocks
    define_post_method!(
        use_fanlevel_point_stocks,
        "/profile/use_fanlevel_point_stocks",
        crate::model::ProfileUseFanlevelPointStocksRequest
    );
}
