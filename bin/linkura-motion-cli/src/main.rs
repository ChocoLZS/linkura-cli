use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
use clap::{Args as ClapArgs, Parser, Subcommand};
use linkura_i18n::t;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    usize,
};
use tracing::{info, warn};

linkura_i18n::init!();

use linkura_common::log;
use linkura_downloader::{AlsDownloader, BaseDownloader, MrsDownloader, R2Uploader};
use linkura_packet::als::{
    converter::AlsConverter,
    extract::{ExtractConfig, ExtractTargetKind, ImageExtractOptions, run_extract},
    proto,
};
use url::Url;

/** ARG PARSER **/
#[derive(Parser, Debug)]
#[clap(version)]
#[command(
    name = "linkura-motion-cli",
    author = "ChocoLZS, chocoielzs@outlook.com",
    about = t!("motion.cli.about").to_string(),
    // long_about = None,
    bin_name = "linkura-motion-cli",
)]
pub struct Args {
    #[clap(short('q'), long = "quiet", help = t!("motion.cli.args.quiet").to_string(), default_value = "false")]
    pub quiet: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsDownload {
    #[clap(short('t'), long = "type", value_name = "TYPE", help = t!("motion.cli.command.download.args.type").to_string())]
    pub download_type: Option<String>,
    #[clap(short('d'),long = "directory",value_name = "DIRECTORY",help = t!("motion.cli.command.download.args.directory").to_string())]
    pub download_directory: Option<String>,
    #[clap(value_name = "URL",help = t!("motion.cli.command.download.args.url").to_string())]
    pub download_url: String,
    #[clap(short('p'), long = "parallel", help = t!("motion.cli.command.download.args.parallel").to_string(), default_value = "16")]
    pub parallel: usize,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsUpload {
    #[clap(short('b'), long = "bucket", value_name = "BUCKET", help = t!("motion.cli.command.upload.args.bucket").to_string())]
    pub bucket: Option<String>,
    #[clap(short('a'), long = "account-id", value_name = "ACCOUNT_ID", help = t!("motion.cli.command.upload.args.account_id").to_string())]
    pub account_id: Option<String>,
    #[clap(short('k'), long = "access-key", value_name = "ACCESS_KEY", help = t!("motion.cli.command.upload.args.access_key").to_string())]
    pub access_key: Option<String>,
    #[clap(short('s'), long = "secret-key", value_name = "SECRET_KEY", help = t!("motion.cli.command.upload.args.secret_key").to_string())]
    pub secret_key: Option<String>,
    #[clap(short('f'), long = "path", value_name = "PATH", help = t!("motion.cli.command.upload.args.path").to_string())]
    pub path: String,
    #[clap(short('p'), long = "prefix", value_name = "PREFIX", help = t!("motion.cli.command.upload.args.prefix").to_string())]
    pub prefix: Option<String>,
    #[clap(short('c'), long = "concurrent", value_name = "CONCURRENT", help = t!("motion.cli.command.upload.args.concurrent").to_string(), default_value = "4")]
    pub concurrent: usize,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsAnalyze {
    #[clap(
        short('t'),
        long = "type",
        value_name = "TYPE",
        help = "Analysis type: 'standard', 'mixed', 'mixed-legacy'",
        default_value = "standard"
    )]
    pub analysis_type: String,
    #[clap(
        short('o'),
        long = "output",
        value_name = "OUTPUT",
        help = "Output file path"
    )]
    pub output_path: String,
    #[clap(short('c'), long = "count", value_name = "COUNT", help = "Number of packets to analyze", default_value_t = usize::MAX)]
    pub packet_count: usize,
    #[clap(long = "file-count-limit", value_name = "FILE COUNT", help = "Limit the number of files to analyze (for mixed files)", default_value_t = usize::MAX)]
    pub file_count_limit: usize,
    #[clap(long = "file-size-limit", value_name = "FILE SIZE", help = "Limit the size of files to analyze in megabytes (for mixed files)", default_value_t = usize::MAX)]
    pub file_size_limit: usize,
    #[clap(
        value_name = "FILE",
        help = "Input binary file path (for diff: first file)"
    )]
    pub file_path: String,
    #[clap(
        short('f'),
        long = "file2",
        value_name = "FILE2",
        help = "Second file path (required for diff mode)"
    )]
    pub file_path2: Option<String>,
    #[clap(
        long = "data-start-time",
        value_name = "TIME",
        help = "Data start time in rfc3339 format (e.g., 2025-08-21T00:00:00Z, 2025-08-21T09:00:00+09:00), will ignore update object packets before this time"
    )]
    pub data_start_time: Option<String>,
    #[clap(
        long = "data-end-time",
        value_name = "TIME",
        help = "Data end time in rfc3339 format (e.g., 2025-08-21T00:00:00Z, 2025-08-21T09:00:00+09:00), will ignore update object packets after this time"
    )]
    pub data_end_time: Option<String>,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsSync {
    // Download parameters
    #[clap(short('t'), long = "type", value_name = "TYPE", help = t!("motion.cli.command.download.args.type").to_string())]
    pub download_type: Option<String>,
    #[clap(short('d'), long = "directory", value_name = "DIRECTORY", help = t!("motion.cli.command.download.args.directory").to_string())]
    pub download_directory: Option<String>,
    #[clap(value_name = "URL", help = t!("motion.cli.command.download.args.url").to_string())]
    pub download_url: String,
    #[clap(long = "download-parallel", help = t!("motion.cli.command.download.args.parallel").to_string(), default_value = "16")]
    pub download_parallel: usize,

    // Upload parameters
    #[clap(short('b'), long = "bucket", value_name = "BUCKET", help = t!("motion.cli.command.upload.args.bucket").to_string())]
    pub bucket: Option<String>,
    #[clap(short('a'), long = "account-id", value_name = "ACCOUNT_ID", help = t!("motion.cli.command.upload.args.account_id").to_string())]
    pub account_id: Option<String>,
    #[clap(short('k'), long = "access-key", value_name = "ACCESS_KEY", help = t!("motion.cli.command.upload.args.access_key").to_string())]
    pub access_key: Option<String>,
    #[clap(short('s'), long = "secret-key", value_name = "SECRET_KEY", help = t!("motion.cli.command.upload.args.secret_key").to_string())]
    pub secret_key: Option<String>,
    #[clap(short('p'), long = "prefix", value_name = "PREFIX", help = t!("motion.cli.command.upload.args.prefix").to_string())]
    pub prefix: Option<String>,
    #[clap(short('c'), long = "concurrent", value_name = "CONCURRENT", help = t!("motion.cli.command.upload.args.concurrent").to_string(), default_value = "4")]
    pub upload_concurrent: usize,

    // Additional options
    #[clap(long = "delete-after-done", help = t!("motion.cli.command.sync.args.delete_after_done").to_string(), default_value = "true")]
    pub delete_after_done: bool,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsConvert {
    #[clap(
        long = "type",
        value_name = "TYPE",
        help = "Conversion type: 'als', 'als-legacy'",
        default_value = "als"
    )]
    pub convert_type: String,
    #[clap(
        short('i'),
        long = "input",
        value_name = "INPUT_FILE",
        help = "Input mixed format file path"
    )]
    pub input_file: String,
    #[clap(
        short('o'),
        long = "output",
        value_name = "OUTPUT_DIR",
        help = "Output directory for converted segments",
        default_value = "output"
    )]
    pub output_dir: String,
    #[clap(
        short('d'),
        long = "duration",
        value_name = "SECONDS",
        help = "Segment duration in seconds",
        default_value = "10"
    )]
    pub segment_duration: u64,
    #[clap(long = "split", help = "Split segments", default_value = "false")]
    pub split: bool,
    #[clap(
        long = "timeshift",
        value_name = "MILLSECONDS",
        help = "Time shift in mill seconds, shift all packets' timestamps",
        default_value = "0"
    )]
    pub timeshift: i64,
    #[clap(
        long = "start-time",
        value_name = "TIME",
        help = "Convert start time in rfc3339 format (e.g., 2025-08-21T00:00:00Z, 2025-08-21T09:00:00+09:00), will ignore any packets before this time"
    )]
    pub start_time: Option<String>,
    #[clap(
        long = "data-start-time",
        value_name = "TIME",
        help = "Data start time in rfc3339 format (e.g., 2025-08-21T00:00:00Z, 2025-08-21T09:00:00+09:00), will ignore update object packets before this time"
    )]
    pub data_start_time: Option<String>,
    #[clap(
        long = "data-end-time",
        value_name = "TIME",
        help = "Data end time in rfc3339 format (e.g., 2025-08-21T00:00:00Z, 2025-08-21T09:00:00+09:00), will ignore update object packets after this time"
    )]
    pub data_end_time: Option<String>,
    #[clap(
        long = "metadata-path",
        value_name = "PATH",
        help = "Metadata path in index.md"
    )]
    pub metadata_path: Option<String>,
    #[clap(
        long = "auto-timestamp",
        help = "Auto adjust timestamps to ensure chronological order",
        default_value = "false"
    )]
    pub auto_timestamp: bool,
    #[cfg(feature = "audio")]
    #[clap(
        long = "audio-only",
        help = "Enable audio decoding (requires 'audio' feature)",
        default_value = "false"
    )]
    pub audio_only: bool,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsExtract {
    #[clap(
        short('i'),
        long = "input",
        value_name = "INPUT_DIR",
        help = "Input standard packet directory",
        default_value = "data",
        global(true)
    )]
    pub input_dir: String,
    #[clap(
        short('o'),
        long = "output",
        value_name = "OUTPUT_DIR",
        help = "Output directory for extracted data",
        global(true)
    )]
    pub output_dir: Option<String>,
    #[clap(short('c'), long = "count", value_name = "COUNT", help = "Maximum number of packets to process", default_value_t = usize::MAX)]
    pub packet_count: usize,
    #[clap(long = "file-count-limit", value_name = "FILE_COUNT", help = "Maximum number of files to process", default_value_t = usize::MAX)]
    pub file_count_limit: usize,
    #[clap(
        long = "data-start-time",
        value_name = "TIME",
        help = "Data start time in rfc3339 format (e.g., 2025-08-21T00:00:00Z)"
    )]
    pub data_start_time: Option<String>,
    #[clap(
        long = "data-end-time",
        value_name = "TIME",
        help = "Data end time in rfc3339 format (e.g., 2025-08-21T00:00:00Z)"
    )]
    pub data_end_time: Option<String>,
    #[clap(
        long = "strict",
        help = "Fail immediately on packet parse errors",
        default_value = "false"
    )]
    pub strict: bool,
    #[command(subcommand)]
    pub target: ExtractSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum ExtractSubcommands {
    #[cfg(feature = "audio")]
    Audio(ArgsExtractAudio),
    Image(ArgsExtractImage),
}

