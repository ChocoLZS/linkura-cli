mod metadata;
mod resources;
mod server;
mod state;
mod tools;

use anyhow::Result;
use clap::Args as ClapArgs;

use crate::config::Global;

use linkura_i18n::t;

pub use server::LinkuraMcpServer;

#[derive(Debug, Clone, ClapArgs)]
pub struct ArgsMcp {
    #[arg(long = "http", default_value_t = false, help = t!("linkura.command.mcp.args.http.about").to_string())]
    pub http: bool,
    #[arg(long = "port", default_value_t = 31023, help = t!("linkura.command.mcp.args.port.about").to_string())]
    pub port: u16,
}

impl Default for ArgsMcp {
    fn default() -> Self {
        Self {
            http: false,
            port: 31023,
        }
    }
}

pub async fn run(ctx: &Global, args: &ArgsMcp) -> Result<()> {
    let server = LinkuraMcpServer::new(ctx);
    if args.http {
        server.serve_http(args.port).await
    } else {
        server.serve_stdio().await
    }
}
