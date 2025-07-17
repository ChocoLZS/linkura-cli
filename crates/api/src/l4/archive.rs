use reqwest::header;
use serde_json::json;

use crate::macros::{define_api_struct, use_common_crate};

use_common_crate!();
define_api_struct!(ArchiveApi);

impl<'a> ArchiveApi<'a> {
    // pub fn new(client: &'a ApiClient) -> Self {
    //     Self { client }
    // }

    pub fn get_home(&self) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_home");
        let res = self
            .api
            .client
            .post(url)
            .headers(self.api.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header(header::CONTENT_LENGTH, 0)
            .send()?;
        Ok(res)
    }

    // TODO: params
    pub fn get_archive_list(&self) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_archive_list");
        let res = self
            .api
            .client
            .post(url)
            .headers(self.api.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                              "order": "desc",
                              "characters": [],
                              "limit": 4,
                              "sort": "live_start_time"
            }))
            .send()?;
        Ok(res)
    }
}
