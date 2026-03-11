use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(WebviewApi);

impl<'a> WebviewApi<'a> {
    // POST /v1/webview/gacha/get_detail
    post_params!(
        gacha_get_detail,
        "/webview/gacha/get_detail",
        crate::model::GetGachaDetailResponse,
        gacha_series_id: Option<i32>,
    );

    // POST /v1/webview/gacha/get_select_card_list
    post_params!(
        gacha_get_select_card_list,
        "/webview/gacha/get_select_card_list",
        crate::model::GetSelectCardListResponse,
        select_ticket_series_id: Option<i32>,
    );

    // POST /v1/webview/school_idol_connect_post/get_theme_list
    post!(
        school_idol_connect_post_get_theme_list,
        "/webview/school_idol_connect_post/get_theme_list",
        Vec<crate::model::GetThemeListResponseInner>
    );

    // POST /v1/webview/shop/get_membership_perk_detail
    post_params!(
        shop_get_membership_perk_detail,
        "/webview/shop/get_membership_perk_detail",
        crate::model::GetMembershipPerkDetailResponse,
        membership_id: Option<i32>,
    );
}





