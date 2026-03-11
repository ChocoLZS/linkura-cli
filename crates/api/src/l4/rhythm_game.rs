use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(RhythmGameApi);

impl<'a> RhythmGameApi<'a> {
    // POST /v1/rhythm_game/home
    post!(
        home,
        "/rhythm_game/home",
        crate::model::RhythmGameHomeGetResponse
    );

    // POST /v1/rhythm_game/receive_class_mission
    post!(
        receive_class_mission,
        "/rhythm_game/receive_class_mission",
        crate::model::RhythmGameReceiveClassMissionRequest,
        crate::model::RhythmGameReceiveClassMissionResponse
    );

    // POST /v1/rhythm_game/receive_total_mission
    post!(
        receive_total_mission,
        "/rhythm_game/receive_total_mission",
        crate::model::RhythmGameReceiveTotalMissionResponse
    );

    // POST /v1/rhythm_game/set_music
    post!(
        set_music,
        "/rhythm_game/set_music",
        crate::model::RhythmGameSetMusicRequest,
        crate::model::RhythmGameSetMusicResponse
    );
}




