use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
    pub ttl: Duration,
}

pub struct PromptCache {
    cache: Arc<DashMap<String, CachedResponse>>,
    default_ttl: Duration,
    max_capacity: usize,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
}

impl PromptCache {
    pub fn new(default_ttl: Duration) -> Self {
        Self::with_capacity(default_ttl, 1000)
    }

    pub fn with_capacity(default_ttl: Duration, max_capacity: usize) -> Self {
        PromptCache {
            cache: Arc::new(DashMap::new()),
            default_ttl,
            max_capacity,
            telemetry_store: None,
        }
    }

    pub fn with_telemetry(
        mut self,
        store: std::sync::Arc<::server_harness::telemetry::ViolationStore>,
    ) -> Self {
        self.telemetry_store = Some(store);
        self
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let entry = self.cache.get_mut(prompt);
        if let Some(mut entry_ref) = entry {
            if entry_ref.value().created_at.elapsed() <= entry_ref.value().ttl {
                // True LRU support: update created_at on access so evict_oldest properly evicts least-recently-used items
                entry_ref.value_mut().created_at = Instant::now();
                return Some(entry_ref.value().clone());
            }
            drop(entry_ref);
            // Remove expired entry atomically
            self.cache
                .remove_if(prompt, |_, v| v.created_at.elapsed() > v.ttl);
        }
        None
    }

