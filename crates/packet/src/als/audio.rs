use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use hound::WavSpec;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use super::proto::{PacketInfo, define::data_frame};

pub struct AudioRawPacket {
    pub timestamp: DateTime<Utc>,
    pub payload: Vec<u8>,
}

pub struct AudioBuilder {
    /// object id sets
    pub channel_sets: HashSet<i32>,
    /// audio raw packets
    pub audio_packets: HashMap<i32, Vec<AudioRawPacket>>,
    output_dir: Option<String>,
}

impl AudioBuilder {
    pub fn new(output_dir: Option<String>) -> Self {
        Self {
            channel_sets: HashSet::new(),
            audio_packets: HashMap::new(),
            output_dir,
        }
    }

    pub fn add_source(&mut self, object_id: i32) {
        self.channel_sets.insert(object_id);
    }

    /// WARNING, BUG, TODO: Maybe bug for packets after convert commands.
    pub fn handle_audio_packet(&mut self, packet: &PacketInfo) {
        let data_pack = &packet.data_pack;
        // 如果InstantiateObject 和 UpdateObject 同时出现，忽略当前包
        if data_pack.frames.iter().any(|f| f.message.is_some()) {
            let has_instantiate = data_pack
                .frames
                .iter()
                .any(|f| matches!(f.message, Some(data_frame::Message::InstantiateObject(_))));
            let has_update = data_pack
                .frames
                .iter()
                .any(|f| matches!(f.message, Some(data_frame::Message::UpdateObject(_))));
            if has_instantiate && has_update {
                tracing::trace!(
                    "Packet contains both InstantiateObject and UpdateObject messages, skipping packet at timestamp: {}",
                    packet.timestamp
                );
                if has_instantiate {
                    for frame in &data_pack.frames {
                        if let Some(message) = &frame.message {
                            if let data_frame::Message::InstantiateObject(_) = message {
                                self.handle_instantiate_audio(message);
                            }
                        }
                    }
                }
                return;
            }
        }
        self.handle_update_audio(&packet);
    }

    pub fn handle_instantiate_audio(&mut self, obj: &data_frame::Message) {
        if let data_frame::Message::InstantiateObject(obj) = obj {
            let name = String::from_utf8_lossy(&obj.prefab_name);
            if name.contains("Voice") {
                let object_id = obj.object_id;
                self.add_source(object_id);
                tracing::trace!(
                    "Found audio source with object id: {} and prefab name: {}",
                    object_id,
                    name
                );
            }
        } else {
            return;
        }
    }

    pub fn handle_update_audio(&mut self, packet: &PacketInfo) {
        let data_pack = &packet.data_pack;
        for frame in &data_pack.frames {
            if let Some(data_frame::Message::UpdateObject(obj)) = &frame.message {
                if self.channel_sets.contains(&obj.object_id) {
                    let audio_packet = AudioRawPacket {
                        timestamp: packet.timestamp,
                        payload: obj.payload[28..].to_vec(), // skip header (28 bytes
                    };
                    self.audio_packets
                        .entry(obj.object_id)
                        .or_default()
                        .push(audio_packet);
                }
            }
        }
    }

    pub fn write(&mut self) -> Result<()> {
        if let Some(output_dir) = self.output_dir.clone() {
            self.write_to_file(&output_dir)
        } else {
            Err(anyhow!("No output directory specified"))
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, output_dir: P) -> Result<()> {
        std::fs::create_dir_all(&output_dir)?;
        // handle things here
        for (object_id, packets) in &self.audio_packets {
            let mut dec = opus::Decoder::new(48000, opus::Channels::Stereo)?;
            tracing::debug!(
                "Writing audio for object id: {} with {} packets",
                object_id,
                packets.len()
            );
            let spec = WavSpec {
                channels: opus::Channels::Stereo as u16,
                sample_rate: 48000,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let file_path = output_dir.as_ref().join(format!("audio_{}.wav", object_id));
            let mut writer = hound::WavWriter::create(&file_path, spec)?;

            for packet in packets {
                if packet.payload.is_empty() {
                    continue;
                }

                let mut decode_buffer = vec![0i16; 5760]; // Magic number: max frame size

                match dec.decode(&packet.payload, &mut decode_buffer[..960 * 2], false) {
                    Ok(samples) => {
                        for &sample in &decode_buffer[..samples * 2] {
                            writer.write_sample(sample)?;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to decode opus packet: {:?}", e);
                        continue;
                    }
                }
            }
            writer.finalize()?;
            tracing::info!("Wrote audio file: {:?}", file_path);
        }
        // clear cache
        self.channel_sets.clear();
        self.audio_packets.clear();

        Ok(())
    }
}
