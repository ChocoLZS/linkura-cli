use crate::als::proto::{
    OutputWriter, PacketInfo, ProtoPacketReader, calculate_digest,
    proto::als::{DataFrame, data_frame},
};
use anyhow::{Context, Result, anyhow};
use prost::Message;
use std::fs::File;

pub fn diff_standard_files(
    file1_path: &str,
    file2_path: &str,
    output_path: Option<&str>,
) -> Result<()> {
    let mut writer = OutputWriter::new(output_path)?;

    writer.writeln("===========================================")?;
    writer.writeln("       ALS Standard Files Diff Analysis")?;
    writer.writeln("===========================================")?;
    writer.writeln(&format!("File 1: {}", file1_path))?;
    writer.writeln(&format!("File 2: {}", file2_path))?;
    writer.writeln("Comparing DataFrames in packets 3 and 4")?;
    writer.writeln("===========================================")?;
    writer.writeln("")?;

    // Read packets from both files
    let packets1 = read_standard_packets(file1_path, &mut writer)?;
    let packets2 = read_standard_packets(file2_path, &mut writer)?;

    if packets1.len() < 4 {
        return Err(anyhow!(
            "File 1 has only {} packets, need at least 4",
            packets1.len()
        ));
    }
    if packets2.len() < 4 {
        return Err(anyhow!(
            "File 2 has only {} packets, need at least 4",
            packets2.len()
        ));
    }

    writer.writeln("Analyzing packet 3 DataFrames...")?;
    writer.writeln("=================================")?;
    let diff_result_3 = compare_packet_dataframes(&packets1[2], &packets2[2], 3, &mut writer)?;

    writer.writeln("")?;
    writer.writeln("Analyzing packet 4 DataFrames...")?;
    writer.writeln("=================================")?;
    let diff_result_4 = compare_packet_dataframes(&packets1[3], &packets2[3], 4, &mut writer)?;

    writer.writeln("")?;
    writer.writeln("===========================================")?;
    writer.writeln("                 SUMMARY")?;
    writer.writeln("===========================================")?;
    writer.writeln(&format!(
        "Packet 3 DataFrames: {}",
        if diff_result_3 {
            "IDENTICAL"
        } else {
            "DIFFERENT"
        }
    ))?;
    writer.writeln(&format!(
        "Packet 4 DataFrames: {}",
        if diff_result_4 {
            "IDENTICAL"
        } else {
            "DIFFERENT"
        }
    ))?;

    if diff_result_3 && diff_result_4 {
        writer.writeln("")?;
        writer.writeln("✅ Both packets have identical DataFrames!")?;
    } else {
        writer.writeln("")?;
        writer.writeln("❌ DataFrames differ between files!")?;
    }

    writer.flush()?;
    Ok(())
}

fn read_standard_packets(file_path: &str, writer: &mut OutputWriter) -> Result<Vec<PacketInfo>> {
    let file =
        File::open(file_path).with_context(|| format!("Failed to open file: {}", file_path))?;

    let metadata = file
        .metadata()
        .with_context(|| format!("Failed to read file metadata: {}", file_path))?;

    writer.writeln(&format!("Reading file: {}", file_path))?;
    writer.writeln(&format!("File size: {} bytes", metadata.len()))?;

    let mut reader = ProtoPacketReader::new(file);
    let packets = reader.read_packets_with_limit(4)?;

    writer.writeln(&format!("Read {} packets", packets.len()))?;
    writer.writeln("")?;
    Ok(packets)
}

