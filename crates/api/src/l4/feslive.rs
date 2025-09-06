use crate::macros::{define_api_struct, use_common_crate};
use serde_json::json;

use_common_crate!();
define_api_struct!(FesLiveApi);

impl<'a> FesLiveApi<'a> {
    pub fn enter(&self, id: &str) -> Result<Response> {
        let url = format!("{API_BASE}/feslive/enter");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "live_id": id,
            }))
            .send()?;
        Ok(res)
    }

    pub fn connect_token(&self, live_id: &str) -> Result<Response> {
        let url = format!("{API_BASE}/feslive/connect_token");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "live_id": live_id,
            }))
            .send()?;
        Ok(res)
    }
}
