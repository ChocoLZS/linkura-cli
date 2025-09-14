#[cfg(test)]
mod tests {
    use crate::{AlsDownloader, Downloader, MrsDownloader};

    #[tokio::test]
    async fn test_downloader_creation() {
        let _downloader = Downloader::new(5);
        assert!(true); // Basic creation test
    }

    #[tokio::test]
    async fn test_als_downloader_creation() {
        let _als_downloader = AlsDownloader::new(5);
        assert!(true); // Basic creation test
    }

    #[tokio::test]
    async fn test_mrs_downloader_creation() {
        let _mrs_downloader = MrsDownloader::new(5);
        assert!(true); // Basic creation test
    }

    #[test]
    fn test_mrs_segment_parsing() {
        let mrs_downloader = MrsDownloader::new(1);

        // Create a test binary content with segment names
        let test_content = b"some_binary_data\x00segment_test-uuid_0001.ias\x00more_data\x00segment_test-uuid_0002.ias\x00";

        let segments = mrs_downloader.parse_iarc_segments(test_content).unwrap();
        assert!(segments.len() >= 1);
        assert!(
            segments
                .iter()
                .any(|s| s.contains("segment_") && s.ends_with(".ias"))
        );
    }

    #[test]
    fn test_als_m3u8_parsing() {
        let als_downloader = AlsDownloader::new(1);

        let test_m3u8 = "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:10\nsegment_001.ts\nsegment_002.ts\n#EXT-X-ENDLIST";

        let segments = als_downloader.parse_m3u8_segments(test_m3u8).unwrap();
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], "segment_001.ts");
        assert_eq!(segments[1], "segment_002.ts");
    }
}
