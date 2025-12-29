use anyhow::{Error, Result};
use linkura_packet::als::editor::Editor;
use std::path::Path;
use tracing::info;

use crate::args::ArgsEdit;

pub fn run(args: ArgsEdit) -> Result<()> {
    info!("✏️ Starting Edit operation");
    info!("📂 Input directory: {}", args.input_dir);
    info!("📁 Output directory: {}", args.output_dir);

    let input_path = Path::new(&args.input_dir);
    if !input_path.exists() {
        return Err(Error::msg(format!(
            "Input directory does not exist: {}",
            args.input_dir
        )));
    }
    if !input_path.is_dir() {
        return Err(Error::msg(format!(
            "Input path must be a directory: {}",
            args.input_dir
        )));
    }

    if args.timeline_ids.is_empty() {
        info!("⚠️ No timeline IDs provided. No specific timeline modifications will be performed.");
    } else {
        info!("🎯 Target Timeline IDs: {:?}", args.timeline_ids);
    }

    if let Some(ts) = args.timeshift {
        info!("⏱️ Applying timeshift: {} ms", ts);
    }

    // Initialize Editor
    let mut editor = Editor::new(
        &args.input_dir,
        &args.output_dir,
        args.timeline_ids,
        args.timeshift,
    )?;

    // Run processing
    editor.process()?;

    info!("✅ Edit operation completed successfully!");
    Ok(())
}
