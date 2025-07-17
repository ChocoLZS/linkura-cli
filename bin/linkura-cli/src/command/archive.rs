use crate::config::{ArgsArchive, Commands, Global};
use anyhow::Result;

pub fn run(ctx: &Global, args: &ArgsArchive) -> Result<()> {
    // let api_client = &ctx.api_client;
    // let hls_url = api_client.assets().get_hls_url_from_archive(&args.save_name)?;
    // println!("HLS URL: {}", hls_url);
    Ok(())
}
