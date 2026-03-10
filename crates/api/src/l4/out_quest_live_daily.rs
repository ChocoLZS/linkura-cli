use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveDailyApi);

impl<'a> OutQuestLiveDailyApi<'a> {
    // POST /v1/out_quest_live/daily/get_recovery_challenge_count
    define_post_method!(
        get_recovery_challenge_count,
        "/out_quest_live/daily/get_recovery_challenge_count",
        crate::model::OutQuestLiveDailyGetRecoveryChallengeCountRequest
    );

    // POST /v1/out_quest_live/daily/get_release_ticket
    define_post_method!(
        get_release_ticket,
        "/out_quest_live/daily/get_release_ticket",
        crate::model::OutQuestLiveDailyGetReleaseTicketRequest
    );

    // POST /v1/out_quest_live/daily/get_stage_data
    define_post_method!(
        get_stage_data,
        "/out_quest_live/daily/get_stage_data",
        crate::model::OutQuestLiveDailyGetStageDataRequest
    );

    // POST /v1/out_quest_live/daily/get_stage_list
    define_post_method!(
        get_stage_list,
        "/out_quest_live/daily/get_stage_list",
        crate::model::OutQuestLiveDailyGetStageListRequest
    );

    // POST /v1/out_quest_live/daily/get_stage_select
    define_post_method!(
        get_stage_select,
        "/out_quest_live/daily/get_stage_select",
        crate::model::OutQuestLiveDailyGetStageSelectRequest
    );

    // POST /v1/out_quest_live/daily/recovery_challenge_count
    define_post_method!(
        recovery_challenge_count,
        "/out_quest_live/daily/recovery_challenge_count",
        crate::model::OutQuestLiveDailyRecoveryChallengeCountRequest
    );

    // POST /v1/out_quest_live/daily/set_release
    define_post_method!(
        set_release,
        "/out_quest_live/daily/set_release",
        crate::model::OutQuestLiveDailySetReleaseRequest
    );
}
