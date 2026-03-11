use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveGrandPrixApi);

impl<'a> OutQuestLiveGrandPrixApi<'a> {
    // POST /v1/out_quest_live/grand_prix/get_history
    post!(
        get_history,
        "/out_quest_live/grand_prix/get_history",
        crate::model::GrandPrixGetHistoryRequest,
        crate::model::GrandPrixGetHistoryResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_ranking_list
    post!(
        get_ranking_list,
        "/out_quest_live/grand_prix/get_ranking_list",
        crate::model::GrandPrixGetRankingListRequest,
        crate::model::GrandPrixGetRankingListResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_result
    post!(
        get_result,
        "/out_quest_live/grand_prix/get_result",
        crate::model::GrandPrixGetResultRequest,
        crate::model::GrandPrixGetResultResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_data
    post!(
        get_stage_data,
        "/out_quest_live/grand_prix/get_stage_data",
        crate::model::GrandPrixGetStageDataRequest,
        crate::model::GrandPrixGetStageDataResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_list
    post!(
        get_stage_list,
        "/out_quest_live/grand_prix/get_stage_list",
        crate::model::GrandPrixGetStageListRequest,
        crate::model::GrandPrixGetStageListResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_stage_select
    post!(
        get_stage_select,
        "/out_quest_live/grand_prix/get_stage_select",
        crate::model::GrandPrixGetStageSelectRequest,
        crate::model::GrandPrixGetStageSelectResponse
    );

    // POST /v1/out_quest_live/grand_prix/get_top_info
    post!(
        get_top_info,
        "/out_quest_live/grand_prix/get_top_info",
        crate::model::GrandPrixGetTopInfoRequest,
        crate::model::GrandPrixGetTopInfoResponse
    );
}




