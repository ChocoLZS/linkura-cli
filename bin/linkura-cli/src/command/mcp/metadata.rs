use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};

use super::state::McpState;

const SERVER_NAME: &str = "linkura-cli";
const SERVER_TITLE: &str = "Linkura MCP";
const SERVER_DESCRIPTION: &str = "MCP server for Linkura CLI";

pub fn server_info(state: &McpState, capabilities: ServerCapabilities) -> ServerInfo {
    ServerInfo::new(capabilities)
        .with_instructions(instructions(state))
        .with_server_info(
            Implementation::new(SERVER_NAME, env!("CARGO_PKG_VERSION"))
                .with_title(SERVER_TITLE)
                .with_description(SERVER_DESCRIPTION),
        )
}

fn instructions(state: &McpState) -> String {
    format!(
        "Linkura MCP server is running. Authenticated as player {} with client version {} and resource version {}. Config path: {}. Started at: {}. Default HTTP endpoint path is {}. Tools and resources can be registered incrementally on top of this server skeleton.",
        state.player_id,
        state.client_version,
        state.res_version,
        state.config_path.display(),
        state.started_at.to_rfc3339(),
        state.http_endpoint_path
    )
}
