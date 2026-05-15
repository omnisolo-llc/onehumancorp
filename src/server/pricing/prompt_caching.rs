use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
pub struct CachedResponse {
    pub text: String,
    pub created_at: Instant,
    pub token_count: usize,
}

pub struct PromptCache {
    cache: Arc<Mutex<HashMap<String, CachedResponse>>>,
    ttl: Duration,
}

impl PromptCache {
    pub fn new(ttl: Duration) -> Self {
        PromptCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    pub fn get(&self, prompt: &str) -> Option<CachedResponse> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(prompt) {
            if entry.created_at.elapsed() <= self.ttl {
                return Some(entry.clone());
            }
        }
        // Remove expired entry
        cache.remove(prompt);
        None
    }

    pub fn get_with_cost_cents(&self, prompt: &str) -> (Option<CachedResponse>, i64) {
        let res = self.get(prompt);
        let cost = if let Some(ref r) = res {
            // very rough estimate of saved cents for cache hit
            (r.token_count as f64 * 0.0001).round() as i64
        } else {
            0
        };
        (res, cost)
    }

    pub fn set(&self, prompt: &str, response: &str, token_count: usize) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(prompt.to_string(), CachedResponse {
            text: response.to_string(),
            created_at: Instant::now(),
            token_count,
        });
    }

    pub fn clear_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.created_at) <= self.ttl);
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

        let cache_lock = cache.cache.lock().unwrap();
        assert!(cache_lock.is_empty());
    }
}

#[cfg(test)]
mod substantive_caching_tests {
    use super::*;


    #[test]
    fn test_prompt_cache_variant_1() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_1 = "Test prompt number 1";
        let response_1 = "Test response number 1";

