use anyhow::{Result, Context, anyhow};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use super::proto::{
    PacketInfo,
    define::{data_frame, data_pack::Control},
    reader::{PacketsBufferReader, PacketReaderTrait},
    extension::{TimelineCommandPacket, UpdateObjectExt, CommandPacket},
};

pub struct Editor {
    input_dir: PathBuf,
    output_dir: PathBuf,
    
    // Configuration
    target_timeline_ids: Vec<i64>,
    timeshift_ms: Option<i64>,

    // State tracking
    // Set of Object IDs that are confirmed "TimelineReceiver"
    timeline_receiver_objects: HashSet<i32>,

    // Map<ObjectId, ModifiedPayload>
    // Stores the payload we assigned to this object ID previously.
    modified_states: HashMap<i32, Vec<u8>>,
    
    // Index for round-robin timeline ID replacement
    current_timeline_index: usize,
}

impl Editor {
    pub fn new(
        input_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
        target_timeline_ids: Vec<i64>,
        timeshift_ms: Option<i64>,
    ) -> Result<Self> {
        let input_path = input_dir.as_ref().to_path_buf();
        let output_path = output_dir.as_ref().to_path_buf();

        if !input_path.is_dir() {
            return Err(anyhow!("Input path must be a directory: {:?}", input_path));
        }

        // Create output directory if it doesn't exist
        if !output_path.exists() {
            fs::create_dir_all(&output_path)
                .with_context(|| format!("Failed to create output directory: {:?}", output_path))?;
        }

        Ok(Self {
            input_dir: input_path,
            output_dir: output_path,
            target_timeline_ids,
            timeshift_ms,
            timeline_receiver_objects: HashSet::new(),
            modified_states: HashMap::new(),
            current_timeline_index: 0,
        })
    }

    pub fn process(&mut self) -> Result<()> {
        tracing::info!("Starting edit process...");
        tracing::info!("Input directory: {:?}", self.input_dir);
        tracing::info!("Output directory: {:?}", self.output_dir);
        
        if !self.target_timeline_ids.is_empty() {
             tracing::info!("Timeline ID replacement enabled. IDs: {:?}", self.target_timeline_ids);
        }

        let mut entries: Vec<_> = fs::read_dir(&self.input_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "ts"))
            .collect();
        
        entries.sort_by_key(|e| e.file_name());

        if entries.is_empty() {
            tracing::warn!("No .ts files found in input directory.");
            return Ok(());
        }

        tracing::info!("Found {} .ts files to process", entries.len());

        for (index, entry) in entries.iter().enumerate() {
            let is_first_file = index == 0;
            let file_name = entry.file_name();
            let output_file_path = self.output_dir.join(&file_name);

            tracing::info!("Processing file: {:?} (First: {})", file_name, is_first_file);

            // Use PacketsBufferReader for robust reading
            // Re-open directory entry by name to satisfy PacketsBufferReader interface
            let entry_for_reader = fs::read_dir(&self.input_dir)?
                .filter_map(|e| e.ok())
                .find(|e| e.file_name() == file_name)
                .ok_or_else(|| anyhow!("Failed to re-open file entry"))?;
            
            let mut file_queue = VecDeque::new();
            file_queue.push_back(entry_for_reader);
            
            let mut reader = PacketsBufferReader::new_standard(file_queue);
            
            let output_file = File::create(&output_file_path)
                .with_context(|| format!("Failed to create output file: {:?}", output_file_path))?;
            let mut writer = BufWriter::new(output_file);

            let mut packet_count = 0;
            let mut has_seen_cache_ended = false;

            while let Some(mut packet) = reader.read_packet()? {
                // Check for CacheEnded control message
                if let Some(Control::CacheEnded(_)) = packet.data_pack.control {
                    has_seen_cache_ended = true;
                }

                self.track_and_modify_packet(&mut packet, is_first_file, has_seen_cache_ended)?;
                writer.write_all(&packet.to_vec())?;
                packet_count += 1;
            }
            
            writer.flush()?;
            tracing::info!("Processed {} packets, wrote to {:?}", packet_count, output_file_path);
        }
        
        tracing::info!("Edit process completed.");
        Ok(())
    }

    fn track_and_modify_packet(
        &mut self, 
        packet: &mut PacketInfo, 
        is_first_file: bool, 
        has_seen_cache_ended: bool
    ) -> Result<()> {
        let mut modified = false;

        for frame in &mut packet.data_pack.frames {
            if let Some(ref mut message) = frame.message {
                match message {
                    data_frame::Message::InstantiateObject(msg) => {
                        // Identify TimelineReceiver objects
                        let name = String::from_utf8_lossy(&msg.prefab_name);
                        if name.ends_with("TimelineReceiver") {
                            self.timeline_receiver_objects.insert(msg.object_id);
                            tracing::debug!("Found TimelineReceiver: ID {}, Name: {}", msg.object_id, name);
                        }
                    }
                    data_frame::Message::UpdateObject(msg) => {
                        // Only process known TimelineReceiver objects
                        if !self.timeline_receiver_objects.contains(&msg.object_id) {
                            continue;
                        }

                        // Try to parse as TimelineCommandPacket
                        if let Ok(mut timeline_cmd) = msg.try_parse_as::<TimelineCommandPacket>() {
                            
                            // LOGIC DECISION:
                            // 1. Is this a "Cached State" packet?
                            let is_cached_state = !is_first_file && !has_seen_cache_ended;

                            if is_cached_state {
                                // Try to restore state
                                if let Some(cached_payload) = self.modified_states.get(&msg.object_id) {
                                    msg.payload = cached_payload.clone();
                                    modified = true;
                                    continue; 
                                }
                            }

                            // 2. New Modification Logic
                            let mut frame_modified = false;

                            // ID Replacement
                            if !self.target_timeline_ids.is_empty() {
                                let _old_id = timeline_cmd.timeline_id;
                                let new_id = self.target_timeline_ids[self.current_timeline_index];
                                
                                timeline_cmd.timeline_id = new_id;
                                
                                // Advance index cyclically
                                self.current_timeline_index = (self.current_timeline_index + 1) % self.target_timeline_ids.len();
                                frame_modified = true;
                            }

                            // Timeshift (always apply if configured)
                            if let Some(shift_ms) = self.timeshift_ms {
                                let shift_sec = shift_ms as f64 / 1000.0;
                                timeline_cmd.start_time_sec += shift_sec;
                                frame_modified = true;
                            }

                            if frame_modified {
                                // Serialize back
                                let mut new_payload = Vec::with_capacity(16);
                                new_payload.extend_from_slice(&timeline_cmd.timeline_id.to_le_bytes());
                                new_payload.extend_from_slice(&timeline_cmd.start_time_sec.to_le_bytes());
                                
                                // Save to state for future restoration
                                self.modified_states.insert(msg.object_id, new_payload.clone());

                                msg.payload = new_payload;
                                modified = true;

                                tracing::debug!(
                                    "Modified Timeline Object {}: ID -> {}, Start -> {:.2} ({})", 
                                    msg.object_id,
                                    timeline_cmd.timeline_id, 
                                    timeline_cmd.start_time_sec,
                                    if is_cached_state { "Cached-New" } else { "New" }
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if modified {
            packet.raw_data.clear(); 
        }

        Ok(())
    }
}
