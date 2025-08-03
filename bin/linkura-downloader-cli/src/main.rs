// use i18n::t;

// i18n::init!();

use std::path::Path;
use anyhow::{Result, Error};
use clap::{Args as ClapArgs, Parser, Subcommand};
use i18n::t;
use tracing::{info, warn};

i18n::init!();

use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader, R2Uploader};
use linkura_common::log;


/** ARG PARSER **/
#[derive(Parser, Debug)]
#[command(
    name = "linkura-downloader-cli",
    version = "0.0.0",
    author = "ChocoLZS, chocoielzs@outlook.com",
    about = t!("downloader.cli.about").to_string(),
    // long_about = None,
    bin_name = "linkura-downloader-cli",
)]
pub struct Args {
    #[clap(short('q'), long = "quiet", help = t!("downloader.cli.args.quiet").to_string(), default_value = "false")]
    pub quiet: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsDownload {
    #[clap(short('t'), long = "type", value_name = "TYPE", help = t!("downloader.cli.command.download.args.type").to_string())]
    pub download_type: Option<String>,
    #[clap(short('d'),long = "directory",value_name = "DIRECTORY",help = t!("downloader.cli.command.download.args.directory").to_string())]
    pub download_directory: Option<String>,
    #[clap(value_name = "URL",help = t!("downloader.cli.command.download.args.url").to_string())]
    pub download_url: String,
    #[clap(short('p'), long = "parallel", help = t!("downloader.cli.command.download.args.parallel").to_string(), default_value = "16")]
    pub parallel: usize,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsUpload {
    #[clap(short('b'), long = "bucket", value_name = "BUCKET", help = t!("downloader.cli.command.upload.args.bucket").to_string())]
    pub bucket: Option<String>,
    #[clap(short('a'), long = "account-id", value_name = "ACCOUNT_ID", help = t!("downloader.cli.command.upload.args.account_id").to_string())]
    pub account_id: Option<String>,
    #[clap(short('k'), long = "access-key", value_name = "ACCESS_KEY", help = t!("downloader.cli.command.upload.args.access_key").to_string())]
    pub access_key: Option<String>,
    #[clap(short('s'), long = "secret-key", value_name = "SECRET_KEY", help = t!("downloader.cli.command.upload.args.secret_key").to_string())]
    pub secret_key: Option<String>,
    #[clap(short('f'), long = "folder", value_name = "FOLDER", help = t!("downloader.cli.command.upload.args.folder").to_string())]
    pub folder: String,
    #[clap(short('p'), long = "prefix", value_name = "PREFIX", help = t!("downloader.cli.command.upload.args.prefix").to_string())]
    pub prefix: Option<String>,
    #[clap(short('c'), long = "concurrent", value_name = "CONCURRENT", help = t!("downloader.cli.command.upload.args.concurrent").to_string(), default_value = "4")]
    pub concurrent: usize,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsSync {
    // Download parameters
    #[clap(short('t'), long = "type", value_name = "TYPE", help = t!("downloader.cli.command.download.args.type").to_string())]
    pub download_type: Option<String>,
    #[clap(short('d'), long = "directory", value_name = "DIRECTORY", help = t!("downloader.cli.command.download.args.directory").to_string())]
    pub download_directory: Option<String>,
    #[clap(value_name = "URL", help = t!("downloader.cli.command.download.args.url").to_string())]
    pub download_url: String,
    #[clap(long = "download-parallel", help = t!("downloader.cli.command.download.args.parallel").to_string(), default_value = "16")]
    pub download_parallel: usize,
    
    // Upload parameters
    #[clap(short('b'), long = "bucket", value_name = "BUCKET", help = t!("downloader.cli.command.upload.args.bucket").to_string())]
    pub bucket: Option<String>,
    #[clap(short('a'), long = "account-id", value_name = "ACCOUNT_ID", help = t!("downloader.cli.command.upload.args.account_id").to_string())]
    pub account_id: Option<String>,
    #[clap(short('k'), long = "access-key", value_name = "ACCESS_KEY", help = t!("downloader.cli.command.upload.args.access_key").to_string())]
    pub access_key: Option<String>,
    #[clap(short('s'), long = "secret-key", value_name = "SECRET_KEY", help = t!("downloader.cli.command.upload.args.secret_key").to_string())]
    pub secret_key: Option<String>,
    #[clap(short('p'), long = "prefix", value_name = "PREFIX", help = t!("downloader.cli.command.upload.args.prefix").to_string())]
    pub prefix: Option<String>,
    #[clap(short('c'), long = "concurrent", value_name = "CONCURRENT", help = t!("downloader.cli.command.upload.args.concurrent").to_string(), default_value = "4")]
    pub upload_concurrent: usize,
    
    // Additional options
    #[clap(long = "delete-after-done", help = t!("downloader.cli.command.sync.args.delete_after_done").to_string())]
    pub delete_after_done: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(ArgsDownload),
    Upload(ArgsUpload),
    Sync(ArgsSync),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let quiet = args.quiet;
    if !quiet {
        log::init(None);
    }
    match args.command {
        Some(Commands::Download(ref download_args)) => {
            let mut download_type = download_args.download_type.clone();
            if download_type.is_none() {
                if download_args.download_url.ends_with(".md") {
                    download_type = Some("als".into());
                }
                if download_args.download_url.ends_with(".iarc") {
                    download_type = Some("mrs".into());
                }
            }
            let downloader: Box<dyn BaseDownloader> = match download_type.as_deref() {
                Some("als") => Box::new(AlsDownloader::with_progress(download_args.parallel, !quiet)),
                Some("mrs") => Box::new(MrsDownloader::with_progress(download_args.parallel, !quiet)),
                _ => {
                    return Err(Error::msg(format!("Unknown download type: {:?}", download_args.download_type)));
                }
            };
            
            downloader.download(&download_args.download_url, Path::new("data")).await?;
        },
        Some(Commands::Upload(ref upload_args)) => {
            let uploader = R2Uploader::from_env_or_args(
                upload_args.account_id.clone(),
                upload_args.access_key.clone(),
                upload_args.secret_key.clone(),
                upload_args.bucket.clone(),
                upload_args.concurrent,
                !quiet,
            ).await?;

            let folder_path = Path::new(&upload_args.folder);
            if !folder_path.exists() {
                return Err(Error::msg(format!("Folder does not exist: {}", upload_args.folder)));
            }

            let env_bucket = std::env::var("R2_BUCKET").ok();
            let bucket_name = upload_args.bucket.as_ref()
                .map(|s| s.as_str())
                .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
                .unwrap_or("[from env]");

            info!("🚀 Starting R2 upload from '{}' to bucket '{}'", 
                upload_args.folder, bucket_name);
            info!("📊 Configuration: {} concurrent workers, {} mode",
                upload_args.concurrent,
                if upload_args.prefix.is_some() { 
                    format!("prefix: '{}'", upload_args.prefix.as_ref().unwrap()) 
                } else { 
                    "no prefix".to_string() 
                });
            uploader.upload_folder(folder_path, upload_args.prefix.as_deref()).await?;
            info!("✅ Upload completed successfully!");
        },
        Some(Commands::Sync(ref sync_args)) => {
            // Step 1: Download
            info!("🔄 Starting sync operation: Download + Upload");
            info!("📥 Phase 1: Downloading from '{}'", sync_args.download_url);

            let download_dir = sync_args.download_directory.as_deref().unwrap_or("data");
            let download_path = Path::new(download_dir);

            let mut download_type = sync_args.download_type.clone();
            if download_type.is_none() {
                if sync_args.download_url.ends_with(".md") {
                    download_type = Some("als".into());
                }
                if sync_args.download_url.ends_with(".iarc") {
                    download_type = Some("mrs".into());
                }
            }

            let downloader: Box<dyn BaseDownloader> = match download_type.as_deref() {
                Some("als") => Box::new(AlsDownloader::with_progress(sync_args.download_parallel, !quiet)),
                Some("mrs") => Box::new(MrsDownloader::with_progress(sync_args.download_parallel, !quiet)),
                _ => {
                    return Err(Error::msg(format!("Unknown download type: {:?}", sync_args.download_type)));
                }
            };
            
            downloader.download(&sync_args.download_url, download_path).await?;

            info!("✅ Download completed successfully!");
            info!("📤 Phase 2: Uploading to R2");

            // Step 2: Upload
            let uploader = R2Uploader::from_env_or_args(
                sync_args.account_id.clone(),
                sync_args.access_key.clone(),
                sync_args.secret_key.clone(),
                sync_args.bucket.clone(),
                sync_args.upload_concurrent,
                !quiet,
            ).await?;

            if !download_path.exists() {
                return Err(Error::msg(format!("Download directory does not exist: {}", download_dir)));
            }

            let folder_name = downloader.extract_folder_name(&sync_args.download_url)?;
            let target_folder = download_path.join(folder_name);
            let env_bucket = std::env::var("R2_BUCKET").ok();
            let bucket_name = sync_args.bucket.as_ref()
                .map(|s| s.as_str())
                .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
                .unwrap_or("[from env]");

            info!("🚀 Starting R2 upload from '{}' to bucket '{}'", 
                target_folder.display(), bucket_name);
            info!("📊 Configuration: {} concurrent workers, {} mode",
                sync_args.upload_concurrent,
                if sync_args.prefix.is_some() { 
                    format!("prefix: '{}'", sync_args.prefix.as_ref().unwrap()) 
                } else { 
                    "no prefix".to_string() 
                });

            uploader.upload_folder(&target_folder, sync_args.prefix.as_deref()).await?;

            // Delete downloaded files if requested
            if sync_args.delete_after_done {
                info!("🗑️ Deleting downloaded files after successful upload...");
                if let Err(e) = std::fs::remove_dir_all(&target_folder) {
                    warn!("Failed to delete downloaded folder '{}': {}", target_folder.display(), e);
                } else {
                    info!("✅ Downloaded files deleted successfully");
                }
            }

            info!("✅ Sync operation completed successfully!");
            info!("🎉 Download + Upload finished!");
        },
        None => {},
    }
    Ok(())
}