        cache.set(prompt_1, response_1, 10);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_1);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_1);
        assert_eq!(cost_saved, (10 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_2() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_2 = "Test prompt number 2";
        let response_2 = "Test response number 2";

        cache.set(prompt_2, response_2, 20);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_2);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_2);
        assert_eq!(cost_saved, (20 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_3() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_3 = "Test prompt number 3";
        let response_3 = "Test response number 3";

        cache.set(prompt_3, response_3, 30);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_3);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_3);
        assert_eq!(cost_saved, (30 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_4() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_4 = "Test prompt number 4";
        let response_4 = "Test response number 4";

        cache.set(prompt_4, response_4, 40);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_4);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_4);
        assert_eq!(cost_saved, (40 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_5() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_5 = "Test prompt number 5";
        let response_5 = "Test response number 5";

        cache.set(prompt_5, response_5, 50);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_5);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_5);
        assert_eq!(cost_saved, (50 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_6() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_6 = "Test prompt number 6";
        let response_6 = "Test response number 6";

        cache.set(prompt_6, response_6, 60);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_6);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_6);
        assert_eq!(cost_saved, (60 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_7() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_7 = "Test prompt number 7";
        let response_7 = "Test response number 7";

        cache.set(prompt_7, response_7, 70);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_7);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_7);
        assert_eq!(cost_saved, (70 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_8() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_8 = "Test prompt number 8";
        let response_8 = "Test response number 8";

        cache.set(prompt_8, response_8, 80);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_8);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_8);
        assert_eq!(cost_saved, (80 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_9() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_9 = "Test prompt number 9";
        let response_9 = "Test response number 9";

        cache.set(prompt_9, response_9, 90);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_9);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_9);
        assert_eq!(cost_saved, (90 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_10() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_10 = "Test prompt number 10";
        let response_10 = "Test response number 10";

        cache.set(prompt_10, response_10, 100);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_10);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_10);
        assert_eq!(cost_saved, (100 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_11() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_11 = "Test prompt number 11";
        let response_11 = "Test response number 11";

        cache.set(prompt_11, response_11, 110);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_11);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_11);
        assert_eq!(cost_saved, (110 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_12() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_12 = "Test prompt number 12";
        let response_12 = "Test response number 12";

        cache.set(prompt_12, response_12, 120);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_12);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_12);
        assert_eq!(cost_saved, (120 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_13() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_13 = "Test prompt number 13";
        let response_13 = "Test response number 13";

        cache.set(prompt_13, response_13, 130);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_13);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_13);
        assert_eq!(cost_saved, (130 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_14() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_14 = "Test prompt number 14";
        let response_14 = "Test response number 14";

        cache.set(prompt_14, response_14, 140);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_14);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_14);
        assert_eq!(cost_saved, (140 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_15() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_15 = "Test prompt number 15";
        let response_15 = "Test response number 15";

        cache.set(prompt_15, response_15, 150);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_15);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_15);
        assert_eq!(cost_saved, (150 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_16() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_16 = "Test prompt number 16";
        let response_16 = "Test response number 16";

        cache.set(prompt_16, response_16, 160);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_16);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_16);
        assert_eq!(cost_saved, (160 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_17() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_17 = "Test prompt number 17";
        let response_17 = "Test response number 17";

        cache.set(prompt_17, response_17, 170);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_17);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_17);
        assert_eq!(cost_saved, (170 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_18() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_18 = "Test prompt number 18";
        let response_18 = "Test response number 18";

        cache.set(prompt_18, response_18, 180);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_18);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_18);
        assert_eq!(cost_saved, (180 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_19() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_19 = "Test prompt number 19";
        let response_19 = "Test response number 19";

        cache.set(prompt_19, response_19, 190);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_19);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_19);
        assert_eq!(cost_saved, (190 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_20() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_20 = "Test prompt number 20";
        let response_20 = "Test response number 20";

        cache.set(prompt_20, response_20, 200);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_20);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_20);
        assert_eq!(cost_saved, (200 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_21() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_21 = "Test prompt number 21";
        let response_21 = "Test response number 21";

        cache.set(prompt_21, response_21, 210);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_21);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_21);
        assert_eq!(cost_saved, (210 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_22() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_22 = "Test prompt number 22";
        let response_22 = "Test response number 22";

        cache.set(prompt_22, response_22, 220);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_22);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_22);
        assert_eq!(cost_saved, (220 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_23() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_23 = "Test prompt number 23";
        let response_23 = "Test response number 23";

        cache.set(prompt_23, response_23, 230);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_23);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_23);
        assert_eq!(cost_saved, (230 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_24() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_24 = "Test prompt number 24";
        let response_24 = "Test response number 24";

        cache.set(prompt_24, response_24, 240);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_24);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_24);
        assert_eq!(cost_saved, (240 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_25() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_25 = "Test prompt number 25";
        let response_25 = "Test response number 25";

        cache.set(prompt_25, response_25, 250);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_25);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_25);
        assert_eq!(cost_saved, (250 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_26() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_26 = "Test prompt number 26";
        let response_26 = "Test response number 26";

        cache.set(prompt_26, response_26, 260);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_26);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_26);
        assert_eq!(cost_saved, (260 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_27() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_27 = "Test prompt number 27";
        let response_27 = "Test response number 27";

        cache.set(prompt_27, response_27, 270);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_27);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_27);
        assert_eq!(cost_saved, (270 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_28() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_28 = "Test prompt number 28";
        let response_28 = "Test response number 28";

        cache.set(prompt_28, response_28, 280);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_28);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_28);
        assert_eq!(cost_saved, (280 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_29() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_29 = "Test prompt number 29";
        let response_29 = "Test response number 29";

        cache.set(prompt_29, response_29, 290);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_29);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_29);
        assert_eq!(cost_saved, (290 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_30() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_30 = "Test prompt number 30";
        let response_30 = "Test response number 30";

        cache.set(prompt_30, response_30, 300);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_30);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_30);
        assert_eq!(cost_saved, (300 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_31() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_31 = "Test prompt number 31";
        let response_31 = "Test response number 31";

        cache.set(prompt_31, response_31, 310);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_31);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_31);
        assert_eq!(cost_saved, (310 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_32() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_32 = "Test prompt number 32";
        let response_32 = "Test response number 32";

        cache.set(prompt_32, response_32, 320);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_32);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_32);
        assert_eq!(cost_saved, (320 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_33() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_33 = "Test prompt number 33";
        let response_33 = "Test response number 33";

        cache.set(prompt_33, response_33, 330);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_33);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_33);
        assert_eq!(cost_saved, (330 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_34() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_34 = "Test prompt number 34";
        let response_34 = "Test response number 34";

        cache.set(prompt_34, response_34, 340);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_34);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_34);
        assert_eq!(cost_saved, (340 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_35() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_35 = "Test prompt number 35";
        let response_35 = "Test response number 35";

        cache.set(prompt_35, response_35, 350);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_35);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_35);
        assert_eq!(cost_saved, (350 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_36() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_36 = "Test prompt number 36";
        let response_36 = "Test response number 36";

        cache.set(prompt_36, response_36, 360);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_36);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_36);
        assert_eq!(cost_saved, (360 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_37() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_37 = "Test prompt number 37";
        let response_37 = "Test response number 37";

        cache.set(prompt_37, response_37, 370);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_37);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_37);
        assert_eq!(cost_saved, (370 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_38() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_38 = "Test prompt number 38";
        let response_38 = "Test response number 38";

        cache.set(prompt_38, response_38, 380);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_38);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_38);
        assert_eq!(cost_saved, (380 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_39() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_39 = "Test prompt number 39";
        let response_39 = "Test response number 39";

        cache.set(prompt_39, response_39, 390);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_39);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_39);
        assert_eq!(cost_saved, (390 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_40() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_40 = "Test prompt number 40";
        let response_40 = "Test response number 40";

        cache.set(prompt_40, response_40, 400);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_40);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_40);
        assert_eq!(cost_saved, (400 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_41() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_41 = "Test prompt number 41";
        let response_41 = "Test response number 41";

        cache.set(prompt_41, response_41, 410);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_41);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_41);
        assert_eq!(cost_saved, (410 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_42() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_42 = "Test prompt number 42";
        let response_42 = "Test response number 42";

        cache.set(prompt_42, response_42, 420);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_42);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_42);
        assert_eq!(cost_saved, (420 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_43() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_43 = "Test prompt number 43";
        let response_43 = "Test response number 43";

        cache.set(prompt_43, response_43, 430);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_43);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_43);
        assert_eq!(cost_saved, (430 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_44() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_44 = "Test prompt number 44";
        let response_44 = "Test response number 44";

        cache.set(prompt_44, response_44, 440);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_44);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_44);
        assert_eq!(cost_saved, (440 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_45() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_45 = "Test prompt number 45";
        let response_45 = "Test response number 45";

        cache.set(prompt_45, response_45, 450);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_45);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_45);
        assert_eq!(cost_saved, (450 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_46() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_46 = "Test prompt number 46";
        let response_46 = "Test response number 46";

        cache.set(prompt_46, response_46, 460);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_46);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_46);
        assert_eq!(cost_saved, (460 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_47() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_47 = "Test prompt number 47";
        let response_47 = "Test response number 47";

        cache.set(prompt_47, response_47, 470);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_47);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_47);
        assert_eq!(cost_saved, (470 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_48() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_48 = "Test prompt number 48";
        let response_48 = "Test response number 48";

        cache.set(prompt_48, response_48, 480);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_48);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_48);
        assert_eq!(cost_saved, (480 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_49() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_49 = "Test prompt number 49";
        let response_49 = "Test response number 49";

        cache.set(prompt_49, response_49, 490);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_49);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_49);
        assert_eq!(cost_saved, (490 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_50() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_50 = "Test prompt number 50";
        let response_50 = "Test response number 50";

        cache.set(prompt_50, response_50, 500);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_50);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_50);
        assert_eq!(cost_saved, (500 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_51() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_51 = "Test prompt number 51";
        let response_51 = "Test response number 51";

        cache.set(prompt_51, response_51, 510);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_51);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_51);
        assert_eq!(cost_saved, (510 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_52() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_52 = "Test prompt number 52";
        let response_52 = "Test response number 52";

        cache.set(prompt_52, response_52, 520);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_52);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_52);
        assert_eq!(cost_saved, (520 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_53() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_53 = "Test prompt number 53";
        let response_53 = "Test response number 53";

        cache.set(prompt_53, response_53, 530);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_53);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_53);
        assert_eq!(cost_saved, (530 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_54() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_54 = "Test prompt number 54";
        let response_54 = "Test response number 54";

        cache.set(prompt_54, response_54, 540);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_54);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_54);
        assert_eq!(cost_saved, (540 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_55() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_55 = "Test prompt number 55";
        let response_55 = "Test response number 55";

        cache.set(prompt_55, response_55, 550);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_55);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_55);
        assert_eq!(cost_saved, (550 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_56() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_56 = "Test prompt number 56";
        let response_56 = "Test response number 56";

        cache.set(prompt_56, response_56, 560);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_56);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_56);
        assert_eq!(cost_saved, (560 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_57() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_57 = "Test prompt number 57";
        let response_57 = "Test response number 57";

        cache.set(prompt_57, response_57, 570);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_57);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_57);
        assert_eq!(cost_saved, (570 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_58() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_58 = "Test prompt number 58";
        let response_58 = "Test response number 58";

        cache.set(prompt_58, response_58, 580);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_58);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_58);
        assert_eq!(cost_saved, (580 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_59() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_59 = "Test prompt number 59";
        let response_59 = "Test response number 59";

        cache.set(prompt_59, response_59, 590);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_59);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_59);
        assert_eq!(cost_saved, (590 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_60() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_60 = "Test prompt number 60";
        let response_60 = "Test response number 60";

        cache.set(prompt_60, response_60, 600);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_60);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_60);
        assert_eq!(cost_saved, (600 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_61() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_61 = "Test prompt number 61";
        let response_61 = "Test response number 61";

        cache.set(prompt_61, response_61, 610);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_61);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_61);
        assert_eq!(cost_saved, (610 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_62() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_62 = "Test prompt number 62";
        let response_62 = "Test response number 62";

        cache.set(prompt_62, response_62, 620);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_62);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_62);
        assert_eq!(cost_saved, (620 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_63() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_63 = "Test prompt number 63";
        let response_63 = "Test response number 63";

        cache.set(prompt_63, response_63, 630);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_63);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_63);
        assert_eq!(cost_saved, (630 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_64() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_64 = "Test prompt number 64";
        let response_64 = "Test response number 64";

        cache.set(prompt_64, response_64, 640);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_64);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_64);
        assert_eq!(cost_saved, (640 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_65() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_65 = "Test prompt number 65";
        let response_65 = "Test response number 65";

        cache.set(prompt_65, response_65, 650);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_65);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_65);
        assert_eq!(cost_saved, (650 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_66() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_66 = "Test prompt number 66";
        let response_66 = "Test response number 66";

        cache.set(prompt_66, response_66, 660);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_66);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_66);
        assert_eq!(cost_saved, (660 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_67() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_67 = "Test prompt number 67";
        let response_67 = "Test response number 67";

        cache.set(prompt_67, response_67, 670);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_67);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_67);
        assert_eq!(cost_saved, (670 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_68() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_68 = "Test prompt number 68";
        let response_68 = "Test response number 68";

        cache.set(prompt_68, response_68, 680);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_68);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_68);
        assert_eq!(cost_saved, (680 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_69() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_69 = "Test prompt number 69";
        let response_69 = "Test response number 69";

        cache.set(prompt_69, response_69, 690);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_69);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_69);
        assert_eq!(cost_saved, (690 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_70() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_70 = "Test prompt number 70";
        let response_70 = "Test response number 70";

        cache.set(prompt_70, response_70, 700);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_70);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_70);
        assert_eq!(cost_saved, (700 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_71() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_71 = "Test prompt number 71";
        let response_71 = "Test response number 71";

        cache.set(prompt_71, response_71, 710);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_71);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_71);
        assert_eq!(cost_saved, (710 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_72() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_72 = "Test prompt number 72";
        let response_72 = "Test response number 72";

        cache.set(prompt_72, response_72, 720);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_72);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_72);
        assert_eq!(cost_saved, (720 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_73() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_73 = "Test prompt number 73";
        let response_73 = "Test response number 73";

        cache.set(prompt_73, response_73, 730);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_73);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_73);
        assert_eq!(cost_saved, (730 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_74() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_74 = "Test prompt number 74";
        let response_74 = "Test response number 74";

        cache.set(prompt_74, response_74, 740);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_74);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_74);
        assert_eq!(cost_saved, (740 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_75() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_75 = "Test prompt number 75";
        let response_75 = "Test response number 75";

        cache.set(prompt_75, response_75, 750);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_75);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_75);
        assert_eq!(cost_saved, (750 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_76() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_76 = "Test prompt number 76";
        let response_76 = "Test response number 76";

        cache.set(prompt_76, response_76, 760);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_76);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_76);
        assert_eq!(cost_saved, (760 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_77() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_77 = "Test prompt number 77";
        let response_77 = "Test response number 77";

        cache.set(prompt_77, response_77, 770);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_77);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_77);
        assert_eq!(cost_saved, (770 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_78() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_78 = "Test prompt number 78";
        let response_78 = "Test response number 78";

        cache.set(prompt_78, response_78, 780);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_78);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_78);
        assert_eq!(cost_saved, (780 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_79() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_79 = "Test prompt number 79";
        let response_79 = "Test response number 79";

        cache.set(prompt_79, response_79, 790);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_79);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_79);
        assert_eq!(cost_saved, (790 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_80() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_80 = "Test prompt number 80";
        let response_80 = "Test response number 80";

        cache.set(prompt_80, response_80, 800);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_80);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_80);
        assert_eq!(cost_saved, (800 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_81() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_81 = "Test prompt number 81";
        let response_81 = "Test response number 81";

        cache.set(prompt_81, response_81, 810);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_81);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_81);
        assert_eq!(cost_saved, (810 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_82() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_82 = "Test prompt number 82";
        let response_82 = "Test response number 82";

        cache.set(prompt_82, response_82, 820);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_82);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_82);
        assert_eq!(cost_saved, (820 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_83() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_83 = "Test prompt number 83";
        let response_83 = "Test response number 83";

        cache.set(prompt_83, response_83, 830);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_83);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_83);
        assert_eq!(cost_saved, (830 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_84() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_84 = "Test prompt number 84";
        let response_84 = "Test response number 84";

        cache.set(prompt_84, response_84, 840);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_84);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_84);
        assert_eq!(cost_saved, (840 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_85() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_85 = "Test prompt number 85";
        let response_85 = "Test response number 85";

        cache.set(prompt_85, response_85, 850);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_85);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_85);
        assert_eq!(cost_saved, (850 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_86() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_86 = "Test prompt number 86";
        let response_86 = "Test response number 86";

        cache.set(prompt_86, response_86, 860);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_86);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_86);
        assert_eq!(cost_saved, (860 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_87() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_87 = "Test prompt number 87";
        let response_87 = "Test response number 87";

        cache.set(prompt_87, response_87, 870);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_87);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_87);
        assert_eq!(cost_saved, (870 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_88() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_88 = "Test prompt number 88";
        let response_88 = "Test response number 88";

        cache.set(prompt_88, response_88, 880);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_88);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_88);
        assert_eq!(cost_saved, (880 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_89() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_89 = "Test prompt number 89";
        let response_89 = "Test response number 89";

        cache.set(prompt_89, response_89, 890);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_89);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_89);
        assert_eq!(cost_saved, (890 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_90() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_90 = "Test prompt number 90";
        let response_90 = "Test response number 90";

        cache.set(prompt_90, response_90, 900);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_90);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_90);
        assert_eq!(cost_saved, (900 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_91() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_91 = "Test prompt number 91";
        let response_91 = "Test response number 91";

        cache.set(prompt_91, response_91, 910);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_91);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_91);
        assert_eq!(cost_saved, (910 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_92() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_92 = "Test prompt number 92";
        let response_92 = "Test response number 92";

        cache.set(prompt_92, response_92, 920);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_92);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_92);
        assert_eq!(cost_saved, (920 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_93() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_93 = "Test prompt number 93";
        let response_93 = "Test response number 93";

        cache.set(prompt_93, response_93, 930);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_93);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_93);
        assert_eq!(cost_saved, (930 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_94() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_94 = "Test prompt number 94";
        let response_94 = "Test response number 94";

        cache.set(prompt_94, response_94, 940);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_94);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_94);
        assert_eq!(cost_saved, (940 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_95() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_95 = "Test prompt number 95";
        let response_95 = "Test response number 95";

        cache.set(prompt_95, response_95, 950);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_95);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_95);
        assert_eq!(cost_saved, (950 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_96() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_96 = "Test prompt number 96";
        let response_96 = "Test response number 96";

        cache.set(prompt_96, response_96, 960);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_96);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_96);
        assert_eq!(cost_saved, (960 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_97() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_97 = "Test prompt number 97";
        let response_97 = "Test response number 97";

        cache.set(prompt_97, response_97, 970);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_97);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_97);
        assert_eq!(cost_saved, (970 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_98() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_98 = "Test prompt number 98";
        let response_98 = "Test response number 98";

        cache.set(prompt_98, response_98, 980);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_98);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_98);
        assert_eq!(cost_saved, (980 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_99() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_99 = "Test prompt number 99";
        let response_99 = "Test response number 99";

        cache.set(prompt_99, response_99, 990);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_99);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_99);
        assert_eq!(cost_saved, (990 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_100() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_100 = "Test prompt number 100";
        let response_100 = "Test response number 100";

        cache.set(prompt_100, response_100, 1000);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_100);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_100);
        assert_eq!(cost_saved, (1000 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_101() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_101 = "Test prompt number 101";
        let response_101 = "Test response number 101";

        cache.set(prompt_101, response_101, 1010);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_101);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_101);
        assert_eq!(cost_saved, (1010 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_102() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_102 = "Test prompt number 102";
        let response_102 = "Test response number 102";

        cache.set(prompt_102, response_102, 1020);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_102);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_102);
        assert_eq!(cost_saved, (1020 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_103() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_103 = "Test prompt number 103";
        let response_103 = "Test response number 103";

        cache.set(prompt_103, response_103, 1030);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_103);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_103);
        assert_eq!(cost_saved, (1030 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_104() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_104 = "Test prompt number 104";
        let response_104 = "Test response number 104";

        cache.set(prompt_104, response_104, 1040);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_104);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_104);
        assert_eq!(cost_saved, (1040 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_105() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_105 = "Test prompt number 105";
        let response_105 = "Test response number 105";

        cache.set(prompt_105, response_105, 1050);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_105);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_105);
        assert_eq!(cost_saved, (1050 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_106() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_106 = "Test prompt number 106";
        let response_106 = "Test response number 106";

        cache.set(prompt_106, response_106, 1060);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_106);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_106);
        assert_eq!(cost_saved, (1060 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_107() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_107 = "Test prompt number 107";
        let response_107 = "Test response number 107";

        cache.set(prompt_107, response_107, 1070);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_107);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_107);
        assert_eq!(cost_saved, (1070 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_108() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_108 = "Test prompt number 108";
        let response_108 = "Test response number 108";

        cache.set(prompt_108, response_108, 1080);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_108);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_108);
        assert_eq!(cost_saved, (1080 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_109() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_109 = "Test prompt number 109";
        let response_109 = "Test response number 109";

        cache.set(prompt_109, response_109, 1090);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_109);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_109);
        assert_eq!(cost_saved, (1090 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_110() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_110 = "Test prompt number 110";
        let response_110 = "Test response number 110";

        cache.set(prompt_110, response_110, 1100);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_110);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_110);
        assert_eq!(cost_saved, (1100 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_111() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_111 = "Test prompt number 111";
        let response_111 = "Test response number 111";

        cache.set(prompt_111, response_111, 1110);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_111);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_111);
        assert_eq!(cost_saved, (1110 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_112() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_112 = "Test prompt number 112";
        let response_112 = "Test response number 112";

        cache.set(prompt_112, response_112, 1120);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_112);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_112);
        assert_eq!(cost_saved, (1120 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_113() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_113 = "Test prompt number 113";
        let response_113 = "Test response number 113";

        cache.set(prompt_113, response_113, 1130);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_113);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_113);
        assert_eq!(cost_saved, (1130 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_114() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_114 = "Test prompt number 114";
        let response_114 = "Test response number 114";

        cache.set(prompt_114, response_114, 1140);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_114);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_114);
        assert_eq!(cost_saved, (1140 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_115() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_115 = "Test prompt number 115";
        let response_115 = "Test response number 115";

        cache.set(prompt_115, response_115, 1150);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_115);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_115);
        assert_eq!(cost_saved, (1150 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_116() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_116 = "Test prompt number 116";
        let response_116 = "Test response number 116";

        cache.set(prompt_116, response_116, 1160);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_116);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_116);
        assert_eq!(cost_saved, (1160 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_117() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_117 = "Test prompt number 117";
        let response_117 = "Test response number 117";

        cache.set(prompt_117, response_117, 1170);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_117);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_117);
        assert_eq!(cost_saved, (1170 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_118() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_118 = "Test prompt number 118";
        let response_118 = "Test response number 118";

        cache.set(prompt_118, response_118, 1180);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_118);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_118);
        assert_eq!(cost_saved, (1180 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_119() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_119 = "Test prompt number 119";
        let response_119 = "Test response number 119";

        cache.set(prompt_119, response_119, 1190);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_119);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_119);
        assert_eq!(cost_saved, (1190 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_120() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_120 = "Test prompt number 120";
        let response_120 = "Test response number 120";

        cache.set(prompt_120, response_120, 1200);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_120);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_120);
        assert_eq!(cost_saved, (1200 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_121() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_121 = "Test prompt number 121";
        let response_121 = "Test response number 121";

        cache.set(prompt_121, response_121, 1210);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_121);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_121);
        assert_eq!(cost_saved, (1210 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_122() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_122 = "Test prompt number 122";
        let response_122 = "Test response number 122";

        cache.set(prompt_122, response_122, 1220);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_122);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_122);
        assert_eq!(cost_saved, (1220 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_123() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_123 = "Test prompt number 123";
        let response_123 = "Test response number 123";

        cache.set(prompt_123, response_123, 1230);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_123);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_123);
        assert_eq!(cost_saved, (1230 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_124() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_124 = "Test prompt number 124";
        let response_124 = "Test response number 124";

        cache.set(prompt_124, response_124, 1240);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_124);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_124);
        assert_eq!(cost_saved, (1240 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_125() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_125 = "Test prompt number 125";
        let response_125 = "Test response number 125";

        cache.set(prompt_125, response_125, 1250);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_125);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_125);
        assert_eq!(cost_saved, (1250 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_126() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_126 = "Test prompt number 126";
        let response_126 = "Test response number 126";

        cache.set(prompt_126, response_126, 1260);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_126);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_126);
        assert_eq!(cost_saved, (1260 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_127() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_127 = "Test prompt number 127";
        let response_127 = "Test response number 127";

        cache.set(prompt_127, response_127, 1270);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_127);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_127);
        assert_eq!(cost_saved, (1270 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_128() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_128 = "Test prompt number 128";
        let response_128 = "Test response number 128";

        cache.set(prompt_128, response_128, 1280);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_128);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_128);
        assert_eq!(cost_saved, (1280 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_129() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_129 = "Test prompt number 129";
        let response_129 = "Test response number 129";

        cache.set(prompt_129, response_129, 1290);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_129);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_129);
        assert_eq!(cost_saved, (1290 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_130() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_130 = "Test prompt number 130";
        let response_130 = "Test response number 130";

        cache.set(prompt_130, response_130, 1300);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_130);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_130);
        assert_eq!(cost_saved, (1300 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_131() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_131 = "Test prompt number 131";
        let response_131 = "Test response number 131";

        cache.set(prompt_131, response_131, 1310);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_131);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_131);
        assert_eq!(cost_saved, (1310 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_132() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_132 = "Test prompt number 132";
        let response_132 = "Test response number 132";

        cache.set(prompt_132, response_132, 1320);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_132);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_132);
        assert_eq!(cost_saved, (1320 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_133() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_133 = "Test prompt number 133";
        let response_133 = "Test response number 133";

        cache.set(prompt_133, response_133, 1330);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_133);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_133);
        assert_eq!(cost_saved, (1330 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_134() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_134 = "Test prompt number 134";
        let response_134 = "Test response number 134";

        cache.set(prompt_134, response_134, 1340);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_134);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_134);
        assert_eq!(cost_saved, (1340 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_135() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_135 = "Test prompt number 135";
        let response_135 = "Test response number 135";

        cache.set(prompt_135, response_135, 1350);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_135);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_135);
        assert_eq!(cost_saved, (1350 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_136() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_136 = "Test prompt number 136";
        let response_136 = "Test response number 136";

        cache.set(prompt_136, response_136, 1360);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_136);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_136);
        assert_eq!(cost_saved, (1360 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_137() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_137 = "Test prompt number 137";
        let response_137 = "Test response number 137";

        cache.set(prompt_137, response_137, 1370);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_137);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_137);
        assert_eq!(cost_saved, (1370 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_138() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_138 = "Test prompt number 138";
        let response_138 = "Test response number 138";

        cache.set(prompt_138, response_138, 1380);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_138);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_138);
        assert_eq!(cost_saved, (1380 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_139() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_139 = "Test prompt number 139";
        let response_139 = "Test response number 139";

        cache.set(prompt_139, response_139, 1390);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_139);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_139);
        assert_eq!(cost_saved, (1390 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_140() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_140 = "Test prompt number 140";
        let response_140 = "Test response number 140";

        cache.set(prompt_140, response_140, 1400);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_140);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_140);
        assert_eq!(cost_saved, (1400 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_141() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_141 = "Test prompt number 141";
        let response_141 = "Test response number 141";

        cache.set(prompt_141, response_141, 1410);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_141);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_141);
        assert_eq!(cost_saved, (1410 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_142() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_142 = "Test prompt number 142";
        let response_142 = "Test response number 142";

        cache.set(prompt_142, response_142, 1420);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_142);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_142);
        assert_eq!(cost_saved, (1420 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_143() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_143 = "Test prompt number 143";
        let response_143 = "Test response number 143";

        cache.set(prompt_143, response_143, 1430);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_143);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_143);
        assert_eq!(cost_saved, (1430 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_144() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_144 = "Test prompt number 144";
        let response_144 = "Test response number 144";

        cache.set(prompt_144, response_144, 1440);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_144);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_144);
        assert_eq!(cost_saved, (1440 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_145() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_145 = "Test prompt number 145";
        let response_145 = "Test response number 145";

        cache.set(prompt_145, response_145, 1450);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_145);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_145);
        assert_eq!(cost_saved, (1450 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_146() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_146 = "Test prompt number 146";
        let response_146 = "Test response number 146";

        cache.set(prompt_146, response_146, 1460);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_146);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_146);
        assert_eq!(cost_saved, (1460 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_147() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_147 = "Test prompt number 147";
        let response_147 = "Test response number 147";

        cache.set(prompt_147, response_147, 1470);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_147);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_147);
        assert_eq!(cost_saved, (1470 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_148() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_148 = "Test prompt number 148";
        let response_148 = "Test response number 148";

        cache.set(prompt_148, response_148, 1480);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_148);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_148);
        assert_eq!(cost_saved, (1480 as f64 * 0.0001).round() as i64);
    }

    #[test]
    fn test_prompt_cache_variant_149() {
        let cache = PromptCache::new(Duration::from_secs(10));
        let prompt_149 = "Test prompt number 149";
        let response_149 = "Test response number 149";

        cache.set(prompt_149, response_149, 1490);

        let (cached_resp, cost_saved) = cache.get_with_cost_cents(prompt_149);
        assert!(cached_resp.is_some());
        assert_eq!(cached_resp.unwrap().text, response_149);
        assert_eq!(cost_saved, (1490 as f64 * 0.0001).round() as i64);
    }
}
