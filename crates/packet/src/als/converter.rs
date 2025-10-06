use super::proto::{
        PacketInfo, define::{
            CurrentPlayer, DataFrame, DataPack, Room, RoomAll, data_frame, data_pack,
            destroy_object, instantiate_object, update_object,
        }, reader::PacketReaderTrait
    };
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{
    cmp::Ordering,
    fs::{DirEntry, File},
    path::PathBuf,
};
use crate::als::proto::reader::{PacketsBufferReader, MixedPacketReader, StandardPacketReader};

#[cfg(feature = "audio")]
use super::audio::{AudioBuilder};

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

    fn get_file_entries(input_dir: &Path) -> Result<std::collections::VecDeque<DirEntry>> {
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
                    .map(|ext| ext == "bin")
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
        let file_entries = Self::get_file_entries(input_dir)?;
        let mut packet_buffer = PacketsBufferReader::new(file_entries, |file| MixedPacketReader::boxed(file));

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
        let mut packet_buffer = PacketsBufferReader::new(Self::get_file_entries(input_dir)?, |file| {
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
}

impl SegmentBuilder {
    pub fn new(metadata_path: Option<String>, output_dir: Option<String>) -> Self {
        SegmentBuilder {
            current_sequence: 0,
            segments: Vec::new(),
            metadata_path,
            output_dir,
            part_count: 0,
        }
    }

    pub fn add(&mut self, packet_info: PacketInfo) -> &mut Self {
        if let Some(segment) = self.segments.last_mut() {
            // check if packet length will exceed 16k bytes 16 * 1024 bytes (maybe the official limit is 16k bytes)
            // but we use 12k bytes as threshold in case of some overhead
            if packet_info.to_vec().len() >= 12 * 1024 {
                let mut check_buf = Vec::new();
                let mut packets_buf: Vec<DataFrame> = Vec::new();
                for p in packet_info.data_pack.frames {
                    let frame_bytes = PacketInfo::frame_to_vec(&p);
                    if check_buf.len() + frame_bytes.len() < 12 * 1024 {
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
    timeshift: i64,
    split_write_mode: bool,
    start_time: Option<DateTime<Utc>>,
    data_start_time: Option<DateTime<Utc>>,
    data_end_time: Option<DateTime<Utc>>,
    use_audio_processing: bool,
    segment_builder: SegmentBuilder,
    #[cfg(feature = "audio")]
    audio_builder: AudioBuilder,

    /// 根据回放包的 audio 与datetime receiver来自动计算时间戳
    /// 不会影响timeshift ?
    auto_timestamp: bool,
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
            st = Some(DateTime::parse_from_rfc3339(&start_time).unwrap().with_timezone(&Utc))
        }
        if let Some(data_start_time) = data_start_time {
            dst = Some(DateTime::parse_from_rfc3339(&data_start_time).unwrap().with_timezone(&Utc))
        }
        if let Some(data_end_time) = data_end_time {
            det = Some(DateTime::parse_from_rfc3339(&data_end_time).unwrap().with_timezone(&Utc))
        }
        Self {
            state: AlsConverterStateMachine::Initial,
            data_room: Room {
                id: vec![0],
                started_at: 0,
                ended_at: 0,
            },
            initial_timestamp: DateTime::<Utc>::from_timestamp_micros(0).unwrap(),
            segment_builder: SegmentBuilder::new(metadata_path, output_dir.clone()),
            initial_dataframes: Vec::new(),
            timeshift,
            split_write_mode,
            start_time: st,
            data_start_time: dst,
            data_end_time: det,
            use_audio_processing,
            auto_timestamp,
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

    fn process_packet(
        &mut self,
        packet_info: PacketInfo,
    ) -> Result<bool> {
        // check if data end time reached
        let timeshift = TimeDelta::microseconds(self.timeshift * 1_000);
        let timestamp = packet_info.timestamp + timeshift;
        if let Some(data_end_time) = &self.data_end_time {
            if let Some(end_datetime) = data_end_time
                .checked_add_signed(timeshift)
            {
                if timestamp > end_datetime {
                    tracing::info!(
                        "Data end time reached: {}, current timestamp: {}, no longer to process remain packets.",
                        end_datetime,
                        timestamp
                    );
                    // end processing
                    self.state = AlsConverterStateMachine::End;
                    return Ok(true);
                }
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
    fn process_first_dataframes_state(
        &mut self,
        mut packet_info: PacketInfo,
    ) -> Result<()> {
        // control message 判断必须是Data
        if !packet_info.data_pack.control
                .as_ref()
                .map_or(false, |c| matches!(c, data_pack::Control::Data(true))) {
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
        let mut timestamp = packet_info.timestamp;
        timestamp = timestamp + TimeDelta::microseconds(self.timeshift * 1_000);
        self.initial_timestamp = timestamp;

        self.segment_builder
            .start()
            .add(PacketInfo::create_segment_started_packet(timestamp))
            .add(PacketInfo::create_room_frame(
                timestamp,
                self.data_room.clone(),
            ))
            .add(PacketInfo::create_cache_end(timestamp));

        packet_info.timestamp = timestamp;

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
        } else {
            self.segment_builder.add(packet_info);
        }
        self.state = AlsConverterStateMachine::UpdateObjects;
        Ok(())
    }

    fn process_update_objects_state(
        &mut self,
        packet_info: PacketInfo,
    ) -> Result<()> {
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
        let mut timestamp = packet_info.timestamp;
        timestamp = timestamp + TimeDelta::microseconds(self.timeshift * 1_000);

        let mut use_custom_data_start_time = false;
        // check if skip this update object packet
        if let Some(data_start_time) = &self.data_start_time {
            if let Some(start_datetime) = data_start_time
                .checked_add_signed(TimeDelta::microseconds(self.timeshift * 1_000))
            {
                if timestamp < start_datetime {
                    // skip and keep checking
                    use_custom_data_start_time = true;
                } else {
                    // only check once
                    self.data_start_time = None;
                    // and update the initial timestamp in the segment builder
                    self.initial_timestamp = timestamp;
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
        // 判断时间戳
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

        let data_info = packet_info.clone(); // TODO: maybe we do not need clone here
        // 获取frames，填充target roomall
        let frames: Vec<DataFrame> = data_info.data_pack.frames
            .into_iter()
            .filter_map(|mut frame| {
                if let Some(message) = &mut frame.message {
                    match message {
                        data_frame::Message::UpdateObject(obj) => {
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
                                tracing::warn!("UpdateObject with id {:?} not found in initial_dataframes at timestamp: {}", obj_id, timestamp);
                            }
                            self.update_initial_dataframes(frame.clone());
                            Some(frame)
                        }
                        data_frame::Message::InstantiateObject(obj) => {
                            obj.target = Some(instantiate_object::Target::RoomAll(RoomAll {
                                room_id: self.data_room.id.clone(),
                            })); // 修改 InstantiateObject 的目标为 RoomAll
                            obj.owner_id = b"sys".to_vec(); // 设置 owner_id 为 "sys"
                            tracing::trace!("New object instantiated in update state: {:?} with id {:?} at timestamp: {}", String::from_utf8_lossy(&obj.prefab_name), obj.object_id, timestamp);
                            let new_frame = frame.clone();
                            self.insert_initial_dataframes(new_frame);
                            Some(frame)
                        }
                        data_frame::Message::DestroyObject(obj) => {
                            obj.target = Some(destroy_object::Target::RoomAll(RoomAll {
                                room_id: self.data_room.id.clone(),
                            }));
                            tracing::trace!("Object destroyed with id {:?} at timestamp: {}", obj.object_id, timestamp);
                            // // also remove from initial_dataframes
                            // self.initial_dataframes.retain(|f| {
                            //     if let Some(data_frame::Message::InstantiateObject(inst_obj)) = &f.message {
                            //         inst_obj.object_id != obj.object_id
                            //     } else if let Some(data_frame::Message::UpdateObject(upd_obj)) = &f.message {
                            //         upd_obj.object_id != obj.object_id
                            //     } else {
                            //         true
                            //     }
                            // });
                            Some(frame)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        if !frames.is_empty()
            && frames.iter().any(|f| {
                if let Some(data_frame::Message::DestroyObject(_)) = &f.message {
                    true
                } else {
                    false
                }
            })
        {
            tracing::debug!(
                "Processing DestroyObject frames at timestamp: {} with initial timestamp segment: {}",
                timestamp,
                self.initial_timestamp
            );
        }
        if frames.is_empty() || use_custom_data_start_time {
            return Ok(());
        }
        // handle destroy object, remove from initial_dataframes
        for frame in &frames {
            if let Some(data_frame::Message::DestroyObject(obj)) = &frame.message {
                self.initial_dataframes.retain(|f| {
                    if let Some(data_frame::Message::InstantiateObject(inst_obj)) = &f.message {
                        inst_obj.object_id != obj.object_id
                    } else if let Some(data_frame::Message::UpdateObject(upd_obj)) = &f.message {
                        upd_obj.object_id != obj.object_id
                    } else {
                        true
                    }
                });
            }
        }
        // if all frames are destroy object, state to Split
        if self.split_write_mode && self.initial_dataframes.is_empty() {
            self.state = AlsConverterStateMachine::Split;
            return Ok(());
        }

        let update_packet = PacketInfo {
            timestamp,
            data_pack: DataPack {
                control: Some(data_pack::Control::Data(true)),
                frames,
            },
            raw_data: Vec::new(),
        };
        if self.use_audio_processing {
            #[cfg(feature = "audio")]
            self.audio_builder.handle_update_audio(&update_packet);
            #[cfg(not(feature = "audio"))]
            unreachable!("Audio feature is not enabled");
        } else {
            self.segment_builder.add(update_packet);
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
}
