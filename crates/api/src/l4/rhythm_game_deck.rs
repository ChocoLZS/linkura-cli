use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameDeckApi);

impl<'a> RhythmGameDeckApi<'a> {
    // POST /v1/rhythm_game_deck/modify_deck_list
    define_post_method!(
        modify_deck_list,
        "/rhythm_game_deck/modify_deck_list",
        crate::model::RhythmGameDeckModifyDeckListRequest
    );

    // POST /v1/rhythm_game_deck/set_deck_name
    define_post_method!(
        set_deck_name,
        "/rhythm_game_deck/set_deck_name",
        crate::model::RhythmGameDeckSetDeckNameRequest
    );

    // POST /v1/rhythm_game_deck/set_reset_deck
    define_post_method!(
        set_reset_deck,
        "/rhythm_game_deck/set_reset_deck",
        crate::model::RhythmGameDeckSetResetDeckRequest
    );
}
