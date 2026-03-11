use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameLiveApi);

impl<'a> RhythmGameLiveApi<'a> {
    // POST /v1/rhythm_game_live/set_finish
    post!(
        set_finish,
        "/rhythm_game_live/set_finish",
        crate::model::RhythmGameLiveSetFinishRequest,
        crate::model::RhythmGameLiveSetFinishResponse
    );

    // POST /v1/rhythm_game_live/set_retire
    post!(
        set_retire,
        "/rhythm_game_live/set_retire",
        serde_json::Value
    );

    // POST /v1/rhythm_game_live/set_start
    post!(
        set_start,
        "/rhythm_game_live/set_start",
        crate::model::RhythmGameLiveSetStartRequest,
        serde_json::Value
    );
}




