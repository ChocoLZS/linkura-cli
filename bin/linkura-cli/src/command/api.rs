use crate::config::Global;
use anyhow::Result;
use clap::{Args as ClapArgs, Subcommand};
use linkura_api::ArchiveListOptions;

use linkura_i18n::t;

#[derive(Debug, Clone, ClapArgs)]
pub struct ArgsAPI {
    #[clap(short('o'), long = "output", value_name = "OUTPUT", help = t!("linkura.command.api.args.output.about").to_string())]
    pub output: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = t!("linkura.command.api.subcommand.archive.about").to_string())]
    Archive(ArgsArchive),
    #[command(about = t!("linkura.command.api.subcommand.archive_details.about").to_string())]
    ArchiveDetails(ArgsArchiveDetails),
}

#[derive(Debug, Clone, ClapArgs)]
pub struct ArgsArchive {
    #[clap(short('l'), long = "limit", value_name = "LIMIT", help = t!("linkura.command.api.subcommand.archive.args.limit.about").to_string())]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, ClapArgs)]
pub struct ArgsArchiveDetails {
    #[clap(short('i'), long = "id", value_name = "ID", help = t!("linkura.command.api.subcommand.archive_details.args.id.about").to_string())]
    pub id: String,
    #[clap(short('t'), long = "type", value_name = "LIVE TYPE", help = t!("linkura.command.api.subcommand.archive_details.args.type.about").to_string())]
    pub live_type: u8,
}

pub async fn run(ctx: &Global, args: &ArgsAPI) -> Result<()> {
    let api_client = &ctx.api_client;
    let save_json = &args.output.clone().unwrap_or_default();
    match &args.command {
        Commands::Archive(archive_args) => {
            let archives = api_client
                .high_level()
                .get_archive_list(ArchiveListOptions {
                    limit: archive_args.limit,
                    ..Default::default()
                })
                .await?;
            if !save_json.is_empty() {
                std::fs::write(save_json, serde_json::to_string_pretty(&archives)?)?;
                tracing::info!("{}", t!("linkura.command.api.archive.saved", path = save_json));
            } else {
                tracing::info!(
                    "{}",
                    t!(
                        "linkura.command.api.archives.output",
                        json = serde_json::to_string_pretty(&archives)?
                    )
                );
            }
        }
        Commands::ArchiveDetails(details_args) => {
            let live_id = &details_args.id;
            let live_type = details_args.live_type;
            let details = api_client
                .high_level()
                .get_archive_details(live_id, live_type)
                .await?;
            if !save_json.is_empty() {
                std::fs::write(save_json, serde_json::to_string_pretty(&details)?)?;
                tracing::info!(
                    "{}",
                    t!("linkura.command.api.archive_details.saved", path = save_json)
                );
            } else {
                tracing::info!(
                    "{}",
                    t!(
                        "linkura.command.api.archive_details.output",
                        json = serde_json::to_string_pretty(&details)?
                    )
                );
            }
            //     tracing::info!("Archive details: {}", serde_json::to_string_pretty(&archive_details)?);
            // }
        }
    }
    Ok(())
}
