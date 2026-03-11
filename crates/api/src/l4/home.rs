use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(HomeApi);

impl<'a> HomeApi<'a> {
    // POST /v1/home/get_custom_setting
    post!(
        get_custom_setting,
        "/home/get_custom_setting",
        crate::model::HomeGetCustomSettingResponse
    );

    // POST /v1/home/get_home
    post!(
        get_home,
        "/home/get_home",
        crate::model::HomeGetHomeResponse
    );

    // POST /v1/home/get_login_bonus
    post!(
        get_login_bonus,
        "/home/get_login_bonus",
        crate::model::HomeGetLoginBonusResponse
    );

    // POST /v1/home/get_wallpaper_setting
    post!(
        get_wallpaper_setting,
        "/home/get_wallpaper_setting",
        crate::model::HomeGetWallpaperSettingResponse
    );

    // POST /v1/home/notify_wallpaper_setting
    post!(
        notify_wallpaper_setting,
        "/home/notify_wallpaper_setting",
        crate::model::NotifyWallpaperSettingRequst,
        serde_json::Value
    );

    // POST /v1/home/set_clock_setting
    post!(
        set_clock_setting,
        "/home/set_clock_setting",
        crate::model::HomeSetClockSettingRequst,
        crate::model::HomeSetClockSettingResponse
    );

    // POST /v1/home/set_current_wallpaper_setting
    post!(
        set_current_wallpaper_setting,
        "/home/set_current_wallpaper_setting",
        crate::model::HomeSetCurrentWallpaperSettingRequst,
        crate::model::HomeSetCurrentWallpaperSettingResponse
    );

    // POST /v1/home/set_show_retire
    post!(
        set_show_retire,
        "/home/set_show_retire",
        crate::model::HomeSetShowRetireResponse
    );

    // POST /v1/home/set_wallpaper_setting
    post!(
        set_wallpaper_setting,
        "/home/set_wallpaper_setting",
        crate::model::HomeSetWallpaperSettingRequst,
        crate::model::HomeSetWallpaperSettingResponse
    );
}




