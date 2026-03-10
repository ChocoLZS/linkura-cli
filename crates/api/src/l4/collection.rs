use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(CollectionApi);

impl<'a> CollectionApi<'a> {
    // POST /v1/collection/get_character_info
    define_post_method!(
        get_character_info,
        "/collection/get_character_info",
        crate::model::CollectionGetCharacterInfoRequest
    );

    // POST /v1/collection/get_emoji_list
    define_post_method!(
        get_emoji_list,
        "/collection/get_emoji_list",
        crate::model::CollectionGetEmojiListRequest
    );

    // POST /v1/collection/get_gallary_list
    define_post_method!(
        get_gallary_list,
        "/collection/get_gallary_list",
        crate::model::CollectionGetGallaryListRequest
    );

    // POST /v1/collection/get_music_list
    define_post_method!(
        get_music_list,
        "/collection/get_music_list",
        crate::model::CollectionGetMusicListRequest
    );

    // POST /v1/collection/get_sticker_list
    define_post_method!(
        get_sticker_list,
        "/collection/get_sticker_list",
        crate::model::CollectionGetStickerListRequest
    );

    // POST /v1/collection/set_gallary_data
    define_post_method!(
        set_gallary_data,
        "/collection/set_gallary_data",
        crate::model::CollectionSetGallaryDataRequest
    );

    // POST /v1/collection/set_music_play
    define_post_method!(
        set_music_play,
        "/collection/set_music_play",
        crate::model::CollectionSetMusicPlayRequest
    );
}
