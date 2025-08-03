use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// 进度报告的trait定义
pub trait ProgressReporter: Send + Sync {
    /// 为线程分配文件时调用
    fn assign_file_to_thread(&self, thread_id: usize, filename: &str, file_size: u64) -> Option<Box<dyn FileProgressReporter>>;
    
    /// 完成文件下载时调用
    fn finish_file(&self, thread_id: usize, filename: &str);
    
    /// 完成所有下载时调用
    fn finish_all(&self);
}

/// 文件级别进度报告的trait
pub trait FileProgressReporter: Send + Sync {
    /// 更新文件下载进度
    fn update_progress(&self, downloaded: u64);
    
    /// 设置文件总大小
    fn set_total_size(&self, total_size: u64);
}

/// 进度报告工厂trait，用于创建进度报告器
pub trait ProgressReporterFactory {
    fn create_reporter(&self, total_files: u64, concurrent_downloads: usize) -> Box<dyn ProgressReporter>;
}

/// 静默的进度报告器工厂
pub struct SilentProgressReporterFactory;

impl ProgressReporterFactory for SilentProgressReporterFactory {
    fn create_reporter(&self, _total_files: u64, _concurrent_downloads: usize) -> Box<dyn ProgressReporter> {
        Box::new(SilentProgressReporter)
    }
}

/// 静默的进度报告器（不显示任何UI）
pub struct SilentProgressReporter;

impl ProgressReporter for SilentProgressReporter {
    fn assign_file_to_thread(&self, _thread_id: usize, _filename: &str, _file_size: u64) -> Option<Box<dyn FileProgressReporter>> {
        Some(Box::new(SilentFileProgressReporter))
    }
    
    fn finish_file(&self, _thread_id: usize, _filename: &str) {}
    
    fn finish_all(&self) {}
}

struct SilentFileProgressReporter;

impl FileProgressReporter for SilentFileProgressReporter {
    fn update_progress(&self, _downloaded: u64) {}
    fn set_total_size(&self, _total_size: u64) {}
}

/// 树状进度报告器工厂
pub struct TreeProgressReporterFactory;

impl ProgressReporterFactory for TreeProgressReporterFactory {
    fn create_reporter(&self, total_files: u64, concurrent_downloads: usize) -> Box<dyn ProgressReporter> {
        Box::new(TreeProgressReporter::new(total_files, concurrent_downloads))
    }
}

/// 基于indicatif的树状进度显示器
pub struct TreeProgressReporter {
    inner: Arc<TreeProgressInner>,
}

struct TreeProgressInner {
    multi_progress: MultiProgress,
    root_progress: ProgressBar,
    thread_progress: Vec<Arc<Mutex<ThreadProgress>>>,
    start_time: Instant,
    total_downloaded: Arc<Mutex<u64>>,
}

#[derive(Debug, Clone)]
struct ThreadProgress {
    progress_bar: ProgressBar,
    current_file: Option<String>,
    file_progress_bar: Option<ProgressBar>,
}

struct IndicatifFileProgressReporter {
    progress_bar: ProgressBar,
    total_downloaded: Arc<Mutex<u64>>,
    root_progress: ProgressBar,
    start_time: Instant,
}

impl TreeProgressReporter {
    pub fn new(total_files: u64, concurrent_downloads: usize) -> Self {
        let multi_progress = MultiProgress::new();
        
        // 根节点样式 - 包含下载速率
        let root_style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} 📁 Total Downloads {msg}"
        )
        .unwrap()
        .progress_chars("#>-");
        
        // 线程节点样式
        let thread_style = ProgressStyle::with_template(
            "  {spinner:.green} 🧵 Thread {msg}"
        )
        .unwrap();
        
        let root_progress = multi_progress.add(ProgressBar::new(total_files));
        root_progress.set_style(root_style);
        root_progress.set_message("(0 B/s)");
        
        let mut thread_progress = Vec::new();
        
        // 为每个下载线程创建进度条
        for i in 0..concurrent_downloads {
            let thread_pb = multi_progress.add(ProgressBar::new_spinner());
            thread_pb.set_style(thread_style.clone());
            thread_pb.set_message(format!("{} - idle", i + 1));
            thread_pb.enable_steady_tick(std::time::Duration::from_millis(120));
            
            thread_progress.push(Arc::new(Mutex::new(ThreadProgress {
                progress_bar: thread_pb,
                current_file: None,
                file_progress_bar: None,
            })));
        }
        
        let inner = Arc::new(TreeProgressInner {
            multi_progress,
            root_progress,
            thread_progress,
            start_time: Instant::now(),
            total_downloaded: Arc::new(Mutex::new(0)),
        });
        
        Self { inner }
    }
}

