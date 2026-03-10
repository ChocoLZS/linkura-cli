use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveMusicLearningApi);

impl<'a> OutQuestLiveMusicLearningApi<'a> {
    // POST /v1/out_quest_live/music_learning/get_music_select
    define_post_method!(
        get_music_select,
        "/out_quest_live/music_learning/get_music_select",
        crate::model::OutQuestLiveMusicLearningGetMusicSelectRequest
    );

    // POST /v1/out_quest_live/music_learning/get_result
    define_post_method!(
        get_result,
        "/out_quest_live/music_learning/get_result",
        crate::model::OutQuestLiveMusicLearningGetResultRequest
    );

    // POST /v1/out_quest_live/music_learning/set_music
    define_post_method!(
        set_music,
        "/out_quest_live/music_learning/set_music",
        crate::model::OutQuestLiveMusicLearningSetMusicRequest
    );
}
