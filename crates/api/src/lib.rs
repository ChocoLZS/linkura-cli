use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};
use rand::Rng;
use rand::distr::Alphanumeric;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Credential {
    /// x-res-version
    pub res_version: String,
    /// x-client-version
    pub client_version: String,
    /// device_specific_id
    pub device_specific_id: String,
    /// user account id
    pub player_id: String,
    /// session token
    pub session_token: Option<String>,
}

const API_BASE: &str = "https://api.link-like-lovelive.app/v1";
const LINKURA_APP_STORE_URL: &str = "https://apps.apple.com/jp/app/link-like-%E3%83%A9%E3%83%96%E3%83%A9%E3%82%A4%E3%83%96-%E8%93%AE%E3%83%8E%E7%A9%BA%E3%82%B9%E3%82%AF%E3%83%BC%E3%83%AB%E3%82%A2%E3%82%A4%E3%83%89%E3%83%AB%E3%82%AF%E3%83%A9%E3%83%96/id1665027261";
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36";
/* CONFIG **/
pub const UA_PREFIX: &str = "inspix-android";
pub const BASE_RES_VERSION: &str = "R2504300";
pub const BASE_CLIENT_VERSION: &str = "3.1.0";

mod api_header {
    /// x-device-type
    pub const DEVICE_TYPE: &str = "android";
    /// inspix-user-api-version
    pub const API_VERSION: &str = "1.0.0";
    pub const ACCEPT: &str = "application/json";
    pub const X_API_KEY: &str = "4e769efa67d8f54be0b67e8f70ccb23d513a3c841191b6b2ba45ffc6fb498068";
    pub const HOST: &str = "api.link-like-lovelive.app";
    pub const ACCEPT_ENCODING: &str = "gzip, deflate";
}

pub fn gen_random_idempotency_key() -> String {
    let mut rng = rand::rng();
    let idempotency_key: String = (0..32).map(|_| rng.sample(Alphanumeric) as char).collect();
    idempotency_key
}

