use anyhow::{Error, Result};
use linkura_packet::als::converter::AlsConverter;
use tracing::info;

use crate::args::ArgsAudio;

pub async fn run(args: ArgsAudio) -> Result<()> {
    info!("🔊 Starting audio extraction from archive file");
    info!("📂 Input file: {}", args.input_file);
    info!("📁 Output directory: {}", args.output_dir);
    let input_path = std::path::Path::new(&args.input_file);
    if !input_path.exists() {
        return Err(Error::msg(format!(
            "Input file does not exist: {}",
            args.input_file
        )));
    }
    // Convert async context to sync for the audio extraction
    let _result = tokio::task::spawn_blocking({
        let input_file = args.input_file.clone();
        let output_dir = args.output_dir.clone();
        move || {
            let converter = AlsConverter::new(10, true);
            converter.extract_audio_from_standard(&input_file, &output_dir)
        }
    })
    .await??;
    info!("✅ Audio extraction completed successfully!");
    info!("📄 Output audio files written to: {}", args.output_dir);
    Ok(())
}
