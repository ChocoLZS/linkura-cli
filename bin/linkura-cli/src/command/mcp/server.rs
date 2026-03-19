use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use linkura_api::ApiClient;
use linkura_i18n::t;
use rmcp::{
    ServerHandler, ServiceExt, tool_handler, transport::stdio,
    handler::server::router::tool::ToolRouter,
    model::{ServerCapabilities, ServerInfo},
    transport::streamable_http_server::{
        StreamableHttpServerConfig,
        StreamableHttpService,
        session::local::LocalSessionManager,
    },
};

use crate::config::Global;

use super::{metadata, resources, state::McpState, tools};

#[derive(Debug, Clone)]
pub struct LinkuraMcpServer {
    pub state: Arc<McpState>,
    pub api_client: Arc<ApiClient>,
    pub tool_router: ToolRouter<Self>,
}

impl LinkuraMcpServer {
    pub fn new(global: &Global) -> Self {
        let mut api_client = ApiClient::new();
        api_client.update_with_credential(&global.config.credential);
        if let Some(session_token) = &global.config.credential.session_token {
            api_client.set_session_token(session_token);
        }

        Self {
            state: Arc::new(McpState::from_global(global)),
            api_client: Arc::new(api_client),
            tool_router: tools::router(),
        }
    }

    pub async fn serve_stdio(self) -> Result<()> {
        self.serve(stdio()).await?;
        Ok(())
    }

    pub async fn serve_http(self, port: u16) -> Result<()> {
        let bind_addr = format!("127.0.0.1:{port}");
        let endpoint_path = self.state.http_endpoint_path;

        let service = StreamableHttpService::new(
            move || Ok(self.clone()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default(),
        );

        let router = Router::new().nest_service(endpoint_path, service);
        let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

        tracing::info!(
            "{}",
            t!(
                "linkura.command.mcp.server.http.listening",
                bind_addr = bind_addr,
                endpoint_path = endpoint_path
            )
        );

        axum::serve(listener, router).await?;
        Ok(())
    }

    fn capabilities(&self) -> ServerCapabilities {
        let has_tools = !self.tool_router.list_all().is_empty();
        let has_resources = resources::has_resources();

        match (has_tools, has_resources) {
            (true, true) => ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            (true, false) => ServerCapabilities::builder().enable_tools().build(),
            (false, true) => ServerCapabilities::builder().enable_resources().build(),
            (false, false) => ServerCapabilities::default(),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for LinkuraMcpServer {
    fn get_info(&self) -> ServerInfo {
        metadata::server_info(&self.state, self.capabilities())
    }
}
