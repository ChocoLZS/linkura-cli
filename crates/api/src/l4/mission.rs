use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(MissionApi);

impl<'a> MissionApi<'a> {
    // POST /v1/beginner_mission/get_list
    post!(
        beginner_mission_get_list,
        "/beginner_mission/get_list",
        crate::model::BeginnerMissionGetListResponse
    );

    // POST /v1/beginner_mission/set_banner_reward
    post!(
        beginner_mission_set_banner_reward,
        "/beginner_mission/set_banner_reward",
        crate::model::BeginnerMissionSetBannerRewardRequest,
        serde_json::Value
    );

    // POST /v1/beginner_mission/set_reward
    post!(
        beginner_mission_set_reward,
        "/beginner_mission/set_reward",
        crate::model::BeginnerMissionSetRewardRequest,
        serde_json::Value
    );

    // POST /v1/beginner_mission/set_reward_all
    post!(
        beginner_mission_set_reward_all,
        "/beginner_mission/set_reward_all",
        crate::model::BeginnerMissionSetRewardAllRequest,
        crate::model::BeginnerMissionSetRewardAllResponse
    );

    // POST /v1/mission/get_list
    post!(
        mission_get_list,
        "/mission/get_list",
        crate::model::MissionGetListResponse
    );

    // POST /v1/mission/receive_common_mission_reward
    post!(
        mission_receive_common_mission_reward,
        "/mission/receive_common_mission_reward",
        crate::model::ReceiveCommonMissionRewardResponse
    );

    // POST /v1/step_up_beginner_mission/get_list
    post!(
        step_up_beginner_mission_get_list,
        "/step_up_beginner_mission/get_list",
        crate::model::StepUpBeginnerMissionGetListResponse
    );

    // POST /v1/step_up_beginner_mission/set_reward
    post!(
        step_up_beginner_mission_set_reward,
        "/step_up_beginner_mission/set_reward",
        crate::model::StepUpBeginnerMissionSetRewardRequest,
        crate::model::StepUpBeginnerMissionSetRewardResponse
    );
}




