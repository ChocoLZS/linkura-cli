use crate::config::{ArgsArchive, Global};
use anyhow::Result;

pub fn run(ctx: &Global, args: &ArgsArchive) -> Result<()> {
    let api_client = &ctx.api_client;
    let archives = api_client.high_level().get_archive_list(args.limit)?;
    if let Some(save_json) = &args.save_json {
        std::fs::write(save_json, serde_json::to_string_pretty(&archives)?)?;
        tracing::info!("Archive saved to {}", save_json);
    } else {
        tracing::info!("Archives: {}", serde_json::to_string_pretty(&archives)?);
    }
    Ok(())
}