    pub fn get_with_cost_cents(&self, prompt: &str, model: &str) -> (Option<CachedResponse>, i64) {
        if self.telemetry_store.is_some() {
            tracing::info!("💰 Miser telemetry: Prompt cache lookup recorded");
        } else {
            tracing::info!("💰 Miser telemetry: Prompt cache lookup");
        }
        let res = self.get(prompt);
        let cost = if let Some(ref r) = res {
            tracing::info!(
                "💰 Miser cost optimization: Prompt cache hit saved {} tokens",
                r.token_count
            ); // pii-safe

            if let Some(store) = &self.telemetry_store {
                store.llm_cost_counter.add(
                    0,
                    &[
                        opentelemetry::KeyValue::new("cache_hit", "true"),
                        opentelemetry::KeyValue::new("model", model.to_string()),
                    ],
                );
            }

            let pricing = super::calculator::get_pricing(model);
            let cost_dollars = (r.token_count as f64 / 1_000_000.0) * pricing.cached_cost;
            (cost_dollars * 100.0).round() as i64
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        self.set_with_ttl(prompt, response, token_count, self.default_ttl);
    }

    pub fn set_with_ttl(&self, prompt: &str, response: &str, token_count: usize, ttl: Duration) {
        if self.cache.len() >= self.max_capacity {
            self.evict_oldest();
        }
        self.cache.insert(
            prompt.to_string(),
            CachedResponse {
                text: response.to_string(),
                created_at: Instant::now(),
                token_count,
                ttl,
            },
        );
    }

    fn evict_oldest(&self) {
        // Clear expired first to see if that frees enough space
        self.clear_expired();

        let len = self.cache.len();
        if len < self.max_capacity {
            return;
        }

        // We need to evict entries to get back down to 90% of max capacity
        let target_len = (self.max_capacity as f64 * 0.9) as usize;
        let to_remove = len.saturating_sub(target_len);

        if to_remove == 0 {
            return;
        }

        // To avoid cloning all strings (which are long prompts), we keep a BinaryHeap
        // of the oldest elements.
        // BinaryHeap is a max-heap. We want to keep the oldest elements (smallest Instant).
        // If we store `Reverse<Instant>`, a max-heap gives us the *largest Reverse<Instant>*,
        // which corresponds to the *smallest Instant*.
        // Wait, if we want to find the `to_remove` oldest elements:
        // A max-heap of size `to_remove` tracking the *newest* of the oldest will help.
        // We push elements. If size > to_remove, we pop the max (which is the newest among the oldest).
        // What's left are the `to_remove` oldest elements.
        use std::collections::BinaryHeap;

        // (Instant, String) tuple for the heap. Instant implements Ord.
        let mut heap: BinaryHeap<(Instant, String)> = BinaryHeap::with_capacity(to_remove + 1);

        for kv in self.cache.iter() {
            let created_at = kv.value().created_at;

            // If heap isn't full, just push. We clone the key here.
            if heap.len() < to_remove {
                heap.push((created_at, kv.key().clone()));
            } else {
                #[allow(clippy::collapsible_if)]
                // If it's full, compare with the max element (the newest of the oldest)
                #[allow(clippy::collapsible_if)]
                if let Some(max) = heap.peek() {
                    if created_at < max.0 {
                        // This element is older than the newest of our oldest.
                        // Clone the key and push it, then pop the max.
                        heap.push((created_at, kv.key().clone()));
                        heap.pop();
                    }
                }
            }
        }

        // Remove the oldest elements from the cache
        for (_, key) in heap.into_iter() {
            self.cache.remove(&key);
        }
    }

    pub fn clear_expired(&self) {
        let now = Instant::now();
        self.cache
            .retain(|_, entry| now.duration_since(entry.created_at) <= entry.ttl);
    }

    /// Intelligently truncates a context string to fit within a given token limit.
    /// This is a fast heuristic using 4 chars per token. Safely handles UTF-8 string slicing.
    pub fn truncate_context(context: &str, max_tokens: usize) -> String {
        if max_tokens == 0 {
            return String::new();
        }

        // Token Efficiency Optimization: Remove markdown image/link URLs as they cost tokens but rarely help context.
        // E.g., [text](https://...) -> [text]
        let mut optimized_context = std::borrow::Cow::Borrowed(context);
        if context.contains("](") {
            let mut result = String::with_capacity(context.len());
            let mut chars = context.char_indices().peekable();
            let mut in_url = false;

            while let Some((i, c)) = chars.next() {
                if !in_url && c == ']' {
                    result.push(c);
                    if let Some(&(_, '(')) = chars.peek() {
                        let rest = &context[i + 2..];
                        if rest.starts_with("http://") || rest.starts_with("https://") {
                            in_url = true;
                            chars.next(); // Skip '('
                        }
                    }
                } else if in_url && c == ')' {
                    in_url = false;
                } else if !in_url {
                    result.push(c);
                }
            }
            if result.len() != context.len() {
                optimized_context = std::borrow::Cow::Owned(result);
            }
        }
        let context = &*optimized_context;

        let max_chars = max_tokens * 4; // Fast heuristic assuming 4 chars per token

        let mut char_count = 0;
        let mut byte_index = context.len();

        // Fast path for ASCII strings where bytes == chars
        if context.is_ascii() {
            if context.len() <= max_chars {
                return context.to_string();
            }
            byte_index = max_chars;
            char_count = max_chars;
        } else {
            // Optimization: If byte length <= max_chars, char length must also be <= max_chars
            if context.len() <= max_chars {
                return context.to_string();
            }

            for (i, _) in context.char_indices() {
                if char_count == max_chars {
                    byte_index = i;
                    break;
                }
                char_count += 1;
            }
        }

        if char_count < max_chars && byte_index == context.len() {
            return context.to_string();
        }

        let mut slice = &context[..byte_index];

        // Try to truncate at a word boundary to keep it "intelligent"
        if let Some(last_space) = slice.rfind(char::is_whitespace) {
            // Keep at least some content if the last space is too early.
            // Using char_count / 2 to avoid slicing a UTF-8 character based on bytes.
            let space_char_count = slice[..last_space].chars().count();
            if space_char_count > max_chars / 2 {
                slice = &slice[..last_space];
            }
        }

        // Clean up trailing whitespace and punctuation before appending ellipsis
        slice = slice.trim_end_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation());

        format!("{}...", slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_prompt_cache_get_set() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1);

        let response = cache.get("What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.expect("failed to unwrap").text, "Paris");
    }

    #[test]
    fn test_prompt_cache_expiration() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Hello", "World", 1);

        thread::sleep(Duration::from_millis(60));
        assert!(cache.get("Hello").is_none());
    }

    #[test]
    fn test_prompt_cache_clear_expired() {
        let cache = PromptCache::new(Duration::from_millis(50));
        cache.set("Test", "Data", 1);

        thread::sleep(Duration::from_millis(60));
        cache.clear_expired();

        assert!(cache.cache.is_empty());
    }

