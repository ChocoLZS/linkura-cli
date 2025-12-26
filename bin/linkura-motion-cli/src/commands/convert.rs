use anyhow::{Error, Result};
use linkura_packet::als::converter::AlsConverter;
use std::path::Path;
use tracing::info;

use crate::args::ArgsConvert;

pub fn run(args: ArgsConvert) -> Result<()> {
    info!("🔄 Starting ALS conversion from mixed to standard format");
    info!("📂 Input file: {}", args.input_file);
    info!("📁 Output directory: {}", args.output_dir);
    info!("⏱️ Segment duration: {} seconds", args.segment_duration);

    let input_path = Path::new(&args.input_file);
    if !input_path.exists() {
        return Err(Error::msg(format!(
            "Input file does not exist: {}",
            args.input_file
        )));
    }

    #[cfg(feature = "audio")]
    let use_audio_processing = args.audio_only;
    #[cfg(not(feature = "audio"))]
    let use_audio_processing = false;
    
    let converter = AlsConverter::new(args.segment_duration, use_audio_processing);
    converter.convert_mixed_to_standard(
        &args.input_file,
        &args.output_dir,
        &args.convert_type,
        args.timeshift,
        args.split,
        args.start_time,
        args.data_start_time,
        args.data_end_time,
        args.metadata_path,
        args.auto_timestamp,
    )?;
    info!("✅ ALS conversion completed successfully!");
    info!("📄 Output files written to: {}", args.output_dir);
    Ok(())
}
