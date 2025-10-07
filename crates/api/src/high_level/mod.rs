use crate::macros::{define_api_struct, use_common_crate};
use reqwest::header;
use serde_json::json;

use crate::{LINKURA_APP_STORE_URL, UA_PREFIX, WEB_UA};

use_common_crate!();
define_api_struct!(AssetsApi);

impl<'a> AssetsApi<'a> {
    pub fn get_hls_url_from_archive(&self, url: &str) -> Result<String> {
        let res = self.assets_client.get(url).send()?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get archive failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
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
    pub fn get_app_version(&self) -> Result<(Option<String>, Option<String>)> {
        let website = reqwest::blocking::Client::new()
            .get(LINKURA_APP_STORE_URL)
            .header(header::USER_AGENT, WEB_UA)
            .send()?;
        if website.status() != reqwest::StatusCode::OK {
            tracing::error!("Failed to get app version from website: {:?}", website);
            return Err(anyhow::anyhow!("Failed to get app version from website"));
        }
        let html = website.text()?;
        let re = regex::Regex::new(r#"\\"versionDisplay\\":\\"(\d+\.\d+\.\d+)\\"#).unwrap();
        let captures = re.captures(&html);
        let app_version = captures
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string());

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
            .send()?;

        if res.status() != reqwest::StatusCode::OK {
            tracing::error!("Linkura api request failed: {:?}", res);
            return Err(anyhow::anyhow!("Linkura api request failed"));
        }
        let res_version = res.headers().get("x-res-version").map(|v| {
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
    pub fn password_login(&self, id: &str, password: &str) -> Result<String> {
        let res = self.raw().account().account_connect(id, password)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Login failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let device_specific_id: &str = json["device_specific_id"].as_str().unwrap_or_default();
        if device_specific_id.is_empty() {
            return Err(anyhow::anyhow!("Login failed, device_specific_id is empty"));
        }
        Ok(device_specific_id.to_string())
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
    pub fn device_id_login(&self, id: &str, device_id: &str) -> Result<String> {
        let res = self.raw().user().user_login(id, device_id)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Login failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let session_token = json["session_token"].as_str().unwrap_or_default();
        if session_token.is_empty() {
            return Err(anyhow::anyhow!("Login failed"));
        }
        Ok(session_token.to_string())
    }

    pub fn get_plan_list(&self) -> Result<serde_json::Value> {
        let res = self.raw().archive().get_home()?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get plan list failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let trailer_archive_list = json
            .get("trailer_archive_list")
            .ok_or_else(|| anyhow::anyhow!("Get plan list failed: {:?}", json))?;
        let live_archive_list = json
            .get("live_archive_list")
            .ok_or_else(|| anyhow::anyhow!("Get plan list failed: {:?}", json))?;
        let mut live_archive_list = live_archive_list.clone();
        live_archive_list
            .as_array_mut()
            .unwrap()
            .append(&mut trailer_archive_list.as_array().unwrap().clone());
        Ok(live_archive_list.clone())
    }

    pub fn get_archive_list(&self, limit: Option<u32>) -> Result<serde_json::Value> {
        let res = self.raw().archive().get_archive_list(limit)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get archive list failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let archive_list = json
            .get("archive_list")
            .ok_or_else(|| anyhow::anyhow!("Get archive list failed: {:?}", json))?;
        Ok(archive_list.clone())
    }

    pub fn get_with_meets_info(&self, id: &str) -> Result<serde_json::Value> {
        let res = self.raw().with_live().enter(id)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get meets info failed: {:?}", res));
        }
        let wm_info: serde_json::Value = res.json()?;
        Ok(json!({
            "room": wm_info.get("room").unwrap().as_object().unwrap(),
            "name": wm_info.get("name").unwrap().as_str().unwrap(),
            "description": wm_info.get("description").unwrap().as_str().unwrap(),
            "thumbnail": wm_info.get("cover_image_url").unwrap().as_str().unwrap(),
            "characters": wm_info.get("characters").unwrap().as_array().unwrap(),
            "hls_url": wm_info.get("hls").unwrap().as_object().unwrap().get("url").unwrap().as_str().unwrap(),
        }))
    }

    pub fn get_with_meets_connect_token(&self, live_id: &str) -> Result<String> {
        let res = self.raw().with_live().connect_token(live_id)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get connect token failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let connect_token = json["audience_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", json))?;
        Ok(connect_token.to_string())
    }

    pub fn get_fes_live_info(&self, id: &str) -> Result<serde_json::Value> {
        let res = self.raw().fes_live().enter(id)?;
        if res.status() != reqwest::StatusCode::OK {
            let status = res.status();
            let error_text = res.text().unwrap_or_default();
            return Err(anyhow::anyhow!("Get fes live info failed with status {}: {:?}", status, error_text));
        }
        let fes_info: serde_json::Value = res.json()?;
        Ok(json!({
            "room": fes_info.get("room").unwrap().as_object().unwrap(),
            "name": fes_info.get("name").unwrap().as_str().unwrap(),
            "description": fes_info.get("description").unwrap().as_str().unwrap(),
            "characters": fes_info.get("characters").unwrap().as_array().unwrap(),
            "hls": fes_info.get("hls").unwrap().as_object().unwrap(),
        }))
    }

    pub fn get_fes_live_connect_token(&self, live_id: &str) -> Result<String> {
        let res = self.raw().fes_live().connect_token(live_id)?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get connect token failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let connect_token = json["audience_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", json))?;
        Ok(connect_token.to_string())
    }

    pub fn get_archive_details(&self, id: &str, live_type: u8) -> Result<serde_json::Value> {
        if live_type == 1 {
            let res = self.raw().archive().get_fes_archive_data(id)?;
            if res.status() != reqwest::StatusCode::OK {
                return Err(anyhow::anyhow!("Get archive details failed: {:?}", res));
            }
            let json: serde_json::Value = res.json()?;
            Ok(json)
        } else if live_type == 2 {
            let res = self.raw().archive().get_with_archive_data(id)?;
            if res.status() != reqwest::StatusCode::OK {
                return Err(anyhow::anyhow!("Get archive details failed: {:?}", res));
            }
            let json: serde_json::Value = res.json()?;
            Ok(json)
        } else {
            Err(anyhow::anyhow!("Unsupported live type: {}", live_type))
        }
    }
}
