use crate::als::proto::{
    proto::als::{
        data_frame, data_pack, instantiate_object, update_object, CurrentPlayer, DataFrame, DataPack, Room, RoomAll
    }, MixedPacketInfo, MixedPacketReader, PacketInfo
};
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, FixedOffset, TimeDelta, Utc};
use std::{cmp::Ordering, fs::{DirEntry, File}};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::fmt;


#[derive(PartialEq, Eq, Debug)]
enum AlsConverterStateMachine {
    Initial,
    FirstDataframes,
    UpdateObjects,
    Pong,
    End,
}

pub struct AlsConverter {
    #[allow(unused)]
    segment_duration: u64, // microseconds, default 10 seconds
}

impl Default for AlsConverter {
    fn default() -> Self {
        Self {
            segment_duration: 10_000_000, // 10 seconds in microseconds
        }
    }
}

impl AlsConverter {
    pub fn new(segment_duration_seconds: u64) -> Self {
        Self {
            segment_duration: segment_duration_seconds * 1_000_000,
        }
    }

    pub fn convert_mixed_to_standard<P: AsRef<Path>>(
        &self,
        input_dir: P,
        output_dir: P,
        timeshift: i64,
        start_time: Option<String>,
    ) -> Result<()> {
        let input_dir = input_dir.as_ref();
        let output_dir = output_dir.as_ref();

        std::fs::create_dir_all(output_dir)?;

        let mut segment_builder = SegmentBuilder::new();
        let mut context = ConversionContext::new(&mut segment_builder, timeshift, start_time);
        let mut packet_buffer = PacketsBuffer::new(input_dir);

        while let Some((data_packet, time_packet)) = self.try_read_packet_pair(&mut packet_buffer)? {
            let end = context.process_packet_pair(data_packet, time_packet)?;
            if end {
                break;
            }
        }
        // calculate the last segment durations for first and last
        let last_segment = context.segment_builder.segments.last_mut().unwrap();
        last_segment.duration = (|| {
            let last_timestamp = last_segment.packets.last().unwrap().timestamp;
            let first_timestamp = last_segment.packets.first().unwrap().timestamp;
            (last_timestamp - first_timestamp).num_microseconds().unwrap_or(0) as f64 / 1_000_000.0
        })();
        context.segment_builder.write_to_file(output_dir, &context)?;
        Ok(())
    }

    /// 尝试读取一对数据包，如果没有更多文件则返回 None
    fn try_read_packet_pair(
        &self,
        packet_buffer: &mut PacketsBuffer,
    ) -> Result<Option<(MixedPacketInfo, MixedPacketInfo)>> {
        match packet_buffer.read() {
            Ok(packets) => Ok(Some(packets)),
            Err(err) => {
                if let Some(PacketsBufferError::NoMoreFileEntries) = err.downcast_ref::<PacketsBufferError>() {
                    Ok(None) // 正常结束
                } else {
                    Err(err) // 其他错误向上传播
                }
            }
        }
    }
}

#[derive(Debug)]
enum PacketsBufferError {
    NoMoreFileEntries,
    NoPacketInBuffer,
    NotEnoughPacketInBuffer,
}

impl fmt::Display for PacketsBufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketsBufferError::NoMoreFileEntries => write!(f, "No more file entries available"),
            PacketsBufferError::NoPacketInBuffer => write!(f, "No packet available in buffer"),
            PacketsBufferError::NotEnoughPacketInBuffer => write!(f, "Not enough packets available in buffer"),
        }
    }
}

impl std::error::Error for PacketsBufferError {}

struct PacketsBuffer {
    // queue
    packets_buffer: std::collections::VecDeque<MixedPacketInfo>,
    file_entries: std::collections::VecDeque<DirEntry>,
}

impl PacketsBuffer {
    pub fn new(input_dir: &Path) -> Self {
        PacketsBuffer {
            packets_buffer: std::collections::VecDeque::new(),
            file_entries: Self::get_file_entries(input_dir).unwrap_or_default(),
        }
    }

