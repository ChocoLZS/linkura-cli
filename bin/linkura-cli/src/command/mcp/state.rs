use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::config::Global;

#[derive(Debug, Clone)]
pub struct McpState {
    pub player_id: String,
    pub client_version: String,
    pub res_version: String,
    pub config_path: PathBuf,
    pub http_endpoint_path: &'static str,
    pub started_at: DateTime<Utc>,
}

impl McpState {
    pub fn from_global(global: &Global) -> Self {
        Self {
            player_id: global.config.credential.player_id.clone(),
            client_version: global.config.credential.client_version.clone(),
            res_version: global.config.credential.res_version.clone(),
            config_path: global.config_manager.get_config_path().clone(),
            http_endpoint_path: "/mcp",
            started_at: Utc::now(),
        }
    }
}
