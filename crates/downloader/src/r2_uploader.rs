use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

use crate::progress_ui::{FileProgressReporter, ProgressReporter, ProgressReporterFactory};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct R2Uploader {
    client: Client,
    account_id: String,
    access_key_id: String,
    secret_access_key: String,
    bucket: String,
    endpoint: String,
    concurrent_uploads: usize,
    progress_reporter: Option<Arc<dyn ProgressReporter>>,
}

#[derive(Debug)]
pub struct UploadTask {
    pub local_path: std::path::PathBuf,
    pub remote_key: String,
    pub file_size: u64,
}

impl R2Uploader {
    pub async fn from_env_or_args(
        account_id: Option<String>,
        access_key_id: Option<String>,
        secret_access_key: Option<String>,
        bucket: Option<String>,
        concurrent_uploads: usize,
        show_progress: bool,
    ) -> Result<Self> {
        let account_id = account_id
            .or_else(|| std::env::var("R2_ACCOUNT_ID").ok())
            .ok_or_else(|| {
                Error::msg(
                    "Account ID not provided via argument or R2_ACCOUNT_ID environment variable",
                )
            })?;

        let access_key_id = access_key_id
            .or_else(|| std::env::var("R2_ACCESS_KEY_ID").ok())
            .ok_or_else(|| Error::msg("Access key ID not provided via argument or R2_ACCESS_KEY_ID environment variable"))?;

        let secret_access_key = secret_access_key
            .or_else(|| std::env::var("R2_SECRET_ACCESS_KEY").ok())
            .ok_or_else(|| Error::msg("Secret access key not provided via argument or R2_SECRET_ACCESS_KEY environment variable"))?;

        let bucket = bucket
            .or_else(|| std::env::var("R2_BUCKET").ok())
            .ok_or_else(|| {
                Error::msg(
                    "Bucket name not provided via argument or R2_BUCKET environment variable",
                )
            })?;

        Self::new(
            &account_id,
            &access_key_id,
            &secret_access_key,
            bucket,
            concurrent_uploads,
            show_progress,
        )
        .await
    }

    pub async fn new(
        account_id: &str,
        access_key_id: &str,
        secret_access_key: &str,
        bucket: String,
        concurrent_uploads: usize,
        show_progress: bool,
    ) -> Result<Self> {
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", account_id);
        let client = Client::new();

        let progress_reporter = if show_progress {
            let reporter = crate::progress_ui::TreeProgressReporterFactory
                .create_upload_reporter(0, concurrent_uploads);
            Some(Arc::from(reporter))
        } else {
            let reporter = crate::progress_ui::SilentProgressReporterFactory
                .create_upload_reporter(0, concurrent_uploads);
            Some(Arc::from(reporter))
        };

        Ok(Self {
            client,
            account_id: account_id.to_string(),
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
            bucket,
            endpoint,
            concurrent_uploads,
            progress_reporter,
        })
    }

    pub async fn upload_file(&self, local_file: &Path, remote_key: Option<&str>) -> Result<()> {
        if !local_file.is_file() {
            return Err(Error::msg("Local path must be a file"));
        }

        let metadata = fs::metadata(local_file)?;
        let file_size = metadata.len();

        // Generate remote key if not provided
        let remote_key = match remote_key {
            Some(key) => key.to_string(),
            None => local_file
                .file_name()
                .ok_or_else(|| Error::msg("Could not extract filename"))?
                .to_string_lossy()
                .to_string(),
        };

        let task = UploadTask {
            local_path: local_file.to_path_buf(),
            remote_key,
            file_size,
        };

        if let Some(reporter) = &self.progress_reporter {
            // Update the total files count to 1
            if let Some(tree_reporter) = reporter
                .as_any()
                .downcast_ref::<crate::progress_ui::TreeProgressReporter>()
            {
                tree_reporter.set_total_files(1);
            }
        }

        self.upload_files_concurrent(vec![task]).await
    }

