use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveApi);

impl<'a> OutQuestLiveApi<'a> {
    // POST /v1/out_quest_live/get_live_setting
    define_post_method!(
        get_live_setting,
        "/out_quest_live/get_live_setting",
        crate::model::OutQuestLiveGetLiveSettingRequest
    );

    // POST /v1/out_quest_live/get_play_report
    define_post_method!(
        get_play_report,
        "/out_quest_live/get_play_report",
        crate::model::OutQuestLiveGetPlayReportRequest
    );

    // POST /v1/out_quest_live/get_quest_clear_status_list
    define_post_method!(
        get_quest_clear_status_list,
        "/out_quest_live/get_quest_clear_status_list",
        crate::model::OutQuestLiveGetQuestClearStatusListRequest
    );

    // POST /v1/out_quest_live/get_quest_list
    define_post_method!(
        get_quest_list,
        "/out_quest_live/get_quest_list",
        crate::model::OutQuestLiveGetQuestListRequest
    );

    // POST /v1/out_quest_live/get_quest_top
    define_post_method!(
        get_quest_top,
        "/out_quest_live/get_quest_top",
        crate::model::OutQuestLiveGetQuestTopRequest
    );

    // POST /v1/out_quest_live/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/get_result",
        crate::model::OutQuestLiveGetResultRequest
    );

    // POST /v1/out_quest_live/get_stamina_recovery_info
    define_post_method!(
        get_stamina_recovery_info,
        "/out_quest_live/get_stamina_recovery_info",
        crate::model::OutQuestLiveGetStaminaRecoveryInfoRequest
    );

    // POST /v1/out_quest_live/recovery_stamina
    define_post_method!(
        recovery_stamina,
        "/out_quest_live/recovery_stamina",
        crate::model::OutQuestLiveRecoveryStaminaRequest
    );

    // POST /v1/out_quest_live/set_live_setting
    define_post_method!(
        set_live_setting,
        "/out_quest_live/set_live_setting",
        crate::model::OutQuestLiveSetLiveSettingRequest
    );

    // POST /v1/out_quest_live/standard/get_area_select
    define_post_method!(
        standard_get_area_select,
        "/out_quest_live/standard/get_area_select",
        crate::model::OutQuestLiveStandardGetAreaSelectRequest
    );

    // POST /v1/out_quest_live/standard/get_stage_data
    define_post_method!(
        standard_get_stage_data,
        "/out_quest_live/standard/get_stage_data",
        crate::model::OutQuestLiveStandardGetStageDataRequest
    );

    // POST /v1/out_quest_live/standard/get_stage_select
    define_post_method!(
        standard_get_stage_select,
        "/out_quest_live/standard/get_stage_select",
        crate::model::OutQuestLiveStandardGetStageSelectRequest
    );

    // POST /v1/out_quest_live/standard/set_area_select_view_hist
    define_post_method!(
        standard_set_area_select_view_hist,
        "/out_quest_live/standard/set_area_select_view_hist",
        crate::model::OutQuestLiveStandardSetAreaSelectViewHistRequest
    );
}
