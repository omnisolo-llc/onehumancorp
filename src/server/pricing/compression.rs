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

pub fn smart_truncate_messages(messages: &mut Vec<serde_json::Value>, max_total_words: usize) {
    if messages.is_empty() {
        return;
    }

    // Identify system message (usually index 0)
    let system_message = if messages[0]["role"] == "system" {
        Some(messages.remove(0))
    } else {
        None
    };

    let mut current_words = 0;
    if let Some(ref sys) = system_message {
        current_words += sys["content"].as_str().unwrap_or("").split_whitespace().count();
    }

    // Keep adding messages from the end (most recent) until we hit the limit
    let mut kept_recent = Vec::new();
    for msg in messages.iter().rev() {
        let content = msg["content"].as_str().unwrap_or("");
        let msg_words = content.split_whitespace().count();

        if current_words + msg_words <= max_total_words || kept_recent.is_empty() {
            kept_recent.push(msg.clone());
            current_words += msg_words;
        } else {
            // If the message itself is too long, we could truncate it, but for now we just stop.
            break;
        }
    }

    kept_recent.reverse();

    // Reconstruct the message list
    messages.clear();
    if let Some(sys) = system_message {
        messages.push(sys);
    }
    messages.extend(kept_recent);
}

pub fn optimize_image(image_data: &[u8], max_width: u32, max_height: u32) -> Result<Vec<u8>, String> {
    use image::GenericImageView;
    use std::io::Cursor;

    let img = image::load_from_memory(image_data).map_err(|e| e.to_string())?;
    let (width, height) = img.dimensions();

    let resized = if width > max_width || height > max_height {
        img.thumbnail(max_width, max_height)
    } else {
        img
    };

    let mut result = Vec::new();
    let mut cursor = Cursor::new(&mut result);

    // Encode as WebP for maximum compression
    resized.write_to(&mut cursor, image::ImageFormat::WebP).map_err(|e| e.to_string())?;

    Ok(result)
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
        // Stop words: a, an, the, is, are, and, or, but, in, on, at, to, for, with, by, about, as, of
        // Result should remove "is", "a", "with", "in", "and", "about".
        // Note: 'it' and 'some' are not stop words here.
        let reduced = reduce_tokens(input);
        assert_eq!(reduced, "This long sentence some stop words it some things.");
    }

    #[test]
    fn test_truncate_by_word_count() {
        let input = "One two three four five six seven eight nine ten";

        assert_eq!(truncate_by_word_count(input, 5), "One two three four five");
        assert_eq!(truncate_by_word_count(input, 15), input); // More words than input
        assert_eq!(truncate_by_word_count(input, 0), "");
    }

    #[test]
    fn test_smart_truncate_messages() {
        let mut messages = vec![
            serde_json::json!({"role": "system", "content": "You are a helpful assistant."}),
            serde_json::json!({"role": "user", "content": "Hello, how are you?"}),
            serde_json::json!({"role": "assistant", "content": "I am fine, thank you!"}),
            serde_json::json!({"role": "user", "content": "What is the weather today?"}),
            serde_json::json!({"role": "assistant", "content": "It is sunny today."}),
        ];

        // Total words: 5 (sys) + 4 (u1) + 5 (a1) + 5 (u2) + 4 (a2) = 23
        // Limit to 15 words. Should keep system, and the last two messages (9 words).
        // 5 + 9 = 14 words.
        smart_truncate_messages(&mut messages, 15);

        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0]["role"], "system");
        assert_eq!(messages[1]["role"], "user");
        assert_eq!(messages[1]["content"], "What is the weather today?");
        assert_eq!(messages[2]["role"], "assistant");
        assert_eq!(messages[2]["content"], "It is sunny today.");
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