    pub async fn upload_folder(
        &self,
        local_folder: &Path,
        remote_prefix: Option<&str>,
    ) -> Result<()> {
        if !local_folder.is_dir() {
            return Err(Error::msg("Local path must be a directory"));
        }

        let tasks = self.collect_upload_tasks(local_folder, local_folder, remote_prefix)?;

        if let Some(reporter) = &self.progress_reporter {
            // Update the total files count
            if let Some(tree_reporter) = reporter
                .as_any()
                .downcast_ref::<crate::progress_ui::TreeProgressReporter>()
            {
                tree_reporter.set_total_files(tasks.len() as u64);
            }
        }

        self.upload_files_concurrent(tasks).await
    }

    fn collect_upload_tasks(
        &self,
        base_folder: &Path,
        current_folder: &Path,
        remote_prefix: Option<&str>,
    ) -> Result<Vec<UploadTask>> {
        let mut tasks = Vec::new();
        let entries = fs::read_dir(current_folder)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let metadata = fs::metadata(&path)?;
                let file_size = metadata.len();

                let relative_path = path.strip_prefix(base_folder)?;
                let remote_key = match remote_prefix {
                    Some(prefix) => format!(
                        "{}/{}",
                        prefix,
                        relative_path.to_string_lossy().replace('\\', "/")
                    ),
                    None => relative_path.to_string_lossy().replace('\\', "/"),
                };

                tasks.push(UploadTask {
                    local_path: path,
                    remote_key,
                    file_size,
                });
            } else if path.is_dir() {
                let mut sub_tasks = self.collect_upload_tasks(base_folder, &path, remote_prefix)?;
                tasks.append(&mut sub_tasks);
            }
        }

        Ok(tasks)
    }

    async fn upload_files_concurrent(&self, tasks: Vec<UploadTask>) -> Result<()> {
        if tasks.is_empty() {
            return Ok(());
        }

        let (task_sender, task_receiver) = mpsc::unbounded_channel::<UploadTask>();
        let task_receiver = Arc::new(tokio::sync::Mutex::new(task_receiver));

        for task in tasks {
            if task_sender.send(task).is_err() {
                return Err(Error::msg("Failed to send task to queue"));
            }
        }
        drop(task_sender);

        let mut handles = Vec::new();

        for thread_id in 0..self.concurrent_uploads {
            let receiver = Arc::clone(&task_receiver);
            let uploader = self.clone();

            let handle = tokio::spawn(async move {
                loop {
                    let task = {
                        let mut receiver = receiver.lock().await;
                        receiver.recv().await
                    };

                    let task = match task {
                        Some(task) => task,
                        None => break, // 没有更多任务
                    };

                    let file_reporter = if let Some(reporter) = &uploader.progress_reporter {
                        reporter.assign_file_to_thread(
                            thread_id,
                            &task
                                .local_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy(),
                            task.file_size,
                        )
                    } else {
                        None
                    };

                    let result = uploader
                        .upload_single_file(&task, file_reporter.as_ref())
                        .await;

                    if let Some(reporter) = &uploader.progress_reporter {
                        reporter.finish_file(
                            thread_id,
                            &task
                                .local_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy(),
                        );
                    }

                    if let Err(e) = result {
                        return Err(e);
                    }

                    if uploader.progress_reporter.is_none()
                        || uploader
                            .progress_reporter
                            .as_ref()
                            .unwrap()
                            .as_any()
                            .is::<crate::progress_ui::SilentProgressReporter>()
                    {
                    }
                }
                Ok(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle
                .await
                .map_err(|e| Error::msg(format!("Thread join error: {}", e)))??;
        }

        if let Some(reporter) = &self.progress_reporter {
            reporter.finish_all();
        }

        Ok(())
    }

    async fn upload_single_file(
        &self,
        task: &UploadTask,
        file_reporter: Option<&Box<dyn FileProgressReporter>>,
    ) -> Result<()> {
        let mut file = File::open(&task.local_path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        if let Some(reporter) = file_reporter {
            reporter.set_total_size(task.file_size);
            reporter.update_progress(task.file_size);
        }

        let content_type = self.guess_content_type(&task.local_path);
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, task.remote_key);

        // Calculate content SHA256
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let content_sha256 = hex::encode(hasher.finalize());

        let now = Utc::now();
        let authorization =
            self.generate_auth_header("PUT", &task.remote_key, &contents, content_type, &now)?;

        let response = self
            .client
            .put(&url)
            .header("Authorization", authorization)
            .header("Content-Type", content_type)
            .header("Content-Length", contents.len().to_string())
            .header("x-amz-date", now.format("%Y%m%dT%H%M%SZ").to_string())
            .header("x-amz-content-sha256", content_sha256)
            .body(contents)
            .send()
            .await
            .map_err(|e| Error::msg(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::msg(format!(
                "Upload failed with status {}: {}",
                status, text
            )));
        }

        Ok(())
    }

    fn generate_auth_header(
        &self,
        method: &str,
        key: &str,
        payload: &[u8],
        _content_type: &str,
        timestamp: &DateTime<Utc>,
    ) -> Result<String> {
        let date_stamp = timestamp.format("%Y%m%d").to_string();
        let amz_date = timestamp.format("%Y%m%dT%H%M%SZ").to_string();
        let region = "auto";
        let service = "s3";

        // Create payload hash
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let payload_hash = hex::encode(hasher.finalize());

        // Create canonical request
        let canonical_uri = format!("/{}/{}", self.bucket, key);
        let canonical_querystring = "";
        let canonical_headers = format!(
            "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
            format!("{}.r2.cloudflarestorage.com", self.account_id),
            payload_hash,
            amz_date
        );
        let signed_headers = "host;x-amz-content-sha256;x-amz-date";

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            canonical_uri,
            canonical_querystring,
            canonical_headers,
            signed_headers,
            payload_hash
        );

        // Create string to sign
        let algorithm = "AWS4-HMAC-SHA256";
        let credential_scope = format!("{}/{}/{}/aws4_request", date_stamp, region, service);
        let mut hasher = Sha256::new();
        hasher.update(canonical_request.as_bytes());
        let canonical_request_hash = hex::encode(hasher.finalize());

        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm, amz_date, credential_scope, canonical_request_hash
        );

        // Calculate signature
        let signature = self.calculate_signature(&string_to_sign, &date_stamp, region, service)?;

        // Create authorization header
        let authorization_header = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm, self.access_key_id, credential_scope, signed_headers, signature
        );

        Ok(authorization_header)
    }

    fn calculate_signature(
        &self,
        string_to_sign: &str,
        date_stamp: &str,
        region: &str,
        service: &str,
    ) -> Result<String> {
        let key = format!("AWS4{}", self.secret_access_key);

        let mut mac = HmacSha256::new_from_slice(key.as_bytes())
            .map_err(|e| Error::msg(format!("HMAC error: {}", e)))?;
        mac.update(date_stamp.as_bytes());
        let date_key = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&date_key)
            .map_err(|e| Error::msg(format!("HMAC error: {}", e)))?;
        mac.update(region.as_bytes());
        let date_region_key = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&date_region_key)
            .map_err(|e| Error::msg(format!("HMAC error: {}", e)))?;
        mac.update(service.as_bytes());
        let date_region_service_key = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&date_region_service_key)
            .map_err(|e| Error::msg(format!("HMAC error: {}", e)))?;
        mac.update(b"aws4_request");
        let signing_key = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&signing_key)
            .map_err(|e| Error::msg(format!("HMAC error: {}", e)))?;
        mac.update(string_to_sign.as_bytes());
        let signature = mac.finalize().into_bytes();

        Ok(hex::encode(signature))
    }

    fn guess_content_type(&self, path: &Path) -> &'static str {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("txt") => "text/plain",
            Some("pdf") => "application/pdf",
            Some("zip") => "application/zip",
            Some("xml") => "application/xml",
            _ => "application/octet-stream",
        }
    }
}

impl Clone for UploadTask {
    fn clone(&self) -> Self {
        Self {
            local_path: self.local_path.clone(),
            remote_key: self.remote_key.clone(),
            file_size: self.file_size,
        }
    }
}
