//! Pure data reading layer - only responsible for parsing binary data
//! No output, no statistics, no formatting
//!
//! This module provides a trait-based abstraction for reading different packet formats.

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use prost::Message;
use std::collections::VecDeque;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::usize;

use super::define::DataPack;
use crate::als::proto::PacketInfo;

/// Trait for reading packets from different formats
///
/// Implement this trait to support new packet formats while reusing
/// all analysis and formatting logic.
pub trait PacketReaderTrait {
    /// Read the next packet, returns None on EOF
    fn read_packet(&mut self) -> Result<Option<PacketInfo>>;
    fn read_packets(&mut self) -> Result<Vec<PacketInfo>>;
}

/// Iterator wrapper for any PacketReaderTrait
pub struct PacketIterator<'a> {
    reader: &'a mut dyn PacketReaderTrait,
}

impl<'a> PacketIterator<'a> {
    pub fn new(reader: &'a mut dyn PacketReaderTrait) -> Self {
        Self { reader }
    }
}

impl<'a> Iterator for PacketIterator<'a> {
    type Item = Result<PacketInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_packet() {
            Ok(Some(packet)) => Some(Ok(packet)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

// ============================================================================
// Standard Format Implementation
// ============================================================================

/// Reader for standard packet format (length + marker + timestamp + protobuf)
pub struct StandardPacketReader {
    reader: BufReader<File>,
}

impl StandardPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    /// Create a boxed trait object for polymorphic use
    pub fn boxed(file: File) -> Box<dyn PacketReaderTrait> {
        Box::new(Self::new(file))
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

impl PacketReaderTrait for StandardPacketReader {
    fn read_packet(&mut self) -> Result<Option<PacketInfo>> {
        // Try to read length, return None on EOF
        let length = match self.read_u16_be() {
            Ok(len) => len,
            Err(e) if is_eof_error(&e) => return Ok(None),
            Err(e) => return Err(e),
        };

        if length < 9 {
            return Err(anyhow!(
                "Invalid packet length: {}, must be at least 9",
                length
            ));
        }

        let marker = self
            .read_u8()
            .with_context(|| "Failed to read marker byte")?;
        if marker != 0x01 {
            return Err(anyhow!(
                "Invalid marker byte: expected 0x01, got 0x{:02x}",
                marker
            ));
        }

        let timestamp_micros = self
            .read_u64_be()
            .with_context(|| "Failed to read timestamp")?;
        let timestamp =
            DateTime::from_timestamp_micros(timestamp_micros as i64).ok_or_else(|| {
                anyhow!(
                    "Invalid timestamp: {} (0x{:x})",
                    timestamp_micros,
                    timestamp_micros
                )
            })?;

        let data_length = length - 9;
        let mut data = vec![0u8; data_length as usize];
        self.reader
            .read_exact(&mut data)
            .with_context(|| format!("Failed to read protobuf data of length {}", data_length))?;

        let data_pack = DataPack::decode(data.as_slice()).map_err(|e| {
            anyhow!(
                "Failed to decode protobuf data (length: {}): {}",
                data_length,
                e
            )
        })?;

        Ok(Some(PacketInfo {
            timestamp,
            data_pack,
            raw_data: data,
        }))
    }

    fn read_packets(&mut self) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        while let Some(packet) = self.read_packet()? {
            packets.push(packet);
        }
        Ok(packets)
    }
}

// ============================================================================
// Mixed Format Implementation
// ============================================================================

/// State for tracking expected packet type in mixed format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MixedReaderState {
    ExpectProtobuf,
    ExpectTimestamp,
}

/// Reader for mixed packet format (alternating protobuf and timestamp packets)
pub struct MixedPacketReader {
    reader: BufReader<File>,
    state: MixedReaderState,
    pending_protobuf: Option<(DataPack, Vec<u8>)>,
}

impl MixedPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            state: MixedReaderState::ExpectProtobuf,
            pending_protobuf: None,
        }
    }

    /// Create a boxed trait object for polymorphic use
    pub fn boxed(file: File) -> Box<dyn PacketReaderTrait> {
        Box::new(Self::new(file))
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

impl PacketReaderTrait for MixedPacketReader {
    // read two packets each time, convert it to one PacketInfo
    fn read_packet(&mut self) -> Result<Option<PacketInfo>> {
        loop {
            // Read length header
            let length = match self.read_u16_be() {
                Ok(len) => len,
                Err(e) if is_eof_error(&e) => return Ok(None),
                Err(e) => return Err(e),
            };

            match self.state {
                MixedReaderState::ExpectProtobuf => {
                    // Read protobuf packet
                    if length < 3 {
                        return Err(anyhow!("Invalid protobuf packet length: {}", length));
                    }

                    let _unused = self.read_u8()?;
                    let data_length = length - 1;
                    let mut data = vec![0u8; data_length as usize];
                    self.reader.read_exact(&mut data)?;

                    let data_pack = DataPack::decode(data.as_slice())
                        .with_context(|| "Failed to decode protobuf")?;

                    // Store protobuf data, wait for timestamp
                    self.pending_protobuf = Some((data_pack, data));
                    self.state = MixedReaderState::ExpectTimestamp;
                    // Continue to read timestamp
                    continue;
                }
                MixedReaderState::ExpectTimestamp => {
                    // Read timestamp packet
                    if length != 8 {
                        return Err(anyhow!("Invalid timestamp packet length: {}", length));
                    }

                    let timestamp_micros = self.read_u64_be()?;
                    let timestamp = DateTime::from_timestamp_micros(timestamp_micros as i64)
                        .ok_or_else(|| anyhow!("Invalid timestamp: {}", timestamp_micros))?;

                    // Combine with pending protobuf
                    let (data_pack, raw_data) = self
                        .pending_protobuf
                        .take()
                        .ok_or_else(|| anyhow!("Missing protobuf packet"))?;

                    self.state = MixedReaderState::ExpectProtobuf;

                    return Ok(Some(PacketInfo {
                        timestamp,
                        data_pack,
                        raw_data,
                    }));
                }
            }
        }
    }

    fn read_packets(&mut self) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        while let Some(packet) = self.read_packet()? {
            packets.push(packet);
        }
        Ok(packets)
    }
}

