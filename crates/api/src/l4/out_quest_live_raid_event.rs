use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveRaidEventApi);

impl<'a> OutQuestLiveRaidEventApi<'a> {
    // POST /v1/out_quest_live/raid_event/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/raid_event/get_result",
        crate::model::OutQuestLiveRaidEventGetResultRequest
    );

    // POST /v1/out_quest_live/raid_event/get_stage_data
    define_post_method!(
        get_stage_data,
        "/out_quest_live/raid_event/get_stage_data",
        crate::model::OutQuestLiveRaidEventGetStageDataRequest
    );

    // POST /v1/out_quest_live/raid_event/get_stage_list
    define_post_method!(
        get_stage_list,
        "/out_quest_live/raid_event/get_stage_list",
        crate::model::OutQuestLiveRaidEventGetStageListRequest
    );

    // POST /v1/out_quest_live/raid_event/get_stamina_recovery_info
    define_post_method!(
        get_stamina_recovery_info,
        "/out_quest_live/raid_event/get_stamina_recovery_info",
        crate::model::OutQuestLiveRaidEventGetStaminaRecoveryInfoRequest
    );

    // POST /v1/out_quest_live/raid_event/get_top_info
    define_post_method!(
        get_top_info,
        "/out_quest_live/raid_event/get_top_info",
        crate::model::OutQuestLiveRaidEventGetTopInfoRequest
    );

    // POST /v1/out_quest_live/raid_event/recovery_stamina
    define_post_method!(
        recovery_stamina,
        "/out_quest_live/raid_event/recovery_stamina",
        crate::model::OutQuestLiveRaidEventRecoveryStaminaRequest
    );

    // POST /v1/out_quest_live/raid_event/set_join_message
    define_post_method!(
        set_join_message,
        "/out_quest_live/raid_event/set_join_message",
        crate::model::OutQuestLiveRaidEventSetJoinMessageRequest
    );

    // POST /v1/out_quest_live/raid_event/set_reward
    define_post_method!(
        set_reward,
        "/out_quest_live/raid_event/set_reward",
        crate::model::OutQuestLiveRaidEventSetRewardRequest
    );
}
