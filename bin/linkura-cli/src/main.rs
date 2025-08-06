use clap::Parser;

use config::init;

use linkura_common::log;

rust_i18n::i18n!("../../locales", fallback = "en");

mod command;
mod config;
mod cli;

use rust_i18n::t;

use crate::config::{Commands, AlsCommands};
fn main() {
    let args = config::Args::parse();
    if !args.quiet {
        log::init(args.log_level.clone());
    }

    let global = init(args).expect(&t!("config.initialize.failed"));
    let args = &global.args;

    match &args.command {
        Some(Commands::MRS(_)) => {
            todo!("Implement MRS client command handling");
        }
        Some(Commands::ALS(args)) => {
            match &args.command {
                Some(AlsCommands::Connect(connect_args)) => {
                    let _ = command::als::run(
                        &global,
                        command::als::AlsConnectionInfo {
                            address: connect_args.addr.clone(),
                            port: connect_args.port,
                            room_id: connect_args.room_id.clone(),
                            token: connect_args.token.clone(),
                        },
                        connect_args.watch,
                    )
                    .map_err(|e| {
                        tracing::error!("Error running ALS connect command: {}", e);
                        std::process::exit(1);
                    });
                }
                Some(AlsCommands::Analyze(analyze_args)) => {
                    let _ = command::als::analyze(
                        &analyze_args.file_path, 
                        analyze_args.output_path.as_deref(),
                        analyze_args.packet_count
                    ).map_err(|e| {
                        tracing::error!("Error running ALS analyze command: {}", e);
                        std::process::exit(1);
                    });
                }
                Some(AlsCommands::AnalyzeMixed(analyze_args)) => {
                    let _ = command::als::analyze_mixed(
                        &analyze_args.file_path, 
                        analyze_args.output_path.as_deref(),
                        analyze_args.packet_count
                    ).map_err(|e| {
                        tracing::error!("Error running ALS analyze-mixed command: {}", e);
                        std::process::exit(1);
                    });
                }
                None => {
                    eprintln!("Please specify a subcommand for ALS. Use --help for more information.");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Archive(args)) => {
            let _ = command::archive::run(&global, &args).map_err(|e| {
                tracing::error!("Error running Archive command: {}", e);
                std::process::exit(1);
            });
        }
        None => {
            command::default::run(&global);
        }
    }

    return;
}
