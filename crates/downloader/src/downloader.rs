use anyhow::{anyhow, Result};
use futures::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    pub filename: String,
}

#[derive(Debug, Clone)]
struct ThreadProgress {
    thread_id: usize,
    progress_bar: ProgressBar,
    current_file: Option<String>,
    file_progress_bar: Option<ProgressBar>,
}

struct DownloadProgress {
    multi_progress: MultiProgress,
    root_progress: ProgressBar,
    thread_progress: Vec<Arc<Mutex<ThreadProgress>>>,
}

impl DownloadProgress {
    fn new(total_files: u64, concurrent_downloads: usize) -> Self {
        let multi_progress = MultiProgress::new();
        
        // 根节点样式
        let root_style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} 📁 Total Downloads"
        )
        .unwrap()
        .progress_chars("##-");
        
        // 线程节点样式 - 显示线程状态而不是计数
        let thread_style = ProgressStyle::with_template(
            "  {spinner:.green} 🧵 Thread {msg}"
        )
        .unwrap();
        
        let root_progress = multi_progress.add(ProgressBar::new(total_files));
        root_progress.set_style(root_style);
        
        let mut thread_progress = Vec::new();
        
        // 为每个下载线程创建进度条
        for i in 0..concurrent_downloads {
            let thread_pb = multi_progress.add(ProgressBar::new_spinner());
            thread_pb.set_style(thread_style.clone());
            thread_pb.set_message(format!("{} - idle", i + 1));
            thread_pb.enable_steady_tick(std::time::Duration::from_millis(120));
            
            thread_progress.push(Arc::new(Mutex::new(ThreadProgress {
                thread_id: i,
                progress_bar: thread_pb,
                current_file: None,
                file_progress_bar: None,
            })));
        }
        
        Self {
            multi_progress,
            root_progress,
            thread_progress,
        }
    }
    
    fn assign_file_to_thread(&self, thread_id: usize, filename: &str, file_size: u64) -> Option<ProgressBar> {
        if thread_id >= self.thread_progress.len() {
            return None;
        }
        
        let mut thread = self.thread_progress[thread_id].lock().unwrap();
        
        // 更新线程状态显示
        thread.progress_bar.set_message(format!("{} - ⏳ {}", thread_id + 1, filename));
        
        // 文件级别的进度条样式
        let file_style = ProgressStyle::with_template(
            "    {bar:30.green/yellow} {bytes:>7}/{total_bytes:7} 📄 {msg}"
        )
        .unwrap()
        .progress_chars("##-");
        
        let file_pb = if let Some(existing_pb) = &thread.file_progress_bar {
            // 如果已经有文件进度条，重用它并重置
            existing_pb.reset();
            existing_pb.set_length(file_size);
            existing_pb.set_style(file_style);
            existing_pb.set_message(filename.to_string());
            existing_pb.clone()
        } else {
            // 如果没有现有的进度条，创建新的
            let new_pb = self.multi_progress.insert_after(&thread.progress_bar, ProgressBar::new(file_size));
            new_pb.set_style(file_style);
            new_pb.set_message(filename.to_string());
            new_pb
        };
        
        thread.current_file = Some(filename.to_string());
        thread.file_progress_bar = Some(file_pb.clone());
        
        Some(file_pb)
    }
    
    fn finish_file(&self, thread_id: usize, filename: &str) {
        if thread_id >= self.thread_progress.len() {
            return;
        }
        
        let mut thread = self.thread_progress[thread_id].lock().unwrap();
        
        if let Some(file_pb) = &thread.file_progress_bar {
            // 完成文件进度条，显示为已完成状态
            file_pb.finish_with_message(format!("✓ {}", filename));
        }
        
        // 更新线程状态为已完成该文件，等待新任务
        thread.progress_bar.set_message(format!("{} - ✅ {}", thread_id + 1, filename));
        thread.current_file = None;
        // 保留 file_progress_bar，显示已完成的文件，直到有新任务替换
        
        // 更新总体进度
        self.root_progress.inc(1);
    }
    
    fn finish_all(&self) {
        self.root_progress.finish_with_message("✓ All downloads completed");
        
        for (i, thread) in self.thread_progress.iter().enumerate() {
            let thread = thread.lock().unwrap();
            thread.progress_bar.finish_with_message(format!("{} - completed", i + 1));
        }
        
        // 给用户一点时间看到完成状态
        std::thread::sleep(std::time::Duration::from_millis(500));
        self.multi_progress.clear().unwrap();
    }
}

