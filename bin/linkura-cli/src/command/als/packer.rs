use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result};
use chrono::Local;
use flate2::{Compression, write::GzEncoder};

use super::model::PlanItem;

pub fn task_raw_dir(base: &Path, item: &PlanItem) -> PathBuf {
    let timestamp = item.live_start_time.with_timezone(&Local).format("%y-%m-%d");
    let live_id = sanitize_path_fragment(&item.live_id);
    base.join(format!("{}+{}", timestamp, live_id))
}

pub fn create_task_raw_dir(base: &Path, item: &PlanItem) -> Result<PathBuf> {
    let dir = task_raw_dir(base, item);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create raw dir {}", dir.display()))?;
    Ok(dir)
}

pub fn package_recording(source_dir: &Path, archive_root: &Path, item: &PlanItem) -> Result<()> {
    if !source_dir.exists() {
        return Ok(());
    }
    if std::fs::read_dir(source_dir)?.next().is_none() {
        return Ok(());
    }

    tracing::info!(
        "Packing directory: live_id={}, live_type={}, source={}",
        item.live_id,
        item.live_type,
        source_dir.display()
    );
    let start = Instant::now();
    let (file_count, total_bytes) = collect_dir_stats(source_dir)?;
    tracing::info!(
        "Pack source stats: live_id={}, live_type={}, files={}, bytes={}",
        item.live_id,
        item.live_type,
        file_count,
        total_bytes
    );

    std::fs::create_dir_all(archive_root)
        .with_context(|| format!("failed to create archive root {}", archive_root.display()))?;
    let base_name = item.live_open_time.with_timezone(&Local).format("%y-%m-%d");
    let mut archive_path = archive_root.join(format!("{base_name}.tar.gz"));
    if archive_path.exists() {
        let suffix = Local::now().format("%H%M%S");
        archive_path = archive_root.join(format!("{base_name}-{suffix}.tar.gz"));
    }

    let file = File::create(&archive_path)
        .with_context(|| format!("failed to create archive {}", archive_path.display()))?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar_builder = tar::Builder::new(encoder);
    let dir_name = source_dir
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("als_record");
    tar_builder
        .append_dir_all(dir_name, source_dir)
        .with_context(|| {
            format!(
                "failed to append dir {} into archive {}",
                source_dir.display(),
                archive_path.display()
            )
        })?;
    let encoder = tar_builder
        .into_inner()
        .context("failed to finalize tar stream")?;
    encoder.finish().context("failed to finish gzip stream")?;

    tracing::info!(
        "Pack done: live_id={}, live_type={}, archive={}, elapsed_ms={}",
        item.live_id,
        item.live_type,
        archive_path.display(),
        start.elapsed().as_millis()
    );
    Ok(())
}

fn collect_dir_stats(root: &Path) -> Result<(u64, u64)> {
    let mut files = 0u64;
    let mut bytes = 0u64;
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)
            .with_context(|| format!("failed to read dir while packing: {}", dir.display()))?
        {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                stack.push(entry.path());
                continue;
            }
            if ty.is_file() {
                files += 1;
                bytes += entry.metadata()?.len();
            }
        }
    }

    Ok((files, bytes))
}

fn sanitize_path_fragment(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}