impl ProgressReporter for TreeProgressReporter {
    
    fn assign_file_to_thread(&self, thread_id: usize, filename: &str, file_size: u64) -> Option<Box<dyn FileProgressReporter>> {
        let inner = &self.inner;
        
        if thread_id >= inner.thread_progress.len() {
            return None;
        }
        
        let mut thread = inner.thread_progress[thread_id].lock().unwrap();
        
        // 更新线程状态显示
        thread.progress_bar.set_message(format!("{} - ⏳ {}", thread_id + 1, filename));
        
        // 文件级别的进度条样式
        let file_style = ProgressStyle::with_template(
            "    {bar:30.green/yellow} 📄 {msg} {bytes:>7}/{total_bytes:7}"
        )
        .unwrap()
        .progress_chars("#>-");
        
        let file_pb = if let Some(existing_pb) = &thread.file_progress_bar {
            // 如果已经有文件进度条，重用它并重置
            existing_pb.reset();
            existing_pb.set_length(file_size);
            existing_pb.set_style(file_style);
            existing_pb.set_message("");
            existing_pb.clone()
        } else {
            // 如果没有现有的进度条，创建新的
            let new_pb = inner.multi_progress.insert_after(&thread.progress_bar, ProgressBar::new(file_size));
            new_pb.set_style(file_style);
            new_pb.set_message("");
            new_pb
        };
        
        thread.current_file = Some(filename.to_string());
        thread.file_progress_bar = Some(file_pb.clone());
        
        Some(Box::new(IndicatifFileProgressReporter {
            progress_bar: file_pb,
            total_downloaded: inner.total_downloaded.clone(),
            root_progress: inner.root_progress.clone(),
            start_time: inner.start_time,
        }))
    }
    
    fn finish_file(&self, thread_id: usize, filename: &str) {
        let inner = &self.inner;
        
        if thread_id >= inner.thread_progress.len() {
            return;
        }
        
        let mut thread = inner.thread_progress[thread_id].lock().unwrap();
        
        if let Some(file_pb) = &thread.file_progress_bar {
            // 完成文件进度条，显示为已完成状态
            file_pb.finish_with_message(format!("✓"));
        }
        
        // 更新线程状态为已完成该文件，等待新任务
        thread.progress_bar.set_message(format!("{} - ✅ {}", thread_id + 1, filename));
        thread.current_file = None;
        // 保留 file_progress_bar，显示已完成的文件，直到有新任务替换
        
        // 更新总体进度
        inner.root_progress.inc(1);
    }
    
    fn finish_all(&self) {
        let inner = &self.inner;
        
        inner.root_progress.finish_with_message("✓ All downloads completed");
        
        for (i, thread) in inner.thread_progress.iter().enumerate() {
            let thread = thread.lock().unwrap();
            thread.progress_bar.finish_with_message(format!("{} - completed", i + 1));
        }
        
        // 给用户一点时间看到完成状态
        std::thread::sleep(std::time::Duration::from_millis(500));
        inner.multi_progress.clear().unwrap();
    }
}

impl FileProgressReporter for IndicatifFileProgressReporter {
    fn update_progress(&self, downloaded: u64) {
        let previous_position = self.progress_bar.position();
        let new_bytes = downloaded.saturating_sub(previous_position);
        
        self.progress_bar.set_position(downloaded);
        
        // 更新总下载量和速率
        if new_bytes > 0 {
            let mut total_downloaded = self.total_downloaded.lock().unwrap();
            *total_downloaded += new_bytes;
            
            let elapsed = self.start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let rate = *total_downloaded as f64 / elapsed;
                let speed_msg = if rate >= 1_048_576.0 {
                    format!("({:.1} MB/s)", rate / 1_048_576.0)
                } else if rate >= 1024.0 {
                    format!("({:.1} KB/s)", rate / 1024.0)
                } else {
                    format!("({:.0} B/s)", rate)
                };
                self.root_progress.set_message(speed_msg);
            }
        }
    }
    
    fn set_total_size(&self, total_size: u64) {
        self.progress_bar.set_length(total_size);
    }
}