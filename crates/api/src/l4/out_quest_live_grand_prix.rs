use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGrandPrixApi);

impl<'a> OutQuestLiveGrandPrixApi<'a> {
    // POST /v1/out_quest_live/grand_prix/get_history
    define_post_method!(
        get_history,
        "/out_quest_live/grand_prix/get_history",
        crate::model::OutQuestLiveGrandPrixGetHistoryRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_ranking_list
    define_post_method!(
        get_ranking_list,
        "/out_quest_live/grand_prix/get_ranking_list",
        crate::model::OutQuestLiveGrandPrixGetRankingListRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/grand_prix/get_result",
        crate::model::OutQuestLiveGrandPrixGetResultRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_data
    define_post_method!(
        get_stage_data,
        "/out_quest_live/grand_prix/get_stage_data",
        crate::model::OutQuestLiveGrandPrixGetStageDataRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_list
    define_post_method!(
        get_stage_list,
        "/out_quest_live/grand_prix/get_stage_list",
        crate::model::OutQuestLiveGrandPrixGetStageListRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_select
    define_post_method!(
        get_stage_select,
        "/out_quest_live/grand_prix/get_stage_select",
        crate::model::OutQuestLiveGrandPrixGetStageSelectRequest
    );

    // POST /v1/out_quest_live/grand_prix/get_top_info
    define_post_method!(
        get_top_info,
        "/out_quest_live/grand_prix/get_top_info",
        crate::model::OutQuestLiveGrandPrixGetTopInfoRequest
    );
}
