use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameLiveApi);

impl<'a> RhythmGameLiveApi<'a> {
    // POST /v1/rhythm_game_live/set_finish
    define_post_method!(
        set_finish,
        "/rhythm_game_live/set_finish",
        crate::model::RhythmGameLiveSetFinishRequest
    );

    // POST /v1/rhythm_game_live/set_retire
    define_post_method!(
        set_retire,
        "/rhythm_game_live/set_retire",
        crate::model::RhythmGameLiveSetRetireRequest
    );

    // POST /v1/rhythm_game_live/set_start
    define_post_method!(
        set_start,
        "/rhythm_game_live/set_start",
        crate::model::RhythmGameLiveSetStartRequest
    );
}
