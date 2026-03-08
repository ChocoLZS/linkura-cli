use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{DirEntry, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use super::proto::PacketInfo;
use super::proto::define::data_frame;
use super::proto::extension::{UpdateObjectExt, prefab_name};
use super::proto::reader::{PacketReaderTrait, PacketsBufferReader, ReaderLimits};

#[derive(Debug, Clone)]
pub struct ExtractConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub packet_count: usize,
    pub file_count_limit: usize,
    pub data_start_time: Option<DateTime<Utc>>,
    pub data_end_time: Option<DateTime<Utc>>,
    pub strict: bool,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("data"),
            output_dir: PathBuf::from("extract"),
            packet_count: usize::MAX,
            file_count_limit: usize::MAX,
            data_start_time: None,
            data_end_time: None,
            strict: false,
        }
    }
}

#[cfg(feature = "audio")]
#[derive(Debug, Clone, Default)]
pub struct AudioExtractOptions {}

#[derive(Debug, Clone)]
pub struct ImageExtractOptions {
    pub metadata_filename: String,
}

impl Default for ImageExtractOptions {
    fn default() -> Self {
        Self {
            metadata_filename: "images.jsonl".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExtractTargetKind {
    #[cfg(feature = "audio")]
    Audio(AudioExtractOptions),
    Image(ImageExtractOptions),
}

impl ExtractTargetKind {
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "audio")]
            ExtractTargetKind::Audio(_) => "audio",
            ExtractTargetKind::Image(_) => "image",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExtractSummary {
    pub target: String,
    pub packets_read: usize,
    pub packets_filtered: usize,
    pub packets_processed: usize,
    pub target_hits: usize,
    pub outputs_written: usize,
    pub errors: usize,
    pub files_processed: usize,
}

#[derive(Debug, Clone, Default)]
struct ExtractStats {
    packets_read: usize,
    packets_filtered: usize,
    packets_processed: usize,
    target_hits: usize,
    outputs_written: usize,
    errors: usize,
    files_processed: usize,
}

#[derive(Debug, Clone)]
struct ObjectInfo {
    prefab_name: String,
}

#[derive(Debug)]
struct ExtractContext {
    object_registry: HashMap<i32, ObjectInfo>,
    stats: ExtractStats,
}

impl ExtractContext {
    fn new() -> Self {
        Self {
            object_registry: HashMap::new(),
            stats: ExtractStats::default(),
        }
    }

    fn update_object_registry(&mut self, packet: &PacketInfo) {
        for frame in &packet.data_pack.frames {
            match &frame.message {
                Some(data_frame::Message::InstantiateObject(obj)) => {
                    let prefab_name = String::from_utf8_lossy(&obj.prefab_name).to_string();
                    self.object_registry
                        .insert(obj.object_id, ObjectInfo { prefab_name });
                }
                Some(data_frame::Message::DestroyObject(obj)) => {
                    self.object_registry.remove(&obj.object_id);
                }
                _ => {}
            }
        }
    }

    #[cfg(feature = "audio")]
    fn is_audio_object(&self, object_id: i32) -> bool {
        self.object_registry
            .get(&object_id)
            .map(|x| x.prefab_name.contains("Voice"))
            .unwrap_or(false)
    }

    fn is_cover_image_object(&self, object_id: i32) -> bool {
        self.object_registry
            .get(&object_id)
            .map(|x| x.prefab_name.contains(prefab_name::COVER_IMAGE_RECEIVER))
            .unwrap_or(false)
    }
}

trait ExtractTarget {
    fn name(&self) -> &'static str;
    fn start(&mut self, _ctx: &mut ExtractContext) -> Result<()> {
        Ok(())
    }
    fn on_packet(&mut self, packet: &PacketInfo, ctx: &mut ExtractContext) -> Result<()>;
    fn finish(&mut self, _ctx: &mut ExtractContext) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "audio")]
struct AudioExtractTarget {
    output_dir: PathBuf,
    builder: super::audio::AudioBuilder,
}

#[cfg(feature = "audio")]
impl AudioExtractTarget {
    fn new(output_dir: PathBuf) -> Self {
        let output_dir_string = output_dir.to_str().map(str::to_owned);
        Self {
            output_dir,
            builder: super::audio::AudioBuilder::new(output_dir_string),
        }
    }
}

#[cfg(feature = "audio")]
impl ExtractTarget for AudioExtractTarget {
    fn name(&self) -> &'static str {
        "audio"
    }

