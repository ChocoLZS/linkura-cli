use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameApi);

impl<'a> RhythmGameApi<'a> {
    // POST /v1/rhythm_game/home
    define_post_method!(
        home,
        "/rhythm_game/home",
        crate::model::RhythmGameHomeRequest
    );

    // POST /v1/rhythm_game/receive_class_mission
    define_post_method!(
        receive_class_mission,
        "/rhythm_game/receive_class_mission",
        crate::model::RhythmGameReceiveClassMissionRequest
    );

    // POST /v1/rhythm_game/receive_total_mission
    define_post_method!(
        receive_total_mission,
        "/rhythm_game/receive_total_mission",
        crate::model::RhythmGameReceiveTotalMissionRequest
    );

    // POST /v1/rhythm_game/set_music
    define_post_method!(
        set_music,
        "/rhythm_game/set_music",
        crate::model::RhythmGameSetMusicRequest
    );
}
