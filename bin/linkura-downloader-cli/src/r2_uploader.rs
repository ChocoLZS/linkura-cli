use std::path::Path;
use std::fs;
use std::pin::Pin;
use std::future::Future;
use anyhow::{Result, Error};
use reqwest::Client;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

type HmacSha256 = Hmac<Sha256>;

pub struct R2Uploader {
    client: Client,
    account_id: String,
    access_key_id: String,
    secret_access_key: String,
    bucket: String,
    endpoint: String,
}

impl R2Uploader {
    pub async fn from_env_or_args(
        account_id: Option<String>,
        access_key_id: Option<String>,
        secret_access_key: Option<String>,
        bucket: Option<String>,
    ) -> Result<Self> {
        let account_id = account_id
            .or_else(|| std::env::var("R2_ACCOUNT_ID").ok())
            .ok_or_else(|| Error::msg("Account ID not provided via argument or R2_ACCOUNT_ID environment variable"))?;
        
        let access_key_id = access_key_id
            .or_else(|| std::env::var("R2_ACCESS_KEY_ID").ok())
            .ok_or_else(|| Error::msg("Access key ID not provided via argument or R2_ACCESS_KEY_ID environment variable"))?;
        
        let secret_access_key = secret_access_key
            .or_else(|| std::env::var("R2_SECRET_ACCESS_KEY").ok())
            .ok_or_else(|| Error::msg("Secret access key not provided via argument or R2_SECRET_ACCESS_KEY environment variable"))?;
        
        let bucket = bucket
            .or_else(|| std::env::var("R2_BUCKET").ok())
            .ok_or_else(|| Error::msg("Bucket name not provided via argument or R2_BUCKET environment variable"))?;

        Self::new(&account_id, &access_key_id, &secret_access_key, bucket).await
    }

    pub async fn new(
        account_id: &str,
        access_key_id: &str,
        secret_access_key: &str,
        bucket: String,
    ) -> Result<Self> {
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", account_id);
        let client = Client::new();

        Ok(Self {
            client,
            account_id: account_id.to_string(),
            access_key_id: access_key_id.to_string(),
            secret_access_key: secret_access_key.to_string(),
            bucket,
            endpoint,
        })
    }

    pub async fn upload_folder(
        &self,
        local_folder: &Path,
        remote_prefix: Option<&str>,
    ) -> Result<()> {
        if !local_folder.is_dir() {
            return Err(Error::msg("Local path must be a directory"));
        }

        self.upload_folder_recursive(local_folder, local_folder, remote_prefix).await
    }

    fn upload_folder_recursive<'a>(
        &'a self,
        base_folder: &'a Path,
        current_folder: &'a Path,
        remote_prefix: Option<&'a str>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
        let entries = fs::read_dir(current_folder)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let relative_path = path.strip_prefix(base_folder)?;
                let remote_key = match remote_prefix {
                    Some(prefix) => format!("{}/{}", prefix, relative_path.to_string_lossy().replace('\\', "/")),
                    None => relative_path.to_string_lossy().replace('\\', "/"),
                };

                self.upload_file(&path, &remote_key).await?;
                println!("Uploaded: {} -> {}", path.display(), remote_key);
            } else if path.is_dir() {
                self.upload_folder_recursive(base_folder, &path, remote_prefix).await?;
            }
        }

        Ok(())
        })
    }

    async fn upload_file(&self, local_path: &Path, remote_key: &str) -> Result<()> {
        let mut file = File::open(local_path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        let content_type = self.guess_content_type(local_path);
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, remote_key);
        
        // Calculate content SHA256
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let content_sha256 = hex::encode(hasher.finalize());
        
        let now = Utc::now();
        let authorization = self.generate_auth_header("PUT", remote_key, &contents, content_type, &now)?;

        let response = self.client
            .put(&url)
            .header("Authorization", authorization)
            .header("Content-Type", content_type)
            .header("x-amz-date", now.format("%Y%m%dT%H%M%SZ").to_string())
            .header("x-amz-content-sha256", content_sha256)
            .body(contents)
            .send()
            .await
            .map_err(|e| Error::msg(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::msg(format!("Upload failed with status {}: {}", status, text)));
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
            method, canonical_uri, canonical_querystring, canonical_headers, signed_headers, payload_hash
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