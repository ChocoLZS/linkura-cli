use clap::Parser;

use config::init;

use linkura_api::ApiClient;
use linkura_common::log;
use linkura_i18n::t;

mod cli;
mod command;
mod config;
mod generated;

use crate::config::Commands;

linkura_i18n::init!();

#[tokio::main]
async fn main() {
    let args = config::Args::parse();
    // Commands that will not need to initialize
    match &args.command {
        Some(Commands::Version) => {
            let (res_version, app_version) = ApiClient::new()
                .high_level()
                .get_app_version()
                .await
                .expect("Fail to get versions");
            // we believe that all versions exist
            println!("{}", app_version.unwrap());
            println!("{}", res_version.unwrap());
            return;
        }
        _ => {}
    }

    if !args.quiet {
        log::init(args.log_level.clone());
    }

    match args.command.clone() {
        Some(Commands::API(api_args)) => {
            let global = init(args).await.expect(&t!("config.initialize.failed"));
            let _ = command::api::run(&global, &api_args).await.map_err(|e| {
                tracing::error!("Error running API command: {}", e);
                std::process::exit(1);
            });
        }
        Some(Commands::Mcp(mcp_args)) => {
            let global = config::init_non_interactive(args)
                .await
                .expect(&t!("config.initialize.failed"));
            let _ = command::mcp::run(&global, &mcp_args).await.map_err(|e| {
                tracing::error!("Error running MCP server: {}", e);
                std::process::exit(1);
            });
        }
        None => {
            let global = init(args).await.expect(&t!("config.initialize.failed"));
            command::default::run(&global).await;
        }
        _ => {
            unimplemented!("Unknown command");
        }
    }

    return;
}
