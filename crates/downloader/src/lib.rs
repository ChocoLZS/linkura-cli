pub mod downloader;
pub mod als_downloader;
pub mod mrs_downloader;

#[cfg(test)]
mod tests;

pub use downloader::Downloader;
pub use als_downloader::AlsDownloader;
pub use mrs_downloader::MrsDownloader;