use anyhow::{Error, Result};
use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader, R2Uploader};
use std::ops::Deref;
use std::path::Path;
use tracing::{info, warn};

use crate::args::ArgsSync;
use crate::utils::get_bucket_prefix;

pub async fn run(args: ArgsSync, quiet: bool) -> Result<()> {
    // Step 1: Download
    info!("🔄 Starting sync operation: Download + Upload");
    let download_url = args.download_url.trim();
    info!("📥 Phase 1: Downloading from '{}'", download_url);

    let download_dir = args.download_directory.as_deref().unwrap_or("data");
    let download_path = Path::new(download_dir);

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
        Some("als") => Box::new(AlsDownloader::with_progress(
            args.download_parallel,
            !quiet,
        )),
        Some("mrs") => Box::new(MrsDownloader::with_progress(
            args.download_parallel,
            !quiet,
        )),
        _ => {
            return Err(Error::msg(format!(
                "Unknown download type: {:?}",
                args.download_type
            )));
        }
    };

    downloader.download(download_url, download_path).await?;

    info!("✅ Download completed successfully!");
    info!("📤 Phase 2: Uploading to R2");

    // Step 2: Upload
    let uploader = R2Uploader::from_env_or_args(
        args.account_id.clone(),
        args.access_key.clone(),
        args.secret_key.clone(),
        args.bucket.clone(),
        args.upload_concurrent,
        !quiet,
    )
    .await?;

    if !download_path.exists() {
        return Err(Error::msg(format!(
            "Download directory does not exist: {}",
            download_dir
        )));
    }

    let folder_name = downloader.extract_folder_name(download_url)?;
    let target_folder = download_path.join(folder_name);
    let env_bucket = std::env::var("R2_BUCKET").ok();
    let bucket_name = args
        .bucket
        .as_ref()
        .map(|s| s.as_str())
        .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
        .unwrap_or("[from env]");

    info!(
        "🚀 Starting R2 upload from '{}' to bucket '{}'",
        target_folder.display(),
        bucket_name
    );
    info!(
        "📊 Configuration: {} concurrent workers, {} mode",
        args.upload_concurrent,
        if args.prefix.is_some() {
            format!("prefix: '{}'", args.prefix.as_ref().unwrap())
        } else {
            "no prefix".to_string()
        }
    );
    uploader
        .upload_folder(
            &target_folder,
            Some(
                args.prefix
                    .clone()
                    .unwrap_or(get_bucket_prefix(download_url)?)
                    .deref(),
            ),
        )
        .await?;

    // Delete downloaded files if requested
    if args.delete_after_done {
        info!("🗑️ Deleting downloaded files after successful upload...");
        if let Err(e) = std::fs::remove_dir_all(&target_folder) {
            warn!(
                "Failed to delete downloaded folder '{}': {}",
                target_folder.display(),
                e
            );
        } else {
            info!("✅ Downloaded files deleted successfully");
        }
    }

    info!("✅ Sync operation completed successfully!");
    info!("🎉 Download + Upload finished!");
    Ok(())
}