#[cfg(feature = "audio")]
#[derive(Debug, ClapArgs, Default)]
pub struct ArgsExtractAudio {}

#[derive(Debug, ClapArgs)]
pub struct ArgsExtractImage {
    #[clap(
        long = "metadata-file",
        value_name = "FILE",
        help = "Image metadata output filename",
        default_value = "images.jsonl"
    )]
    pub metadata_filename: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(ArgsDownload),
    Upload(ArgsUpload),
    Sync(ArgsSync),
    Analyze(ArgsAnalyze),
    Convert(ArgsConvert),
    Extract(ArgsExtract),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let quiet = args.quiet;
    if !quiet {
        log::init(None);
    }
    match args.command {
        Some(Commands::Download(ref download_args)) => {
            let download_url = download_args.download_url.trim();
            let mut download_type = download_args.download_type.clone();
            if download_type.is_none() {
                if download_url.ends_with(".md") {
                    download_type = Some("als".into());
                }
                if download_url.ends_with(".iarc") {
                    download_type = Some("mrs".into());
                }
            }
            let downloader: Box<dyn BaseDownloader> = match download_type.as_deref() {
                Some("als") => {
                    Box::new(AlsDownloader::with_progress(download_args.parallel, !quiet))
                }
                Some("mrs") => {
                    Box::new(MrsDownloader::with_progress(download_args.parallel, !quiet))
                }
                _ => {
                    return Err(Error::msg(format!(
                        "Unknown download type: {:?}",
                        download_args.download_type
                    )));
                }
            };
            let download_dir = download_args
                .download_directory
                .as_deref()
                .unwrap_or("data");
            downloader
                .download(download_url, Path::new(download_dir))
                .await?;
        }
        Some(Commands::Upload(ref upload_args)) => {
            let uploader = R2Uploader::from_env_or_args(
                upload_args.account_id.clone(),
                upload_args.access_key.clone(),
                upload_args.secret_key.clone(),
                upload_args.bucket.clone(),
                upload_args.concurrent,
                !quiet,
            )
            .await?;

            let path = Path::new(&upload_args.path);
            if !path.exists() {
                return Err(Error::msg(format!(
                    "Path does not exist: {}",
                    upload_args.path
                )));
            }

            let env_bucket = std::env::var("R2_BUCKET").ok();
            let bucket_name = upload_args
                .bucket
                .as_ref()
                .map(|s| s.as_str())
                .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
                .unwrap_or("[from env]");

            if path.is_file() {
                info!(
                    "🚀 Starting R2 file upload from '{}' to bucket '{}'",
                    upload_args.path, bucket_name
                );
                info!(
                    "📊 Configuration: {} concurrent workers, {} mode",
                    upload_args.concurrent,
                    if upload_args.prefix.is_some() {
                        format!("prefix: '{}'", upload_args.prefix.as_ref().unwrap())
                    } else {
                        "no prefix".to_string()
                    }
                );
                uploader
                    .upload_file(path, upload_args.prefix.as_deref())
                    .await?;
            } else if path.is_dir() {
                info!(
                    "🚀 Starting R2 folder upload from '{}' to bucket '{}'",
                    upload_args.path, bucket_name
                );
                info!(
                    "📊 Configuration: {} concurrent workers, {} mode",
                    upload_args.concurrent,
                    if upload_args.prefix.is_some() {
                        format!("prefix: '{}'", upload_args.prefix.as_ref().unwrap())
                    } else {
                        "no prefix".to_string()
                    }
                );
                uploader
                    .upload_folder(path, upload_args.prefix.as_deref())
                    .await?;
            } else {
                return Err(Error::msg(format!(
                    "Path is neither a file nor a directory: {}",
                    upload_args.path
                )));
            }
            info!("✅ Upload completed successfully!");
        }
        Some(Commands::Sync(ref sync_args)) => {
            // Step 1: Download
            info!("🔄 Starting sync operation: Download + Upload");
            let download_url = sync_args.download_url.trim();
            info!("📥 Phase 1: Downloading from '{}'", download_url);

            let download_dir = sync_args.download_directory.as_deref().unwrap_or("data");
            let download_path = Path::new(download_dir);

            let mut download_type = sync_args.download_type.clone();
            if download_type.is_none() {
                if download_url.ends_with(".md") {
                    download_type = Some("als".into());
                }
                if download_url.ends_with(".iarc") {
                    download_type = Some("mrs".into());
                }
            }

            let downloader: Box<dyn BaseDownloader> = match download_type.as_deref() {
                Some("als") => Box::new(AlsDownloader::with_progress(
                    sync_args.download_parallel,
                    !quiet,
                )),
                Some("mrs") => Box::new(MrsDownloader::with_progress(
                    sync_args.download_parallel,
                    !quiet,
                )),
                _ => {
                    return Err(Error::msg(format!(
                        "Unknown download type: {:?}",
                        sync_args.download_type
                    )));
                }
            };

            downloader.download(download_url, download_path).await?;

            info!("✅ Download completed successfully!");
            info!("📤 Phase 2: Uploading to R2");

            // Step 2: Upload
            let uploader = R2Uploader::from_env_or_args(
                sync_args.account_id.clone(),
                sync_args.access_key.clone(),
                sync_args.secret_key.clone(),
                sync_args.bucket.clone(),
                sync_args.upload_concurrent,
                !quiet,
            )
            .await?;

            if !download_path.exists() {
                return Err(Error::msg(format!(
                    "Download directory does not exist: {}",
                    download_dir
                )));
            }

            let folder_name = downloader.extract_folder_name(download_url)?;
            let target_folder = download_path.join(folder_name);
            let env_bucket = std::env::var("R2_BUCKET").ok();
            let bucket_name = sync_args
                .bucket
                .as_ref()
                .map(|s| s.as_str())
                .or_else(|| env_bucket.as_ref().map(|s| s.as_str()))
                .unwrap_or("[from env]");

            info!(
                "🚀 Starting R2 upload from '{}' to bucket '{}'",
                target_folder.display(),
                bucket_name
            );
            info!(
                "📊 Configuration: {} concurrent workers, {} mode",
                sync_args.upload_concurrent,
                if sync_args.prefix.is_some() {
                    format!("prefix: '{}'", sync_args.prefix.as_ref().unwrap())
                } else {
                    "no prefix".to_string()
                }
            );
            uploader
                .upload_folder(
                    &target_folder,
                    Some(
                        sync_args
                            .prefix
                            .clone()
                            .unwrap_or(get_bucket_prefix(download_url)?)
                            .deref(),
                    ),
                )
                .await?;

            // Delete downloaded files if requested
            if sync_args.delete_after_done {
                info!("🗑️ Deleting downloaded files after successful upload...");
                if let Err(e) = std::fs::remove_dir_all(&target_folder) {
                    warn!(
                        "Failed to delete downloaded folder '{}': {}",
                        target_folder.display(),
                        e
                    );
                } else {
                    info!("✅ Downloaded files deleted successfully");
                }
            }

            info!("✅ Sync operation completed successfully!");
            info!("🎉 Download + Upload finished!");
        }
        Some(Commands::Analyze(analyze_args)) => {
            // Handle standard and mixed analysis
            info!(
                "🔍 Starting ALS packet analysis for file: {}",
                analyze_args.file_path
            );
            info!(
                "📊 Analysis type: {}, Packet count: {}",
                analyze_args.analysis_type, analyze_args.packet_count
            );
            info!("📄 Output will be written to: {}", analyze_args.output_path);
            // Convert async context to sync for the analysis functions
            let file_path = analyze_args.file_path.clone();
            let output_path = analyze_args.output_path.clone();
            let packet_count = analyze_args.packet_count;
            let analysis_type = analyze_args.analysis_type.clone();
            proto::application::analyze(
                file_path.as_ref(),
                Some(output_path.as_ref()),
                analysis_type.as_ref(),
                packet_count,
                analyze_args.data_start_time,
                analyze_args.data_end_time,
            )?;
            info!("✅ ALS packet analysis completed successfully!");
        }
        Some(Commands::Convert(convert_args)) => {
            info!("🔄 Starting ALS conversion from mixed to standard format");
            info!("📂 Input file: {}", convert_args.input_file);
            info!("📁 Output directory: {}", convert_args.output_dir);
            info!(
                "⏱️ Segment duration: {} seconds",
                convert_args.segment_duration
            );

            let input_path = std::path::Path::new(&convert_args.input_file);
            if !input_path.exists() {
                return Err(Error::msg(format!(
                    "Input file does not exist: {}",
                    convert_args.input_file
                )));
            }

            // Convert async context to sync for the conversion
            let input_file = convert_args.input_file.clone();
            let output_dir = convert_args.output_dir.clone();
            let segment_duration = convert_args.segment_duration;

            #[cfg(feature = "audio")]
            let use_audio_processing = convert_args.audio_only;
            #[cfg(not(feature = "audio"))]
            let use_audio_processing = false;
            let converter = AlsConverter::new(segment_duration, use_audio_processing);
            converter.convert_mixed_to_standard(
                &input_file,
                &output_dir,
                &convert_args.convert_type,
                // TODO: better args passing
                convert_args.timeshift,
                convert_args.split,
                convert_args.start_time,
                convert_args.data_start_time,
                convert_args.data_end_time,
                convert_args.metadata_path,
                convert_args.auto_timestamp,
            )?;
            info!("✅ ALS conversion completed successfully!");
            info!("📄 Output files written to: {}", convert_args.output_dir);
        }
        Some(Commands::Extract(extract_args)) => {
            let default_output_dir = match &extract_args.target {
                #[cfg(feature = "audio")]
                ExtractSubcommands::Audio(_) => "audio",
                ExtractSubcommands::Image(_) => "image",
            };
            let output_dir = extract_args
                .output_dir
                .clone()
                .unwrap_or_else(|| default_output_dir.to_string());
            let data_start_time =
                parse_rfc3339_utc("data-start-time", extract_args.data_start_time.as_deref())?;
            let data_end_time =
                parse_rfc3339_utc("data-end-time", extract_args.data_end_time.as_deref())?;

            let target_kind = match &extract_args.target {
                #[cfg(feature = "audio")]
                ExtractSubcommands::Audio(_) => ExtractTargetKind::Audio(Default::default()),
                ExtractSubcommands::Image(image_args) => {
                    ExtractTargetKind::Image(ImageExtractOptions {
                        metadata_filename: image_args.metadata_filename.clone(),
                    })
                }
            };

            let extract_config = ExtractConfig {
                input_dir: PathBuf::from(&extract_args.input_dir),
                output_dir: PathBuf::from(&output_dir),
                packet_count: extract_args.packet_count,
                file_count_limit: extract_args.file_count_limit,
                data_start_time,
                data_end_time,
                strict: extract_args.strict,
            };

            info!("🔍 Starting '{}' extraction", target_kind.name());
            info!("📂 Input directory: {}", extract_args.input_dir);
            info!("📁 Output directory: {}", output_dir);

            let summary =
                tokio::task::spawn_blocking(move || run_extract(extract_config, target_kind))
                    .await??;

            info!(
                "✅ Extraction completed: target={}, files={}, packets_read={}, packets_processed={}, hits={}, outputs={}, errors={}",
                summary.target,
                summary.files_processed,
                summary.packets_read,
                summary.packets_processed,
                summary.target_hits,
                summary.outputs_written,
                summary.errors
            );
        }
        None => {}
    }
    Ok(())
}

fn parse_rfc3339_utc(field_name: &str, value: Option<&str>) -> Result<Option<DateTime<Utc>>> {
    let Some(value) = value else {
        return Ok(None);
    };

    let parsed = DateTime::parse_from_rfc3339(value)
        .map_err(|e| Error::msg(format!("Invalid {} '{}': {}", field_name, value, e)))?;
    Ok(Some(parsed.with_timezone(&Utc)))
}

fn get_bucket_prefix(url: &str) -> Result<String> {
    let parsed_url = Url::parse(url)?;
    let path = parsed_url.path();

    // Remove leading slash if present
    let path = path.strip_prefix('/').unwrap_or(path);

    // Split path into segments and remove the last segment (filename)
    let segments: Vec<&str> = path.split('/').collect();
    if segments.len() <= 1 {
        return Err(Error::msg("URL path too short to extract prefix"));
    }

    // Join all segments except the last one
    let prefix = segments[..segments.len() - 1].join("/");

    Ok(prefix)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bucket_prefix() {
        let url = "https://example.org/archive/alst/directory_name/index.md";
        let prefix = get_bucket_prefix(url);
        assert!(prefix.is_ok());
        assert_eq!(prefix.unwrap(), "archive/alst/directory_name");
    }
}
