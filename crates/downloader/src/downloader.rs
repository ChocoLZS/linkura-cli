use anyhow::{anyhow, Result};
use futures::future::join_all;
use reqwest::Client;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::progress_ui::{ProgressReporter, ProgressReporterFactory, SilentProgressReporterFactory, TreeProgressReporterFactory};

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    pub filename: String,
}

pub struct Downloader {
    client: Client,
    concurrent_downloads: usize,
    progress_factory: Box<dyn ProgressReporterFactory + Send + Sync>,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Downloader {
    pub fn new(concurrent_downloads: usize) -> Self {
        Self::with_progress_factory(concurrent_downloads, Box::new(TreeProgressReporterFactory))
    }

    pub fn with_progress(concurrent_downloads: usize, show_progress: bool) -> Self {
        let factory: Box<dyn ProgressReporterFactory + Send + Sync> = if show_progress {
            Box::new(TreeProgressReporterFactory)
        } else {
            Box::new(SilentProgressReporterFactory)
        };
        Self::with_progress_factory(concurrent_downloads, factory)
    }

    pub fn with_progress_factory(
        concurrent_downloads: usize,
        progress_factory: Box<dyn ProgressReporterFactory + Send + Sync>,
    ) -> Self {
        Self {
            client: Client::new(),
            concurrent_downloads,
            progress_factory,
        }
    }

    pub async fn download_files(
        &self,
        items: Vec<DownloadItem>,
        output_dir: &Path,
    ) -> Result<()> {
        fs::create_dir_all(output_dir).await?;

        let total_files = items.len() as u64;
        let progress_reporter = self.progress_factory.create_reporter(total_files, self.concurrent_downloads);

        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.concurrent_downloads));
        let active_threads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        let tasks: Vec<_> = items
            .into_iter()
            .map(|item| {
                let client = self.client.clone();
                let semaphore = semaphore.clone();
                let output_path = output_dir.join(&item.filename);
                let progress_reporter = progress_reporter.as_ref();
                let active_threads = active_threads.clone();
                
                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    
                    // 获取一个空闲的线程ID (使用循环分配)
                    let thread_id = active_threads.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % self.concurrent_downloads;
                    
                    let result = self.download_single_file_with_progress_reporter(
                        &client, 
                        &item.url, 
                        &output_path,
                        thread_id,
                        &item.filename,
                        progress_reporter
                    ).await;
                    
                    progress_reporter.finish_file(thread_id, &item.filename);
                    
                    result
                }
            })
            .collect();

        let results = join_all(tasks).await;
        
        progress_reporter.finish_all();
        
        for result in results {
            result?;
        }

        Ok(())
    }

    async fn download_single_file_with_progress_reporter(
        &self,
        client: &Client,
        url: &str,
        output_path: &Path,
        thread_id: usize,
        filename: &str,
        progress_reporter: &dyn ProgressReporter,
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

        let total_size = response.content_length().unwrap_or(0);
        let file_progress = progress_reporter.assign_file_to_thread(thread_id, filename, total_size);

        let mut file = fs::File::create(output_path)
            .await
            .map_err(|e| anyhow!("Failed to create file {:?}: {}", output_path, e))?;

        let content = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read response body from {}: {}", url, e))?;

        if let Some(file_progress) = file_progress {
            if total_size > 0 {
                file_progress.set_total_size(total_size);
            }
            file_progress.update_progress(content.len() as u64);
        }

        file.write_all(&content)
            .await
            .map_err(|e| anyhow!("Failed to write to file {:?}: {}", output_path, e))?;

        Ok(())
    }
}