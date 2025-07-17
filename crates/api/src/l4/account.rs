use crate::macros::{define_api_struct, use_common_crate};
use serde_json::json;

use_common_crate!();
define_api_struct!(AccountApi);

impl<'a> AccountApi<'a> {
    // pub fn new(client: &'a ApiClient) -> Self {
    //     Self { client }
    // }

    /// Returns the `device_specific_id`
    ///
    /// **Response example**
    ///
    /// ```json
    /// {   
    ///     "player_id": "114514",
    ///     "device_specific_id": "1919810",
    ///     "session_token": "1919810",
    ///     "player_name": "yaju senpai",
    ///     "player_level": 114514
    /// }
    /// ```
    pub fn account_connect(&self, id: &str, password: &str) -> Result<Response> {
        let url = format!("{API_BASE}/account/connect");
        let res = self
            .api
            .client
            .post(url)
            .headers(self.api.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "provider": 1,
                "player_id": id,
                "id_token": password,
                "platform_type": 1
            }))
            .send()?;
        Ok(res)
    }
}