#[derive(Debug)]
pub struct ApiClient {
    pub client: reqwest::blocking::Client,
    pub runtime_header: header::HeaderMap,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .default_headers({
                    let mut headers = header::HeaderMap::new();
                    headers.insert("x-res-version", BASE_RES_VERSION.parse().unwrap());
                    headers.insert("x-client-version", BASE_CLIENT_VERSION.parse().unwrap());
                    headers.insert("x-device-type", api_header::DEVICE_TYPE.parse().unwrap());
                    // headers.insert(
                    //     "x-idempotency-key",
                    //     gen_random_idempotency_key().parse().unwrap(),
                    // );
                    headers.insert(
                        "inspix-user-api-version",
                        api_header::API_VERSION.parse().unwrap(),
                    );
                    headers.insert(header::ACCEPT, api_header::ACCEPT.parse().unwrap());
                    headers.insert("x-api-key", api_header::X_API_KEY.parse().unwrap());
                    headers.insert(
                        header::USER_AGENT,
                        format!("{UA_PREFIX}/{BASE_RES_VERSION}").parse().unwrap(),
                    );
                    headers.insert(header::HOST, api_header::HOST.parse().unwrap());
                    headers.insert(
                        header::ACCEPT_ENCODING,
                        api_header::ACCEPT_ENCODING.parse().unwrap(),
                    );
                    headers
                })
                .build()
                .unwrap(),
            runtime_header: header::HeaderMap::new(),
        }
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
        let url = format!("{API_BASE}/account/connect");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "provider": 1,
                "player_id": id,
                "id_token": password,
                "platform_type": 1
            }))
            .send()?;
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

    /// Get x-res-version from headers
    ///
    /// x-res-version is like: `R2504300@XXX`
    ///
    /// Get client version from
    ///
    /// Returns (x-res-version, `app version from website`)
    pub fn get_app_version(&self) -> Result<(Option<String>, Option<String>)> {
        let mut app_version: Option<String> = None;
        let website = reqwest::blocking::Client::new()
            .get(LINKURA_APP_STORE_URL)
            .header(header::USER_AGENT, UA)
            .send()?;
        if website.status() != reqwest::StatusCode::OK {
            tracing::error!("Download linkura app store website failed: {:?}", website);
        } else {
            let html = website.text()?;
            let re = regex::Regex::new(r#"\\"versionDisplay\\":\\"(\d+\.\d+\.\d+)\\"#).unwrap();
            let captures = re.captures(&html);
            app_version = captures
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().to_string());
        }
        // empty id login check
        let url = format!("{API_BASE}/user/login");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header("x-client-version", app_version.clone().unwrap_or_default())
            .json(&json!({
                "player_id": "",
                "device_specific_id": "",
                "version": 1
            }))
            .send()?;

        let mut res_version: Option<String> = None;
        if res.status() != reqwest::StatusCode::OK {
            tracing::error!("Linkura api request failed: {:?}", res);
        } else {
            res_version = res.headers().get("x-res-version").map(|v| {
                let version = v.to_str().unwrap_or_default();
                version.split('@').next().unwrap_or_default().to_string()
            });
        }

        Ok((res_version, app_version))
    }

    pub fn get_plan_list(&self) -> Result<serde_json::Value> {
        let url = format!("{API_BASE}/archive/get_home");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header(header::CONTENT_LENGTH, 0)
            .send()?;
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

    pub fn get_archive_list(&self) -> Result<serde_json::Value> {
        let url = format!("{API_BASE}/archive/get_archive_list");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                              "order": "desc",
                              "characters": [],
                              "limit": 4,
                              "sort": "live_start_time"
            }))
            .send()?;
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
        let url = format!("{API_BASE}/withlive/enter");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "live_id": id,
            }))
            .send()?;
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
        let url = format!("{API_BASE}/withlive/connect_token");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .json(&json!({
                "live_id": live_id,
            }))
            .send()?;
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
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get fes live info failed: {:?}", res));
        }
        let fes_info: serde_json::Value = res.json()?;
        Ok(json!({
            "room": fes_info.get("room").unwrap().as_object().unwrap(),
            "name": fes_info.get("name").unwrap().as_str().unwrap(),
            "description": fes_info.get("description").unwrap().as_str().unwrap(),
            "characters": fes_info.get("characters").unwrap().as_array().unwrap(),
        }))
    }

    pub fn get_fes_live_connect_token(&self, live_id: &str) -> Result<String> {
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
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get connect token failed: {:?}", res));
        }
        let json: serde_json::Value = res.json()?;
        let connect_token = json["audience_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", json))?;
        Ok(connect_token.to_string())
    }
}

