use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGradeChallengeApi);

impl<'a> OutQuestLiveGradeChallengeApi<'a> {
    // POST /v1/out_quest_live/grade_challenge/get_quest_list
    post!(
        get_quest_list,
        "/out_quest_live/grade_challenge/get_quest_list",
        crate::model::GradeChallengeGetQuestListRequest,
        crate::model::GradeChallengeGetQuestListResponse
    );

    // POST /v1/out_quest_live/grade_challenge/get_ranking_list
    post!(
        get_ranking_list,
        "/out_quest_live/grade_challenge/get_ranking_list",
        crate::model::GradeChallengeGetRankingListRequest,
        crate::model::GradeChallengeGetRankingListResponse
    );

    // POST /v1/out_quest_live/grade_challenge/get_result
    post!(
        get_result,
        "/out_quest_live/grade_challenge/get_result",
        crate::model::GradeChallengeGetResultRequest,
        crate::model::GradeChallengeGetResultResponse
    );
}




