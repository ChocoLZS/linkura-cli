use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(OutQuestLiveMusicLearningApi);

impl<'a> OutQuestLiveMusicLearningApi<'a> {
    // POST /v1/out_quest_live/music_learning/get_music_select
    post!(
        get_music_select,
        "/out_quest_live/music_learning/get_music_select",
        crate::model::GetMusicSelectResponse
    );

    // POST /v1/out_quest_live/music_learning/get_result
    post!(
        get_result,
        "/out_quest_live/music_learning/get_result",
        crate::model::MusicLearningGetResultRequest,
        crate::model::MusicLearningGetResultResponse
    );

    // POST /v1/out_quest_live/music_learning/set_music
    post!(
        set_music,
        "/out_quest_live/music_learning/set_music",
        crate::model::MusicLearningSetMusicRequest,
        crate::model::MusicLearningSetMusicResponse
    );
}




