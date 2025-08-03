// use i18n::t;

// i18n::init!();

use std::path::Path;
use anyhow::{Result, Error};
use clap::{Args as ClapArgs, Parser, Subcommand};
use i18n::t;

i18n::init!();

use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader};


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
    #[clap(short('h'), long = "help", help = t!("downloader.cli.command.upload.args.help").to_string())]
    pub help: bool,
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
        Some(Commands::Upload(ref upload_args)) => {},
        None => {},
    }
    Ok(())
    // let downloader = AlsDownloader::with_progress(16, true);
    // downloader
    // .download(URL, Path::new("data")).await.map_err(|e| {
    //     Error::msg(format!("Error downloading file, maybe the url provided is not fit for als format: {}", e))
    // })
    // println!("{}", t!("hello_world"));
}
