use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGradeChallengeApi);

impl<'a> OutQuestLiveGradeChallengeApi<'a> {
    // POST /v1/out_quest_live/grade_challenge/get_quest_list
    define_post_method!(
        get_quest_list,
        "/out_quest_live/grade_challenge/get_quest_list",
        crate::model::OutQuestLiveGradeChallengeGetQuestListRequest
    );

    // POST /v1/out_quest_live/grade_challenge/get_ranking_list
    define_post_method!(
        get_ranking_list,
        "/out_quest_live/grade_challenge/get_ranking_list",
        crate::model::OutQuestLiveGradeChallengeGetRankingListRequest
    );

    // POST /v1/out_quest_live/grade_challenge/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/grade_challenge/get_result",
        crate::model::OutQuestLiveGradeChallengeGetResultRequest
    );
}
