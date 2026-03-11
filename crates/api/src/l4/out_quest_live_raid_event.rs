use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveRaidEventApi);

impl<'a> OutQuestLiveRaidEventApi<'a> {
    // POST /v1/out_quest_live/raid_event/get_result
    post!(
        get_result,
        "/out_quest_live/raid_event/get_result",
        crate::model::RaidEventGetResultRequest,
        crate::model::RaidEventGetResultResponse
    );

    // POST /v1/out_quest_live/raid_event/get_stage_data
    post!(
        get_stage_data,
        "/out_quest_live/raid_event/get_stage_data",
        crate::model::RaidEventGetStageDataRequest,
        crate::model::RaidEventGetStageDataResponse
    );

    // POST /v1/out_quest_live/raid_event/get_stage_list
    post!(
        get_stage_list,
        "/out_quest_live/raid_event/get_stage_list",
        crate::model::RaidEventGetStageListRequest,
        crate::model::RaidEventGetStageListResponse
    );

    // POST /v1/out_quest_live/raid_event/get_stamina_recovery_info
    post!(
        get_stamina_recovery_info,
        "/out_quest_live/raid_event/get_stamina_recovery_info",
        crate::model::RaidEventGetStaminaRecoveryInfoRequest,
        crate::model::RaidEventGetStaminaRecoveryInfoResponse
    );

    // POST /v1/out_quest_live/raid_event/get_top_info
    post!(
        get_top_info,
        "/out_quest_live/raid_event/get_top_info",
        crate::model::RaidEventGetTopInfoRequest,
        crate::model::RaidEventGetTopInfoResponse
    );

    // POST /v1/out_quest_live/raid_event/recovery_stamina
    post!(
        recovery_stamina,
        "/out_quest_live/raid_event/recovery_stamina",
        crate::model::RaidEventRecoveryStaminaRequest,
        crate::model::RaidEventRecoveryStaminaResponse
    );

    // POST /v1/out_quest_live/raid_event/set_join_message
    post!(
        set_join_message,
        "/out_quest_live/raid_event/set_join_message",
        crate::model::RaidEventSetJoinMessageRequest,
        serde_json::Value
    );

    // POST /v1/out_quest_live/raid_event/set_reward
    post!(
        set_reward,
        "/out_quest_live/raid_event/set_reward",
        crate::model::RaidEventSetRewardRequest,
        crate::model::RaidEventSetRewardResponse
    );
}




