use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGradeApi);

impl<'a> OutQuestLiveGradeApi<'a> {
    // POST /v1/out_quest_live/grade/get_live_list
    post!(
        get_live_list,
        "/out_quest_live/grade/get_live_list",
        crate::model::GradeGetLiveListResponse
    );

    // POST /v1/out_quest_live/grade/get_quest_list
    post!(
        get_quest_list,
        "/out_quest_live/grade/get_quest_list",
        crate::model::GradeGetQuestListResponse
    );

    // POST /v1/out_quest_live/grade/get_rank_list
    post!(
        get_rank_list,
        "/out_quest_live/grade/get_rank_list",
        crate::model::GradeGetRankListRequest,
        crate::model::GradeGetRankListResponse
    );

    // POST /v1/out_quest_live/grade/get_ranking_list
    post!(
        get_ranking_list,
        "/out_quest_live/grade/get_ranking_list",
        crate::model::GradeGetRankingListRequest,
        crate::model::GradeGetRankingListResponse
    );

    // POST /v1/out_quest_live/grade/get_result
    post!(
        get_result,
        "/out_quest_live/grade/get_result",
        crate::model::GradeGetResultRequest,
        crate::model::GradeGetResultResponse
    );

    // POST /v1/out_quest_live/grade/get_stage_data
    post!(
        get_stage_data,
        "/out_quest_live/grade/get_stage_data",
        crate::model::GradeGetStageDataRequest,
        crate::model::GradeGetStageDataResponse
    );

    // POST /v1/out_quest_live/grade/get_stage_list
    post!(
        get_stage_list,
        "/out_quest_live/grade/get_stage_list",
        crate::model::GradeGetStageListRequest,
        crate::model::GradeGetStageListResponse
    );

    // POST /v1/out_quest_live/grade/set_quest_action
    post!(
        set_quest_action,
        "/out_quest_live/grade/set_quest_action",
        crate::model::GradeSetQuestActionRequest,
        crate::model::GradeSetQuestActionResponse
    );

    // POST /v1/out_quest_live/grade/set_quest_add_skill
    post!(
        set_quest_add_skill,
        "/out_quest_live/grade/set_quest_add_skill",
        crate::model::GradeSetQuestAddSkillRequest,
        crate::model::GradeSetQuestAddSkillResponse
    );

    // POST /v1/out_quest_live/grade/set_quest_retire
    post!(
        set_quest_retire,
        "/out_quest_live/grade/set_quest_retire",
        crate::model::GradeSetQuestRetireRequest,
        crate::model::GradeSetQuestRetireResponse
    );

    // POST /v1/out_quest_live/grade/set_quest_start
    post!(
        set_quest_start,
        "/out_quest_live/grade/set_quest_start",
        crate::model::GradeSetQuestStartRequest,
        crate::model::GradeSetQuestStartResponse
    );

    // POST /v1/out_quest_live/grade/set_reward
    post!(
        set_reward,
        "/out_quest_live/grade/set_reward",
        crate::model::GradeSetRewardResponse
    );
}