fn compare_packet_dataframes(
    packet1: &PacketInfo,
    packet2: &PacketInfo,
    packet_num: usize,
    writer: &mut OutputWriter,
) -> Result<bool> {
    writer.writeln(&format!("Packet {} comparison:", packet_num))?;

    // Show protobuf segment digests
    let protobuf_digest1 = packet1.protobuf_digest();
    let protobuf_digest2 = packet2.protobuf_digest();
    writer.writeln(&format!("  Protobuf segment digest comparison:"))?;
    writer.writeln(&format!("    File 1: {}", protobuf_digest1))?;
    writer.writeln(&format!("    File 2: {}", protobuf_digest2))?;
    writer.writeln(&format!(
        "    Segments equal: {}",
        if protobuf_digest1 == protobuf_digest2 {
            "YES"
        } else {
            "NO"
        }
    ))?;
    writer.writeln("")?;

    let frames1 = extract_dataframes_from_packet(packet1);
    let frames2 = extract_dataframes_from_packet(packet2);

    writer.writeln(&format!("  File 1 has {} DataFrames", frames1.len()))?;
    writer.writeln(&format!("  File 2 has {} DataFrames", frames2.len()))?;

    if frames1.len() != frames2.len() {
        writer.writeln("  ❌ Different number of DataFrames")?;
        return Ok(false);
    }

    if frames1.is_empty() {
        writer.writeln("  ⚠️  No DataFrames found in either packet")?;
        return Ok(true);
    }

    // Calculate digests for all frames in both files
    let frame_digests1 = packet1.frame_digests();
    let frame_digests2 = packet2.frame_digests();

    writer.writeln("  DataFrame digest comparison:")?;
    if !frames1.is_empty() {
        writer.writeln("    File 1 DataFrame digests:")?;
        for (index, digest) in &frame_digests1 {
            writer.writeln(&format!("      Frame {}: {}", index + 1, digest))?;
        }
        writer.writeln("    File 2 DataFrame digests:")?;
        for (index, digest) in &frame_digests2 {
            writer.writeln(&format!("      Frame {}: {}", index + 1, digest))?;
        }
        writer.writeln("")?;
    }

    // Compare DataFrames (order doesn't matter)
    let mut matched_frames = 0;
    let mut unmatched_frames_1 = Vec::new();
    let mut unmatched_frames_2 = frames2.clone();

    for (i, frame1) in frames1.iter().enumerate() {
        let mut found_match = false;

        for (j, frame2) in unmatched_frames_2.iter().enumerate() {
            if dataframes_equal(frame1, frame2) {
                matched_frames += 1;
                unmatched_frames_2.remove(j);
                found_match = true;
                let digest1 = frame_digests1
                    .get(i)
                    .map(|(_, d)| d.as_str())
                    .unwrap_or("unknown");
                writer.writeln(&format!(
                    "  ✅ Frame {} matches (digest: {})",
                    i + 1,
                    digest1
                ))?;
                break;
            }
        }

        if !found_match {
            unmatched_frames_1.push(frame1);
            let digest1 = frame_digests1
                .get(i)
                .map(|(_, d)| d.as_str())
                .unwrap_or("unknown");
            writer.writeln(&format!(
                "  ❌ Frame {} has no match (digest: {})",
                i + 1,
                digest1
            ))?;
        }
    }

    writer.writeln(&format!(
        "  Result: {}/{} frames matched",
        matched_frames,
        frames1.len()
    ))?;

    if !unmatched_frames_1.is_empty() {
        writer.writeln("  Unmatched frames in File 1:")?;
        for (i, frame) in unmatched_frames_1.iter().enumerate() {
            writer.writeln(&format!(
                "    Frame {}: {}",
                i + 1,
                format_dataframe_summary(frame)
            ))?;
        }
    }

    if !unmatched_frames_2.is_empty() {
        writer.writeln("  Unmatched frames in File 2:")?;
        for (i, frame) in unmatched_frames_2.iter().enumerate() {
            writer.writeln(&format!(
                "    Frame {}: {}",
                i + 1,
                format_dataframe_summary(frame)
            ))?;
        }
    }

    Ok(matched_frames == frames1.len())
}

fn extract_dataframes_from_packet(packet: &PacketInfo) -> Vec<DataFrame> {
    packet.data_pack.frames.clone()
}

fn dataframes_equal(frame1: &DataFrame, frame2: &DataFrame) -> bool {
    // Compare using SHA-256 digest for more reliable comparison
    let bytes1 = frame1.encode_to_vec();
    let bytes2 = frame2.encode_to_vec();
    let digest1 = calculate_digest(&bytes1);
    let digest2 = calculate_digest(&bytes2);
    digest1 == digest2
}

fn format_dataframe_summary(frame: &DataFrame) -> String {
    if let Some(message) = &frame.message {
        match message {
            data_frame::Message::Room(room) => {
                format!("Room(id: {})", String::from_utf8_lossy(&room.id))
            }
            data_frame::Message::InstantiateObject(obj) => {
                format!(
                    "InstantiateObject(prefab: {}, object_id: {})",
                    String::from_utf8_lossy(&obj.prefab_name),
                    obj.object_id
                )
            }
            data_frame::Message::UpdateObject(obj) => {
                format!(
                    "UpdateObject(object_id: {}, method: {}, payload: {} bytes)",
                    obj.object_id,
                    obj.method,
                    obj.payload.len()
                )
            }
            data_frame::Message::DestroyObject(obj) => {
                format!("DestroyObject(object_id: {})", obj.object_id)
            }
            _ => "Unknown".to_string(),
        }
    } else {
        "Empty DataFrame".to_string()
    }
}
