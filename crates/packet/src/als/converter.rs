use super::proto::{
    PacketInfo,
    define::{
        CurrentPlayer, DataFrame, DataPack, Room, RoomAll, data_frame, data_pack, destroy_object,
        instantiate_object, update_object,
    },
    reader::PacketReaderTrait,
};
use crate::als::proto::{
    extension::UpdateObjectExt,
    reader::{LegacyPacketReader, MixedPacketReader, PacketsBufferReader, StandardPacketReader},
};
use anyhow::{Context, Ok, Result, anyhow};
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use std::path::Path;
use std::{
    cmp::Ordering,
    fs::{DirEntry, File},
    path::PathBuf,
};
use std::{
    collections::HashSet,
    io::{BufWriter, Write},
};

#[cfg(feature = "audio")]
use super::audio::AudioBuilder;

#[derive(PartialEq, Eq, Debug)]
enum AlsConverterStateMachine {
    Initial,
    FirstDataframes,
    UpdateObjects,
    Pong,
    Split,
    End,
}

pub struct AlsConverter {
    #[allow(unused)]
    segment_duration: u64, // microseconds, default 10 seconds
    use_audio_processing: bool, // 是否启用音频处理
}

impl Default for AlsConverter {
    fn default() -> Self {
        Self {
            segment_duration: 10_000_000, // 10 seconds in microseconds
            use_audio_processing: false,
        }
    }
}

impl AlsConverter {
    pub fn new(segment_duration_seconds: u64, use_audio_processing: bool) -> Self {
        Self {
            segment_duration: segment_duration_seconds * 1_000_000,
            use_audio_processing,
        }
    }

    fn get_file_entries(input_dir: &Path, ext: Option<&str>) -> Result<std::collections::VecDeque<DirEntry>> {
        if !input_dir.is_dir() {
            return Err(anyhow!("Input path is not a directory"));
        }
        // Read mixed packets from input directory
        let mut input_files = std::fs::read_dir(input_dir)?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|_ext| _ext == ext.unwrap_or("bin"))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        input_files.sort_by(|a, b| {
            let extract_number = |entry: &std::fs::DirEntry| -> Option<u64> {
                entry
                    .file_name()
                    .to_str()?
                    .rsplit('_')
                    .next()?
                    .split('.')
                    .next()?
                    .parse::<u64>()
                    .ok()
            };

            let num_a = extract_number(a).unwrap_or(0);
            let num_b = extract_number(b).unwrap_or(0);
            num_a.cmp(&num_b)
        });