/// Reader for legacy mixed packet format, no timestamp packet
pub struct LegacyPacketReader {
    reader: BufReader<File>,
}

impl LegacyPacketReader {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    /// Create a boxed trait object for polymorphic use
    pub fn boxed(file: File) -> Box<dyn PacketReaderTrait> {
        Box::new(Self::new(file))
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    // fn read_u64_be(&mut self) -> Result<u64> {
    //     let mut buf = [0u8; 8];
    //     self.reader.read_exact(&mut buf)?;
    //     Ok(u64::from_be_bytes(buf))
    // }
}

impl PacketReaderTrait for LegacyPacketReader {
    // read two packets each time, convert it to one PacketInfo
    fn read_packet(&mut self) -> Result<Option<PacketInfo>> {
        // Read length header
        let length = match self.read_u16_be() {
            Ok(len) => len,
            Err(e) if is_eof_error(&e) => return Ok(None),
            Err(e) => return Err(e),
        };

        // Read protobuf packet
        if length < 3 {
            return Err(anyhow!("Invalid protobuf packet length: {}", length));
        }
        let _unused = self.read_u8()?;
        let data_length = length - 1;
        let mut data = vec![0u8; data_length as usize];
        self.reader.read_exact(&mut data)?;
        let data_pack =
            DataPack::decode(data.as_slice()).with_context(|| "Failed to decode protobuf")?;
        return Ok(Some(PacketInfo {
            timestamp: Utc::now(),
            raw_data: data_pack.encode_to_vec(),
            data_pack: data_pack,
        }));
    }
    fn read_packets(&mut self) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        while let Some(packet) = self.read_packet()? {
            packets.push(packet);
        }
        Ok(packets)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn is_eof_error(e: &anyhow::Error) -> bool {
    let error_msg = e.to_string().to_lowercase();
    error_msg.contains("eof")
        || error_msg.contains("unexpected end")
        || error_msg.contains("failed to fill whole buffer")
}

// ============================================================================
// Convenience Type Aliases
// ============================================================================

/// Alias for standard reader (backward compatibility)
pub type PacketReader = StandardPacketReader;

/// Reader factory function type
type ReaderFactory = Box<dyn Fn(File) -> Box<dyn PacketReaderTrait>>;

/// Configuration for PacketsBufferReader limits
#[derive(Debug, Clone, Copy)]
pub struct ReaderLimits {
    /// Maximum number of packets to read across all files (None = unlimited)
    pub max_packets: usize,
    /// Maximum number of files to process (None = unlimited)
    pub max_files: usize,
    /// Maximum number of packets per file (None = unlimited)
    pub max_packets_per_file: usize,
}

impl Default for ReaderLimits {
    fn default() -> Self {
        Self {
            max_packets: usize::MAX,
            max_files: usize::MAX,
            max_packets_per_file: usize::MAX,
        }
    }
}

impl ReaderLimits {
    /// No limits at all
    pub fn unlimited() -> Self {
        Self::default()
    }

    /// Set maximum total packets
    pub fn with_max_packets(mut self, max: usize) -> Self {
        self.max_packets = max;
        self
    }

    /// Set maximum files to process
    pub fn with_max_files(mut self, max: usize) -> Self {
        self.max_files = max;
        self
    }

