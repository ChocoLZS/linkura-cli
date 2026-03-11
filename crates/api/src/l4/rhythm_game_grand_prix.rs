use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameGrandPrixApi);

impl<'a> RhythmGameGrandPrixApi<'a> {
    // POST /v1/rhythm_game_grand_prix/set_center
    post!(
        set_center,
        "/rhythm_game_grand_prix/set_center",
        crate::model::RhythmGameGrandPrixSetCenterRequest,
        crate::model::RhythmGameGrandPrixSetCenterResponse
    );

    // POST /v1/rhythm_game_grand_prix/set_deck
    post!(
        set_deck,
        "/rhythm_game_grand_prix/set_deck",
        crate::model::RhythmGameGrandPrixSetDeckRequest,
        crate::model::RhythmGameGrandPrixSetDeckResponse
    );

    // POST /v1/rhythm_game_grand_prix/set_finish_live
    post!(
        set_finish_live,
        "/rhythm_game_grand_prix/set_finish_live",
        crate::model::RhythmGameGrandPrixSetFinishLiveRequest,
        crate::model::RhythmGameGrandPrixSetFinishLiveResponse
    );

    // POST /v1/rhythm_game_grand_prix/set_position
    post!(
        set_position,
        "/rhythm_game_grand_prix/set_position",
        crate::model::RhythmGameGrandPrixSetPositionRequest,
        crate::model::RhythmGameGrandPrixSetPositionResponse
    );

    // POST /v1/rhythm_game_grand_prix/set_reset
    post!(
        set_reset,
        "/rhythm_game_grand_prix/set_reset",
        crate::model::RhythmGameGrandPrixSetResetRequest,
        serde_json::Value
    );

    // POST /v1/rhythm_game_grand_prix/set_retire_live
    post!(
        set_retire_live,
        "/rhythm_game_grand_prix/set_retire_live",
        serde_json::Value
    );

    // POST /v1/rhythm_game_grand_prix/set_start_live
    post!(
        set_start_live,
        "/rhythm_game_grand_prix/set_start_live",
        crate::model::RhythmGameGrandPrixSetStartLiveRequest,
        serde_json::Value
    );

    // POST /v1/rhythm_game_grand_prix/top
    post!(
        top,
        "/rhythm_game_grand_prix/top",
        crate::model::RhythmGameGrandPrixTopRequest,
        crate::model::RhythmGameGrandPrixTopResponse
    );
}




