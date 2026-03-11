use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(WebviewLiveApi);

impl<'a> WebviewLiveApi<'a> {
    // POST /v1/webview/live/enter
    post!(
        enter,
        "/webview/live/enter",
        crate::model::WebviewLiveEnterRequest,
        crate::model::WebviewLiveEnterResponse
    );

    // POST /v1/webview/live/live_info
    post!(
        live_info,
        "/webview/live/live_info",
        crate::model::WebviewLiveInfoRequest,
        crate::model::WebviewLiveInfoResponse
    );

    // POST /v1/webview/live/login
    post!(
        login,
        "/webview/live/login",
        crate::model::WebviewLiveLoginRequest,
        crate::model::WebviewLiveLoginResponse
    );
}




