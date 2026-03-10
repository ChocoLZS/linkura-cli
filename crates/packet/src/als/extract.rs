use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::DirEntry;
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
    pub json: bool,
    pub output_file: Option<PathBuf>,
}

impl Default for ImageExtractOptions {
    fn default() -> Self {
        Self {
            json: false,
            output_file: None,
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

    fn is_scene_prop_object(&self, object_id: i32) -> bool {
        self.object_registry
            .get(&object_id)
            .map(|x| x.prefab_name.contains(prefab_name::SCENE_PROP_MANIPULATOR))
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

const COVER_IMAGE_WEB_PREFIX: &str = "http://assets.link-like-lovelive.app/wiht_fes_images";

struct ImageExtractTarget {
    options: ImageExtractOptions,
    cover_image_object_ids: HashSet<i32>,
    scene_prop_object_ids: HashSet<i32>,
    seen_names: HashSet<String>,
    names: Vec<String>,
}

impl ImageExtractTarget {
    fn new(options: ImageExtractOptions) -> Self {
        Self {
            options,
            cover_image_object_ids: HashSet::new(),
            scene_prop_object_ids: HashSet::new(),
            seen_names: HashSet::new(),
            names: Vec::new(),
        }
    }

    fn normalize_name(name: &str) -> Option<String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return None;
        }
        if trimmed.to_ascii_lowercase().ends_with(".jpg") {
            let normalized_path = trimmed.trim_start_matches('/');
            return Some(format!("{}/{}", COVER_IMAGE_WEB_PREFIX, normalized_path));
        }
        Some(trimmed.to_string())
    }

    fn push_name(&mut self, name: &str, ctx: &mut ExtractContext) {
        let Some(normalized) = Self::normalize_name(name) else {
            return;
        };
        if self.seen_names.insert(normalized.clone()) {
            self.names.push(normalized);
            ctx.stats.target_hits += 1;
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
                    if prefab_name.contains(prefab_name::SCENE_PROP_MANIPULATOR) {
                        self.scene_prop_object_ids.insert(obj.object_id);
                    }
                }
                Some(data_frame::Message::DestroyObject(obj)) => {
                    self.cover_image_object_ids.remove(&obj.object_id);
                    self.scene_prop_object_ids.remove(&obj.object_id);
                }
                Some(data_frame::Message::UpdateObject(obj)) => {
                    if self.cover_image_object_ids.contains(&obj.object_id)
                        || ctx.is_cover_image_object(obj.object_id)
                    {
                        match obj.try_parse_cover_image() {
                            Ok(parsed) => {
                                self.push_name(&parsed.cover_image_name, ctx);
                            }
                            Err(_) => {
                                // noop;
                            }
                        }
                    }

                    if self.scene_prop_object_ids.contains(&obj.object_id)
                        || ctx.is_scene_prop_object(obj.object_id)
                    {
                        let parsed = obj.try_parse_scene_prop_manipulator().with_context(|| {
                            format!(
                                "failed to parse scene prop payload for object {} (method={})",
                                obj.object_id, obj.method
                            )
                        })?;
                        if let Some(trigger) = parsed.animation_trigger {
                            self.push_name(&trigger, ctx);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn finish(&mut self, ctx: &mut ExtractContext) -> Result<()> {
        if let Some(output_file) = &self.options.output_file {
            if let Some(parent) = output_file.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).with_context(|| {
                        format!(
                            "failed to create output file parent directory: {}",
                            parent.display()
                        )
                    })?;
                }
            }

            if self.options.json {
                let json = serde_json::to_string(&self.names)
                    .with_context(|| "failed to serialize image names as json")?;
                std::fs::write(output_file, json.as_bytes()).with_context(|| {
                    format!("failed to write output file: {}", output_file.display())
                })?;
            } else {
                let mut text = self.names.join("\n");
                if !text.is_empty() {
                    text.push('\n');
                }
                std::fs::write(output_file, text.as_bytes()).with_context(|| {
                    format!("failed to write output file: {}", output_file.display())
                })?;
            }
        } else if self.options.json {
            let json = serde_json::to_string(&self.names)
                .with_context(|| "failed to serialize image names as json")?;
            println!("{json}");
        } else {
            for name in &self.names {
                println!("{name}");
            }
        }
        ctx.stats.outputs_written += self.names.len();
        Ok(())
    }
}

fn build_target(
    target_kind: &ExtractTargetKind,
    _output_dir: PathBuf,
) -> Result<Box<dyn ExtractTarget>> {
    match target_kind {
        #[cfg(feature = "audio")]
        ExtractTargetKind::Audio(_) => Ok(Box::new(AudioExtractTarget::new(_output_dir))),
        ExtractTargetKind::Image(options) => Ok(Box::new(ImageExtractTarget::new(options.clone()))),
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

    let requires_output_dir = match &target_kind {
        #[cfg(feature = "audio")]
        ExtractTargetKind::Audio(_) => true,
        ExtractTargetKind::Image(_) => false,
    };
    if requires_output_dir {
        std::fs::create_dir_all(&config.output_dir).with_context(|| {
            format!(
                "failed to create output directory: {}",
                config.output_dir.display()
            )
        })?;
    }

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
