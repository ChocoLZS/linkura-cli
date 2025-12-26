use anyhow::Result;
use clap::Parser;
use linkura_common::log;

mod args;
mod commands;
mod utils;

use args::{Args, Commands};

linkura_i18n::init!();

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let quiet = args.quiet;
    if !quiet {
        log::init(None);
    }
    match args.command {
        Some(Commands::Download(args)) => commands::download::run(args, quiet).await?,
        Some(Commands::Upload(args)) => commands::upload::run(args, quiet).await?,
        Some(Commands::Sync(args)) => commands::sync::run(args, quiet).await?,
        Some(Commands::Analyze(args)) => commands::analyze::run(args)?,
        Some(Commands::Convert(args)) => commands::convert::run(args)?,
        Some(Commands::Edit(args)) => commands::edit::run(args)?,
        #[cfg(feature = "audio")]
        Some(Commands::Audio(args)) => commands::audio::run(args).await?,
        None => {}
    }
    Ok(())
}