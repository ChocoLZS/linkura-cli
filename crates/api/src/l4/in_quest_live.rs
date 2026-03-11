use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(InQuestLiveApi);

impl<'a> InQuestLiveApi<'a> {
    // POST /v1/in_quest_live/get_live_info
    post!(
        get_live_info,
        "/in_quest_live/get_live_info",
        crate::model::GetLiveInfoRequest,
        crate::model::GetLiveInfoResponse
    );

    // POST /v1/in_quest_live/set_finish
    post!(
        set_finish,
        "/in_quest_live/set_finish",
        crate::model::SetFinishRequest,
        crate::model::SetFinishResponse
    );

    // POST /v1/in_quest_live/set_retire
    post!(
        set_retire,
        "/in_quest_live/set_retire",
        crate::model::SetRetireRequest,
        crate::model::SetRetireResponse
    );

    // POST /v1/in_quest_live/set_start
    post!(
        set_start,
        "/in_quest_live/set_start",
        crate::model::SetStartRequest,
        crate::model::SetStartResponse
    );

    // POST /v1/in_quest_live/skip_quest
    post!(
        skip_quest,
        "/in_quest_live/skip_quest",
        crate::model::SkipQuestRequest,
        crate::model::SkipQuestResponse
    );
}




