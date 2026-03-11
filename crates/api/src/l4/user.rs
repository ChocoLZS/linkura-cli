use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(UserApi);

impl<'a> UserApi<'a> {
    // POST /v1/user/card/check_evolution
    post!(
        card_check_evolution,
        "/user/card/check_evolution",
        crate::model::CheckEvolutionRequest,
        crate::model::CheckEvolutionResponse
    );

    // POST /v1/user/card/check_limit_break
    post!(
        card_check_limit_break,
        "/user/card/check_limit_break",
        crate::model::CheckLimitBreakRequest,
        crate::model::CheckLimitBreakResponse
    );

    // POST /v1/user/card/check_skill_level_up
    post!(
        card_check_skill_level_up,
        "/user/card/check_skill_level_up",
        crate::model::CheckSkillLevelUpRequest,
        crate::model::CheckSkillLevelUpResponse
    );

    // POST /v1/user/card/check_style_level_up
    post!(
        card_check_style_level_up,
        "/user/card/check_style_level_up",
        crate::model::CheckStyleLevelUpRequest,
        crate::model::CheckStyleLevelUpResponse
    );

    // POST /v1/user/card/evolution
    post!(
        card_evolution,
        "/user/card/evolution",
        crate::model::EvolutionRequest,
        crate::model::EvolutionResponse
    );

    // POST /v1/user/card/get_detail
    post!(
        card_get_detail,
        "/user/card/get_detail",
        crate::model::GetDetailRequest,
        crate::model::GetDetailResponse
    );

    // POST /v1/user/card/get_detail_rental
    post!(
        card_get_detail_rental,
        "/user/card/get_detail_rental",
        crate::model::GetDetailRentalRequest,
        crate::model::GetDetailRentalResponse
    );

    // POST /v1/user/card/get_list
    post!(
        card_get_list,
        "/user/card/get_list",
        crate::model::CardGetListRequest,
        crate::model::CardGetListResponse
    );

    // POST /v1/user/card/limit_break
    post!(
        card_limit_break,
        "/user/card/limit_break",
        crate::model::LimitBreakRequest,
        crate::model::LimitBreakResponse
    );

    // POST /v1/user/card/rhythm_game_skill_level_up
    post!(
        card_rhythm_game_skill_level_up,
        "/user/card/rhythm_game_skill_level_up",
        crate::model::RhythmGameSkillLevelUpRequest,
        crate::model::RhythmGameSkillLevelUpResponse
    );

    // POST /v1/user/card/skill_level_up
    post!(
        card_skill_level_up,
        "/user/card/skill_level_up",
        crate::model::SkillLevelUpRequest,
        crate::model::SkillLevelUpResponse
    );

    // POST /v1/user/card/style_level_up
    post!(
        card_style_level_up,
        "/user/card/style_level_up",
        crate::model::StyleLevelUpRequest,
        crate::model::StyleLevelUpResponse
    );

    // POST /v1/user/deck/get_card_list
    post!(
        deck_get_card_list,
        "/user/deck/get_card_list",
        crate::model::DeckGetCardListRequest,
        crate::model::DeckGetCardListResponse
    );

    // POST /v1/user/deck/get_list
    post!(
        deck_get_list,
        "/user/deck/get_list",
        crate::model::DeckGetListResponse
    );

    // POST /v1/user/deck/modify_deck_list
    post!(
        deck_modify_deck_list,
        "/user/deck/modify_deck_list",
        crate::model::DeckModifyDeckListRequest,
        crate::model::DeckModifyDeckListResponse
    );

    // POST /v1/user/deck/notify_auto_deck
    post!(
        deck_notify_auto_deck,
        "/user/deck/notify_auto_deck",
        serde_json::Value
    );

    // POST /v1/user/deck/remove_side_style
    post!(
        deck_remove_side_style,
        "/user/deck/remove_side_style",
        crate::model::DeckRemoveSideStyleRequest,
        crate::model::DeckRemoveSideStyleResponse
    );

    // POST /v1/user/deck/set_copy_deck
    post!(
        deck_set_copy_deck,
        "/user/deck/set_copy_deck",
        crate::model::DeckSetCopyDeckRequest,
        crate::model::DeckSetCopyDeckResponse
    );

    // POST /v1/user/deck/set_deck
    post!(
        deck_set_deck,
        "/user/deck/set_deck",
        crate::model::DeckSetCreateRequest,
        crate::model::DeckSetCreateResponse
    );

    // POST /v1/user/deck/set_delete_deck
    post!(
        deck_set_delete_deck,
        "/user/deck/set_delete_deck",
        crate::model::DeckSetDeleteRequest,
        crate::model::DeckSetDeleteResponse
    );

    // POST /v1/user/deck/set_position
    post!(
        deck_set_position,
        "/user/deck/set_position",
        crate::model::DeckSetPositionRequest,
        crate::model::DeckSetPositionResponse
    );

    // POST /v1/user/deck/set_side_style
    post!(
        deck_set_side_style,
        "/user/deck/set_side_style",
        crate::model::DeckSetSideStyleRequest,
        crate::model::DeckSetSideStyleResponse
    );

    // POST /v1/user/item/get_detail
    post!(
        item_get_detail,
        "/user/item/get_detail",
        crate::model::UserItemGetDetailRequest,
        crate::model::UserItemGetDetailResponse
    );

    // POST /v1/user/items/get_list
    post!(
        items_get_list,
        "/user/items/get_list",
        crate::model::UserItemGetListResponse
    );

    // POST /v1/user/jewel/get_history
    post!(
        jewel_get_history,
        "/user/jewel/get_history",
        crate::model::UserJewelGetHistoryResponse
    );

    // POST /v1/user/login
    post!(
        login,
        "/user/login",
        crate::model::UserLoginRequest,
        crate::model::UserLoginResponse
    );

    // POST /v1/user/push/device
    post!(
        push_device,
        "/user/push/device",
        crate::model::UserPushDevicePutRequest,
        serde_json::Value
    );

    // POST /v1/user/push/devices
    post!(
        push_devices,
        "/user/push/devices",
        crate::model::UserPushDevicesPostRequest,
        serde_json::Value
    );

    // POST /v1/user/set_contents_release_effect_history
    post!(
        set_contents_release_effect_history,
        "/user/set_contents_release_effect_history",
        crate::model::UserSetContentsReleaseEffectHistoryRequest,
        crate::model::UserSetContentsReleaseEffectHistoryResponse
    );

    // POST /v1/user/set_simple_tutorial_finish
    post!(
        set_simple_tutorial_finish,
        "/user/set_simple_tutorial_finish",
        crate::model::UserSetSimpleTutorialFinishRequest,
        serde_json::Value
    );
}




