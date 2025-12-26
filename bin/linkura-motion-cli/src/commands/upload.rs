use anyhow::{Error, Result};
use linkura_downloader::R2Uploader;
use std::path::Path;
use tracing::info;

use crate::args::ArgsUpload;

pub async fn run(args: ArgsUpload, quiet: bool) -> Result<()> {
    let uploader = R2Uploader::from_env_or_args(
        args.account_id.clone(),
        args.access_key.clone(),
        args.secret_key.clone(),
        args.bucket.clone(),
        args.concurrent,
        !quiet,
    )
    .await?;

    let path = Path::new(&args.path);
    if !path.exists() {
        return Err(Error::msg(format!("Path does not exist: {}", args.path)));
    }

    let env_bucket = std::env::var("R2_BUCKET").ok();
    let bucket_name = args
        .bucket
        .as_ref()
        .map(|s| s.as_str())
        .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
        .unwrap_or("[from env]");

    if path.is_file() {
        info!(
            "🚀 Starting R2 file upload from '{}' to bucket '{}'",
            args.path, bucket_name
        );
        info!(
            "📊 Configuration: {} concurrent workers, {} mode",
            args.concurrent,
            if args.prefix.is_some() {
                format!("prefix: '{}'", args.prefix.as_ref().unwrap())
            } else {
                "no prefix".to_string()
            }
        );
        uploader
            .upload_file(path, args.prefix.as_deref())
            .await?;
    } else if path.is_dir() {
        info!(
            "🚀 Starting R2 folder upload from '{}' to bucket '{}'",
            args.path, bucket_name
        );
        info!(
            "📊 Configuration: {} concurrent workers, {} mode",
            args.concurrent,
            if args.prefix.is_some() {
                format!("prefix: '{}'", args.prefix.as_ref().unwrap())
            } else {
                "no prefix".to_string()
            }
        );
        uploader
            .upload_folder(path, args.prefix.as_deref())
            .await?;
    } else {
        return Err(Error::msg(format!(
            "Path is neither a file nor a directory: {}",
            args.path
        )));
    }
    info!("✅ Upload completed successfully!");
    Ok(())
}
