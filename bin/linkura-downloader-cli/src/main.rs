// use i18n::t;

// i18n::init!();

use std::path::Path;
use anyhow::{Result, Error};

use linkura_downloader::{AlsDownloader, MrsDownloader};


static URL: &str = "https://assets.link-like-lovelive.app/archive/mrs/20250519202000-a42f15a10899bb2890546585014eaaf3/archive_86153322_20250519202000-994578.iarc";

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Args::parse();
    let downloader = MrsDownloader::new(16);
    downloader
    .download(URL, Path::new("data")).await.map_err(|e| {
        Error::msg(format!("Error downloading file, maybe the url provided is not fit for als format: {}", e))
    })
    // println!("{}", t!("hello_world"));
}
