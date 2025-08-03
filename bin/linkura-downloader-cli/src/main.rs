// use i18n::t;

// i18n::init!();

use std::path::Path;
use anyhow::{Result, Error};
use clap::{Args as ClapArgs, Parser, Subcommand};
use i18n::t;

i18n::init!();

use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader};

mod r2_uploader;
use r2_uploader::R2Uploader;


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
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(ArgsDownload),
    Upload(ArgsUpload),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let quiet = args.quiet;
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

            println!("Starting upload from {} to bucket {}", upload_args.folder, bucket_name);
            uploader.upload_folder(folder_path, upload_args.prefix.as_deref()).await?;
            println!("Upload completed successfully!");
        },
        None => {},
    }
    Ok(())
}
