//! Application layer example - orchestrates reader, analyzer, and formatter
//! This shows how to use the refactored architecture

use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;

use super::reader::{PacketReaderTrait, PacketReader, MixedPacketReader, LegacyPacketReader};
use super::analyzer::{PacketAnalyzer, PacketFilter};
use super::formatter::{OutputWriter, PacketFormatter, StatsFormatter};

pub fn analyze(
    input_path: &str,
    output_path: Option<&str>,
    packet_type: &str,
    max_packets: usize,
    start_time: Option<String>,
    end_time: Option<String>,
) -> Result<()> {
    let path = Path::new(input_path);
    let reader_factory: Box<dyn Fn(File) -> Box<dyn PacketReaderTrait>> = match packet_type {
        "standard" => Box::new(|file| Box::new(PacketReader::new(file))),
        "mixed" => Box::new(|file| Box::new(MixedPacketReader::new(file))),
        "mixed-legacy" => Box::new(|file| Box::new(LegacyPacketReader::new(file))),
        // Future types can be added here
        _ => return Err(anyhow::anyhow!("Unsupported packet type: {}", packet_type)),
    };
    if path.is_file() {
        analyze_file(
            input_path,
            output_path,
            max_packets,
            start_time,
            end_time,
            &reader_factory,
        )
    } else if path.is_dir() {
        analyze_directory(
            input_path,
            output_path,
            max_packets,
            start_time,
            end_time,
            &reader_factory,
        )
    } else {
        Err(anyhow::anyhow!("Input path is neither file nor directory"))
    }
}

/// Analyze a single file with the new architecture
pub fn analyze_file(
    file_path: &str,
    output_path: Option<&str>,
    max_packets: usize,
    start_time: Option<String>,
    end_time: Option<String>,
    reader_factory: &dyn Fn(File) -> Box<dyn PacketReaderTrait>,
) -> Result<()> {
    let mut writer = OutputWriter::new(output_path)?;
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;

    writer.writeln(&format!("=== Analyzing: {} ===", file_path))?;
    writer.writeln(&format!("Max packets: {}", max_packets))?;
    writer.writeln("")?;

    // Create components
    let mut reader = reader_factory(file);
    let mut analyzer = PacketAnalyzer::new();
    let filter = PacketFilter::new(start_time, end_time);

    // Process packets
    let mut packet_count = 0;
    let mut processed_count = 0;
    let mut objects_map = std::collections::HashMap::<i32, String>::new();

    for packet in reader.read_packets()? {
        packet_count += 1;

        // Apply time filter
        if !filter.should_include(&packet.timestamp) {
            continue;
        }

        // Check if we should stop
        if filter.is_past_end(&packet.timestamp) {
            writer.writeln(&format!("Reached end time filter at packet #{}", packet_count))?;
            break;
        }

        // Analyze packet
        analyzer.analyze_packet(&packet);
        processed_count += 1;

        // Format each packet
        PacketFormatter::new(&mut objects_map).format_packet(&mut writer, processed_count, &packet)?;

        // Check limit
        if processed_count >= max_packets {
            writer.writeln(&format!("Reached packet limit: {}", max_packets))?;
            break;
        }
    }

    // Show statistics
    writer.writeln(&format!("Total packets read: {}", packet_count))?;
    writer.writeln(&format!("Packets processed: {}", processed_count))?;
    StatsFormatter::format_stats(&mut writer, analyzer.stats())?;

    writer.flush()?;
    Ok(())
}

/// Analyze multiple files in a directory
pub fn analyze_directory(
    dir_path: &str,
    output_path: Option<&str>,
    max_packets_per_file: usize,
    start_time: Option<String>,
    end_time: Option<String>,
    reader_factory: &dyn Fn(File) -> Box<dyn PacketReaderTrait>,
) -> Result<()> {
    let mut writer = OutputWriter::new(output_path)?;
    let path = Path::new(dir_path);

    // Collect and sort files
    let files = collect_files(path)?;

    writer.writeln(&format!("=== Batch Analysis: {} ===", dir_path))?;
    writer.writeln(&format!("Total files: {}", files.len()))?;
    writer.writeln(&format!("Max packets per file: {}", max_packets_per_file))?;
    writer.writeln("")?;

    // Combined analyzer for all files
    let mut combined_analyzer = PacketAnalyzer::new();
    let filter = PacketFilter::new(start_time.clone(), end_time.clone());

    // Process each file
    for (index, file_path) in files.iter().enumerate() {
        writer.writeln(&format!(
            "--- File {}/{}: {} ---",
            index + 1,
            files.len(),
            file_path.display()
        ))?;

        match analyze_single_file(file_path, max_packets_per_file, &filter, &reader_factory) {
            Ok(file_analyzer) => {
                let stats = file_analyzer.stats();
                writer.writeln(&format!("  Packets analyzed: {}", stats.total_packets))?;
                combined_analyzer.merge(&file_analyzer);
            }
            Err(e) => {
                writer.writeln(&format!("  Error: {}", e))?;
            }
        }

        writer.writeln("")?;
    }

    // Show combined statistics
    writer.writeln("=== COMBINED STATISTICS ===")?;
    StatsFormatter::format_stats(&mut writer, combined_analyzer.stats())?;

    writer.flush()?;
    Ok(())
}

// Helper: analyze single file without output
fn analyze_single_file(
    file_path: &Path,
    max_packets: usize,
    filter: &PacketFilter,
    reader_factory: &dyn Fn(File) -> Box<dyn PacketReaderTrait>
) -> Result<PacketAnalyzer> {
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;

    let mut reader = reader_factory(file);
    let mut analyzer = PacketAnalyzer::new();

    let mut count = 0;
    for packet in reader.read_packets()? {

        if !filter.should_include(&packet.timestamp) {
            continue;
        }

        if filter.is_past_end(&packet.timestamp) {
            break;
        }

        analyzer.analyze_packet(&packet);
        count += 1;

        if count >= max_packets {
            break;
        }
    }

    Ok(analyzer)
}

// Helper: collect files from directory
fn collect_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    use std::fs;

    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let metadata = path.metadata().ok()?;
            let modified = metadata.modified().ok()?;
            Some((path, modified))
        })
        .collect();

    // Sort by modification time
    entries.sort_by_key(|(_, time)| *time);

    Ok(entries.into_iter().map(|(path, _)| path).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_file_architecture() {
        // Test the new architecture with a sample file
    }
}
