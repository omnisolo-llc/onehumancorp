
pub mod payload_compression {
    use std::io::{Read, Write};
    use flate2::Compression;
    use flate2::read::GzDecoder;
    use flate2::write::GzEncoder;

    pub fn compress_payload(payload: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(payload).map_err(|e| e.to_string())?;
        encoder.finish().map_err(|e| e.to_string())
    }

    pub fn decompress_payload(compressed: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = GzDecoder::new(compressed);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| e.to_string())?;
        Ok(decompressed)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compression_roundtrip() {
            let original = b"Hello, Mesh! This is a test payload for interop.".to_vec();
            let compressed = compress_payload(&original).unwrap();
            let decompressed = decompress_payload(&compressed).unwrap();
            assert_eq!(original, decompressed);
        }

        #[test]
        fn test_compression_empty() {
            let original = vec![];
            let compressed = compress_payload(&original).unwrap();
            let decompressed = decompress_payload(&compressed).unwrap();
            assert_eq!(original, decompressed);
        }
    }
}
