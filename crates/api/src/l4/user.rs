use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(UserApi);

impl<'a> UserApi<'a> {
    // POST /v1/user/card/check_evolution
    define_post_method!(
        card_check_evolution,
        "/user/card/check_evolution",
        crate::model::UserCardCheckEvolutionRequest
    );

    // POST /v1/user/card/check_limit_break
    define_post_method!(
        card_check_limit_break,
        "/user/card/check_limit_break",
        crate::model::UserCardCheckLimitBreakRequest
    );

    // POST /v1/user/card/check_skill_level_up
    define_post_method!(
        card_check_skill_level_up,
        "/user/card/check_skill_level_up",
        crate::model::UserCardCheckSkillLevelUpRequest
    );

    // POST /v1/user/card/check_style_level_up
    define_post_method!(
        card_check_style_level_up,
        "/user/card/check_style_level_up",
        crate::model::UserCardCheckStyleLevelUpRequest
    );

    // POST /v1/user/card/evolution
    define_post_method!(
        card_evolution,
        "/user/card/evolution",
        crate::model::UserCardEvolutionRequest
    );

    // POST /v1/user/card/get_detail
    define_post_method!(
        card_get_detail,
        "/user/card/get_detail",
        crate::model::UserCardGetDetailRequest
    );

    // POST /v1/user/card/get_detail_rental
    define_post_method!(
        card_get_detail_rental,
        "/user/card/get_detail_rental",
        crate::model::UserCardGetDetailRentalRequest
    );

    // POST /v1/user/card/get_list
    define_post_method!(
        card_get_list,
        "/user/card/get_list",
        crate::model::UserCardGetListRequest
    );

    // POST /v1/user/card/limit_break
    define_post_method!(
        card_limit_break,
        "/user/card/limit_break",
        crate::model::UserCardLimitBreakRequest
    );

    // POST /v1/user/card/rhythm_game_skill_level_up
    define_post_method!(
        card_rhythm_game_skill_level_up,
        "/user/card/rhythm_game_skill_level_up",
        crate::model::UserCardRhythmGameSkillLevelUpRequest
    );

    // POST /v1/user/card/skill_level_up
    define_post_method!(
        card_skill_level_up,
        "/user/card/skill_level_up",
        crate::model::UserCardSkillLevelUpRequest
    );

    // POST /v1/user/card/style_level_up
    define_post_method!(
        card_style_level_up,
        "/user/card/style_level_up",
        crate::model::UserCardStyleLevelUpRequest
    );

    // POST /v1/user/deck/get_card_list
    define_post_method!(
        deck_get_card_list,
        "/user/deck/get_card_list",
        crate::model::UserDeckGetCardListRequest
    );

    // POST /v1/user/deck/get_list
    define_post_method!(
        deck_get_list,
        "/user/deck/get_list",
        crate::model::UserDeckGetListRequest
    );

    // POST /v1/user/deck/modify_deck_list
    define_post_method!(
        deck_modify_deck_list,
        "/user/deck/modify_deck_list",
        crate::model::UserDeckModifyDeckListRequest
    );

    // POST /v1/user/deck/notify_auto_deck
    define_post_method!(
        deck_notify_auto_deck,
        "/user/deck/notify_auto_deck",
        crate::model::UserDeckNotifyAutoDeckRequest
    );

    // POST /v1/user/deck/remove_side_style
    define_post_method!(
        deck_remove_side_style,
        "/user/deck/remove_side_style",
        crate::model::UserDeckRemoveSideStyleRequest
    );

    // POST /v1/user/deck/set_copy_deck
    define_post_method!(
        deck_set_copy_deck,
        "/user/deck/set_copy_deck",
        crate::model::UserDeckSetCopyDeckRequest
    );

    // POST /v1/user/deck/set_deck
    define_post_method!(
        deck_set_deck,
        "/user/deck/set_deck",
        crate::model::UserDeckSetDeckRequest
    );

    // POST /v1/user/deck/set_delete_deck
    define_post_method!(
        deck_set_delete_deck,
        "/user/deck/set_delete_deck",
        crate::model::UserDeckSetDeleteDeckRequest
    );

    // POST /v1/user/deck/set_position
    define_post_method!(
        deck_set_position,
        "/user/deck/set_position",
        crate::model::UserDeckSetPositionRequest
    );

    // POST /v1/user/deck/set_side_style
    define_post_method!(
        deck_set_side_style,
        "/user/deck/set_side_style",
        crate::model::UserDeckSetSideStyleRequest
    );

    // POST /v1/user/item/get_detail
    define_post_method!(
        item_get_detail,
        "/user/item/get_detail",
        crate::model::UserItemGetDetailRequest
    );

    // POST /v1/user/items/get_list
    define_post_method!(
        items_get_list,
        "/user/items/get_list",
        crate::model::UserItemsGetListRequest
    );

    // POST /v1/user/jewel/get_history
    define_post_method!(
        jewel_get_history,
        "/user/jewel/get_history",
        crate::model::UserJewelGetHistoryRequest
    );

    // POST /v1/user/login
    define_post_method!(login, "/user/login", crate::model::UserLoginRequest);

    // POST /v1/user/push/device
    define_post_method!(
        push_device,
        "/user/push/device",
        crate::model::UserPushDeviceRequest
    );

    // POST /v1/user/push/devices
    define_post_method!(
        push_devices,
        "/user/push/devices",
        crate::model::UserPushDevicesRequest
    );

    // POST /v1/user/set_contents_release_effect_history
    define_post_method!(
        set_contents_release_effect_history,
        "/user/set_contents_release_effect_history",
        crate::model::UserSetContentsReleaseEffectHistoryRequest
    );

    // POST /v1/user/set_simple_tutorial_finish
    define_post_method!(
        set_simple_tutorial_finish,
        "/user/set_simple_tutorial_finish",
        crate::model::UserSetSimpleTutorialFinishRequest
    );
}
