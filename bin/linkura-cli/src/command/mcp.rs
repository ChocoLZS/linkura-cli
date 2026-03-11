use anyhow::Result;
use clap::Args as ClapArgs;
use rmcp::{
    ServerHandler, ServiceExt,
    model::{Implementation, ServerCapabilities, ServerInfo},
};

use crate::config::Global;

#[derive(Debug, Clone, Default, ClapArgs)]
pub struct ArgsMcp {}

#[derive(Debug, Clone, Default)]
struct LinkuraMcpServer;

impl ServerHandler for LinkuraMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::default())
            .with_instructions(
                "Minimal Linkura MCP server. Tools and resources are not registered yet."
                    .to_string(),
            )
            .with_server_info(Implementation::new(
                "linkura-cli",
                env!("CARGO_PKG_VERSION"),
            ))
    }
}

pub async fn run(_ctx: &Global, _args: &ArgsMcp) -> Result<()> {
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let server = LinkuraMcpServer;
    server.serve(transport).await?;
    Ok(())
}
