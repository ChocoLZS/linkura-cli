use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveDreamApi);

impl<'a> OutQuestLiveDreamApi<'a> {
    // POST /v1/out_quest_live/dream/get_member_select
    post!(
        get_member_select,
        "/out_quest_live/dream/get_member_select",
        crate::model::GetMemberSelectResponse
    );

    // POST /v1/out_quest_live/dream/get_result
    post!(
        get_result,
        "/out_quest_live/dream/get_result",
        crate::model::DreamGetResultRequest,
        crate::model::DreamGetResultResponse
    );

    // POST /v1/out_quest_live/dream/notify_member_release_confirm
    post!(
        notify_member_release_confirm,
        "/out_quest_live/dream/notify_member_release_confirm",
        serde_json::Value
    );

    // POST /v1/out_quest_live/dream/set_card
    post!(
        set_card,
        "/out_quest_live/dream/set_card",
        crate::model::DreamSetCardRequest,
        crate::model::DreamSetCardResponse
    );
}




