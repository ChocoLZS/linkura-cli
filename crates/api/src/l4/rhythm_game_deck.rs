use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameDeckApi);

impl<'a> RhythmGameDeckApi<'a> {
    // POST /v1/rhythm_game_deck/modify_deck_list
    post!(
        modify_deck_list,
        "/rhythm_game_deck/modify_deck_list",
        crate::model::RhythmGameDeckModifyDeckListRequest,
        serde_json::Value
    );

    // POST /v1/rhythm_game_deck/set_deck_name
    post!(
        set_deck_name,
        "/rhythm_game_deck/set_deck_name",
        crate::model::RhythmGameDeckSetDeckNameRequest,
        serde_json::Value
    );

    // POST /v1/rhythm_game_deck/set_reset_deck
    post!(
        set_reset_deck,
        "/rhythm_game_deck/set_reset_deck",
        crate::model::RhythmGameDeckSetResetRequest,
        serde_json::Value
    );
}




