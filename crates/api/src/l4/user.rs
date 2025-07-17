use crate::macros::{define_api_struct, use_common_crate};
use serde_json::json;

use_common_crate!();
define_api_struct!(UserApi);

impl<'a> UserApi<'a> {
    /// Returns the `session_token`
    ///
    /// **Return example**
    ///
    /// ```json
    /// {   
    ///     ...
    ///     "session_token": "1919810",
    ///     ...
    /// }
    /// ```
    pub fn user_login(&self, id: &str, device_id: &str) -> Result<Response> {
        let url = format!("{API_BASE}/user/login");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header("x-device-specific-id", device_id)
            .json(&json!({
                "player_id": id,
                "device_specific_id": device_id,
                "version": 1
            }))
            .send()?;
        Ok(res)
    }
}
