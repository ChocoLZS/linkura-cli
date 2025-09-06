use crate::downloader::{BaseDownloader, BaseDownloaderImpl, DownloadItem, ProgressConfig};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct AlsMetadata {
    pub path: String,
    pub room_id: String,
    pub playlist_file: String,
    pub live_started_at: String,
    pub joined_room_at: String,
}

pub struct AlsDownloader {
    base: BaseDownloaderImpl,
}

impl Default for AlsDownloader {
    fn default() -> Self {
        Self::new(16)
    }
}

impl AlsDownloader {
    pub fn new(concurrent_downloads: usize) -> Self {
        Self {
            base: ProgressConfig::new(concurrent_downloads),
        }
    }

    pub fn with_progress(concurrent_downloads: usize, show_progress: bool) -> Self {
        Self {
            base: ProgressConfig::with_progress(concurrent_downloads, show_progress),
        }
    }

    async fn fetch_metadata(&self, url: &str) -> Result<AlsMetadata> {
        let response = self.base.client().get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch metadata: HTTP {}", response.status()));
        }

        let text = response.text().await?;
        let metadata: AlsMetadata = serde_json::from_str(&text)
            .map_err(|e| anyhow!("Failed to parse JSON metadata: {}", e))?;

        Ok(metadata)
    }

    async fn fetch_m3u8_content(&self, url: &str) -> Result<String> {
        let response = self.base.client().get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch m3u8 file: HTTP {}", response.status()));
        }

        Ok(response.text().await?)
    }

    pub fn parse_m3u8_segments(&self, content: &str) -> Result<Vec<String>> {
        let mut segments = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                if line.ends_with(".ts") {
                    segments.push(line.to_string());
                }
            }
        }

        if segments.is_empty() {
            return Err(anyhow!("No .ts segments found in m3u8 file"));
        }

        Ok(segments)
    }
}

#[async_trait]
impl BaseDownloader for AlsDownloader {
    async fn download(&self, url: &str, output_dir: &Path) -> Result<()> {
        let metadata = self.fetch_metadata(url).await?;
        let mut download_items = Vec::new();

        let base_url = &metadata.path;
        let folder_name = self.base.extract_folder_name_from_url(&metadata.path)?;
        let target_dir = output_dir.join(folder_name);

        download_items.push(DownloadItem {
            url: url.to_string(),
            filename: metadata.playlist_file.replace(".m3u8", ".md"),
        });

        let m3u8_url = format!("{}/{}", base_url, metadata.playlist_file);
        download_items.push(DownloadItem {
            url: m3u8_url.clone(),
            filename: metadata.playlist_file.clone(),
        });

        if download_items.len() < 2 {
            return Err(anyhow!("The url provided is invalid!"));
        }

        let m3u8_content = self.fetch_m3u8_content(&m3u8_url).await?;
        let ts_files = self.parse_m3u8_segments(&m3u8_content)?;

        for ts_file in ts_files {
            let ts_url = format!("{}/{}", base_url, ts_file);
            download_items.push(DownloadItem {
                url: ts_url,
                filename: ts_file,
            });
        }
        self.base.download_files(download_items, &target_dir).await?;

        Ok(())
    }

    // TODO: maybe use self.fetch_metadata in the future
    fn extract_folder_name(&self, url: &str) -> Result<String> {
        use url::Url;
        let url_obj = Url::parse(url)
            .map_err(|e| anyhow!("Invalid URL: {}", e))?;
        
        let path_segments: Vec<&str> = url_obj.path_segments()
            .ok_or_else(|| anyhow!("URL has no path segments"))?
            .collect();

        if path_segments.len() < 2 {
            return Err(anyhow!("URL path does not contain enough segments"));
        }

        let folder_name = path_segments[path_segments.len() - 2];
        Ok(folder_name.to_string())
    }
}