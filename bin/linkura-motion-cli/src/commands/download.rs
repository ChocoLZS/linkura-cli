use anyhow::{Error, Result};
use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader};
use std::path::Path;

use crate::args::ArgsDownload;

pub async fn run(args: ArgsDownload, quiet: bool) -> Result<()> {
    let download_url = args.download_url.trim();
    let mut download_type = args.download_type.clone();
    if download_type.is_none() {
        if download_url.ends_with(".md") {
            download_type = Some("als".into());
        }
        if download_url.ends_with(".iarc") {
            download_type = Some("mrs".into());
        }
    }
    let downloader: Box<dyn BaseDownloader> = match download_type.as_deref() {
        Some("als") => Box::new(AlsDownloader::with_progress(args.parallel, !quiet)),
        Some("mrs") => Box::new(MrsDownloader::with_progress(args.parallel, !quiet)),
        _ => {
            return Err(Error::msg(format!(
                "Unknown download type: {:?}",
                args.download_type
            )));
        }
    };
    let download_dir = args.download_directory.as_deref().unwrap_or("data");
    downloader
        .download(download_url, Path::new(download_dir))
        .await?;
    Ok(())
}
