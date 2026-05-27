use std::sync::Arc;
use std::time::{Instant, Duration};
use dashmap::DashMap;

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
}

pub struct PromptCache {
    cache: Arc<DashMap<String, CachedResponse>>,
    ttl: Duration,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
            cache: Arc::new(DashMap::new()),
            ttl,
        }
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let entry = self.cache.get(prompt);
        if let Some(entry_ref) = entry {
            if entry_ref.created_at.elapsed() <= self.ttl {
                return Some(entry_ref.clone());
            }
            drop(entry_ref);
            // Remove expired entry atomically
            self.cache.remove_if(prompt, |_, v| v.created_at.elapsed() > self.ttl);
        }
        None
    }

    pub fn get_with_cost_cents(&self, prompt: &str) -> (Option<CachedResponse>, i64) {
        let res = self.get(prompt);
        let cost = if let Some(ref r) = res {
            tracing::info!("💰 Miser cost optimization: Prompt cache hit saved {} tokens", r.token_count);
            // very rough estimate of saved cents for cache hit
            static RATIO: std::sync::OnceLock<f64> = std::sync::OnceLock::new(); let ratio = RATIO.get_or_init(|| { std::env::var("MISER_TOKEN_RATIO").unwrap_or_else(|_| "0.0001".to_string()).parse::<f64>().unwrap_or(0.0001) }); (r.token_count as f64 * ratio).round() as i64
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        self.cache.insert(prompt.to_string(), CachedResponse {
            text: response.to_string(),
            created_at: Instant::now(),
            token_count,
        });
    }

    pub fn clear_expired(&self) {
        let now = Instant::now();
        self.cache.retain(|_, entry| now.duration_since(entry.created_at) <= self.ttl);
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
        assert_eq!(response.unwrap().text, "Paris");
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
}

pub fn truncate_context(context: &str, max_tokens: usize) -> String {
    // Note: Do not truncate JSON payloads as it corrupts them.
    if context.trim_start().starts_with('{') || context.trim_start().starts_with('[') {
        return context.to_string();
    }

    // Rough estimation: 1 char is approx 0.25 tokens (4 chars per token).
    let max_chars = max_tokens * 4;

    // Using chars().count() safely counts Unicode characters
    let total_chars = context.chars().count();
    if total_chars <= max_chars {
        return context.to_string();
    }

    // Intelligently truncate by keeping the beginning and the end of the context
    // preserving the most important instructions (start) and immediate context (end).
    let keep_start = max_chars / 2;
    let keep_end = max_chars - keep_start;

    let start_idx = context.char_indices().nth(keep_start).map(|(i, _)| i).unwrap_or(context.len());
    let end_idx = context.char_indices().rev().nth(keep_end.saturating_sub(1)).map(|(i, _)| i).unwrap_or(0);

    if start_idx >= end_idx {
        return context.to_string();
    }

    let start_portion = &context[..start_idx];
    let end_portion = &context[end_idx..];

    format!("{}...\n[TRUNCATED FOR TOKEN EFFICIENCY]\n...{}", start_portion, end_portion)
}

#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn test_truncate_context() {
        let context = "This is a very long context string that needs to be truncated intelligently by the new prompt caching cost efficiency feature implemented in OHC. We want to save tokens without losing the core message.\n\nHere is a new line.";
        let truncated = truncate_context(context, 10);
        assert!(truncated.contains("This is a"));
        assert!(truncated.contains("a new line."));
        assert!(truncated.contains("[TRUNCATED FOR TOKEN EFFICIENCY]"));
        assert!(truncated.contains("..."));
    }

    #[test]
    fn test_truncate_context_json() {
        let context = "{\"key\": \"this is a very long json string that should not be truncated because it would corrupt the json payload and cause downstream errors\"}";
        let truncated = truncate_context(context, 10);
        assert_eq!(truncated, context); // Should not be truncated
    }

    #[test]
    fn test_truncate_context_multibyte() {
        let context = "🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀🚀";
        let truncated = truncate_context(context, 10);
        assert!(truncated.contains("🚀"));
        assert!(truncated.contains("[TRUNCATED FOR TOKEN EFFICIENCY]"));
    }
}
