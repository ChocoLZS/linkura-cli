use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameGrandPrixApi);

impl<'a> RhythmGameGrandPrixApi<'a> {
    // POST /v1/rhythm_game_grand_prix/set_center
    define_post_method!(
        set_center,
        "/rhythm_game_grand_prix/set_center",
        crate::model::RhythmGameGrandPrixSetCenterRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_deck
    define_post_method!(
        set_deck,
        "/rhythm_game_grand_prix/set_deck",
        crate::model::RhythmGameGrandPrixSetDeckRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_finish_live
    define_post_method!(
        set_finish_live,
        "/rhythm_game_grand_prix/set_finish_live",
        crate::model::RhythmGameGrandPrixSetFinishLiveRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_position
    define_post_method!(
        set_position,
        "/rhythm_game_grand_prix/set_position",
        crate::model::RhythmGameGrandPrixSetPositionRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_reset
    define_post_method!(
        set_reset,
        "/rhythm_game_grand_prix/set_reset",
        crate::model::RhythmGameGrandPrixSetResetRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_retire_live
    define_post_method!(
        set_retire_live,
        "/rhythm_game_grand_prix/set_retire_live",
        crate::model::RhythmGameGrandPrixSetRetireLiveRequest
    );

    // POST /v1/rhythm_game_grand_prix/set_start_live
    define_post_method!(
        set_start_live,
        "/rhythm_game_grand_prix/set_start_live",
        crate::model::RhythmGameGrandPrixSetStartLiveRequest
    );

    // POST /v1/rhythm_game_grand_prix/top
    define_post_method!(
        top,
        "/rhythm_game_grand_prix/top",
        crate::model::RhythmGameGrandPrixTopRequest
    );
}