    // 一次读两个
    fn read(&mut self) -> Result<(MixedPacketInfo, MixedPacketInfo)> {
        if self.packets_buffer.is_empty() {
            if self.file_entries.is_empty() {
                return Err(anyhow::Error::from(PacketsBufferError::NoMoreFileEntries));
            }
            // try reading from file entries
            let file_entry = self.file_entries.pop_front().ok_or_else(|| {
                anyhow!("No more file entries available")
            })?;
            let packets = self.read_mixed_packets(Some(&file_entry))
                    .with_context(|| "Failed to read mixed packets from file entry")?;
            self.packets_buffer.extend(packets);

        }
        let datapack_packet = self.packets_buffer.pop_front().ok_or_else(|| {
            anyhow::Error::from(PacketsBufferError::NoPacketInBuffer)
        })?;
        let timestamp_packet = self.packets_buffer.pop_front().ok_or_else(|| {
            anyhow::Error::from(PacketsBufferError::NotEnoughPacketInBuffer)
        })?;
        Ok((datapack_packet, timestamp_packet))
    }

    fn read_mixed_packets(&self, file_entry: Option<&DirEntry>) -> Result<std::collections::VecDeque<MixedPacketInfo>> {
        let Some(file_entry) = file_entry else {
            return Err(anyhow!("No file entry provided"));
        };
        let file = File::open(file_entry.path())
            .with_context(|| format!("Failed to open initial fragment: {:?}", file_entry))?;
        let mut reader = MixedPacketReader::new(file);
        let packets = reader.read_mixed_packets()
            .with_context(|| "Failed to read mixed packets")?;
        Ok(std::collections::VecDeque::from(packets))

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
}

impl SegmentBuilder {
    pub fn new() -> Self {
        SegmentBuilder {
            current_sequence: 0,
            segments: Vec::new(),
        }
    }