        if input_files.is_empty() {
            return Err(anyhow!("No input files found"));
        }
        Ok(std::collections::VecDeque::from(input_files))
    }

    pub fn convert_mixed_to_standard<P: AsRef<Path>>(
        &self,
        input_dir: P,
        output_dir: P,
        convert_type: &str,
        // todo: config struct
        timeshift: i64,
        split: bool,
        start_time: Option<String>,
        data_start_time: Option<String>,
        data_end_time: Option<String>,
        metadata_path: Option<String>,
        auto_timestamp: bool,
    ) -> Result<()> {
        let input_dir = input_dir.as_ref();
        let output_dir = output_dir.as_ref();
        let mut context = ConversionContext::new(
            timeshift,
            split,
            start_time,
            data_start_time,
            data_end_time,
            metadata_path,
            output_dir.to_str().map(String::from),
            self.use_audio_processing,
            auto_timestamp,
        );
        let file_entries = Self::get_file_entries(input_dir, None)?;
        let mut packet_buffer = if convert_type == "als-legacy" {
            PacketsBufferReader::new(file_entries, |file| LegacyPacketReader::boxed(file))
        } else {
            PacketsBufferReader::new(file_entries, |file| MixedPacketReader::boxed(file))
        };

        self.process_all_packets(&mut context, &mut packet_buffer)?;
        self.finalize_conversion(&mut context, output_dir)?;
        Ok(())
    }

    #[cfg(feature = "audio")]
    pub fn extract_audio_from_standard<P: AsRef<Path>>(
        &self,
        input_dir: P,
        output_dir: P,
    ) -> Result<()> {
        let input_dir = input_dir.as_ref();
        let output_dir = output_dir.as_ref();
        // Process each audio file in the input directory
        let mut packet_buffer =
            PacketsBufferReader::new(Self::get_file_entries(input_dir, Some("ts"))?, |file| {
                StandardPacketReader::boxed(file)
            });
        let mut audio_builder = AudioBuilder::new(output_dir.to_str().map(String::from));
        while let Some(packet) = packet_buffer.read_packet()? {
            audio_builder.handle_audio_packet(&packet);
        }
        audio_builder.write_to_file(output_dir).unwrap_or_else(|e| {
            tracing::error!("Failed to write audio files: {:?}", e);
        });
        Ok(())
    }

    fn process_all_packets(
        &self,
        context: &mut ConversionContext,
        packet_buffer: &mut PacketsBufferReader,
    ) -> Result<()> {
        while let Some(packet_info) = packet_buffer.read_packet()? {
            let end = context.process_packet(packet_info)?;
            if end {
                break;
            }
        }
        Ok(())
    }

    fn finalize_conversion(
        &self,
        context: &mut ConversionContext,
        output_dir: &Path,
    ) -> Result<()> {
        tracing::debug!("All packets processed, writing final segment if exists.");
        if self.use_audio_processing {
            #[cfg(feature = "audio")]
            context.audio_builder.write_to_file(output_dir)?;
            #[cfg(not(feature = "audio"))]
            unreachable!("Audio feature is not enabled");
        } else {
            if context.auto_timestamp {
                context.handle_packetinfo_buffer()?;
            }
            context.segment_builder.write_to_file(
                output_dir,
                context.data_room.started_at,
                &context.data_room.id,
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Segment {
    number: u32,
    duration: f64, // seconds
    packets: Vec<PacketInfo>,
}

impl Segment {
    pub fn new(sequence: u32) -> Self {
        Segment {
            number: sequence,
            duration: 0.0,
            packets: Vec::new(),
        }
    }

    pub fn add(&mut self, packet_info: PacketInfo) -> &mut Self {
        self.packets.push(packet_info);
        self
    }
}

#[derive(Debug, Default)]
struct SegmentBuilder {
    current_sequence: u32,
    segments: Vec<Segment>,
    metadata_path: Option<String>,
    output_dir: Option<String>,
    part_count: u32,
    timeshift: i64,
}

impl SegmentBuilder {
    pub fn new(metadata_path: Option<String>, output_dir: Option<String>, timeshift: i64) -> Self {
        SegmentBuilder {
            current_sequence: 0,
            segments: Vec::new(),
            metadata_path,
            output_dir,
            part_count: 0,
            timeshift,
        }
    }

    pub fn add(&mut self, mut packet_info: PacketInfo) -> &mut Self {
        // add timeshift
        packet_info.timestamp = packet_info.timestamp + TimeDelta::microseconds(self.timeshift);
        if let Some(segment) = self.segments.last_mut() {
            // check if packet length will exceed 16k bytes 16 * 1024 bytes (maybe the official limit is 16k bytes)
            // but we use 12k bytes as threshold in case of some overhead
            if packet_info.to_vec().len() >= 15 * 1024 {
                let mut check_buf = Vec::new();
                let mut packets_buf: Vec<DataFrame> = Vec::new();
                for p in packet_info.data_pack.frames {
                    let frame_bytes = PacketInfo::frame_to_vec(&p);
                    if check_buf.len() + frame_bytes.len() < 15 * 1024 {
                        check_buf.extend_from_slice(&frame_bytes);
                        packets_buf.push(p);
                    } else {
                        segment.add(PacketInfo {
                            timestamp: packet_info.timestamp,
                            data_pack: DataPack {
                                control: packet_info.data_pack.control.clone(),
                                frames: std::mem::take(&mut packets_buf),
                            },
                            raw_data: Vec::new(),
                        });
                        check_buf.clear();
                        packets_buf.clear();
                    }
                }
                if !packets_buf.is_empty() {
                    segment.add(PacketInfo {
                        timestamp: packet_info.timestamp,
                        data_pack: DataPack {
                            control: packet_info.data_pack.control.clone(),
                            frames: std::mem::take(&mut packets_buf),
                        },
                        raw_data: Vec::new(),
                    });
                }
            } else {
                segment.add(packet_info);
            }
        }
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.segments.push(Segment::new(self.current_sequence));
        self.current_sequence += 1;
        self
    }

    pub fn start(&mut self) -> &mut Self {
        self.segments.clear();
        self.current_sequence = 0;
        self.part_count += 1;
        return self.next();
    }

    pub fn set_current_segment_duration(&mut self, duration: f64) -> &mut Self {
        if let Some(segment) = self.segments.last_mut() {
            segment.duration = duration;
        }
        self
    }

    // pub fn update_first_

    pub fn write(&mut self, started_at: i64, data_room_id: &[u8]) -> Result<()> {
        if let Some(output_dir) = self.output_dir.clone() {
            self.write_to_file(&output_dir, started_at, data_room_id)
        } else {
            Err(anyhow!("No output directory specified"))
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(
        &mut self,
        output_dir: P,
        started_at: i64,
        data_room_id: &[u8],
    ) -> Result<()> {
        let output_dir = if self.part_count > 1 {
            PathBuf::from(format!(
                "{}_{:03}",
                output_dir.as_ref().to_string_lossy(),
                self.part_count - 1
            ))
        } else {
            PathBuf::from(output_dir.as_ref())
        };
        tracing::debug!("Writing segments to directory: {:?}", output_dir);
        std::fs::create_dir_all(&output_dir)?;
        let last_segment = self.segments.last_mut().unwrap();
        last_segment.duration = (|| {
            let last_timestamp = last_segment.packets.last().unwrap().timestamp;
            let first_timestamp = last_segment.packets.first().unwrap().timestamp;
            (last_timestamp - first_timestamp)
                .num_microseconds()
                .unwrap_or(0) as f64
                / 1_000_000.0
        })();
        for segment in &self.segments {
            let segment_file_path = output_dir.join(format!("segment_{:05}.ts", segment.number));
            let file = File::create(&segment_file_path).with_context(|| {
                format!("Failed to create segment file: {:?}", segment_file_path)
            })?;
            let mut writer = BufWriter::new(file);

            for packet in &segment.packets {
                writer.write_all(&packet.to_vec())?;
            }
            writer.flush()?;
        }
        // m3u8
        let m3u8_file_path = output_dir.join("index.m3u8");
        let mut m3u8_file = File::create(&m3u8_file_path)
            .with_context(|| format!("Failed to create m3u8 file: {:?}", m3u8_file_path))?;
        // write template
        writeln!(m3u8_file, "#EXTM3U8")?;
        writeln!(m3u8_file, "#EXT-X-VERSION:3")?;
        writeln!(m3u8_file, "#EXT-X-PLAYLIST-TYPE:VOD")?;
        writeln!(m3u8_file, "#EXT-X-MEDIA-SEQUENCE:0")?;
        writeln!(m3u8_file, "#EXT-X-TARGETDURATION:10")?;
        for segment in &self.segments {
            writeln!(
                m3u8_file,
                "#EXTINF:{:.3},\nsegment_{:05}.ts",
                segment.duration, segment.number
            )?;
        }
        writeln!(m3u8_file, "#EXT-X-ENDLIST")?;

        // metadata file
        let metadata_file_path = output_dir.join("index.md");
        let mut metadata_file = File::create(&metadata_file_path)
            .with_context(|| format!("Failed to create metadata file: {:?}", metadata_file_path))?;
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let live_started_at = chrono::DateTime::<Utc>::from_timestamp_micros(started_at)
            .unwrap_or_else(|| Utc::now())
            .with_timezone(&jst_offset)
            .to_rfc3339();
        let joined_room_at = self
            .segments
            .first()
            .unwrap()
            .packets
            .first()
            .unwrap()
            .timestamp
            .with_timezone(&jst_offset)
            .to_rfc3339();
        let metadata = serde_json::json!({
            "path": self.metadata_path.as_deref().unwrap_or("/"),
            "room_id": std::str::from_utf8(&data_room_id)
                .unwrap_or("unknown_room_id"),
            "playlist_file": "index.m3u8",
            "live_started_at": live_started_at,
            "joined_room_at": joined_room_at,
        });
        writeln!(metadata_file, "{}", metadata.to_string())?;

        Ok(())
    }
}

static DURATION: TimeDelta = TimeDelta::seconds(10);
// 创建一个上下文结构体来管理状态
struct ConversionContext {
    state: AlsConverterStateMachine,
    data_room: Room,
    initial_timestamp: DateTime<Utc>,
    initial_dataframes: Vec<DataFrame>,
    split_write_mode: bool,
    start_time: Option<DateTime<Utc>>,
    data_start_time: Option<DateTime<Utc>>,
    data_end_time: Option<DateTime<Utc>>,
    use_audio_processing: bool,
    segment_builder: SegmentBuilder,
    #[cfg(feature = "audio")]
    audio_builder: AudioBuilder,

    /// 根据回放包的 audio 与datetime receiver来自动计算时间戳
    auto_timestamp: bool,
    packetinfo_buffer: Vec<PacketInfo>,
}

impl ConversionContext {
    fn new(
        timeshift: i64,
        split_write_mode: bool,
        start_time: Option<String>,
        data_start_time: Option<String>,
        data_end_time: Option<String>,
        metadata_path: Option<String>,
        output_dir: Option<String>,
        use_audio_processing: bool,
        auto_timestamp: bool,
    ) -> Self {
        let mut st: Option<DateTime<Utc>> = None;
        let mut dst: Option<DateTime<Utc>> = None;
        let mut det: Option<DateTime<Utc>> = None;
        if let Some(start_time) = start_time {
            st = Some(
                DateTime::parse_from_rfc3339(&start_time)
                    .unwrap()
                    .with_timezone(&Utc),
            )
        }
        if let Some(data_start_time) = data_start_time {
            dst = Some(
                DateTime::parse_from_rfc3339(&data_start_time)
                    .unwrap()
                    .with_timezone(&Utc),
            )
        }
        if let Some(data_end_time) = data_end_time {
            det = Some(
                DateTime::parse_from_rfc3339(&data_end_time)
                    .unwrap()
                    .with_timezone(&Utc),
            )
        }
        Self {
            state: AlsConverterStateMachine::Initial,
            data_room: Room {
                id: vec![0],
                started_at: 0,
                ended_at: 0,
            },
            initial_timestamp: DateTime::<Utc>::from_timestamp_micros(0).unwrap(),
            segment_builder: SegmentBuilder::new(metadata_path, output_dir.clone(), timeshift),
            initial_dataframes: Vec::new(),
            split_write_mode,
            start_time: st,
            data_start_time: dst,
            data_end_time: det,
            use_audio_processing,
            auto_timestamp,
            packetinfo_buffer: Vec::new(),
            #[cfg(feature = "audio")]
            audio_builder: AudioBuilder::new(output_dir),
        }
    }

    fn swap_order(dataframes: &mut Vec<DataFrame>) {
        let mut fixed_camera_index = None;
        let mut cameraman_index = None;

        // Find the indices of FixedCamera and Cameraman
        for (i, frame) in dataframes.iter().enumerate() {
            if let Some(data_frame::Message::InstantiateObject(obj)) = &frame.message {
                let name = String::from_utf8_lossy(&obj.prefab_name);
                if name.contains("Camera/FixedCamera") {
                    fixed_camera_index = Some(i);
                } else if name.contains("Camera/Cameraman") {
                    cameraman_index = Some(i);
                }
            }
        }

        // If both are found and FixedCamera comes before Cameraman, swap them
        if let (Some(fixed_idx), Some(cameraman_idx)) = (fixed_camera_index, cameraman_index) {
            if fixed_idx > cameraman_idx {
                dataframes.swap(fixed_idx, cameraman_idx);
            }
        }
    }

    fn compare_dataframes(a: &DataFrame, b: &DataFrame) -> Ordering {
        match (a.message.as_ref(), b.message.as_ref()) {
            (
                Some(data_frame::Message::InstantiateObject(_)),
                Some(data_frame::Message::InstantiateObject(_)),
            ) => Ordering::Equal,
            (Some(data_frame::Message::InstantiateObject(_)), _) => Ordering::Less,
            (_, Some(data_frame::Message::InstantiateObject(_))) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    fn process_packet(&mut self, packet_info: PacketInfo) -> Result<bool> {
        let timestamp = packet_info.timestamp;
        if let Some(data_end_time) = &self.data_end_time {
            if timestamp > *data_end_time {
                tracing::info!(
                    "Data end time reached: {}, current timestamp: {}, no longer to process remain packets.",
                    data_end_time,
                    timestamp
                );
                // end processing
                self.state = AlsConverterStateMachine::End;
                return Ok(true);
            }
        }

        match self.state {
            AlsConverterStateMachine::Initial => {
                self.process_initial_state(packet_info)?;
            }
            AlsConverterStateMachine::FirstDataframes => {
                self.process_first_dataframes_state(packet_info)?;
            }
            AlsConverterStateMachine::Pong | AlsConverterStateMachine::UpdateObjects => {
                self.process_update_objects_state(packet_info)?;
            }
            AlsConverterStateMachine::Split => {
                tracing::debug!("Segment ended, writing to file and starting new segment.");
                if self.use_audio_processing {
                    #[cfg(feature = "audio")]
                    self.audio_builder.write()?;
                    #[cfg(not(feature = "audio"))]
                    unreachable!("Audio feature is not enabled");
                } else {
                    if self.auto_timestamp {
                        self.handle_packetinfo_buffer()?;
                    }
                    self.segment_builder
                        .write(self.data_room.started_at, &self.data_room.id)?;
                }
                self.state = AlsConverterStateMachine::FirstDataframes;
            }
            AlsConverterStateMachine::End => {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn process_initial_state(&mut self, data_packet: PacketInfo) -> Result<()> {
        let frame = self
            .get_first_dataframe(&data_packet)
            .ok_or_else(|| anyhow!("No DataFrame found in initial fragment"))?;

        let message = frame
            .message
            .as_ref()
            .ok_or_else(|| anyhow!("No message in DataFrame"))?;

        match message {
            data_frame::Message::AuthorizeResponse(_) => {
                // Continue to next iteration
            }
            data_frame::Message::JoinRoomResponse(_) => {
                // Continue to next state
                self.state = AlsConverterStateMachine::FirstDataframes;
            }
            data_frame::Message::Room(msg) => {
                self.data_room.clone_from(&msg);
                // self.data_room.id = "default-114514".as_bytes().to_vec();
                // self.data_room.id = "".as_bytes().to_vec();
            }
            _ => {}
        }

        Ok(())
    }

    /// data packet 应该是 DataFrames(InstantiateObject|UpdateObject)
    fn process_first_dataframes_state(&mut self, mut packet_info: PacketInfo) -> Result<()> {
        // control message 判断必须是Data
        if !packet_info
            .data_pack
            .control
            .as_ref()
            .map_or(false, |c| matches!(c, data_pack::Control::Data(true)))
        {
            return Ok(());
        }
        // 第一个 dataframe 必须是 InstantiateObject
        if !packet_info.data_pack.frames.first().map_or(false, |f| {
            matches!(f.message, Some(data_frame::Message::InstantiateObject(_)))
        }) {
            return Ok(());
        }
        if let Some(start_time) = &self.start_time {
            let timestamp = packet_info.timestamp;
            if timestamp < *start_time {
                // skip this packet
                return Ok(());
            } else {
                // only check once
                self.start_time = None;
            }
        }
        let timestamp = packet_info.timestamp;
        self.initial_timestamp = timestamp;

        if !self.auto_timestamp {
            self.segment_builder
                .start()
                .add(PacketInfo::create_segment_started_packet(timestamp))
                .add(PacketInfo::create_room_frame(
                    timestamp,
                    self.data_room.clone(),
                ))
                .add(PacketInfo::create_cache_end(timestamp));
        }

        for frame in &mut packet_info.data_pack.frames {
            if let Some(message) = &mut frame.message {
                match message {
                    data_frame::Message::InstantiateObject(obj) => {
                        obj.target = Some(instantiate_object::Target::RoomAll(RoomAll {
                            room_id: self.data_room.id.clone(),
                        })); // 修改 InstantiateObject 的目标为 RoomAll
                        obj.owner_id = b"sys".to_vec(); // 设置 owner_id 为 "sys"
                        tracing::trace!(
                            "New object instantiated in initial dataframes: {:?} with id {:?} at timestamp: {}",
                            String::from_utf8_lossy(&obj.prefab_name),
                            obj.object_id,
                            timestamp
                        );
                    }
                    data_frame::Message::UpdateObject(obj) => {
                        obj.target = Some(update_object::Target::RoomAll(RoomAll {
                            room_id: self.data_room.id.clone(),
                        })); // 修改 UpdateObject 的目标为 RoomAll
                    }
                    _ => {}
                }
            }
            // save initial_dataframes
            self.insert_initial_dataframes(frame.clone());
        }
        if self.use_audio_processing {
            #[cfg(feature = "audio")]
            // do nothing
            #[cfg(not(feature = "audio"))]
            unreachable!("Audio processing is disabled");
        } else if self.auto_timestamp {
            self.packetinfo_buffer.push(packet_info);
        } else {
            self.segment_builder.add(packet_info);
        }
        self.state = AlsConverterStateMachine::UpdateObjects;
        Ok(())
    }

    fn process_update_objects_state(&mut self, mut packet_info: PacketInfo) -> Result<()> {
        // control message 判断必须是Data
        if let Some(control) = &packet_info.data_pack.control {
            match control {
                data_pack::Control::Data(true) => {
                    self.state = AlsConverterStateMachine::UpdateObjects;
                }
                data_pack::Control::Pong(_) => {
                    // ignore pong until no packet more
                    // if self.state == AlsConverterStateMachine::Pong {
                    //     self.state = AlsConverterStateMachine::End;
                    //     return Ok(());
                    // }
                    self.state = AlsConverterStateMachine::Pong;
                    return Ok(());
                }
                _ => {
                    // 跳过
                    return Ok(());
                }
            }
        }

        let timestamp = packet_info.timestamp;
        let mut use_custom_data_start_time = false;
        if !self.auto_timestamp {
            // 保留初始initial_dataframe, 但是跳过指定时间之前的包
            if let Some(data_start_time) = &self.data_start_time {
                if timestamp < *data_start_time {
                    // skip and keep checking
                    use_custom_data_start_time = true;
                } else {
                    // only check once
                    self.data_start_time = None;
                    self.initial_timestamp = timestamp;
                    {
                        // and update the initial timestamp in the segment builder
                        // because this time only initial packets in the Buffer
                        self.segment_builder.segments[0]
                            .packets
                            .iter_mut()
                            .for_each(|packet| {
                                packet.timestamp = timestamp;
                            });
                        // update initial dataframes for segment builder,
                        // remove last and inset new one
                        if let Some(_last_packet) = self.segment_builder.segments[0].packets.pop() {
                            let mut new_initial_packet =
                                PacketInfo::create_room_frame(timestamp, self.data_room.clone());
                            new_initial_packet
                                .data_pack
                                .frames
                                .extend(self.initial_dataframes.clone());
                            self.segment_builder.segments[0]
                                .packets
                                .push(new_initial_packet);
                        }
                    }
                }
            }

            // 如果不是通过数据规律分段，则手动判断时间戳，添加新的回放段（对timestamp正常的包管用 ）
            if timestamp - self.initial_timestamp > DURATION {
                self.initial_timestamp += DURATION;
                if !use_custom_data_start_time {
                    // 处理新分片的头
                    self.segment_builder
                        .set_current_segment_duration(DURATION.as_seconds_f64())
                        .next()
                        .add(PacketInfo::create_segment_started_packet(
                            self.initial_timestamp,
                        ))
                        .add(PacketInfo::create_room_frame(
                            timestamp,
                            self.data_room.clone(),
                        ))
                        .add(PacketInfo {
                            timestamp,
                            data_pack: DataPack {
                                control: Some(data_pack::Control::Data(true)),
                                frames: self.initial_dataframes.clone(),
                            },
                            raw_data: Vec::new(),
                        })
                        .add(PacketInfo::create_cache_end(timestamp));
                }
            }
        }

        // 过滤我们不需要的包, 也许这里有逻辑上的问题
        packet_info.data_pack.frames.retain(|frame| {
            if let Some(message) = &frame.message {
                match message {
                    data_frame::Message::UpdateObject(_)
                    | data_frame::Message::InstantiateObject(_)
                    | data_frame::Message::DestroyObject(_) => true,
                    _ => false,
                }
            } else {
                false
            }
        });

        for frame in &mut packet_info.data_pack.frames {
            match &mut frame.message {
                Some(data_frame::Message::UpdateObject(obj)) => {
                    obj.target = Some(update_object::Target::RoomAll(RoomAll {
                        room_id: self.data_room.id.clone(),
                    }));
                    // tacing if update object id is exist in initial_dataframes
                    let obj_id = obj.object_id;
                    if !self.initial_dataframes.iter().any(|f| {
                        if let Some(data_frame::Message::InstantiateObject(inst_obj)) = &f.message {
                            inst_obj.object_id == obj_id
                        } else {
                            false
                        }
                    }) {
                        tracing::warn!(
                            "UpdateObject with id {:?} not found in initial_dataframes at timestamp: {}",
                            obj_id,
                            timestamp
                        );
                    }
                    self.update_initial_dataframes(frame.clone());
                }
                Some(data_frame::Message::InstantiateObject(obj)) => {
                    obj.target = Some(instantiate_object::Target::RoomAll(RoomAll {
                        room_id: self.data_room.id.clone(),
                    })); // 修改 InstantiateObject 的目标为 RoomAll
                    obj.owner_id = b"sys".to_vec(); // 设置 owner_id 为 "sys"
                    tracing::trace!(
                        "New object instantiated in update state: {:?} with id {:?} at timestamp: {}",
                        String::from_utf8_lossy(&obj.prefab_name),
                        obj.object_id,
                        timestamp
                    );
                    let new_frame = frame.clone();
                    self.insert_initial_dataframes(new_frame);
                }
                Some(data_frame::Message::DestroyObject(obj)) => {
                    obj.target = Some(destroy_object::Target::RoomAll(RoomAll {
                        room_id: self.data_room.id.clone(),
                    }));
                    tracing::trace!(
                        "Object destroyed with id {:?} at timestamp: {}",
                        obj.object_id,
                        timestamp
                    );
                    // remove it in initial_dataframes
                    self.initial_dataframes.retain(|f| {
                        if let Some(data_frame::Message::InstantiateObject(inst_obj)) = &f.message {
                            inst_obj.object_id != obj.object_id
                        } else if let Some(data_frame::Message::UpdateObject(upd_obj)) = &f.message
                        {
                            upd_obj.object_id != obj.object_id
                        } else {
                            true
                        }
                    });
                }
                Some(_) | None => unreachable!("Frame can't be None here."),
            }
        }

        if packet_info.data_pack.frames.is_empty() || use_custom_data_start_time {
            return Ok(());
        }
        // if all frames are destroy object, state to Split
        if self.split_write_mode && self.initial_dataframes.is_empty() {
            self.state = AlsConverterStateMachine::Split;
            return Ok(());
        }

        if self.use_audio_processing {
            #[cfg(feature = "audio")]
            self.audio_builder.handle_update_audio(&packet_info);
            #[cfg(not(feature = "audio"))]
            unreachable!("Audio feature is not enabled");
        } else if self.auto_timestamp {
            self.packetinfo_buffer.push(packet_info);
        } else {
            self.segment_builder.add(packet_info);
        }
        Ok(())
    }

    fn get_first_dataframe<'b>(&self, packet_info: &'b PacketInfo) -> Option<&'b DataFrame> {
        packet_info.data_pack.frames.first()
    }

    fn update_initial_dataframes(&mut self, mut dataframe: DataFrame) {
        if let Some(existing_frame) = self.initial_dataframes.iter_mut().find(|f| {
            // message都是UpdateObject
            if let (Some(existing_message), Some(new_message)) =
                (f.message.as_ref(), dataframe.message.as_ref())
            {
                if let (
                    data_frame::Message::UpdateObject(existing_obj),
                    data_frame::Message::UpdateObject(new_obj),
                ) = (existing_message, new_message)
                {
                    existing_obj.object_id == new_obj.object_id
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            // change dataframe's target
            if let Some(message) = &mut dataframe.message {
                match message {
                    data_frame::Message::UpdateObject(obj) => {
                        obj.target = Some(update_object::Target::CurrentPlayer(CurrentPlayer {}));
                    }
                    _ => {}
                }
            }
            *existing_frame = dataframe;
        } else {
            self.insert_initial_dataframes(dataframe);
        }
    }
    fn insert_initial_dataframes(&mut self, mut dataframe: DataFrame) {
        if let Some(message) = &mut dataframe.message {
            match message {
                data_frame::Message::InstantiateObject(obj) => {
                    if self.use_audio_processing {
                        #[cfg(feature = "audio")]
                        self.audio_builder.handle_instantiate_audio(message);
                        #[cfg(not(feature = "audio"))]
                        unreachable!("Audio feature is not enabled");
                    } else {
                        obj.target =
                            Some(instantiate_object::Target::CurrentPlayer(CurrentPlayer {})); // 修改 InstantiateObject 的目标为 CurrentPlayer
                    }
                }
                data_frame::Message::UpdateObject(obj) => {
                    obj.target = Some(update_object::Target::CurrentPlayer(CurrentPlayer {})); // 修改 UpdateObject 的目标为 CurrentPlayer
                }
                _ => {}
            }
        }
        self.initial_dataframes.push(dataframe);
        // swap camera order first
        Self::swap_order(&mut self.initial_dataframes);
        // then sort InitialObject is first
        self.initial_dataframes.sort_by(Self::compare_dataframes);
    }

    // 仅仅更新时间戳，不要考虑其他逻辑
    // 逻辑:
    // 1. 统计两个 DateTimeReceiver 之间的 MusicBroadcaster 包数量
    // 2. 根据音频包数量,将总时间 delta 均分给每个音频包
    // 3. 在每两个音频包之间,将其他包的时间也均分
    fn handle_auto_timestamp(
        &mut self,
        start_index: usize,
        end_index: usize,
        last_timestamp: DateTime<Utc>,
        cur_timestamp: DateTime<Utc>,
        music_broadcasters: &HashSet<i32>,
    ) -> Result<()> {
        let total_delta = cur_timestamp - last_timestamp;
        if total_delta <= TimeDelta::zero() {
            return Err(anyhow::anyhow!(
                "Non-positive time delta between confirmed timestamps: {} to {}, skipping adjustment.",
                last_timestamp,
                cur_timestamp
            ));
        }

        if start_index >= end_index {
            return Ok(());
        }

        // 结构体记录每个包的信息
        struct PacketTimeInfo {
            index: usize,
            is_music: bool,
        }

        let mut packet_infos: Vec<PacketTimeInfo> = Vec::new();
        let mut music_packet_count = 0;

        // 第一步: 统计所有包的信息
        for i in start_index..=end_index {
            let packet_info = &self.packetinfo_buffer[i];
            let mut is_music_packet = false;

            // 检查这个包是否包含 MusicBroadcaster 的 UpdateObject
            for frame in &packet_info.data_pack.frames {
                if let Some(data_frame::Message::UpdateObject(obj)) = &frame.message {
                    if music_broadcasters.contains(&obj.object_id) {
                        is_music_packet = true;
                        music_packet_count += 1;
                        break;
                    }
                }
            }

            packet_infos.push(PacketTimeInfo {
                index: i,
                is_music: is_music_packet,
            });
        }

        // 如果没有找到音频包，使用均匀分布所有包
        if music_packet_count == 0 {
            tracing::info!("HashSet of MusicBroadcaster IDs: {:?}", music_broadcasters);
            tracing::warn!(
                "No MusicBroadcaster found between confirmed timestamps: {} to {}, using uniform distribution.",
                last_timestamp,
                cur_timestamp
            );

            let total_packets = packet_infos.len();
            if total_packets == 1 {
                // 只有一个包，直接设置为结束时间
                self.packetinfo_buffer[end_index].timestamp = cur_timestamp;
            } else {
                // 均匀分布时间
                let time_step = total_delta / (total_packets as i32 - 1);
                for (local_idx, info) in packet_infos.iter().enumerate() {
                    let new_timestamp = last_timestamp + time_step * (local_idx as i32);
                    self.packetinfo_buffer[info.index].timestamp = new_timestamp;
                    tracing::trace!("Packet {} (uniform): {}", info.index, new_timestamp);
                }
                // 确保最后一个包精确匹配结束时间
                self.packetinfo_buffer[end_index].timestamp = cur_timestamp;
            }
            return Ok(());
        }

        tracing::debug!(
            "Found {} music packets between index {} and {}, total delta: {:?}",
            music_packet_count,
            start_index,
            end_index,
            total_delta
        );

        // 第二步: 将总时间按音频包数量均分
        let time_per_music_segment = total_delta / music_packet_count;

        // 第三步: 计算每个包的时间戳
        let mut current_time = last_timestamp;
        let mut music_segment_index = 0;
        let mut music_segment_start_packet_idx = 0;

        for (local_idx, packet_info) in packet_infos.iter().enumerate() {
            if packet_info.is_music {
                // 这是一个音频包
                // 先处理上一个音频段到当前音频包之间的其他包
                let packets_before_this_music = local_idx - music_segment_start_packet_idx;
                if packets_before_this_music > 0 {
                    // 有其他包需要插值（包括第一个音频包之前的包）
                    let time_step = time_per_music_segment / (packets_before_this_music as i32 + 1);
                    for (step, info) in packet_infos[music_segment_start_packet_idx..local_idx]
                        .iter()
                        .enumerate()
                    {
                        let new_timestamp = current_time + time_step * ((step + 1) as i32);
                        self.packetinfo_buffer[info.index].timestamp = new_timestamp;
                        tracing::trace!(
                            "Packet {} (before music {}): {}",
                            info.index,
                            music_segment_index,
                            new_timestamp
                        );
                    }
                }

                // 更新当前音频段的时间
                current_time = current_time + time_per_music_segment;
                self.packetinfo_buffer[packet_info.index].timestamp = current_time;
                tracing::debug!(
                    "Music packet {} at index {}: {}",
                    music_segment_index,
                    packet_info.index,
                    current_time
                );

                music_segment_index += 1;
                music_segment_start_packet_idx = local_idx + 1; // 下一个区间从这个音频包的下一个包开始
            }
        }

        // 第四步: 处理最后一个音频包之后到 end_index 之间的包
        if music_segment_start_packet_idx < packet_infos.len() {
            let remaining_packets = packet_infos.len() - music_segment_start_packet_idx;
            if remaining_packets > 0 {
                let remaining_time = cur_timestamp - current_time;
                let time_step = remaining_time / (remaining_packets as i32 + 1);
                for (step, info) in packet_infos[music_segment_start_packet_idx..]
                    .iter()
                    .enumerate()
                {
                    let new_timestamp = current_time + time_step * ((step + 1) as i32);
                    self.packetinfo_buffer[info.index].timestamp = new_timestamp;
                    tracing::trace!(
                        "Packet {} (after last music): {}",
                        info.index,
                        new_timestamp
                    );
                }
            }
        }

        // 确保最后一个包的时间戳正确
        self.packetinfo_buffer[end_index].timestamp = cur_timestamp;
        tracing::debug!("End packet {}: {}", end_index, cur_timestamp);

        Ok(())
    }
    ///
    ///
    fn handle_packetinfo_buffer(&mut self) -> Result<()> {
        if self.packetinfo_buffer.is_empty() {
            return Ok(());
        }
        tracing::warn!(
            "This is experimental auto timestamp feature, please report issues if any bugs found."
        );
        self.initial_dataframes.clear(); // clear initial dataframes first
        let mut last_confirmed_timestamp: Option<DateTime<Utc>> = None;
        let mut last_confirmed_packet_index: usize = 0;
        let mut music_broadcasters: HashSet<i32> = HashSet::new();
        let mut datetime_receiver_id = 0;

        // 收集需要处理的时间段信息
        struct TimestampRange {
            start_index: usize,
            end_index: usize,
            start_time: DateTime<Utc>,
            end_time: DateTime<Utc>,
        }
        let mut ranges_to_process: Vec<TimestampRange> = Vec::new();

        for (index, packet_info) in self.packetinfo_buffer.iter().enumerate() {
            for frame in &packet_info.data_pack.frames {
                match &frame.message {
                    Some(data_frame::Message::InstantiateObject(obj)) => {
                        let name = String::from_utf8_lossy(&obj.prefab_name);
                        if name.contains("TimedAsset/DateTimeReceiver") {
                            datetime_receiver_id = obj.object_id;
                        }
                        if name.contains("VoiceObject/MusicBroadcaster") {
                            music_broadcasters.insert(obj.object_id);
                        }
                    }
                    Some(data_frame::Message::UpdateObject(obj)) => {
                        if obj.object_id == datetime_receiver_id {
                            let date_convert = obj.try_parse_date_time()?;
                            if last_confirmed_timestamp.is_none() {
                                last_confirmed_timestamp = Some(date_convert.date_time);
                                last_confirmed_packet_index = index;
                                tracing::debug!(
                                    "First confirmed timestamp: {} at packet index: {}",
                                    date_convert.date_time,
                                    index
                                );
                                break; // next loop
                            }
                            let confirmed_timestamp = last_confirmed_timestamp.unwrap();

                            // 收集需要处理的范围
                            ranges_to_process.push(TimestampRange {
                                start_index: last_confirmed_packet_index,
                                end_index: index,
                                start_time: confirmed_timestamp,
                                end_time: date_convert.date_time,
                            });

                            // 更新为当前确认的时间戳
                            last_confirmed_timestamp = Some(date_convert.date_time);
                            last_confirmed_packet_index = index;
                        }
                    }
                    Some(data_frame::Message::DestroyObject(_)) => {
                        // do nothing right now
                        // tracing::info!("DestroyObject with id {:?} at packet index: {}", obj.object_id, index);
                        // if obj.object_id == datetime_receiver_id {
                        //     datetime_receiver_id = 0; // reset
                        //     last_confirmed_timestamp = None;
                        //     last_confirmed_packet_index = 0;
                        //     music_broadcasters.clear(); // clear all broadcasters
                        // }
                        // special handle datetime
                    }
                    _ => {}
                }
            }
        }

        // 现在统一处理所有范围
        let last_range_info = ranges_to_process.last().map(|r| (r.end_index, r.end_time));
        for range in ranges_to_process {
            self.handle_auto_timestamp(
                range.start_index,
                range.end_index,
                range.start_time,
                range.end_time,
                &music_broadcasters,
            )?;
        }

        // 处理剩余的包(如果最后一个 range 的 end_index 后面还有包)
        // 这些包没有 DateTimeReceiver,按固定 20ms 间隔处理
        if let Some((last_end_index, last_end_time)) = last_range_info {
            let remaining_start = last_end_index + 1;
            let remaining_end = self.packetinfo_buffer.len() - 1;

            if remaining_start <= remaining_end {
                const FIXED_INTERVAL_MS: i64 = 20; // 每个包 20 毫秒

                tracing::debug!(
                    "Processing remaining {} packets with fixed 20ms interval from index {} to {}",
                    remaining_end - remaining_start + 1,
                    remaining_start,
                    remaining_end
                );

                for i in remaining_start..=remaining_end {
                    let offset = (i - remaining_start + 1) as i64;
                    let new_timestamp =
                        last_end_time + TimeDelta::milliseconds(FIXED_INTERVAL_MS * offset);
                    self.packetinfo_buffer[i].timestamp = new_timestamp;
                    tracing::trace!("Fixed interval packet {}: {}", i, new_timestamp);
                }
            }
        }
        // after timestamp confirmed, we can use segment_builder then.
        for packet_info in std::mem::take(&mut self.packetinfo_buffer) {
            let timestamp = packet_info.timestamp;
            tracing::debug!("Processing packet with confirmed timestamp: {}", timestamp);
            // first segment start
            if self.segment_builder.segments.is_empty() {
                self.initial_timestamp = timestamp;
                self.segment_builder
                    .start()
                    .add(PacketInfo::create_segment_started_packet(timestamp))
                    .add(PacketInfo::create_room_frame(
                        timestamp,
                        self.data_room.clone(),
                    ))
                    .add(PacketInfo::create_cache_end(timestamp));
            }
            // timestamp segment
            if timestamp - self.initial_timestamp > DURATION {
                self.initial_timestamp += DURATION;
                self.segment_builder
                    .set_current_segment_duration(DURATION.as_seconds_f64())
                    .next()
                    .add(PacketInfo::create_segment_started_packet(
                        self.initial_timestamp,
                    ))
                    .add(PacketInfo::create_room_frame(
                        timestamp,
                        self.data_room.clone(),
                    ))
                    .add(PacketInfo {
                        timestamp,
                        data_pack: DataPack {
                            control: Some(data_pack::Control::Data(true)),
                            frames: self.initial_dataframes.clone(),
                        },
                        raw_data: Vec::new(),
                    })
                    .add(PacketInfo::create_cache_end(timestamp));
            }
            // update initial frames
            for frame in &packet_info.data_pack.frames {
                match &frame.message {
                    Some(data_frame::Message::InstantiateObject(_)) => {
                        self.insert_initial_dataframes(frame.clone()); // clone it will not change the original frame
                    }
                    Some(data_frame::Message::UpdateObject(_)) => {
                        self.update_initial_dataframes(frame.clone()); // clone it will not change the original frame
                    }
                    Some(data_frame::Message::DestroyObject(obj)) => {
                        self.initial_dataframes.retain(|f| {
                            if let Some(data_frame::Message::InstantiateObject(inst_obj)) =
                                &f.message
                            {
                                inst_obj.object_id != obj.object_id
                            } else if let Some(data_frame::Message::UpdateObject(upd_obj)) =
                                &f.message
                            {
                                upd_obj.object_id != obj.object_id
                            } else {
                                true
                            }
                        });
                    }
                    _ => {
                        // do nothing
                    }
                }
            }
            self.segment_builder.add(packet_info);
        }
        Ok(())
    }
}
