use crate::downloader::{DownloadItem, Downloader};
use crate::progress_ui::{ProgressReporterFactory, SilentProgressReporterFactory, TreeProgressReporterFactory};
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::path::Path;
use url::Url;

pub struct MrsDownloader {
    downloader: Downloader,
    client: Client,
}

impl Default for MrsDownloader {
    fn default() -> Self {
        Self::new(16)
    }
}

impl MrsDownloader {
    pub fn new(concurrent_downloads: usize) -> Self {
        Self::with_progress(concurrent_downloads, true)
    }

    pub fn with_progress(concurrent_downloads: usize, show_progress: bool) -> Self {
        let factory: Box<dyn ProgressReporterFactory + Send + Sync> = if show_progress {
            Box::new(TreeProgressReporterFactory)
        } else {
            Box::new(SilentProgressReporterFactory)
        };
        Self::with_progress_factory(concurrent_downloads, factory)
    }

    fn with_progress_factory(
        concurrent_downloads: usize,
        progress_factory: Box<dyn ProgressReporterFactory + Send + Sync>,
    ) -> Self {
        Self {
            downloader: Downloader::with_progress_factory(concurrent_downloads, progress_factory),
            client: Client::new(),
        }
    }

    pub async fn download(&self, url: &str, output_dir: &Path) -> Result<()> {
        let iarc_content = self.fetch_iarc_content(url).await?;
        let mut download_items = Vec::new();
        
        let folder_name = self.extract_folder_name(url)?;
        let target_dir = output_dir.join(&folder_name);
        let base_url = self.get_base_url(url)?;

        download_items.push(DownloadItem {
            url: url.to_string(),
            filename: self.extract_filename(url)?,
        });

        let segment_files = self.parse_iarc_segments(&iarc_content)?;

        for segment_file in segment_files {
            let segment_url = format!("{}/{}", base_url, segment_file);
            download_items.push(DownloadItem {
                url: segment_url,
                filename: segment_file,
            });
        }

        self.downloader.download_files(download_items, &target_dir).await?;

        Ok(())
    }

    async fn fetch_iarc_content(&self, url: &str) -> Result<Vec<u8>> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch iarc file: HTTP {}", response.status()));
        }

        Ok(response.bytes().await?.to_vec())
    }

    pub fn parse_iarc_segments(&self, content: &[u8]) -> Result<Vec<String>> {
        let mut segments = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        
        let segment_pattern = "segment_";
        let ias_extension = ".ias";
        
        let mut pos = 0;
        while let Some(start) = content_str[pos..].find(segment_pattern) {
            let absolute_start = pos + start;
            
            if let Some(end) = content_str[absolute_start..].find(ias_extension) {
                let absolute_end = absolute_start + end + ias_extension.len();
                let segment_name = &content_str[absolute_start..absolute_end];
                
                if segment_name.len() > 0 && segment_name.chars().all(|c| c.is_ascii()) {
                    segments.push(segment_name.to_string());
                }
                
                pos = absolute_end;
            } else {
                break;
            }
        }

        if segments.is_empty() {
            return Err(anyhow!("No .ias segments found in iarc file"));
        }

        segments.sort();
        segments.dedup();

        Ok(segments)
    }

    fn extract_folder_name(&self, url: &str) -> Result<String> {
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

    fn extract_filename(&self, url: &str) -> Result<String> {
        let url_obj = Url::parse(url)
            .map_err(|e| anyhow!("Invalid URL: {}", e))?;
        
        let path_segments: Vec<&str> = url_obj.path_segments()
            .ok_or_else(|| anyhow!("URL has no path segments"))?
            .collect();

        let filename = path_segments
            .last()
            .ok_or_else(|| anyhow!("No filename found in URL"))?;

        Ok(filename.to_string())
    }

    fn get_base_url(&self, url: &str) -> Result<String> {
        let url_obj = Url::parse(url)
            .map_err(|e| anyhow!("Invalid URL: {}", e))?;
        
        let mut path_segments: Vec<&str> = url_obj.path_segments()
            .ok_or_else(|| anyhow!("URL has no path segments"))?
            .collect();

        path_segments.pop();

        let base_path = path_segments.join("/");
        let base_url = format!("{}://{}/{}", 
            url_obj.scheme(), 
            url_obj.host_str().ok_or_else(|| anyhow!("URL has no host"))?,
            base_path
        );

        Ok(base_url)
    }
}