pub struct Downloader {
    client: Client,
    concurrent_downloads: usize,
    show_progress: bool,
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Downloader {
    pub fn new(concurrent_downloads: usize) -> Self {
        Self::with_progress(concurrent_downloads, true)
    }

    pub fn with_progress(concurrent_downloads: usize, show_progress: bool) -> Self {
        Self {
            client: Client::new(),
            concurrent_downloads,
            show_progress,
        }
    }

    pub async fn download_files(
        &self,
        items: Vec<DownloadItem>,
        output_dir: &Path,
    ) -> Result<()> {
        fs::create_dir_all(output_dir).await?;

        let total_files = items.len() as u64;
        let progress = if self.show_progress {
            Some(Arc::new(DownloadProgress::new(total_files, self.concurrent_downloads)))
        } else {
            None
        };

        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.concurrent_downloads));
        let active_threads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        let tasks: Vec<_> = items
            .into_iter()
            .map(|item| {
                let client = self.client.clone();
                let semaphore = semaphore.clone();
                let output_path = output_dir.join(&item.filename);
                let progress = progress.clone();
                let active_threads = active_threads.clone();
                
                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    
                    // 获取一个空闲的线程ID (使用循环分配)
                    let thread_id = active_threads.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % self.concurrent_downloads;
                    
                    let result = self.download_single_file_with_thread_progress(
                        &client, 
                        &item.url, 
                        &output_path,
                        thread_id,
                        &item.filename,
                        progress.as_ref().map(|v| &**v)
                    ).await;
                    
                    if let Some(ref progress) = progress {
                        progress.finish_file(thread_id, &item.filename);
                    }
                    
                    result
                }
            })
            .collect();

        let results = join_all(tasks).await;
        
        if let Some(progress) = progress {
            progress.finish_all();
        }
        
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
        self.download_single_file_with_progress(client, url, output_path, None).await
    }

    async fn download_single_file_with_progress(
        &self,
        client: &Client,
        url: &str,
        output_path: &Path,
        progress_bar: Option<&ProgressBar>,
    ) -> Result<()> {
        if let Some(pb) = progress_bar {
            pb.set_message(format!("Connecting: {}", 
                output_path.file_name().unwrap_or_default().to_string_lossy()));
        }

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
        if let Some(pb) = progress_bar {
            pb.set_length(total_size);
            pb.set_message(format!("Downloading: {}", 
                output_path.file_name().unwrap_or_default().to_string_lossy()));
        }

        let mut file = fs::File::create(output_path)
            .await
            .map_err(|e| anyhow!("Failed to create file {:?}: {}", output_path, e))?;

        // For simplicity, we'll read the entire response body at once
        // In a real implementation, you might want to use a streaming approach
        let content = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read response body from {}: {}", url, e))?;

        if let Some(pb) = progress_bar {
            pb.set_position(content.len() as u64);
        }

        file.write_all(&content)
            .await
            .map_err(|e| anyhow!("Failed to write to file {:?}: {}", output_path, e))?;

        Ok(())
    }

    async fn download_single_file_with_thread_progress(
        &self,
        client: &Client,
        url: &str,
        output_path: &Path,
        thread_id: usize,
        filename: &str,
        progress: Option<&DownloadProgress>,
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
        let file_pb = if let Some(progress) = progress {
            progress.assign_file_to_thread(thread_id, filename, total_size)
        } else {
            None
        };

        let mut file = fs::File::create(output_path)
            .await
            .map_err(|e| anyhow!("Failed to create file {:?}: {}", output_path, e))?;

        let content = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read response body from {}: {}", url, e))?;

        if let Some(pb) = file_pb {
            pb.set_position(content.len() as u64);
        }

        file.write_all(&content)
            .await
            .map_err(|e| anyhow!("Failed to write to file {:?}: {}", output_path, e))?;

        Ok(())
    }
}