use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveApi);

impl<'a> OutQuestLiveApi<'a> {
    // POST /v1/out_quest_live/get_live_setting
    post!(
        get_live_setting,
        "/out_quest_live/get_live_setting",
        crate::model::GetLiveSettingRequest,
        crate::model::GetLiveSettingResponse
    );

    // POST /v1/out_quest_live/get_play_report
    post!(
        get_play_report,
        "/out_quest_live/get_play_report",
        crate::model::GetPlayReportRequest,
        crate::model::GetPlayReportResponse
    );

    // POST /v1/out_quest_live/get_quest_clear_status_list
    post!(
        get_quest_clear_status_list,
        "/out_quest_live/get_quest_clear_status_list",
        crate::model::GetQuestClearStatusListRequest,
        crate::model::GetQuestClearStatusListResponse
    );

    // POST /v1/out_quest_live/get_quest_list
    post!(
        get_quest_list,
        "/out_quest_live/get_quest_list",
        crate::model::GetQuestListResponse
    );

    // POST /v1/out_quest_live/get_quest_top
    post!(
        get_quest_top,
        "/out_quest_live/get_quest_top",
        crate::model::GetQuestTopResponse
    );

    // POST /v1/out_quest_live/get_result
    post!(
        get_result,
        "/out_quest_live/get_result",
        crate::model::GetResultRequest,
        crate::model::GetResultResponse
    );

    // POST /v1/out_quest_live/get_stamina_recovery_info
    post!(
        get_stamina_recovery_info,
        "/out_quest_live/get_stamina_recovery_info",
        crate::model::GetStaminaRecoveryInfoResponse
    );

    // POST /v1/out_quest_live/recovery_stamina
    post!(
        recovery_stamina,
        "/out_quest_live/recovery_stamina",
        crate::model::RecoveryStaminaRequest,
        crate::model::RecoveryStaminaResponse
    );

    // POST /v1/out_quest_live/set_live_setting
    post!(
        set_live_setting,
        "/out_quest_live/set_live_setting",
        crate::model::SetLiveSettingRequest,
        crate::model::GetLiveInfoResponse
    );

    // POST /v1/out_quest_live/standard/get_area_select
    post!(
        standard_get_area_select,
        "/out_quest_live/standard/get_area_select",
        crate::model::GetAreaSelectResponse
    );

    // POST /v1/out_quest_live/standard/get_stage_data
    post!(
        standard_get_stage_data,
        "/out_quest_live/standard/get_stage_data",
        crate::model::GetStageDataRequest,
        crate::model::GetStageDataResponse
    );

    // POST /v1/out_quest_live/standard/get_stage_select
    post!(
        standard_get_stage_select,
        "/out_quest_live/standard/get_stage_select",
        crate::model::GetStageSelectRequest,
        crate::model::GetStageSelectResponse
    );

    // POST /v1/out_quest_live/standard/set_area_select_view_hist
    post!(
        standard_set_area_select_view_hist,
        "/out_quest_live/standard/set_area_select_view_hist",
        serde_json::Value
    );
}




