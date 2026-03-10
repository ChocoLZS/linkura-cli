use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(InQuestLiveApi);

impl<'a> InQuestLiveApi<'a> {
    // POST /v1/in_quest_live/get_live_info
    define_post_method!(
        get_live_info,
        "/in_quest_live/get_live_info",
        crate::model::InQuestLiveGetLiveInfoRequest
    );

    // POST /v1/in_quest_live/set_finish
    define_post_method!(
        set_finish,
        "/in_quest_live/set_finish",
        crate::model::InQuestLiveSetFinishRequest
    );

    // POST /v1/in_quest_live/set_retire
    define_post_method!(
        set_retire,
        "/in_quest_live/set_retire",
        crate::model::InQuestLiveSetRetireRequest
    );

    // POST /v1/in_quest_live/set_start
    define_post_method!(
        set_start,
        "/in_quest_live/set_start",
        crate::model::InQuestLiveSetStartRequest
    );

    // POST /v1/in_quest_live/skip_quest
    define_post_method!(
        skip_quest,
        "/in_quest_live/skip_quest",
        crate::model::InQuestLiveSkipQuestRequest
    );
}
