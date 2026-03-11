use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(ProfileApi);

impl<'a> ProfileApi<'a> {
    // POST /v1/profile/delete_my_design
    post!(
        delete_my_design,
        "/profile/delete_my_design",
        crate::model::ProfileDeleteMyDesignRequest,
        crate::model::ProfileDeleteMyDesignResponse
    );

    // POST /v1/profile/get_fan_level_info
    post!(
        get_fan_level_info,
        "/profile/get_fan_level_info",
        crate::model::ProfileGetFanLevelInfoRequest,
        crate::model::ProfileGetFanLevelInfoResponse
    );

    // POST /v1/profile/get_info
    post!(
        get_info,
        "/profile/get_info",
        crate::model::ProfileGetInfoRequest,
        crate::model::ProfileGetInfoResponse
    );

    // POST /v1/profile/get_mute_list
    post!(
        get_mute_list,
        "/profile/get_mute_list",
        crate::model::ProfileGetMuteListResponse
    );

    // POST /v1/profile/get_my_design_card_list
    post!(
        get_my_design_card_list,
        "/profile/get_my_design_card_list",
        crate::model::ProfileGetMyDesignCardListResponse
    );

    // POST /v1/profile/get_my_design_icon
    post!(
        get_my_design_icon,
        "/profile/get_my_design_icon",
        crate::model::ProfileGetMyDesignIconRequest,
        crate::model::ProfileGetMyDesignIconResponse
    );

    // POST /v1/profile/get_my_design_icon_list
    post!(
        get_my_design_icon_list,
        "/profile/get_my_design_icon_list",
        crate::model::ProfileGetMyDesignIconListResponse
    );

    // POST /v1/profile/get_profile_card
    post!(
        get_profile_card,
        "/profile/get_profile_card",
        crate::model::ProfileGetProfileCardResponse
    );

    // POST /v1/profile/get_profile_icon
    post!(
        get_profile_icon,
        "/profile/get_profile_icon",
        crate::model::ProfileGetProfileIconResponse
    );

    // POST /v1/profile/set_birthday
    post!(
        set_birthday,
        "/profile/set_birthday",
        crate::model::ProfileSetBirthdayRequest,
        crate::model::ProfileSetBirthdayResponse
    );

    // POST /v1/profile/set_comment
    post!(
        set_comment,
        "/profile/set_comment",
        crate::model::ProfileSetCommentRequest,
        crate::model::ProfileSetCommentResponse
    );

    // POST /v1/profile/set_mute
    post!(
        set_mute,
        "/profile/set_mute",
        crate::model::ProfileSetMuteRequest,
        crate::model::ProfileSetMuteResponse
    );

    // POST /v1/profile/set_mute_cancel
    post!(
        set_mute_cancel,
        "/profile/set_mute_cancel",
        crate::model::ProfileSetMuteCancelRequest,
        crate::model::ProfileSetMuteCancelResponse
    );

    // POST /v1/profile/set_my_design_card
    post!(
        set_my_design_card,
        "/profile/set_my_design_card",
        crate::model::ProfileSetMyDesignCardRequest,
        crate::model::ProfileSetMyDesignCardResponse
    );

    // POST /v1/profile/set_my_design_icon
    post!(
        set_my_design_icon,
        "/profile/set_my_design_icon",
        crate::model::ProfileSetMyDesignIconRequest,
        crate::model::ProfileSetMyDesignIconResponse
    );

    // POST /v1/profile/set_my_design_name
    post!(
        set_my_design_name,
        "/profile/set_my_design_name",
        crate::model::ProfileSetMyDesignNameRequest,
        crate::model::ProfileSetMyDesignNameResponse
    );

    // POST /v1/profile/set_name
    post!(
        set_name,
        "/profile/set_name",
        crate::model::ProfileSetNameRequest,
        crate::model::ProfileSetNameResponse
    );

    // POST /v1/profile/set_profile_card
    post!(
        set_profile_card,
        "/profile/set_profile_card",
        crate::model::ProfileSetProfileCardRequest,
        crate::model::ProfileSetProfileCardResponse
    );

    // POST /v1/profile/set_profile_icon
    post!(
        set_profile_icon,
        "/profile/set_profile_icon",
        crate::model::ProfileSetProfileIconRequest,
        crate::model::ProfileSetProfileIconResponse
    );

    // POST /v1/profile/set_report
    post!(
        set_report,
        "/profile/set_report",
        crate::model::ProfileSetReportRequest,
        crate::model::ProfileSetReportResponse
    );

    // POST /v1/profile/use_fanlevel_point_stocks
    post!(
        use_fanlevel_point_stocks,
        "/profile/use_fanlevel_point_stocks",
        crate::model::ProfileUseFanLevelPointStocksRequest,
        crate::model::ProfileUseFanLevelPointStocksResponse
    );
}




