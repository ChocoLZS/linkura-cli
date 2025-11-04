use anyhow::Result;
use rand::Rng;
use rand::distr::Alphanumeric;
use reqwest::header;
use serde::{Deserialize, Serialize};

mod high_level;
mod l4;
mod macros;

pub mod model {}
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
const LINKURA_GOOGLE_PLAY_URL: &str = "https://play.google.com/store/apps/details?id=com.oddno.lovelive&hl=en";
const WEB_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36";
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
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) assets_client: reqwest::blocking::Client,
    pub(crate) runtime_header: header::HeaderMap,
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
            assets_client: reqwest::blocking::Client::builder()
                .default_headers({
                    let mut headers = header::HeaderMap::new();
                    headers.insert(
                        header::USER_AGENT,
                        "UnityPlayer/2021.3.36f1 (UnityWebRequest/1.0, libcurl/8.5.0-DEV)"
                            .parse()
                            .unwrap(),
                    );
                    headers.insert(header::ACCEPT, "*/*".parse().unwrap());
                    headers.insert(
                        header::HOST,
                        "assets.link-like-lovelive.app".parse().unwrap(),
                    );
                    headers.insert(header::ACCEPT_ENCODING, "deflate, gzip".parse().unwrap());
                    headers.insert("X-Unity-Version", "2021.3.36f1".parse().unwrap());
                    headers
                })
                .build()
                .unwrap(),
        }
    }

    pub fn raw(&self) -> l4::LinkuraApi {
        l4::LinkuraApi { api: self }
    }

    pub fn high_level(&self) -> high_level::HighLevelApi {
        high_level::HighLevelApi { api: self }
    }

    pub fn assets(&self) -> high_level::AssetsApi {
        high_level::AssetsApi { api: self }
    }
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

fn _get_appstore_version() -> Result<Option<String>> {
    let website = reqwest::blocking::Client::new()
        .get(LINKURA_APP_STORE_URL)
        .header(header::USER_AGENT, WEB_UA)
        .send()?;
    if website.status() != reqwest::StatusCode::OK {
        tracing::error!("Failed to get app version from website: {:?}", website);
    }
    let re = regex::Regex::new(r#""primarySubtitle":\s*"(\d+\.\d+\.\d+)"#).unwrap();
    let text = website.text()?;
    let captures = re.captures(&text);
    Ok(captures
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string()))
}

pub fn get_appstore_version() -> Option<String> {
    return _get_appstore_version().ok().flatten();
}
    
fn _get_google_play_version() -> Result<Option<String>> {
    let website = reqwest::blocking::Client::new()
        .get(LINKURA_GOOGLE_PLAY_URL)
        .header(header::USER_AGENT, WEB_UA)
        .send()?;
    if website.status() != reqwest::StatusCode::OK {
        tracing::error!("Failed to get app version from website: {:?}", website);
    }
    let re = regex::Regex::new(r#"Link！Like！ラブライブ！蓮ノ空スクールアイドルクラブ"[^\n]*\["(\d+\.\d+\.\d+)"\]"#).unwrap();
    let text = website.text()?;
    let captures = re.captures(&text);
    Ok(captures
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string()))
}

pub fn get_google_play_version() -> Option<String> {
    return _get_google_play_version().ok().flatten();
}