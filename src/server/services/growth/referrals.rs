use std::collections::HashMap;
use std::sync::RwLock;
use rand::RngCore;

pub struct ReferralTracker {
    total_referrals: RwLock<i32>,
    user_referrals: RwLock<HashMap<String, i32>>,
    user_codes: RwLock<HashMap<String, String>>,
    code_to_user: RwLock<HashMap<String, String>>,
    channel_stats: RwLock<HashMap<String, i32>>,
}

impl ReferralTracker {
    pub fn new() -> Self {
        ReferralTracker {
            total_referrals: RwLock::new(0),
            user_referrals: RwLock::new(HashMap::new()),
            user_codes: RwLock::new(HashMap::new()),
            code_to_user: RwLock::new(HashMap::new()),
            channel_stats: RwLock::new(HashMap::new()),
        }
    }

    pub fn generate_referral_code(&self, user_id: &str) -> String {
        let mut user_codes = self.user_codes.write().unwrap();
        if let Some(code) = user_codes.get(user_id) {
            return code.clone();
        }

        let mut rng = rand::thread_rng();
        let bytes: [u8; 4] = rng.next_u32().to_le_bytes();
        let code = hex::encode(bytes);

        user_codes.insert(user_id.to_string(), code.clone());
        let mut code_to_user = self.code_to_user.write().unwrap();
        code_to_user.insert(code.clone(), user_id.to_string());

        code
    }

    pub fn record_referral(&self, code: &str) -> bool {
        let code_to_user = self.code_to_user.read().unwrap();
        if let Some(user_id) = code_to_user.get(code) {
            let mut user_referrals = self.user_referrals.write().unwrap();
            let current = user_referrals.entry(user_id.clone()).or_insert(0);
            *current += 1;

            let mut total_referrals = self.total_referrals.write().unwrap();
            *total_referrals += 1;

            true
        } else {
            false
        }
    }

    pub fn record_referral_with_channel(&self, code: &str, channel: &str) -> bool {
        let code_to_user = self.code_to_user.read().unwrap();
        if let Some(user_id) = code_to_user.get(code) {
            let mut user_referrals = self.user_referrals.write().unwrap();
            let current = user_referrals.entry(user_id.clone()).or_insert(0);
            *current += 1;

            let mut total_referrals = self.total_referrals.write().unwrap();
            *total_referrals += 1;

            if !channel.is_empty() {
                let mut channel_stats = self.channel_stats.write().unwrap();
                let current = channel_stats.entry(channel.to_string()).or_insert(0);
                *current += 1;
            }

            true
        } else {
            false
        }
    }

    pub fn get_user_referrals(&self, user_id: &str) -> i32 {
        let user_referrals = self.user_referrals.read().unwrap();
        *user_referrals.get(user_id).unwrap_or(&0)
    }

    pub fn get_total_referrals(&self) -> i32 {
        let total_referrals = self.total_referrals.read().unwrap();
        *total_referrals
    }

    pub fn get_channel_stats(&self) -> HashMap<String, i32> {
        let channel_stats = self.channel_stats.read().unwrap();
        channel_stats.clone()
    }
}

pub fn calculate_referral_tier(referrals: i32) -> &'static str {
    if referrals >= 50 {
        "Platinum"
    } else if referrals >= 20 {
        "Gold"
    } else if referrals >= 5 {
        "Silver"
    } else {
        "Bronze"
    }
}

pub fn calculate_tier_discount(tier: &str) -> f64 {
    match tier {
        "Platinum" => 0.20,
        "Gold" => 0.10,
        "Silver" => 0.05,
        "Bronze" | _ => 0.00,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referral_tracker() {
        let tracker = ReferralTracker::new();
        
        let code = tracker.generate_referral_code("user1");
        assert_eq!(code.len(), 8); // 4 bytes hex encoded!
        
        // Test idempotency
        let code2 = tracker.generate_referral_code("user1");
        assert_eq!(code, code2);
        
        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);
        
        assert!(!tracker.record_referral("invalid_code"));
        
        assert!(tracker.record_referral_with_channel(&code, "twitter"));
        assert_eq!(tracker.get_user_referrals("user1"), 2);
        
        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("twitter").unwrap(), 1);
    }

    #[test]
    fn test_calculate_referral_tier() {
        assert_eq!(calculate_referral_tier(0), "Bronze");
        assert_eq!(calculate_referral_tier(5), "Silver");
        assert_eq!(calculate_referral_tier(20), "Gold");
        assert_eq!(calculate_referral_tier(50), "Platinum");
    }

    #[test]
    fn test_calculate_tier_discount() {
        assert_eq!(calculate_tier_discount("Platinum"), 0.20);
        assert_eq!(calculate_tier_discount("Gold"), 0.10);
        assert_eq!(calculate_tier_discount("Silver"), 0.05);
        assert_eq!(calculate_tier_discount("Bronze"), 0.00);
    }
}

#[cfg(test)]
mod referral_scenario_tests_1 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_1() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_2 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_2() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_3 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_3() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_4 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_4() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_5 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_5() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_6 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_6() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_7 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_7() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_8 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_8() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_9 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_9() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_10 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_10() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_11 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_11() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_12 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_12() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_13 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_13() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_14 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_14() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_15 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_15() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_16 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_16() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_17 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_17() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_18 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_18() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_19 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_19() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_20 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_20() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_21 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_21() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_22 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_22() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_23 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_23() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_24 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_24() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_25 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_25() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_26 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_26() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_27 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_27() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_28 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_28() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_29 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_29() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_30 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_30() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_31 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_31() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_32 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_32() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_33 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_33() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_34 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_34() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_35 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_35() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_36 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_36() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_37 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_37() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_38 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_38() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}

#[cfg(test)]
mod referral_scenario_tests_39 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_referral_generation_39() {
        let tracker = Arc::new(ReferralTracker::new());
        let mut handles = vec![];

        for j in 0..10 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                let code = tracker_clone.generate_referral_code(&format!("user_concurrent_{}", j));
                tracker_clone.record_referral(&code);
                tracker_clone.record_referral_with_channel(&code, "twitter");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(tracker.get_total_referrals(), 20);
    }
}
