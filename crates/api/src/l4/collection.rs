use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(CollectionApi);

impl<'a> CollectionApi<'a> {
    // POST /v1/collection/get_character_info
    post!(
        get_character_info,
        "/collection/get_character_info",
        crate::model::GetCharacterInfoRequest,
        crate::model::GetCharacterInfoResponse
    );

    // POST /v1/collection/get_emoji_list
    post!(
        get_emoji_list,
        "/collection/get_emoji_list",
        crate::model::GetEmojiListResponse
    );

    // POST /v1/collection/get_gallary_list
    post!(
        get_gallary_list,
        "/collection/get_gallary_list",
        crate::model::GetGallaryListResponse
    );

    // POST /v1/collection/get_music_list
    post!(
        get_music_list,
        "/collection/get_music_list",
        crate::model::GetMusicListResponse
    );

    // POST /v1/collection/get_sticker_list
    post!(
        get_sticker_list,
        "/collection/get_sticker_list",
        crate::model::GetStickerListResponse
    );

    // POST /v1/collection/set_gallary_data
    post!(
        set_gallary_data,
        "/collection/set_gallary_data",
        crate::model::SetGallaryDataRequest,
        crate::model::SetGallaryDataResponse
    );

    // POST /v1/collection/set_music_play
    post!(
        set_music_play,
        "/collection/set_music_play",
        crate::model::SetMusicPlayRequest,
        crate::model::SetMusicPlayResponse
    );
}




