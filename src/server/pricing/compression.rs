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
        "about", "as", "of", "be", "been", "being",
        "have", "has", "had", "do", "does", "did",
        "will", "would", "shall", "should", "can", "could",
        "may", "might", "must", "if", "then", "else",
        "which", "who", "whom", "whose", "this", "that", "these", "those",
        "it", "its", "they", "them", "their", "we", "us", "our",
        "you", "your", "he", "him", "his", "she", "her",
    ].iter().cloned().collect();

    data.split_whitespace()
        .filter(|word| {
            let clean_word = word.to_lowercase();
            let clean_word = clean_word.trim_matches(|c: char| !c.is_alphanumeric());
            !stop_words.contains(clean_word)
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn minify_system_prompt(data: &str) -> String {
    let mut minified = String::new();
    for line in data.lines() {
        let trimmed = line.trim();
        // Preserve Markdown headers (#), but strip empty lines and // comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        minified.push_str(trimmed);
        minified.push('\n');
    }
    minified.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_lossless() {
        let original_data = "This is a prompt that needs to be compressed to save space and tokens in our database.";

        let compressed = compress_lossless(original_data).unwrap();
        assert!(compressed.starts_with(COMPRESSION_PREFIX));
        assert_ne!(compressed, original_data);

        let decompressed = decompress_lossless(&compressed).unwrap();
        assert_eq!(decompressed, original_data);
    }

    #[test]
    fn test_decompress_uncompressed() {
        let original_data = "Plain text data";
        let decompressed = decompress_lossless(original_data).unwrap();
        assert_eq!(decompressed, original_data);
    }

    #[test]
    fn test_reduce_tokens() {
        let input = "This is a long sentence with some stop words in it and about some things.";
        // Stop words: a, an, the, is, are, and, or, but, in, on, at, to, for, with, by, about, as, of, it, this, etc.
        let reduced = reduce_tokens(input);
        // "This", "is", "a", "with", "in", "it", "and", "about" are removed.
        assert_eq!(reduced, "long sentence some stop words some things.");
    }

    #[test]
    fn test_reduce_tokens_consecutive() {
        let input = "the is a test and or but";
        let reduced = reduce_tokens(input);
        assert_eq!(reduced, "test");
    }

    #[test]
    fn test_minify_system_prompt() {
        let input = r#"
            # System Instructions
            You are a helpful assistant.

            // Rules:
            1. Be concise.
            2. Be helpful.
        "#;
        let minified = minify_system_prompt(input);
        assert!(minified.contains("# System Instructions"));
        assert!(minified.contains("You are a helpful assistant."));
        assert!(!minified.contains("// Rules:"));
        assert!(minified.contains("1. Be concise."));
    }

    #[test]
    fn test_truncate_by_word_count() {
        let input = "One two three four five six seven eight nine ten";

        assert_eq!(truncate_by_word_count(input, 5), "One two three four five");
        assert_eq!(truncate_by_word_count(input, 15), input); // More words than input
        assert_eq!(truncate_by_word_count(input, 0), "");
    }

    #[test]
    fn test_minify_json_prompt() {
        let input_json = r#"{
            "role": "system",
            "content": "You are a helpful assistant."
        }"#;

        let minified = minify_json_prompt(input_json);
        // Field order might vary depending on serde implementation. Parse it back.
        let parsed_min: serde_json::Value = serde_json::from_str(&minified).unwrap();
        let parsed_orig: serde_json::Value = serde_json::from_str(input_json).unwrap();
        assert_eq!(parsed_min, parsed_orig);

        // Ensure no whitespace outside strings
        assert!(!minified.contains("\n"));

        // Invalid json should return as-is
        let invalid = "{ invalid json ]";
        assert_eq!(minify_json_prompt(invalid), invalid);
    }
}
