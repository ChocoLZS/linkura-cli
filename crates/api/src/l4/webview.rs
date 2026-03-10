use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(WebviewApi);

impl<'a> WebviewApi<'a> {
    // POST /v1/webview/gacha/get_detail
    define_post_method!(
        gacha_get_detail,
        "/webview/gacha/get_detail",
        crate::model::WebviewGachaGetDetailRequest
    );

    // POST /v1/webview/gacha/get_select_card_list
    define_post_method!(
        gacha_get_select_card_list,
        "/webview/gacha/get_select_card_list",
        crate::model::WebviewGachaGetSelectCardListRequest
    );

    // POST /v1/webview/school_idol_connect_post/get_theme_list
    define_post_method!(
        school_idol_connect_post_get_theme_list,
        "/webview/school_idol_connect_post/get_theme_list",
        crate::model::WebviewSchoolIdolConnectPostGetThemeListRequest
    );

    // POST /v1/webview/shop/get_membership_perk_detail
    define_post_method!(
        shop_get_membership_perk_detail,
        "/webview/shop/get_membership_perk_detail",
        crate::model::WebviewShopGetMembershipPerkDetailRequest
    );
}