    /// Set maximum packets per file
    pub fn with_max_packets_per_file(mut self, max: usize) -> Self {
        self.max_packets_per_file = max;
        self
    }
}

pub struct PacketsBufferReader {
    current_reader: Option<Box<dyn PacketReaderTrait>>,
    file_entries: VecDeque<DirEntry>,
    reader_factory: ReaderFactory,
    limits: ReaderLimits,
    // Tracking counters
    total_packets_read: usize,
    files_processed: usize,
    current_file_packets: usize,
}

impl PacketsBufferReader {
    /// Create a new multi-file reader with a custom reader factory
    pub fn new<F>(file_entries: VecDeque<DirEntry>, reader_factory: F) -> Self
    where
        F: Fn(File) -> Box<dyn PacketReaderTrait> + 'static,
    {
        Self {
            current_reader: None,
            file_entries,
            reader_factory: Box::new(reader_factory),
            limits: ReaderLimits::default(),
            total_packets_read: 0,
            files_processed: 0,
            current_file_packets: 0,
        }
    }

    /// Create with standard packet reader factory
    pub fn new_standard(file_entries: VecDeque<DirEntry>) -> Self {
        Self::new(file_entries, |file| StandardPacketReader::boxed(file))
    }

    /// Create with mixed packet reader factory
    pub fn new_mixed(file_entries: VecDeque<DirEntry>) -> Self {
        Self::new(file_entries, |file| MixedPacketReader::boxed(file))
    }

    /// Set limits (Builder pattern)
    pub fn with_limits(mut self, limits: ReaderLimits) -> Self {
        self.limits = limits;

        // Apply max_files limit by truncating the queue
        self.file_entries.truncate(limits.max_files);

        self
    }

    /// Create a boxed trait object for polymorphic use
    pub fn boxed<F>(
        file_entries: VecDeque<DirEntry>,
        reader_factory: F,
    ) -> Box<dyn PacketReaderTrait>
    where
        F: Fn(File) -> Box<dyn PacketReaderTrait> + 'static,
    {
        Box::new(Self::new(file_entries, reader_factory))
    }

    /// Get statistics about reading progress
    pub fn stats(&self) -> ReaderStats {
        ReaderStats {
            total_packets_read: self.total_packets_read,
            files_processed: self.files_processed,
            current_file_packets: self.current_file_packets,
            files_remaining: self.file_entries.len(),
        }
    }

    /// Check if any limit has been reached
    fn check_limits(&self) -> bool {
        // Check total packets limit
        if self.total_packets_read >= self.limits.max_packets {
            return true;
        }

        // Check per-file packets limit
        if self.current_file_packets >= self.limits.max_packets_per_file {
            return true;
        }

        false
    }

    /// Try to open the next file and create a new reader
    fn open_next_file(&mut self) -> Result<bool> {
        // Reset per-file counter
        self.current_file_packets = 0;

        if let Some(entry) = self.file_entries.pop_front() {
            let file = File::open(entry.path())
                .with_context(|| format!("Failed to open file: {:?}", entry.path()))?;
            self.current_reader = Some((self.reader_factory)(file));
            self.files_processed += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Statistics about reader progress
#[derive(Debug, Clone, Copy)]
pub struct ReaderStats {
    /// Total packets read across all files
    pub total_packets_read: usize,
    /// Number of files completely processed
    pub files_processed: usize,
    /// Packets read from current file
    pub current_file_packets: usize,
    /// Number of files remaining in queue
    pub files_remaining: usize,
}

impl PacketReaderTrait for PacketsBufferReader {
    fn read_packet(&mut self) -> Result<Option<PacketInfo>> {
        loop {
            // Check if limits reached
            if self.check_limits() {
                // Per-file limit reached, move to next file
                if self.current_file_packets >= self.limits.max_packets_per_file {
                    self.current_reader = None; // Close current file
                // Try next file (will be opened below)
                } else {
                    // Total packets limit reached, stop completely
                    return Ok(None);
                }
            }

            // Try to read from current reader
            if let Some(reader) = &mut self.current_reader {
                match reader.read_packet()? {
                    Some(packet) => {
                        // Update counters
                        self.total_packets_read += 1;
                        self.current_file_packets += 1;
                        return Ok(Some(packet));
                    }
                    None => {
                        // Current file exhausted, try next file
                        self.current_reader = None;
                    }
                }
            }

            // No current reader or current file exhausted, open next file
            if !self.open_next_file()? {
                // No more files
                return Ok(None);
            }
            // Loop will retry reading from the new reader
        }
    }

    fn read_packets(&mut self) -> Result<Vec<PacketInfo>> {
        let mut packets = Vec::new();
        while let Some(packet) = self.read_packet()? {
            packets.push(packet);
        }
        Ok(packets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_packet_reader_eof() {
        // Test that EOF is handled gracefully
    }

    #[test]
    fn test_trait_object() {
        // Test that we can use readers polymorphically
    }
}
