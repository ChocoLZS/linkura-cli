use reqwest::header;
use serde_json::json;

use crate::macros::{define_api_struct, use_common_crate};

use_common_crate!();
define_api_struct!(ArchiveApi);

impl<'a> ArchiveApi<'a> {
    pub fn get_home(&self) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_home");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header(header::CONTENT_LENGTH, 0)
            .send()?;
        Ok(res)
    }

    // TODO: params
    pub fn get_archive_list(&self, limit: Option<u32>) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_archive_list");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "order": "desc",
                "characters": [],
                "limit": limit.unwrap_or(4),
                "sort": "live_start_time"
            }))
            .send()?;
        Ok(res)
    }

    pub fn get_fes_archive_data(&self, id: &str) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_fes_archive_data");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "archives_id": id
            }))
            .send()?;
        Ok(res)
    }

    pub fn get_with_archive_data(&self, id: &str) -> Result<Response> {
        let url = format!("{API_BASE}/archive/get_with_archive_data");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "archives_id": id
            }))
            .send()?;
        Ok(res)
    }
}
