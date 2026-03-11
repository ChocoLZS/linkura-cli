use std::fmt;

use crate::{
    get_appstore_version, get_google_play_version,
    macros::{define_api_struct, use_common_crate},
    model::{
        AccountConnectRequest, ArchiveGetArchiveListRequest, ArchiveGetFesArchiveDataRequest,
        ArchiveGetWithArchiveDataRequest, FesliveConnectTokenRequest, FesliveEnterRequest,
        LiveConnectTokenRequest, UserLoginRequest, WithliveEnterRequest,
    },
};
use reqwest::header;
use serde_json::json;

use crate::UA_PREFIX;

use_common_crate!();

/// Helper struct to format Response with body for debugging
pub struct ResponseDebug {
    pub url: String,
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
}

impl fmt::Debug for ResponseDebug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Response")
            .field("url", &self.url)
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl ResponseDebug {
    #[allow(unused)]
    /// Create a ResponseDebug from a reqwest::Response (consumes the response)
    pub async fn from_response(res: reqwest::Response) -> Result<Self> {
        let url = res.url().to_string();
        let status = res.status();
        let headers = res.headers().clone();
        let body = res.text().await?;

        Ok(Self {
            url,
            status,
            headers,
            body,
        })
    }
}
define_api_struct!(AssetsApi);

impl<'a> AssetsApi<'a> {
    pub async fn get_hls_url_from_archive(&self, url: &str) -> Result<String> {
        let res = self.assets_client.get(url).send().await?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get archive failed: {:?}", res));
        }
        let json: serde_json::Value = res.json().await?;
        let hls_url = format!(
            "{}/{}",
            json["path"].as_str().unwrap(),
            json["playlist_file"].as_str().unwrap()
        );
        Ok(hls_url.to_string())
    }
}

define_api_struct!(HighLevelApi);

impl<'a> HighLevelApi<'a> {
    /// Get x-res-version from headers
    ///
    /// x-res-version is like: `R2504300@XXX`
    ///
    /// Get client version from
    ///
    /// Returns (x-res-version, `app version from website`)
    pub async fn get_app_version(&self) -> Result<(Option<String>, Option<String>)> {
        let app_version = match get_appstore_version().await {
            Some(version) => Some(version),
            None => get_google_play_version().await,
        };
        tracing::info!("Detected app version: {:?}", app_version);
        // empty id login check
        let url = format!("{API_BASE}/user/login");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header("x-client-version", app_version.clone().unwrap_or_default())
            .header(
                header::USER_AGENT,
                format!("{UA_PREFIX}/{}", app_version.clone().unwrap_or_default()),
            )
            .json(&json!({
                "player_id": "",
                "device_specific_id": "",
                "version": 1
            }))
            .send()
            .await?;

        let headers = res.headers().clone();
        if res.status() != reqwest::StatusCode::OK {
            tracing::error!(
                "Linkura api request failed: {:?}",
                ResponseDebug::from_response(res).await?
            );
            return Ok((None, app_version));
        }
        let res_version = headers.get("x-res-version").map(|v| {
            let version = v.to_str().unwrap_or_default();
            version.split('@').next().unwrap_or_default().to_string()
        });

        Ok((res_version, app_version))
    }

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
    pub async fn password_login(&self, id: &str, password: &str) -> Result<String> {
        let request = AccountConnectRequest {
            provider: Some(1),
            player_id: Some(id.to_string()),
            id_token: Some(password.to_string()),
            platform_type: Some(1),
            ..Default::default()
        };
        let body = self.raw().account().connect(&request).await?;
        let device_specific_id = body.device_specific_id.unwrap_or_default();
        if device_specific_id.is_empty() {
            return Err(anyhow::anyhow!("Login failed, device_specific_id is empty"));
        }
        Ok(device_specific_id)
    }

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
    pub async fn device_id_login(&self, id: &str, device_id: &str) -> Result<String> {
        let request = UserLoginRequest {
            player_id: Some(id.to_string()),
            device_specific_id: Some(device_id.to_string()),
            version: Some(1),
            ..Default::default()
        };
        let body = self.raw().user().login(&request).await?;
        let session_token = body.session_token.unwrap_or_default();
        if session_token.is_empty() {
            return Err(anyhow::anyhow!("Login failed"));
        }
        Ok(session_token)
    }

    pub async fn get_plan_list(&self) -> Result<serde_json::Value> {
        let body = self.raw().archive().get_home().await?;
        let mut merged = body.live_archive_list.unwrap_or_default();
        merged.extend(body.trailer_archive_list.unwrap_or_default());
        Ok(serde_json::to_value(merged)?)
    }

    pub async fn get_archive_list(&self, limit: Option<u32>) -> Result<serde_json::Value> {
        let request = ArchiveGetArchiveListRequest {
            order: Some("desc".to_string()),
            characters: Some(Vec::new()),
            limit: Some(limit.unwrap_or(4) as i32),
            sort: Some("live_start_time".to_string()),
            ..Default::default()
        };
        let body = self.raw().archive().get_archive_list(&request).await?;
        Ok(serde_json::to_value(body.archive_list.unwrap_or_default())?)
    }

    pub async fn get_with_meets_info(&self, id: &str) -> Result<serde_json::Value> {
        let request = WithliveEnterRequest {
            live_id: Some(id.to_string()),
            ..Default::default()
        };
        let body = self.raw().with_live().enter(&request).await?;
        Ok(serde_json::to_value(body)?)
    }

    pub async fn get_with_meets_connect_token(&self, live_id: &str) -> Result<String> {
        let request = LiveConnectTokenRequest {
            live_id: Some(live_id.to_string()),
            ..Default::default()
        };
        let body = self.raw().with_live().connect_token(&request).await?;
        let connect_token = body
            .audience_token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", body))?;
        Ok(connect_token)
    }

    pub async fn get_fes_live_info(&self, id: &str) -> Result<serde_json::Value> {
        let request = FesliveEnterRequest {
            live_id: Some(id.to_string()),
            ..Default::default()
        };
        let body = self.raw().fes_live().enter(&request).await?;
        Ok(serde_json::to_value(body)?)
    }

    pub async fn get_fes_live_connect_token(&self, live_id: &str) -> Result<String> {
        let request = FesliveConnectTokenRequest {
            live_id: Some(live_id.to_string()),
            ..Default::default()
        };
        let body = self.raw().fes_live().connect_token(&request).await?;
        let connect_token = body
            .audience_token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", body))?;
        Ok(connect_token)
    }

    pub async fn get_archive_details(&self, id: &str, live_type: u8) -> Result<serde_json::Value> {
        if live_type == 1 {
            let request = ArchiveGetFesArchiveDataRequest {
                archives_id: Some(id.to_string()),
                ..Default::default()
            };
            let body = self.raw().archive().get_fes_archive_data(&request).await?;
            Ok(serde_json::to_value(body)?)
        } else if live_type == 2 {
            let request = ArchiveGetWithArchiveDataRequest {
                archives_id: Some(id.to_string()),
                ..Default::default()
            };
            let body = self.raw().archive().get_with_archive_data(&request).await?;
            Ok(serde_json::to_value(body)?)
        } else {
            Err(anyhow::anyhow!("Unsupported live type: {}", live_type))
        }
    }
}
