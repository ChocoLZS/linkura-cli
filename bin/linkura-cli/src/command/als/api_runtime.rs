use anyhow::{Context, Result};
use linkura_api::{ApiClient, Credential};
use linkura_api::model::FesliveLobbyRequest;
use linkura_common::jwt::extract_jwt_payload;
use linkura_packet::als::client::ConnectionInfo;
use tokio::sync::Mutex;

use crate::config::Global;

use super::model::{PlanItem, parse_plan_items};

#[derive(Debug)]
pub struct ApiRuntime {
    client: Mutex<ApiClient>,
    player_id: String,
    device_specific_id: String,
    versions: Mutex<RuntimeVersions>,
}

#[derive(Debug, Clone)]
struct RuntimeVersions {
    res_version: String,
    client_version: String,
}

impl ApiRuntime {
    pub fn from_global(ctx: &Global) -> Self {
        let mut client = ApiClient::new_with_login_type(ctx.config.login_type.clone());
        let credential = Credential {
            res_version: ctx.config.credential.res_version.clone(),
            client_version: ctx.config.credential.client_version.clone(),
            device_specific_id: ctx.config.credential.device_specific_id.clone(),
            player_id: ctx.config.credential.player_id.clone(),
            session_token: ctx.config.credential.session_token.clone(),
        };
        client.update_with_credential(&credential);
        if let Some(token) = credential.session_token.as_deref() {
            client.set_session_token(token);
        }

        Self {
            client: Mutex::new(client),
            player_id: credential.player_id,
            device_specific_id: credential.device_specific_id,
            versions: Mutex::new(RuntimeVersions {
                res_version: credential.res_version,
                client_version: credential.client_version,
            }),
        }
    }

    pub async fn refresh_token(&self) -> Result<()> {
        let mut client = self.client.lock().await;

        // Force-refresh session state:
        // 1) refresh app/resource version
        // 2) clear local session token
        // 3) login again with device id (take over login)
        // 4) verify session by get_plan_list, retry force-login once if needed
        tracing::info!(
            "ALS refresh_token start: player_id={}, device_specific_id={}",
            self.player_id,
            self.device_specific_id
        );
        self.refresh_runtime_versions(&mut client).await?;
        force_device_login(&mut client, &self.player_id, &self.device_specific_id).await?;
        if let Err(e) = client.high_level().get_plan_list().await {
            tracing::warn!(
                "refresh_token verification failed, force login retry once: {}",
                e
            );
            force_device_login(&mut client, &self.player_id, &self.device_specific_id).await?;
            // second failure should bubble up
            client
                .high_level()
                .get_plan_list()
                .await
                .context("refresh_token verification failed after force-login retry")?;
        }
        tracing::info!("ALS refresh_token done: player_id={}", self.player_id);
        Ok(())
    }

    async fn refresh_runtime_versions(&self, client: &mut ApiClient) -> Result<()> {
        let (latest_res_version, latest_client_version) =
            client.high_level().get_app_version().await?;
        let mut versions = self.versions.lock().await;

        let mut changed = false;
        if let Some(res_version) = latest_res_version {
            if res_version != versions.res_version {
                tracing::info!(
                    "ALS update res version: {} -> {}",
                    versions.res_version,
                    res_version
                );
                versions.res_version = res_version;
                changed = true;
            }
        }

        if let Some(client_version) = latest_client_version {
            if client_version != versions.client_version {
                tracing::info!(
                    "ALS update client version: {} -> {}",
                    versions.client_version,
                    client_version
                );
                versions.client_version = client_version;
                changed = true;
            }
        }

        if changed {
            client.update_version(&versions.res_version, &versions.client_version);
        }

        tracing::info!(
            "ALS runtime versions: res_version={}, client_version={}",
            versions.res_version,
            versions.client_version
        );
        Ok(())
    }

    pub async fn get_plan_items(&self) -> Result<Vec<PlanItem>> {
        let client = self.client.lock().await;
        let value = client.high_level().get_plan_list().await?;
        parse_plan_items(value)
    }

    pub async fn get_plan_items_with_auto_refresh(&self) -> Result<Vec<PlanItem>> {
        match self.get_plan_items().await {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::warn!("get_plan_list failed, refreshing token and retrying: {}", e);
                self.refresh_token().await?;
                self.get_plan_items().await
            }
        }
    }

    pub async fn contains_live(&self, live_id: &str, live_type: u64) -> Result<bool> {
        let plans = self.get_plan_items().await?;
        Ok(plans
            .iter()
            .any(|p| p.live_id == live_id && p.live_type == live_type))
    }

    pub async fn fetch_live_connection_info(
        &self,
        live_id: &str,
        live_type: u64,
    ) -> Result<ConnectionInfo> {
        let client = self.client.lock().await;
        let token = fetch_token(&client, live_id, live_type).await?;
        get_connect_info(token)
    }
}

async fn force_device_login(
    client: &mut ApiClient,
    player_id: &str,
    device_specific_id: &str,
) -> Result<()> {
    tracing::info!("ALS force_device_login start: player_id={}", player_id);
    client.del_session_token();
    let token = client
        .high_level()
        .device_id_login(player_id, device_specific_id)
        .await?;
    client.set_session_token(&token);
    tracing::info!("ALS force_device_login done: player_id={}", player_id);
    Ok(())
}

pub async fn fetch_token(api_client: &ApiClient, live_id: &str, live_type: u64) -> Result<String> {
    if live_type == 2 {
        return api_client
            .high_level()
            .get_with_meets_connect_token(live_id)
            .await;
    }
    if live_type == 1 {
        let request = FesliveLobbyRequest {
            live_id: Some(live_id.to_string()),
            ..Default::default()
        };
        let _ = api_client.raw().fes_live().lobby(&request).await?;
        return api_client
            .high_level()
            .get_fes_live_connect_token(live_id)
            .await;
    }
    Err(anyhow::anyhow!(
        "unsupported live_type={}, expected 1(fes) or 2(meets)",
        live_type
    ))
}

pub fn is_connect_token_room_unavailable_error(err: &anyhow::Error) -> bool {
    const TARGET_CODE: &str = "14002_111604";
    const TARGET_MESSAGE: &str = "入室可能なライブのルームがありません。";

    let joined = format!("{:#}", err);
    let has_target_code =
        err.chain().any(|e| e.to_string().contains(TARGET_CODE)) || joined.contains(TARGET_CODE);
    let has_target_message = err.chain().any(|e| e.to_string().contains(TARGET_MESSAGE))
        || joined.contains(TARGET_MESSAGE);

    if !(has_target_code || has_target_message) {
        return false;
    }

    joined.contains("/withlive/connect_token")
        || joined.contains("/feslive/connect_token")
        || joined.contains("Get connect token failed")
}

pub fn get_connect_info(token: String) -> Result<ConnectionInfo> {
    let payload_json = extract_jwt_payload(token.as_str())?;
    Ok(ConnectionInfo {
        host: payload_json["pod"]["address"]
            .as_str()
            .context("missing pod.address in token payload")?
            .to_string(),
        port: payload_json["pod"]["port"].as_u64().unwrap_or(9201) as u16,
        room_id: payload_json["room_id"]
            .as_str()
            .context("missing room_id in token payload")?
            .to_string(),
        token,
    })
}
