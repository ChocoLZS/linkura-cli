use anyhow::Result;
use linkura_packet::als::proto;
use tracing::info;

use crate::args::ArgsAnalyze;

pub fn run(args: ArgsAnalyze) -> Result<()> {
    // Handle standard and mixed analysis
    info!(
        "🔍 Starting ALS packet analysis for file: {}",
        args.file_path
    );
    info!(
        "📊 Analysis type: {}, Packet count: {}",
        args.analysis_type, args.packet_count
    );
    info!("📄 Output will be written to: {}", args.output_path);

    proto::application::analyze(
        args.file_path.as_ref(),
        Some(args.output_path.as_ref()),
        args.analysis_type.as_ref(),
        args.packet_count,
        args.data_start_time,
        args.data_end_time,
    )?;
    info!("✅ ALS packet analysis completed successfully!");
    Ok(())
}
