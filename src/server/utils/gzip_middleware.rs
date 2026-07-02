use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

/// GzipCompress compresses data using gzip.
pub fn gzip_compress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

/// should_compress checks headers to decide if response should be compressed.
pub fn should_compress(headers: &std::collections::HashMap<String, String>) -> bool {
    let accept_encoding = headers.get("Accept-Encoding")
        .or_else(|| headers.get("accept-encoding"))
        .map(|s| s.as_str())
        .unwrap_or("");
    let upgrade = headers.get("Upgrade")
        .or_else(|| headers.get("upgrade"))
        .map(|s| s.as_str())
        .unwrap_or("");
    let accept = headers.get("Accept")
        .or_else(|| headers.get("accept"))
        .map(|s| s.as_str())
        .unwrap_or("");

    if !accept_encoding.contains("gzip") {
        return false;
    }

    if !upgrade.is_empty() || accept == "text/event-stream" {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;
    use std::io::Read;
    use std::collections::HashMap;

    #[test]
    fn test_gzip_compress() {
        let data = b"hello world";
        let compressed = gzip_compress(data).unwrap();
        
        assert!(!compressed.is_empty());
        
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_should_compress() {
        let mut headers = HashMap::new();
        headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
        assert!(should_compress(&headers));

        headers.clear();
        headers.insert("Accept-Encoding".to_string(), "deflate".to_string());
        assert!(!should_compress(&headers));

        headers.clear();
        headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
        headers.insert("Upgrade".to_string(), "websocket".to_string());
        assert!(!should_compress(&headers));

        headers.clear();
        headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
        headers.insert("Accept".to_string(), "text/event-stream".to_string());
        assert!(!should_compress(&headers));

        headers.clear();
        headers.insert("accept-encoding".to_string(), "gzip".to_string());
        assert!(should_compress(&headers));
    }
}
