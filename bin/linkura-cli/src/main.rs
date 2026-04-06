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
    let cli_args = config::Args::parse();
    // Commands that will not need to initialize
    match &cli_args.command {
        Some(Commands::Version) => {
            let (res_version, app_version) = ApiClient::new()
                .high_level()
                .get_app_version()
                .await
                .expect(&t!("linkura.main.version.fetch.failed"));
            // we believe that all versions exist
            println!("{}", app_version.unwrap());
            println!("{}", res_version.unwrap());
            return;
        }
        _ => {}
    }

    if !cli_args.quiet {
        log::init(cli_args.log_level.clone());
    }

    match cli_args.command.clone() {
        Some(Commands::MRS(mrs_args)) => {
            let global = init(cli_args.clone())
                .await
                .expect(&t!("common.config.initialize.failed"));
            let _ = command::mrs::run(&global, &mrs_args).map_err(|e| {
                tracing::error!("Error running MRS command: {}", e);
                std::process::exit(1);
            });
        }
        Some(Commands::ALS(als_args)) => {
            let mut global = init(cli_args.clone())
                .await
                .expect(&t!("common.config.initialize.failed"));
            let _ = command::als::run(&mut global, &als_args).await.map_err(|e| {
                tracing::error!("Error running ALS command: {}", e);
                std::process::exit(1);
            });
        }
        Some(Commands::API(api_args)) => {
            let global = init(cli_args.clone())
                .await
                .expect(&t!("common.config.initialize.failed"));
            let _ = command::api::run(&global, &api_args).await.map_err(|e| {
                tracing::error!(
                    "{}",
                    t!("linkura.main.command.api.run.failed", error = e.to_string())
                );
                std::process::exit(1);
            });
        }
        Some(Commands::Mcp(mcp_args)) => {
            let global = config::init_non_interactive(cli_args.clone())
                .await
                .expect(&t!("common.config.initialize.failed"));
            let _ = command::mcp::run(&global, &mcp_args).await.map_err(|e| {
                tracing::error!(
                    "{}",
                    t!("linkura.main.command.mcp.run.failed", error = e.to_string())
                );
                std::process::exit(1);
            });
        }
        None => {
            let global = init(cli_args.clone())
                .await
                .expect(&t!("common.config.initialize.failed"));
            command::default::run(&global).await;
        }
        _ => {
            unimplemented!("{}", t!("linkura.main.command.unknown"));
        }
    }

    return;
}