    fn on_packet(&mut self, packet: &PacketInfo, ctx: &mut ExtractContext) -> Result<()> {
        for frame in &packet.data_pack.frames {
            if let Some(data_frame::Message::UpdateObject(obj)) = &frame.message {
                if ctx.is_audio_object(obj.object_id) {
                    ctx.stats.target_hits += 1;
                    break;
                }
            }
        }
        self.builder.handle_audio_packet(packet);
        Ok(())
    }

    fn finish(&mut self, ctx: &mut ExtractContext) -> Result<()> {
        let generated_file_count = self.builder.audio_packets.len();
        self.builder
            .write_to_file(&self.output_dir)
            .with_context(|| "failed to write extracted audio files")?;
        ctx.stats.outputs_written += generated_file_count;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct ImageRecord {
    timestamp: String,
    object_id: i32,
    cover_image_name: String,
    sync_time: f64,
}

struct ImageExtractTarget {
    output_dir: PathBuf,
    options: ImageExtractOptions,
    cover_image_object_ids: HashSet<i32>,
    records: Vec<ImageRecord>,
}

impl ImageExtractTarget {
    fn new(output_dir: PathBuf, options: ImageExtractOptions) -> Self {
        Self {
            output_dir,
            options,
            cover_image_object_ids: HashSet::new(),
            records: Vec::new(),
        }
    }
}

impl ExtractTarget for ImageExtractTarget {
    fn name(&self) -> &'static str {
        "image"
    }

    fn on_packet(&mut self, packet: &PacketInfo, ctx: &mut ExtractContext) -> Result<()> {
        for frame in &packet.data_pack.frames {
            match &frame.message {
                Some(data_frame::Message::InstantiateObject(obj)) => {
                    let prefab_name = String::from_utf8_lossy(&obj.prefab_name);
                    if prefab_name.contains(prefab_name::COVER_IMAGE_RECEIVER) {
                        self.cover_image_object_ids.insert(obj.object_id);
                    }
                }
                Some(data_frame::Message::DestroyObject(obj)) => {
                    self.cover_image_object_ids.remove(&obj.object_id);
                }
                Some(data_frame::Message::UpdateObject(obj)) => {
                    if !(self.cover_image_object_ids.contains(&obj.object_id)
                        || ctx.is_cover_image_object(obj.object_id))
                    {
                        continue;
                    }

                    let parsed = obj.try_parse_cover_image().with_context(|| {
                        format!(
                            "failed to parse cover image payload for object {}",
                            obj.object_id
                        )
                    })?;

                    self.records.push(ImageRecord {
                        timestamp: packet.timestamp.to_rfc3339(),
                        object_id: obj.object_id,
                        cover_image_name: parsed.cover_image_name,
                        sync_time: parsed.sync_time,
                    });
                    ctx.stats.target_hits += 1;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn finish(&mut self, ctx: &mut ExtractContext) -> Result<()> {
        std::fs::create_dir_all(&self.output_dir).with_context(|| {
            format!(
                "failed to create image output directory: {}",
                self.output_dir.display()
            )
        })?;

        let output_file = self.output_dir.join(&self.options.metadata_filename);
        let file = File::create(&output_file)
            .with_context(|| format!("failed to create output file: {}", output_file.display()))?;
        let mut writer = BufWriter::new(file);

        for record in &self.records {
            let line = serde_json::to_string(record)
                .with_context(|| "failed to serialize image record")?;
            writeln!(writer, "{line}")?;
        }
        writer.flush()?;

        ctx.stats.outputs_written += self.records.len();
        Ok(())
    }
}

fn build_target(
    target_kind: &ExtractTargetKind,
    output_dir: PathBuf,
) -> Result<Box<dyn ExtractTarget>> {
    match target_kind {
        #[cfg(feature = "audio")]
        ExtractTargetKind::Audio(_) => Ok(Box::new(AudioExtractTarget::new(output_dir))),
        ExtractTargetKind::Image(options) => Ok(Box::new(ImageExtractTarget::new(
            output_dir,
            options.clone(),
        ))),
    }
}

fn collect_standard_entries(input_dir: &Path) -> Result<VecDeque<DirEntry>> {
    if !input_dir.is_dir() {
        return Err(anyhow!(
            "input path is not a directory: {}",
            input_dir.display()
        ));
    }

    let mut input_files = std::fs::read_dir(input_dir)
        .with_context(|| format!("failed to read directory: {}", input_dir.display()))?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "ts")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    input_files.sort_by(|a, b| {
        let num_a = extract_segment_number(a).unwrap_or(0);
        let num_b = extract_segment_number(b).unwrap_or(0);
        num_a.cmp(&num_b)
    });

    if input_files.is_empty() {
        return Err(anyhow!(
            "no standard segment files (*.ts) found in {}",
            input_dir.display()
        ));
    }

    Ok(VecDeque::from(input_files))
}

fn extract_segment_number(entry: &DirEntry) -> Option<u64> {
    entry
        .file_name()
        .to_str()?
        .rsplit('_')
        .next()?
        .split('.')
        .next()?
        .parse::<u64>()
        .ok()
}

fn run_pipeline(
    config: &ExtractConfig,
    ctx: &mut ExtractContext,
    target: &mut dyn ExtractTarget,
) -> Result<()> {
    let file_entries = collect_standard_entries(&config.input_dir)?;
    let limits = ReaderLimits::unlimited()
        .with_max_packets(config.packet_count)
        .with_max_files(config.file_count_limit);
    let mut packet_reader = PacketsBufferReader::new_standard(file_entries).with_limits(limits);

    target.start(ctx)?;

    while let Some(packet) = packet_reader.read_packet()? {
        ctx.stats.packets_read += 1;
        ctx.update_object_registry(&packet);

        if let Some(end_time) = config.data_end_time {
            if packet.timestamp > end_time {
                break;
            }
        }

        if let Some(start_time) = config.data_start_time {
            if packet.timestamp < start_time {
                ctx.stats.packets_filtered += 1;
                continue;
            }
        }

        match target.on_packet(&packet, ctx) {
            Ok(()) => {
                ctx.stats.packets_processed += 1;
            }
            Err(err) => {
                ctx.stats.errors += 1;
                if config.strict {
                    return Err(err).with_context(|| {
                        format!(
                            "extract target '{}' failed at timestamp {}",
                            target.name(),
                            packet.timestamp
                        )
                    });
                }
                tracing::warn!(
                    "extract target '{}' failed at {}: {:#}",
                    target.name(),
                    packet.timestamp,
                    err
                );
            }
        }
    }

    ctx.stats.files_processed = packet_reader.stats().files_processed;
    target.finish(ctx)?;
    Ok(())
}

pub fn run_extract(
    config: ExtractConfig,
    target_kind: ExtractTargetKind,
) -> Result<ExtractSummary> {
    if !config.input_dir.exists() {
        return Err(anyhow!(
            "input directory does not exist: {}",
            config.input_dir.display()
        ));
    }

    std::fs::create_dir_all(&config.output_dir).with_context(|| {
        format!(
            "failed to create output directory: {}",
            config.output_dir.display()
        )
    })?;

    let mut ctx = ExtractContext::new();
    let mut target = build_target(&target_kind, config.output_dir.clone())?;
    tracing::info!(
        "starting '{}' extraction: input={}, output={}",
        target.name(),
        config.input_dir.display(),
        config.output_dir.display()
    );

    run_pipeline(&config, &mut ctx, target.as_mut())?;

    Ok(ExtractSummary {
        target: target_kind.name().to_string(),
        packets_read: ctx.stats.packets_read,
        packets_filtered: ctx.stats.packets_filtered,
        packets_processed: ctx.stats.packets_processed,
        target_hits: ctx.stats.target_hits,
        outputs_written: ctx.stats.outputs_written,
        errors: ctx.stats.errors,
        files_processed: ctx.stats.files_processed,
    })
}
