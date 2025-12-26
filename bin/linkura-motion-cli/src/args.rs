use clap::{Args as ClapArgs, Parser, Subcommand};
use linkura_i18n::t;

/** ARG PARSER **/
#[derive(Parser, Debug)]
#[clap(version)]
#[command(
    name = "linkura-motion-cli",
    author = "ChocoLZS, chocoielzs@outlook.com",
    about = t!("motion.cli.about").to_string(),
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

#[cfg(feature = "audio")]
#[derive(Debug, ClapArgs)]
/// Extract archive audio from archive files
pub struct ArgsAudio {
    #[clap(
        short('i'),
        long = "input",
        value_name = "INPUT_FILE",
        help = "Input archive file path"
    )]
    pub input_file: String,
    #[clap(
        short('o'),
        long = "output",
        value_name = "OUTPUT_DIR",
        help = "Output directory for extracted audio",
        default_value = "audio"
    )]
    pub output_dir: String,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsEdit {
    #[clap(short('i'), long = "input", value_name = "INPUT_FILE", help = "Input file path")]
    pub input_file: String,

    #[clap(short('o'), long = "output", value_name = "OUTPUT_FILE", help = "Output file path")]
    pub output_file: String,

    #[clap(long = "timeshift", value_name = "MS", help = "Time shift in milliseconds")]
    pub timeshift: Option<i64>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(ArgsDownload),
    Upload(ArgsUpload),
    Sync(ArgsSync),
    Analyze(ArgsAnalyze),
    Convert(ArgsConvert),
    Edit(ArgsEdit),
    #[cfg(feature = "audio")]
    Audio(ArgsAudio),
}