    #[test]
    fn test_prompt_cache_clear_expired_partial() {
        let cache = PromptCache::new(Duration::from_millis(100));
        cache.set_with_ttl("Expired", "Data", 1, Duration::from_millis(10));
        cache.set_with_ttl("Keep", "Data", 1, Duration::from_millis(1000));

        thread::sleep(Duration::from_millis(20));
        cache.clear_expired();

        assert!(cache.get("Expired").is_none());
        assert!(cache.get("Keep").is_some());
    }

    #[test]
    fn test_prompt_cache_with_telemetry() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));
        let cache = PromptCache::new(Duration::from_secs(10)).with_telemetry(store.clone());

        cache.set("What is the capital of France?", "Paris", 100);
        let (response, _cost) =
            cache.get_with_cost_cents("What is the capital of France?", "gpt-4o");

        assert!(response.is_some());
        // Since telemetry doesn't have an easily readable getter in this mock test without accessing the internal structure,
        // we're primarily testing that it doesn't crash and the path is executed.
    }

    #[test]
    fn test_prompt_cache_get_with_cost_cents() {
        unsafe {
            std::env::set_var("OHC_LLM_MODEL", "gpt-4o");
        } // 5.00 per 1M tokens
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set("What is the capital of France?", "Paris", 1_000_000);

        let (response, cost) =
            cache.get_with_cost_cents("What is the capital of France?", "gpt-4o");
        assert!(response.is_some());
        assert_eq!(response.expect("failed to unwrap").text, "Paris");
        // 1,000,000 tokens * 2.50 / 1M = 2.5 dollars = 250 cents
        assert_eq!(cost, 250);
    }

    #[test]
    fn test_prompt_cache_get_missing() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let response = cache.get("What is the capital of France?");
        assert!(response.is_none());
        let (response_with_cost, cost) =
            cache.get_with_cost_cents("What is the capital of France?", "gpt-4o");
        assert!(response_with_cost.is_none());
        assert_eq!(cost, 0);
    }

    #[test]
    fn test_prompt_cache_capacity_eviction() {
        let cache = PromptCache::with_capacity(Duration::from_secs(10), 3);

        // Insert 3 items
        cache.set("key1", "val1", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("key2", "val2", 1);
        thread::sleep(Duration::from_millis(10));
        cache.set("key3", "val3", 1);

        assert_eq!(cache.cache.len(), 3);

        // Insert 4th item, triggering eviction
        // target capacity is 90% of 3 = 2.
        // currently 3 items, len = 3. target_len = 2. to_remove = 3 - 2 = 1.
        thread::sleep(Duration::from_millis(10));
        cache.set("key4", "val4", 1);

        assert_eq!(cache.cache.len(), 3); // 3 items inserted, 1 removed (oldest) + 1 newly inserted
        assert!(cache.get("key1").is_none()); // key1 was oldest, should be gone
        assert!(cache.get("key2").is_some());
        assert!(cache.get("key3").is_some());
        assert!(cache.get("key4").is_some());
    }

    #[test]
    fn test_prompt_cache_mass_eviction_memory_optimization() {
        let max_cap = 100;
        let cache = PromptCache::with_capacity(Duration::from_secs(10), max_cap);

        // Insert `max_cap` items
        for i in 0..max_cap {
            let key = format!("prompt-key-{}", i);
            cache.set(&key, "val", 1);
            // We sleep very briefly to guarantee different timestamps, though Instant::now() might be enough.
            // On fast systems, multiple may have the exact same Instant::now(), making ordering non-deterministic.
            // For a test, we can just ensure the first few have a definite gap.
            if i < 20 {
                thread::sleep(Duration::from_millis(1));
            }
        }

        assert_eq!(cache.cache.len(), 100);

        // Insert `max_cap + 1`th item, triggering eviction.
        // target capacity = 90% of 100 = 90.
        // len = 100, target_len = 90, to_remove = 10.
        // So 10 items will be evicted (the 10 oldest).
        cache.set("prompt-key-new", "new_val", 1);

        assert_eq!(cache.cache.len(), 91); // 100 - 10 + 1

        // Verify the oldest 10 items were removed (prompt-key-0 to prompt-key-9)
        for i in 0..10 {
            let key = format!("prompt-key-{}", i);
            assert!(cache.get(&key).is_none(), "Old key {} was not evicted!", key);
        }

        // Verify the newly inserted item is there
        assert!(cache.get("prompt-key-new").is_some());
    }

    #[test]
    fn test_prompt_cache_set_with_ttl() {
        let cache = PromptCache::new(Duration::from_secs(10));
        cache.set_with_ttl(
            "What is the capital of France?",
            "Paris",
            1,
            Duration::from_millis(10),
        );

        let response = cache.get("What is the capital of France?");
        assert!(response.is_some());
        assert_eq!(response.expect("failed to unwrap").text, "Paris");

        thread::sleep(Duration::from_millis(20));
        assert!(cache.get("What is the capital of France?").is_none());
    }

    #[test]
    fn test_truncate_context() {
        let text =
            "This is a very long string that we need to truncate to save some tokens and money.";

        // No truncation needed
        let res = PromptCache::truncate_context(text, 100);
        assert_eq!(res, text);

        // Truncate based on 4 chars per token
        // "This" = 4 chars (1 token). 10 tokens = 40 chars
        // text[..40] -> "This is a very long string that we need "
        // Last space at index 39
        let res2 = PromptCache::truncate_context(text, 10);
        assert!(res2.len() <= 43); // 39 chars + "..."
        assert!(res2.ends_with("..."));

        // Extreme truncation
        let res3 = PromptCache::truncate_context(text, 1);
        assert_eq!(res3, "This...");

        // Zero truncation
        let res4 = PromptCache::truncate_context(text, 0);
        assert_eq!(res4, "");
    }

    #[test]
    fn test_truncate_context_trailing_punctuation() {
        let text = "This is a sentence, with a comma that we want to truncate.";
        // "This is a sentence, with a " = 27 chars. 27/4 = 6 tokens. Let's say 7 tokens = 28 chars.
        // max_chars = 28 -> "This is a sentence, with a c" -> last_space = 26
        // truncated -> "This is a sentence, with a"
        let res = PromptCache::truncate_context(text, 7);
        // "This is a sentence, with a..." instead of "This is a sentence, with a ..."
        assert_eq!(res, "This is a sentence, with a...");

        let text2 = "Sentence,  more";
        // max_tokens = 2 -> 8 chars -> "Sentence" -> no space -> stays "Sentence" -> no punctuation to trim
        let res2 = PromptCache::truncate_context(text2, 2);
        assert_eq!(res2, "Sentence...");

        let text3 = "Sentence ,  more";
        // max_tokens = 2 -> 8 chars -> "Sentence"
        let res3 = PromptCache::truncate_context(text3, 2);
        assert_eq!(res3, "Sentence...");
    }

    #[test]
    fn test_truncate_context_multibyte_characters() {
        let text = "こんにちは世界！これは長い文字列です。";
        // 1 token = 4 chars. max_tokens = 2 -> 8 chars.
        // The text has 19 chars.
        let res = PromptCache::truncate_context(text, 2);
        assert_eq!(res, "こんにちは世界！..."); // 8 chars + "..."
    }

    #[test]
    fn test_truncate_context_strips_urls() {
        let text = "Check out this [link](https://example.com/very/long/url/that/wastes/tokens) for more info.";
        // Without stripping URL, length is 90 chars.
        // With stripping, it becomes "Check out this [link] for more info." -> 36 chars.
        // If max_tokens = 10, max_chars = 40.
        // If URL is not stripped, it will truncate. If stripped, it fits fully.
        let res = PromptCache::truncate_context(text, 10);
        assert_eq!(res, "Check out this [link] for more info.");
    }

    #[test]
    fn test_truncate_context_incomplete_url() {
        let text = "Check out this [link](https://example.com/very/long/url/that/wastes/tokens";
        // The URL is incomplete (missing closing ')').
        // Truncate based on 4 chars per token. 10 tokens = 40 chars.
        let res = PromptCache::truncate_context(text, 10);
        // The function shouldn't panic, it should just truncate normally or strip part of the URL.
        assert!(res.len() > 0);

        let text2 = "Check out this [link](http";
        let res2 = PromptCache::truncate_context(text2, 10);
        assert!(res2.len() > 0);
    }
}
