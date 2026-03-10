use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(HomeApi);

impl<'a> HomeApi<'a> {
    // POST /v1/home/get_custom_setting
    define_post_method!(
        get_custom_setting,
        "/home/get_custom_setting",
        crate::model::HomeGetCustomSettingRequest
    );

    // POST /v1/home/get_home
    define_post_method!(get_home, "/home/get_home", crate::model::HomeGetHomeRequest);

    // POST /v1/home/get_login_bonus
    define_post_method!(
        get_login_bonus,
        "/home/get_login_bonus",
        crate::model::HomeGetLoginBonusRequest
    );

    // POST /v1/home/get_wallpaper_setting
    define_post_method!(
        get_wallpaper_setting,
        "/home/get_wallpaper_setting",
        crate::model::HomeGetWallpaperSettingRequest
    );

    // POST /v1/home/notify_wallpaper_setting
    define_post_method!(
        notify_wallpaper_setting,
        "/home/notify_wallpaper_setting",
        crate::model::HomeNotifyWallpaperSettingRequest
    );

    // POST /v1/home/set_clock_setting
    define_post_method!(
        set_clock_setting,
        "/home/set_clock_setting",
        crate::model::HomeSetClockSettingRequest
    );

    // POST /v1/home/set_current_wallpaper_setting
    define_post_method!(
        set_current_wallpaper_setting,
        "/home/set_current_wallpaper_setting",
        crate::model::HomeSetCurrentWallpaperSettingRequest
    );

    // POST /v1/home/set_show_retire
    define_post_method!(
        set_show_retire,
        "/home/set_show_retire",
        crate::model::HomeSetShowRetireRequest
    );

    // POST /v1/home/set_wallpaper_setting
    define_post_method!(
        set_wallpaper_setting,
        "/home/set_wallpaper_setting",
        crate::model::HomeSetWallpaperSettingRequest
    );
}
