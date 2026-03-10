use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(WebviewLiveApi);

impl<'a> WebviewLiveApi<'a> {
    // POST /v1/webview/live/enter
    define_post_method!(
        enter,
        "/webview/live/enter",
        crate::model::WebviewLiveEnterRequest
    );

    // POST /v1/webview/live/live_info
    define_post_method!(
        live_info,
        "/webview/live/live_info",
        crate::model::WebviewLiveLiveInfoRequest
    );

    // POST /v1/webview/live/login
    define_post_method!(
        login,
        "/webview/live/login",
        crate::model::WebviewLiveLoginRequest
    );
}
