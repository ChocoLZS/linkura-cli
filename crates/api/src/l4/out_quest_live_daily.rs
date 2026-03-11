use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveDailyApi);

impl<'a> OutQuestLiveDailyApi<'a> {
    // POST /v1/out_quest_live/daily/get_recovery_challenge_count
    post!(
        get_recovery_challenge_count,
        "/out_quest_live/daily/get_recovery_challenge_count",
        crate::model::DailyGetRecoveryChallengeCountResponse
    );

    // POST /v1/out_quest_live/daily/get_release_ticket
    post!(
        get_release_ticket,
        "/out_quest_live/daily/get_release_ticket",
        crate::model::DailySeriesCommonRequest,
        crate::model::DailyGetReleaseTicketResponse
    );

    // POST /v1/out_quest_live/daily/get_stage_data
    post!(
        get_stage_data,
        "/out_quest_live/daily/get_stage_data",
        crate::model::DailyGetStageDataRequest,
        crate::model::DailyGetStageDataResponse
    );

    // POST /v1/out_quest_live/daily/get_stage_list
    post!(
        get_stage_list,
        "/out_quest_live/daily/get_stage_list",
        crate::model::DailyGetStageListRequest,
        crate::model::DailyGetStageListResponse
    );

    // POST /v1/out_quest_live/daily/get_stage_select
    post!(
        get_stage_select,
        "/out_quest_live/daily/get_stage_select",
        crate::model::DailyGetStageSelectResponse
    );

    // POST /v1/out_quest_live/daily/recovery_challenge_count
    post!(
        recovery_challenge_count,
        "/out_quest_live/daily/recovery_challenge_count",
        crate::model::DailyRecoveryChallengeCountRequest,
        crate::model::DailyRecoveryChallengeCountResponse
    );

    // POST /v1/out_quest_live/daily/set_release
    post!(
        set_release,
        "/out_quest_live/daily/set_release",
        crate::model::DailySeriesCommonRequest,
        crate::model::DailySetReleaseResponse
    );
}




