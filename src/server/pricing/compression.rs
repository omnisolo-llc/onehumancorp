use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::io::{Write, Read, Cursor};
use dashmap::DashMap;
use std::sync::OnceLock;

const COMPRESSION_PREFIX: &str = "gz_b64:";

#[inline]
pub fn compress_lossless(data: &str) -> Result<String, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;
    
    let b64 = STANDARD.encode(&compressed);
    Ok(format!("{}{}", COMPRESSION_PREFIX, b64))
}

#[inline]
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

static STOP_WORDS: &[&str] = &[
    "a", "about", "above", "after", "again", "against", "all", "am", "an", "and",
    "any", "are", "aren't", "as", "at", "be", "because", "been", "before", "being",
    "below", "between", "both", "but", "by", "can", "can't", "cannot", "could", "couldn't",
    "did", "didn't", "do", "does", "doesn't", "doing", "don't", "down", "during", "each",
    "few", "for", "from", "further", "had", "hadn't", "has", "hasn't", "have", "haven't",
    "having", "he", "he'd", "he'll", "he's", "her", "here", "here's", "hers", "herself",
    "him", "himself", "his", "how", "how's", "i", "i'd", "i'll", "i'm", "i've",
    "if", "in", "into", "is", "isn't", "it", "it's", "its", "itself", "just",
    "let's", "me", "more", "most", "mustn't", "my", "myself", "no", "nor", "not",
    "now", "of", "off", "on", "once", "only", "or", "other", "ought", "our",
    "ours", "ourselves", "out", "over", "own", "same", "shan't", "she", "she'd", "she'll",
    "she's", "should", "shouldn't", "so", "some", "such", "than", "that", "that's", "the",
    "their", "theirs", "them", "themselves", "then", "there", "there's", "these", "they", "they'd",
    "they'll", "they're", "they've", "this", "those", "through", "to", "too", "under", "until",
    "up", "very", "was", "wasn't", "we", "we'd", "we'll", "we're", "we've", "were",
    "weren't", "what", "what's", "when", "when's", "where", "where's", "which", "while", "who",
    "who's", "whom", "why", "why's", "will", "with", "won't", "would", "wouldn't", "you",
    "you'd", "you'll", "you're", "you've", "your", "yours", "yourself", "yourselves",
];

use std::sync::atomic::{AtomicU32, Ordering};

struct CacheEntry {
    value: String,
    access_count: AtomicU32,
}

static REDUCE_TOKENS_CACHE: OnceLock<DashMap<String, CacheEntry>> = OnceLock::new();


pub fn reduce_tokens(data: &str) -> String {
    let cache = REDUCE_TOKENS_CACHE.get_or_init(DashMap::new);

    if let Some(cached) = cache.get(data) {
        cached.access_count.fetch_add(1, Ordering::Relaxed);
        return cached.value.clone();
    }

    let reduced = data.split_whitespace()
        .filter(|word| {
            let len = word.len();
            if len == 0 || len > 10 {
                return true;
            }

            // Fast stack-allocated case-insensitive binary search
            let mut buf = [0u8; 10];
            for (i, b) in word.bytes().take(10).enumerate() {
                buf[i] = b.to_ascii_lowercase();
            }
            let res = STOP_WORDS.binary_search_by(|probe| {
                probe.as_bytes().cmp(&buf[..len])
            });
            res.is_err()
        })
        .fold(String::with_capacity(data.len()), |mut acc, w| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(w);
            acc
        });

    if cache.len() > 10_000 {
        // Fast eviction pass: retain items with access_count > 0,
        // halve the count of retained items.
        // We use retain() to avoid manual iteration and locking.
        cache.retain(|_, v| {
            let count = v.access_count.load(Ordering::Relaxed);
            if count == 0 {
                false
            } else {
                v.access_count.store(count / 2, Ordering::Relaxed);
                true
            }
        });

        // If still too large, forcefully clear out some
        if cache.len() > 9_500 {
            cache.clear();
        }
    }

    cache.insert(data.to_string(), CacheEntry {
        value: reduced.clone(),
        // Start count at 1 so new items aren't immediately evicted in the first pass
        access_count: AtomicU32::new(1),
    });
    reduced
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
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(data)
        && let Ok(minified) = serde_json::to_string(&val) {
            return minified;
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
    // This reduces storage compression and CDN transit costs significantly.
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut cursor);
    resized.write_with_encoder(encoder).map_err(|e| e.to_string())?;

    Ok((webp_data, "image/webp".to_string()))
}

pub fn is_image_extension(ext: &str) -> bool {
    matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp")
}

pub fn get_optimized_key(key: &str) -> String {
    let path = std::path::Path::new(key);
    if let Some(ext) = path.extension().and_then(|e| e.to_str())
        && is_image_extension(ext) && ext.to_lowercase() != "webp" {
            return path.with_extension("webp").to_string_lossy().to_string();
        }
    key.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_lossless() {
        let original_data = "This is a prompt that needs to be compressed to save space and tokens in our database.";

        let compressed = compress_lossless(original_data).expect("failed to unwrap");
        assert!(compressed.starts_with(COMPRESSION_PREFIX));
        assert_ne!(compressed, original_data);

        let decompressed = decompress_lossless(&compressed).expect("failed to unwrap");
        assert_eq!(decompressed, original_data);
    }

    #[test]
    fn test_decompress_uncompressed() {
        let original_data = "Plain text data";
        let decompressed = decompress_lossless(original_data).expect("failed to unwrap");
        assert_eq!(decompressed, original_data);
    }

    #[test]
    fn test_reduce_tokens() {
        let input = "This is a long sentence with some stop words in it and about some things.";
        // Stop words now include: a, an, the, is, are, and, or, but, in, on, at, to, for, with, by, about, as, of, it, this, some...
        // Result should remove "is", "a", "with", "some", "in", "it", "and", "about", "some".
        let reduced = reduce_tokens(input);
        // Note: "This" is removed because "this" is in stop words and the check is case-insensitive.
        assert_eq!(reduced, "long sentence stop words things.");
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
        let parsed_min: serde_json::Value = serde_json::from_str(&minified).expect("failed to unwrap");
        let parsed_orig: serde_json::Value = serde_json::from_str(input_json).expect("failed to unwrap");
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
    fn test_optimize_image_valid() {
        use image::{ImageBuffer, Rgb};

        // Create a 10x10 RGB image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);
        let mut png_data = Vec::new();
        let mut cursor = Cursor::new(&mut png_data);
        img.write_to(&mut cursor, image::ImageFormat::Png).expect("failed to unwrap");

        let (optimized, mime) = optimize_image(&png_data, 5).expect("failed to unwrap");
        assert_eq!(mime, "image/webp");
        assert!(!optimized.is_empty());

        // Verify it's actually WebP and resized
        let opt_img = image::load_from_memory(&optimized).expect("failed to unwrap");
        let (w, h) = image::GenericImageView::dimensions(&opt_img);
        assert!(w <= 5);
        assert!(h <= 5);
    }

    #[test]
    fn test_optimize_image_invalid() {
        let invalid_data = vec![0, 1, 2, 3];
        let result = optimize_image(&invalid_data, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_lossless_invalid_base64() {
        let invalid_base64 = format!("{}invalid_base64", COMPRESSION_PREFIX);
        let result = decompress_lossless(&invalid_base64);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_lossless_invalid_gzip() {
        // "AAAA" is valid base64 but invalid gzip
        let invalid_gzip = format!("{}AAAA", COMPRESSION_PREFIX);
        let result = decompress_lossless(&invalid_gzip);
        assert!(result.is_err());
    }
}
