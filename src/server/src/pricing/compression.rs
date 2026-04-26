use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::io::{Write, Read};

const COMPRESSION_PREFIX: &str = "gz_b64:";

pub fn compress_lossless(data: &str) -> Result<String, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;
    
    let b64 = STANDARD.encode(&compressed);
    Ok(format!("{}{}", COMPRESSION_PREFIX, b64))
}

pub fn decompress_lossless(data: &str) -> Result<String, String> {
    if !data.starts_with(COMPRESSION_PREFIX) {
        return Ok(data.to_string());
    }

    let b64_data = &data[COMPRESSION_PREFIX.len()..];
    let decoded = STANDARD.decode(b64_data).map_err(|e| e.to_string())?;
    
    let mut decoder = GzDecoder::new(&decoded[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| e.to_string())?;
    
    String::from_utf8(decompressed).map_err(|e| e.to_string())
}

pub fn reduce_tokens(data: &str) -> String {
    let stop_words: std::collections::HashSet<&str> = [
        "a", "an", "the", "is", "are",
        "and", "or", "but", "in", "on",
        "at", "to", "for", "with", "by",
        "about", "as", "of",
    ].iter().cloned().collect();

    data.split_whitespace()
        .filter(|word| {
            let clean_word = word.to_lowercase();
            !stop_words.contains(clean_word.as_str())
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn truncate_by_word_count(data: &str, max_words: usize) -> String {
    if max_words == 0 {
        return "".to_string();
    }
    let words: Vec<&str> = data.split_whitespace().collect();
    if words.len() <= max_words {
        return data.to_string();
    }
    words[..max_words].join(" ")
}

pub fn minify_json_prompt(data: &str) -> String {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
        if let Ok(minified) = serde_json::to_string(&val) {
            return minified;
        }
    }
    data.to_string()
}
