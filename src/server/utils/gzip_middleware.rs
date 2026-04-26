use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

/// GzipCompress compresses data using gzip.
pub fn gzip_compress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;
    use std::io::Read;

    #[test]
    fn test_gzip_compress() {
        let data = b"hello world";
        let compressed = gzip_compress(data).unwrap();
        
        assert!(compressed.len() > 0);
        
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        
        assert_eq!(decompressed, data);
    }
}
