use clap::Parser;

use config::init;

use linkura_common::log;

rust_i18n::i18n!("../../locales", fallback = "en");

mod cli;
mod command;
mod config;

use rust_i18n::t;

use crate::config::Commands;
fn main() {
    let args = config::Args::parse();
    if !args.quiet {
        log::init(args.log_level.clone());
    }

    let global = init(args).expect(&t!("config.initialize.failed"));
    let args = &global.args;

    match &args.command {
        Some(Commands::API(args)) => {
            let _ = command::api::run(&global, &args).map_err(|e| {
                tracing::error!("Error running API command: {}", e);
                std::process::exit(1);
            });
        }
        None => {
            command::default::run(&global);
        }
    }

    return;
}
