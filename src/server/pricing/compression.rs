use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::io::{Write, Read, Cursor};

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

use std::sync::OnceLock;

static STOP_WORDS: OnceLock<std::collections::HashSet<&'static str>> = OnceLock::new();

pub fn reduce_tokens(data: &str) -> String {
    let stop_words = STOP_WORDS.get_or_init(|| {
        [
            "a", "an", "the", "is", "are",
            "and", "or", "but", "in", "on",
            "at", "to", "for", "with", "by",
            "about", "as", "of",
        ].iter().cloned().collect()
    });

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

/// Optimizes an image by resizing it to a maximum dimension and converting it to WebP.
pub fn optimize_image(data: &[u8], max_dim: u32) -> Result<(Vec<u8>, String), String> {
    let img = image::load_from_memory(data).map_err(|e| e.to_string())?;

    // Only resize if it exceeds max_dim
    let (width, height) = image::GenericImageView::dimensions(&img);
    let resized = if width > max_dim || height > max_dim {
        img.thumbnail(max_dim, max_dim)
    } else {
        img
    };

    let mut webp_data = Vec::new();
    let mut cursor = Cursor::new(&mut webp_data);

    // We use a default quality for WebP encoding
    // 💰 Miser: Implement image auto-resizing and WebP conversion for product photos
    // This reduces storage compression and CDN transit costs significantly.
    resized.write_to(&mut cursor, image::ImageFormat::WebP).map_err(|e| e.to_string())?;

    Ok((webp_data, "image/webp".to_string()))
}

pub fn is_image_extension(ext: &str) -> bool {
    matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp")
}

pub fn get_optimized_key(key: &str) -> String {
    let path = std::path::Path::new(key);
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if is_image_extension(ext) && ext.to_lowercase() != "webp" {
            return path.with_extension("webp").to_string_lossy().to_string();
        }
    }
    key.to_string()
}

pub fn cdn_url(path: &str) -> String {
    let cdn_host = std::env::var("OHC_CDN_HOST").unwrap_or_default();
    if cdn_host.is_empty() || path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }
    let cdn_host = cdn_host.trim_end_matches('/');
    let path_str = if path.starts_with('/') { path.to_string() } else { format!("/{}", path) };
    format!("{}{}", cdn_host, path_str)
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

    #[test]
    fn test_is_image_extension() {
        assert!(is_image_extension("png"));
        assert!(is_image_extension("JPG"));
        assert!(!is_image_extension("txt"));
    }

    #[test]
    fn test_get_optimized_key() {
        assert_eq!(get_optimized_key("test.png"), "test.webp");
        assert_eq!(get_optimized_key("test.jpg"), "test.webp");
        assert_eq!(get_optimized_key("test.webp"), "test.webp");
        assert_eq!(get_optimized_key("test.txt"), "test.txt");
    }

    #[test]
    fn test_cdn_url() {
        unsafe { std::env::set_var("OHC_CDN_HOST", "https://cdn.example.com") };
        assert_eq!(cdn_url("/assets/img.png"), "https://cdn.example.com/assets/img.png");
        assert_eq!(cdn_url("assets/img.png"), "https://cdn.example.com/assets/img.png");
        assert_eq!(cdn_url("https://other.com/img.png"), "https://other.com/img.png");

        unsafe { std::env::set_var("OHC_CDN_HOST", "https://cdn.example.com/") };
        assert_eq!(cdn_url("/assets/img.png"), "https://cdn.example.com/assets/img.png");

        unsafe { std::env::remove_var("OHC_CDN_HOST") };
        assert_eq!(cdn_url("/assets/img.png"), "/assets/img.png");
    }

    #[test]
    fn test_optimize_image_valid() {
        use image::{ImageBuffer, Rgb};

        // Create a 10x10 RGB image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);
        let mut png_data = Vec::new();
        let mut cursor = Cursor::new(&mut png_data);
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

        let (optimized, mime) = optimize_image(&png_data, 5).unwrap();
        assert_eq!(mime, "image/webp");
        assert!(!optimized.is_empty());

        // Verify it's actually WebP and resized
        let opt_img = image::load_from_memory(&optimized).unwrap();
        let (w, h) = image::GenericImageView::dimensions(&opt_img);
        assert!(w <= 5);
        assert!(h <= 5);
    }
}
