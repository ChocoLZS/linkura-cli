use clap::Parser;

use config::init;

use linkura_api::ApiClient;
use linkura_common::log;
use linkura_i18n::t;

mod cli;
mod command;
mod config;

use crate::config::Commands;

linkura_i18n::init!();

fn main() {
    let args = config::Args::parse();
    // Commands that will not need to initialize
    match &args.command {
        Some(Commands::Version) => {
            let (res_version, app_version) = ApiClient::new().high_level().get_app_version().expect(
                "Fail to get versions"
            );
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

    let global = init(args).expect(&t!("config.initialize.failed"));

    match &global.args.command {
        Some(Commands::API(args)) => {
            let _ = command::api::run(&global, &args).map_err(|e| {
                tracing::error!("Error running API command: {}", e);
                std::process::exit(1);
            });
        }
        None => {
            command::default::run(&global);
        }
        _ => {
            unimplemented!("Unknown command");
        }
    }

    return;
}