pub fn get_hls_url_from_archive(url: &str) -> Result<String> {
    let res = reqwest::blocking::Client::new()
        .get(url)
        .headers(
            [
                (
                    header::USER_AGENT,
                    "UnityPlayer/2021.3.36f1 (UnityWebRequest/1.0, libcurl/8.5.0-DEV)"
                        .parse()
                        .unwrap(),
                ),
                (header::ACCEPT, "*/*".parse().unwrap()),
                (
                    header::HOST,
                    "assets.link-like-lovelive.app".parse().unwrap(),
                ),
                (header::ACCEPT_ENCODING, "deflate, gzip".parse().unwrap()),
                (
                    "X-Unity-Version".parse().unwrap(),
                    "2021.3.36f1".parse().unwrap(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        )
        .send()?;
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

// setter
impl ApiClient {
    pub fn update_version(&mut self, res_version: &str, client_version: &str) {
        self.runtime_header
            .insert("x-res-version", res_version.parse().unwrap());
        self.runtime_header
            .insert("x-client-version", client_version.parse().unwrap());
        self.runtime_header.insert(
            header::USER_AGENT,
            format!("{UA_PREFIX}/{0}", client_version).parse().unwrap(),
        );
    }

    pub fn update_with_credential(&mut self, credential: &Credential) {
        self.update_version(&credential.res_version, &credential.client_version);
        self.runtime_header.insert(
            "x-device-specific-id",
            credential.device_specific_id.parse().unwrap(),
        );
    }

    pub fn set_session_token(&mut self, token: &str) {
        self.runtime_header.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
    }

    pub fn del_session_token(&mut self) {
        self.runtime_header.remove(header::AUTHORIZATION);
    }
}

pub fn extract_jwt_payload(jwt: &str) -> Result<serde_json::Value> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid JWT format"));
    }
    let payload = parts[1];
    let decoded_payload = general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .or_else(|_| general_purpose::URL_SAFE.decode(payload))?;
    let json_payload: serde_json::Value = serde_json::from_slice(&decoded_payload)?;
    Ok(json_payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    const JWT: &'static str = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzZXJ2aWNlX2RvbWFpbiI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJsaW5rX2xpa2VfaWQiOiJBQUFBQUFBQUEiLCJyb29tX2lkIjoiZGVmYXVsdC1mYWNiZGE1MS1iYjlkLTQyNjctYjRhYi01ZWYzYzg3OGJhZWMiLCJyb2xlIjoiYXVkaWVuY2UiLCJwb2QiOnsicm9sZSI6ImF1ZGllbmNlIiwic2NoZW1lIjoidGNwIiwiYWRkcmVzcyI6IjEwLjExNC41MTQuMTkxIiwicG9ydCI6OTgxMH0sImlzcyI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJzdWIiOiJBQUFBQUFBQUEiLCJhdWQiOlsiQUFBQUFBQUFBIl0sImV4cCI6MTc0ODUxODU3NSwibmJmIjoxNzQ4NTE4NTYwLCJpYXQiOjE3NDg1MTg1NjB9.eddiZjzEH_I88w9lmOVBr2Z4BWShIv6yeM9TPZvKIts5rmPFwvBbJEKffkobXglOuUBp80svLoufyzOM_YSmDg";
    #[test]
    fn test_extract_jwt_payload() {
        let jwt = JWT;
        let result = extract_jwt_payload(jwt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_jwt_payload_invalid() {
        let jwt = "invalid.jwt";
        let result = extract_jwt_payload(jwt);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_payload_base64_decoding() {
        let payload = extract_jwt_payload(JWT).unwrap();
        assert_eq!(
            payload["service_domain"].as_str().unwrap(),
            "https://api.link-like-lovelive.app"
        );
        assert_eq!(payload["link_like_id"].as_str().unwrap(), "AAAAAAAAA");
        assert_eq!(
            payload["room_id"].as_str().unwrap(),
            "default-facbda51-bb9d-4267-b4ab-5ef3c878baec"
        );
        assert_eq!(payload["pod"]["role"].as_str().unwrap(), "audience");
        assert_eq!(payload["pod"]["scheme"].as_str().unwrap(), "tcp");
        assert_eq!(
            payload["pod"]["address"].as_str().unwrap(),
            "10.114.514.191"
        );
        assert_eq!(payload["pod"]["port"].as_u64().unwrap(), 9810);
        assert_eq!(
            payload["iss"].as_str().unwrap(),
            "https://api.link-like-lovelive.app"
        );
        assert_eq!(payload["sub"].as_str().unwrap(), "AAAAAAAAA");
        assert_eq!(
            payload["aud"].as_array().unwrap()[0].as_str().unwrap(),
            "AAAAAAAAA"
        );
        assert_eq!(payload["exp"].as_u64().unwrap(), 1748518575);
        assert_eq!(payload["nbf"].as_u64().unwrap(), 1748518560);
        assert_eq!(payload["iat"].as_u64().unwrap(), 1748518560);
    }
}
