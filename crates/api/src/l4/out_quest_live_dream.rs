use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveDreamApi);

impl<'a> OutQuestLiveDreamApi<'a> {
    // POST /v1/out_quest_live/dream/get_member_select
    define_post_method!(
        get_member_select,
        "/out_quest_live/dream/get_member_select",
        crate::model::OutQuestLiveDreamGetMemberSelectRequest
    );

    // POST /v1/out_quest_live/dream/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/dream/get_result",
        crate::model::OutQuestLiveDreamGetResultRequest
    );

    // POST /v1/out_quest_live/dream/notify_member_release_confirm
    define_post_method!(
        notify_member_release_confirm,
        "/out_quest_live/dream/notify_member_release_confirm",
        crate::model::OutQuestLiveDreamNotifyMemberReleaseConfirmRequest
    );

    // POST /v1/out_quest_live/dream/set_card
    define_post_method!(
        set_card,
        "/out_quest_live/dream/set_card",
        crate::model::OutQuestLiveDreamSetCardRequest
    );
}
