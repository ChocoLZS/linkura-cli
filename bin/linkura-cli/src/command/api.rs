use clap::{Args as ClapArgs, Subcommand};
use crate::config::Global;
use anyhow::Result;

#[derive(Debug, ClapArgs)]
pub struct ArgsAPI {
    #[clap(short('s'), long = "save-json", value_name = "SAVE_JSON")]
    /// if provided, will save the API response to the file
    /// with the given name, otherwise will just print the API response info
    /// to the console.
    pub save_json: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Archive(ArgsArchive),
    ArchiveDetails(ArgsArchiveDetails),
}

#[derive(Debug, ClapArgs)]
pub struct ArgsArchive {
    /// The maximum number of archives to return, default is 4
    #[clap(short('l'), long = "limit", value_name = "LIMIT")]
    pub limit: Option<u32>,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsArchiveDetails {
    /// The ID of the archive to retrieve details for
    #[clap(short('i'), long = "id", value_name = "ID")]
    /// 1 for fes live, 2 for with meets
    pub id: String,
    #[clap(short('t'), long = "type", value_name = "LIVE TYPE")]
    pub live_type: u8,
}

pub fn run(ctx: &Global, args: &ArgsAPI) -> Result<()> {
    let api_client = &ctx.api_client;
    let save_json = &args.save_json.clone().unwrap_or_default();
    match &args.command {
        Commands::Archive(archive_args) => {
            let archives = api_client.high_level().get_archive_list(archive_args.limit)?;
            if !save_json.is_empty() {
                std::fs::write(save_json, serde_json::to_string_pretty(&archives)?)?;
                tracing::info!("Archive saved to {}", save_json);
            } else {
                tracing::info!("Archives: {}", serde_json::to_string_pretty(&archives)?);
            }
        }
        Commands::ArchiveDetails(details_args) => {
            let live_id = &details_args.id;
            let live_type = details_args.live_type;
            let details = api_client.high_level().get_archive_details(live_id, live_type)?;
            if !save_json.is_empty() {
                std::fs::write(save_json, serde_json::to_string_pretty(&details)?)?;
                tracing::info!("Archive details saved to {}", save_json);
            } else {
                tracing::info!("Archive details: {}", serde_json::to_string_pretty(&details)?);
            }
            //     tracing::info!("Archive details: {}", serde_json::to_string_pretty(&archive_details)?);
            // }
        }
    }
    Ok(())
}
