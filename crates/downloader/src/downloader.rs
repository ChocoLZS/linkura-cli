use anyhow::{anyhow, Result};
use futures::future::join_all;
use reqwest::Client;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    pub filename: String,
}

pub struct Downloader {
    client: Client,
    concurrent_downloads: usize,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Downloader {
    pub fn new(concurrent_downloads: usize) -> Self {
        Self {
            client: Client::new(),
            concurrent_downloads,
        }
    }

    pub async fn download_files(
        &self,
        items: Vec<DownloadItem>,
        output_dir: &Path,
    ) -> Result<()> {
        fs::create_dir_all(output_dir).await?;

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(self.concurrent_downloads));
        
        let tasks: Vec<_> = items
            .into_iter()
            .map(|item| {
                let client = self.client.clone();
                let semaphore = semaphore.clone();
                let output_path = output_dir.join(&item.filename);
                
                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    self.download_single_file(&client, &item.url, &output_path).await
                }
            })
            .collect();

        let results = join_all(tasks).await;
        
        for result in results {
            result?;
        }

        Ok(())
    }

    async fn download_single_file(
        &self,
        client: &Client,
        url: &str,
        output_path: &Path,
    ) -> Result<()> {
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch {}: {}", url, e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "HTTP error {} for URL: {}",
                response.status(),
                url
            ));
        }

        let content = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read response body from {}: {}", url, e))?;

        let mut file = fs::File::create(output_path)
            .await
            .map_err(|e| anyhow!("Failed to create file {:?}: {}", output_path, e))?;

        file.write_all(&content)
            .await
            .map_err(|e| anyhow!("Failed to write to file {:?}: {}", output_path, e))?;

        Ok(())
    }
}