    pub fn add(&mut self, packet_info: PacketInfo) -> &mut Self {
        if let Some(segment) = self.segments.last_mut() {
            segment.add(packet_info);
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
        return self.next();
    }

    pub fn set_current_segment_duration(&mut self, duration: f64) -> &mut Self {
        if let Some(segment) = self.segments.last_mut() {
            segment.duration = duration;
        }
        self
    }

    pub fn write_to_file<P: AsRef<Path>>(
        &self,
        output_dir: P,
        context: &ConversionContext,
    ) -> Result<()> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;
        for segment in &self.segments {
            let segment_file_path = output_dir.join(format!("segment_{:05}.ts", segment.number));
            let file = File::create(&segment_file_path)
                .with_context(|| format!("Failed to create segment file: {:?}", segment_file_path))?;
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
            writeln!(m3u8_file, "#EXTINF:{:.3},\nsegment_{:05}.ts", segment.duration, segment.number)?;
        }
        writeln!(m3u8_file, "#EXT-X-ENDLIST")?;

        // metadata file
        let metadata_file_path = output_dir.join("index.md");
        let mut metadata_file = File::create(&metadata_file_path)
            .with_context(|| format!("Failed to create metadata file: {:?}", metadata_file_path))?;
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let live_started_at = chrono::DateTime::<Utc>::from_timestamp_micros(context.data_room.started_at)
            .unwrap_or_else(|| Utc::now()).with_timezone(&jst_offset).to_rfc3339();
        let joined_room_at = self.segments.first().unwrap()
                                        .packets.first().unwrap()
                                        .timestamp.with_timezone(&jst_offset).to_rfc3339();
        let metadata = serde_json::json!({
            "path": "/",
            "room_id": std::str::from_utf8(&context.data_room.id)
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
struct ConversionContext<'a> {
    state: AlsConverterStateMachine,
    data_room: Room,
    segment_builder: &'a mut SegmentBuilder,
    initial_timestamp: DateTime<Utc>,
    initial_dataframes: Vec<DataFrame>,
    timeshift: i64,
    start_time: Option<String>,
}

impl<'a> ConversionContext<'a> {
    fn new(segment_builder: &'a mut SegmentBuilder, timeshift: i64, start_time: Option<String>) -> Self {
        Self {
            state: AlsConverterStateMachine::Initial,
            data_room: Room {
                id: vec![0],
                started_at: 0,
                ended_at: 0,
            },
            initial_timestamp: DateTime::<Utc>::from_timestamp_micros(0).unwrap(),
            segment_builder,
            initial_dataframes: Vec::new(),
            timeshift,
            start_time,
        }
    }

    fn process_packet_pair(
        &mut self,
        data_packet: MixedPacketInfo,
        time_packet: MixedPacketInfo
    ) -> Result<bool> {
        match self.state {
            AlsConverterStateMachine::Initial => {
                self.process_initial_state(data_packet)?;
            }
            AlsConverterStateMachine::FirstDataframes => {
                self.process_first_dataframes_state(data_packet, time_packet)?;
            }
            AlsConverterStateMachine::Pong |
            AlsConverterStateMachine::UpdateObjects => {
                self.process_update_objects_state(data_packet, time_packet)?;
            }
            AlsConverterStateMachine::End => {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn process_initial_state(
        &mut self,
        data_packet: MixedPacketInfo
    ) -> Result<()> {
        let frame = self.get_first_dataframe(&data_packet)
            .ok_or_else(|| anyhow!("No DataFrame found in initial fragment"))?;

        let message = frame.message.as_ref()
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
    fn process_first_dataframes_state(&mut self, data_packet: MixedPacketInfo, time_packet: MixedPacketInfo) -> Result<()> {
        // control message 判断必须是Data
        if !data_packet.data_pack.as_ref().map_or(false, |dp| dp.control.as_ref().map_or(false, |c| matches!(c, data_pack::Control::Data(true)))) {
            return Ok(());
        }
        // 第一个 dataframe 必须是 InstantiateObject
        if !data_packet.data_pack.as_ref().map_or(false, |dp| dp.frames.first().map_or(false, |f| matches!(f.message, Some(data_frame::Message::InstantiateObject(_))))) {
            return Ok(());
        }
        if let Some(start_time) = &self.start_time {
            if let Some(timestamp) = time_packet.timestamp {
                let start_datetime = DateTime::parse_from_rfc3339(start_time)
                    .with_context(|| format!("Failed to parse start_time: {}", start_time))?;
                if timestamp < start_datetime {
                    // skip this packet
                    return Ok(());
                } else {
                    // only check once
                    self.start_time = None;
                }
            }
        }
        let mut timestamp = time_packet.timestamp
            .ok_or_else(|| anyhow!("No timestamp in time packet"))?;
        timestamp = timestamp + TimeDelta::microseconds(self.timeshift * 1_000);
        self.initial_timestamp = timestamp;

        self.segment_builder
            .start()
            .add(PacketInfo::create_segment_started_packet(timestamp))
            .add(PacketInfo::create_room_frame(timestamp, self.data_room.clone()))
            .add(PacketInfo::create_cache_end(timestamp));
        
        let mut data_info = PacketInfo::try_from(data_packet)
            .context("Failed to convert MixedPacketInfo to PacketInfo")?;
        data_info.timestamp = timestamp;
        
        let mut frames = std::mem::take(&mut data_info.data_pack.frames);
        for frame in &mut frames {
            if let Some(message) = &mut frame.message {
                match message {
                    data_frame::Message::InstantiateObject(obj) => {
                        obj.target = Some(instantiate_object::Target::RoomAll(RoomAll {
                            room_id: self.data_room.id.clone(),
                        })); // 修改 InstantiateObject 的目标为 RoomAll
                        obj.owner_id = b"sys".to_vec(); // 设置 owner_id 为 "sys"
                    }
                    data_frame::Message::UpdateObject(obj) => {
                        obj.target = Some(update_object::Target::RoomAll(RoomAll {
                            room_id: self.data_room.id.clone(),
                        }));  // 修改 UpdateObject 的目标为 RoomAll
                    }
                    _ => {}
                }
            }
        }
        let mut data_frames_packet = PacketInfo::create_room_frame(timestamp, self.data_room.clone());
        data_frames_packet.data_pack.frames.extend(frames.clone());
        self.segment_builder.add(data_frames_packet);
        // save initial_dataframes
        for frame in &mut frames {
            if let Some(message) = &mut frame.message {
                match message {
                    data_frame::Message::InstantiateObject(obj) => {
                        obj.target = Some(instantiate_object::Target::CurrentPlayer(CurrentPlayer {  })); // 修改 InstantiateObject 的目标为 CurrentPlayer
                    }
                    data_frame::Message::UpdateObject(obj) => {
                        obj.target = Some(update_object::Target::CurrentPlayer(CurrentPlayer {  }));  // 修改 UpdateObject 的目标为 CurrentPlayer
                    }
                    _ => {}
                }
            }
        }
        self.initial_dataframes = frames;
        self.state = AlsConverterStateMachine::UpdateObjects;
        Ok(())
    }

    fn process_update_objects_state(
        &mut self,
        data_packet: MixedPacketInfo,
        time_packet: MixedPacketInfo,
    ) -> Result<()> {
        // control message 判断必须是Data
        if let Some(control) = &data_packet.data_pack.as_ref().unwrap().control {
            match control {
                data_pack::Control::Data(true) => {
                    self.state = AlsConverterStateMachine::UpdateObjects;
                }
                data_pack::Control::Pong(_) => {
                    // last time is pong
                    if self.state == AlsConverterStateMachine::Pong {
                        self.state = AlsConverterStateMachine::End;
                        return Ok(());
                    }
                    self.state = AlsConverterStateMachine::Pong;
                    return Ok(());
                }
                _ => {
                    // 跳过
                    return Ok(());
                }
            }
        }
        let mut timestamp = time_packet.timestamp
            .ok_or_else(|| anyhow!("No timestamp in time packet"))?;
        timestamp = timestamp + TimeDelta::microseconds(self.timeshift * 1_000);
        // 判断时间戳
        if timestamp - self.initial_timestamp > DURATION {
            self.initial_timestamp += DURATION;
            // 处理新分片的头
            self.segment_builder
                .set_current_segment_duration(DURATION.as_seconds_f64())
                .next()
                .add(PacketInfo::create_segment_started_packet(self.initial_timestamp))
                .add(PacketInfo::create_room_frame(timestamp, self.data_room.clone()))
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

        let data_info = PacketInfo::try_from(data_packet)
            .context("Failed to convert MixedPacketInfo to PacketInfo")?;
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
                            self.update_initial_dataframes(frame.clone());
                            Some(frame)
                        }
                        data_frame::Message::InstantiateObject(obj) => {
                            obj.target = Some(instantiate_object::Target::RoomAll(RoomAll {
                                room_id: self.data_room.id.clone(),
                            })); // 修改 InstantiateObject 的目标为 RoomAll
                            obj.owner_id = b"sys".to_vec(); // 设置 owner_id 为 "sys"
                            let new_frame = frame.clone();
                            self.insert_initial_dataframes(new_frame);
                            Some(frame)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        
        if frames.is_empty() {
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
        self.segment_builder.add(update_packet);
        Ok(())
    }

    fn get_first_dataframe<'b>(&self, mixed_packet: &'b MixedPacketInfo) -> Option<&'b DataFrame> {
        if let Some(data_pack) = &mixed_packet.data_pack {
            data_pack.frames.first()
        } else {
            None
        }
    }

    fn update_initial_dataframes(&mut self, mut dataframe: DataFrame) {
        if let Some(existing_frame) = self.initial_dataframes.iter_mut().find(|f| {
            // message都是UpdateObject
            if let (Some(existing_message), Some(new_message)) = (f.message.as_ref(), dataframe.message.as_ref()) {
                if let (data_frame::Message::UpdateObject(existing_obj), data_frame::Message::UpdateObject(new_obj)) =
                    (existing_message, new_message) {
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
                        obj.target = Some(update_object::Target::CurrentPlayer(CurrentPlayer {  }));
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
                    obj.target = Some(instantiate_object::Target::CurrentPlayer(CurrentPlayer {  })); // 修改 InstantiateObject 的目标为 CurrentPlayer
                }
                data_frame::Message::UpdateObject(obj) => {
                    obj.target = Some(update_object::Target::CurrentPlayer(CurrentPlayer {  }));  // 修改 UpdateObject 的目标为 CurrentPlayer
                }
                _ => {}
            }
        }
        self.initial_dataframes.push(dataframe);
        // sort InitialObject is first
        self.initial_dataframes.sort_by(|a, b| {
            match (a.message.as_ref(), b.message.as_ref()) {
                (Some(data_frame::Message::InstantiateObject(_)), Some(data_frame::Message::InstantiateObject(_))) => Ordering::Equal,
                (Some(data_frame::Message::InstantiateObject(_)), _) => Ordering::Less,
                (_, Some(data_frame::Message::InstantiateObject(_))) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        });
    }
}