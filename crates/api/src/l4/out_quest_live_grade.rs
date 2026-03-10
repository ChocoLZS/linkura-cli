use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGradeApi);

impl<'a> OutQuestLiveGradeApi<'a> {
    // POST /v1/out_quest_live/grade/get_live_list
    define_post_method!(
        get_live_list,
        "/out_quest_live/grade/get_live_list",
        crate::model::OutQuestLiveGradeGetLiveListRequest
    );

    // POST /v1/out_quest_live/grade/get_quest_list
    define_post_method!(
        get_quest_list,
        "/out_quest_live/grade/get_quest_list",
        crate::model::OutQuestLiveGradeGetQuestListRequest
    );

    // POST /v1/out_quest_live/grade/get_rank_list
    define_post_method!(
        get_rank_list,
        "/out_quest_live/grade/get_rank_list",
        crate::model::OutQuestLiveGradeGetRankListRequest
    );

    // POST /v1/out_quest_live/grade/get_ranking_list
    define_post_method!(
        get_ranking_list,
        "/out_quest_live/grade/get_ranking_list",
        crate::model::OutQuestLiveGradeGetRankingListRequest
    );

    // POST /v1/out_quest_live/grade/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/grade/get_result",
        crate::model::OutQuestLiveGradeGetResultRequest
    );

    // POST /v1/out_quest_live/grade/get_stage_data
    define_post_method!(
        get_stage_data,
        "/out_quest_live/grade/get_stage_data",
        crate::model::OutQuestLiveGradeGetStageDataRequest
    );

    // POST /v1/out_quest_live/grade/get_stage_list
    define_post_method!(
        get_stage_list,
        "/out_quest_live/grade/get_stage_list",
        crate::model::OutQuestLiveGradeGetStageListRequest
    );

    // POST /v1/out_quest_live/grade/set_quest_action
    define_post_method!(
        set_quest_action,
        "/out_quest_live/grade/set_quest_action",
        crate::model::OutQuestLiveGradeSetQuestActionRequest
    );

    // POST /v1/out_quest_live/grade/set_quest_add_skill
    define_post_method!(
        set_quest_add_skill,
        "/out_quest_live/grade/set_quest_add_skill",
        crate::model::OutQuestLiveGradeSetQuestAddSkillRequest
    );

    // POST /v1/out_quest_live/grade/set_quest_retire
    define_post_method!(
        set_quest_retire,
        "/out_quest_live/grade/set_quest_retire",
        crate::model::OutQuestLiveGradeSetQuestRetireRequest
    );

    // POST /v1/out_quest_live/grade/set_quest_start
    define_post_method!(
        set_quest_start,
        "/out_quest_live/grade/set_quest_start",
        crate::model::OutQuestLiveGradeSetQuestStartRequest
    );

    // POST /v1/out_quest_live/grade/set_reward
    define_post_method!(
        set_reward,
        "/out_quest_live/grade/set_reward",
        crate::model::OutQuestLiveGradeSetRewardRequest
    );
}
