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

    #[test]
    fn test_referral_tracker_edge_case_0() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0e899ff6");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0e899ff6");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0e899ff6"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_956f7476"));
        assert_eq!(tracker.get_user_referrals("user_test_0e899ff6"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_956f7476").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_1() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0df17a8a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0df17a8a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0df17a8a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_d3a45b69"));
        assert_eq!(tracker.get_user_referrals("user_test_0df17a8a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_d3a45b69").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_2() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ef24b74c");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ef24b74c");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ef24b74c"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c7c34b7c"));
        assert_eq!(tracker.get_user_referrals("user_test_ef24b74c"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c7c34b7c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_3() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_cf5912ee");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_cf5912ee");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_cf5912ee"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_5bb71937"));
        assert_eq!(tracker.get_user_referrals("user_test_cf5912ee"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_5bb71937").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_4() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_43e72967");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_43e72967");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_43e72967"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_01a76804"));
        assert_eq!(tracker.get_user_referrals("user_test_43e72967"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_01a76804").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_5() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_979c9b46");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_979c9b46");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_979c9b46"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_01d415a3"));
        assert_eq!(tracker.get_user_referrals("user_test_979c9b46"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_01d415a3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_6() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_2649fafc");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_2649fafc");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_2649fafc"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_94cc2067"));
        assert_eq!(tracker.get_user_referrals("user_test_2649fafc"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_94cc2067").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_7() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_367ef25b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_367ef25b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_367ef25b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bb8f2208"));
        assert_eq!(tracker.get_user_referrals("user_test_367ef25b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bb8f2208").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_8() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_821290be");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_821290be");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_821290be"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a9b746b4"));
        assert_eq!(tracker.get_user_referrals("user_test_821290be"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a9b746b4").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_9() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_573bd900");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_573bd900");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_573bd900"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7a00f9f2"));
        assert_eq!(tracker.get_user_referrals("user_test_573bd900"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7a00f9f2").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_10() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_297470b3");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_297470b3");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_297470b3"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a2e11a42"));
        assert_eq!(tracker.get_user_referrals("user_test_297470b3"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a2e11a42").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_11() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_5b5417ff");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_5b5417ff");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_5b5417ff"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_2a80b08a"));
        assert_eq!(tracker.get_user_referrals("user_test_5b5417ff"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_2a80b08a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_12() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_5632b402");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_5632b402");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_5632b402"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_28ba5e4e"));
        assert_eq!(tracker.get_user_referrals("user_test_5632b402"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_28ba5e4e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_13() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_14bfc237");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_14bfc237");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_14bfc237"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b5445ec1"));
        assert_eq!(tracker.get_user_referrals("user_test_14bfc237"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b5445ec1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_14() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d78d227b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d78d227b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d78d227b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_840d68c7"));
        assert_eq!(tracker.get_user_referrals("user_test_d78d227b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_840d68c7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_15() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7c984b93");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7c984b93");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7c984b93"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_502f541e"));
        assert_eq!(tracker.get_user_referrals("user_test_7c984b93"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_502f541e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_16() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_4784d4f3");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_4784d4f3");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_4784d4f3"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c1f963c5"));
        assert_eq!(tracker.get_user_referrals("user_test_4784d4f3"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c1f963c5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_17() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_2cc238f2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_2cc238f2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_2cc238f2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c9469bbd"));
        assert_eq!(tracker.get_user_referrals("user_test_2cc238f2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c9469bbd").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_18() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f84870d5");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f84870d5");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f84870d5"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_fa2cdd2f"));
        assert_eq!(tracker.get_user_referrals("user_test_f84870d5"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_fa2cdd2f").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_19() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_1fe7a9d2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_1fe7a9d2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_1fe7a9d2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bb82c0fe"));
        assert_eq!(tracker.get_user_referrals("user_test_1fe7a9d2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bb82c0fe").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_20() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_4028c46d");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_4028c46d");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_4028c46d"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_974c56df"));
        assert_eq!(tracker.get_user_referrals("user_test_4028c46d"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_974c56df").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_21() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_b993964e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_b993964e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_b993964e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_0acb07a8"));
        assert_eq!(tracker.get_user_referrals("user_test_b993964e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_0acb07a8").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_22() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_6541132d");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_6541132d");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_6541132d"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c4f32482"));
        assert_eq!(tracker.get_user_referrals("user_test_6541132d"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c4f32482").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_23() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_41feeb24");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_41feeb24");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_41feeb24"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a5ad8ff4"));
        assert_eq!(tracker.get_user_referrals("user_test_41feeb24"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a5ad8ff4").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_24() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e6b853ab");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e6b853ab");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e6b853ab"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_fb591335"));
        assert_eq!(tracker.get_user_referrals("user_test_e6b853ab"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_fb591335").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_25() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e378b6e8");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e378b6e8");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e378b6e8"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e63e412e"));
        assert_eq!(tracker.get_user_referrals("user_test_e378b6e8"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e63e412e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_26() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_8148de48");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_8148de48");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_8148de48"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_137c0beb"));
        assert_eq!(tracker.get_user_referrals("user_test_8148de48"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_137c0beb").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_27() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bf69f153");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bf69f153");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bf69f153"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_44130789"));
        assert_eq!(tracker.get_user_referrals("user_test_bf69f153"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_44130789").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_28() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_987e6f20");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_987e6f20");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_987e6f20"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_8353c5c2"));
        assert_eq!(tracker.get_user_referrals("user_test_987e6f20"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_8353c5c2").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_29() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_04cc3f47");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_04cc3f47");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_04cc3f47"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_178b242b"));
        assert_eq!(tracker.get_user_referrals("user_test_04cc3f47"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_178b242b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_30() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_11c1e430");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_11c1e430");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_11c1e430"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bbe0e26a"));
        assert_eq!(tracker.get_user_referrals("user_test_11c1e430"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bbe0e26a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_31() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ed0ebf0f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ed0ebf0f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ed0ebf0f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ade233f3"));
        assert_eq!(tracker.get_user_referrals("user_test_ed0ebf0f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ade233f3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_32() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f4836fc1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f4836fc1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f4836fc1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7977b7d8"));
        assert_eq!(tracker.get_user_referrals("user_test_f4836fc1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7977b7d8").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_33() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_13a8037e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_13a8037e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_13a8037e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e57d37bd"));
        assert_eq!(tracker.get_user_referrals("user_test_13a8037e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e57d37bd").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_34() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_14a65da1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_14a65da1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_14a65da1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_6f77c16d"));
        assert_eq!(tracker.get_user_referrals("user_test_14a65da1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_6f77c16d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_35() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_32bb58f6");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_32bb58f6");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_32bb58f6"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ddc346f5"));
        assert_eq!(tracker.get_user_referrals("user_test_32bb58f6"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ddc346f5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_36() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_933569e9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_933569e9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_933569e9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_cd407cec"));
        assert_eq!(tracker.get_user_referrals("user_test_933569e9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_cd407cec").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_37() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_c3d6c33f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_c3d6c33f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_c3d6c33f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_9ef9e11a"));
        assert_eq!(tracker.get_user_referrals("user_test_c3d6c33f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_9ef9e11a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_38() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d54ef0a8");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d54ef0a8");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d54ef0a8"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_903b0a47"));
        assert_eq!(tracker.get_user_referrals("user_test_d54ef0a8"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_903b0a47").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_39() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_1bf2cc63");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_1bf2cc63");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_1bf2cc63"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_375b8e85"));
        assert_eq!(tracker.get_user_referrals("user_test_1bf2cc63"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_375b8e85").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_40() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ea0d1e9e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ea0d1e9e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ea0d1e9e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_49071f3d"));
        assert_eq!(tracker.get_user_referrals("user_test_ea0d1e9e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_49071f3d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_41() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0792b401");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0792b401");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0792b401"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_97ec0cfb"));
        assert_eq!(tracker.get_user_referrals("user_test_0792b401"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_97ec0cfb").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_42() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_3920198c");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_3920198c");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_3920198c"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7977dcb4"));
        assert_eq!(tracker.get_user_referrals("user_test_3920198c"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7977dcb4").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_43() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_535333db");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_535333db");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_535333db"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_96358498"));
        assert_eq!(tracker.get_user_referrals("user_test_535333db"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_96358498").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_44() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_dc3a4176");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_dc3a4176");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_dc3a4176"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_784f2788"));
        assert_eq!(tracker.get_user_referrals("user_test_dc3a4176"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_784f2788").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_45() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0e0be52a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0e0be52a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0e0be52a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c1edc9a1"));
        assert_eq!(tracker.get_user_referrals("user_test_0e0be52a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c1edc9a1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_46() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_04910ef3");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_04910ef3");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_04910ef3"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_da1635c7"));
        assert_eq!(tracker.get_user_referrals("user_test_04910ef3"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_da1635c7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_47() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a5fa7ae8");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a5fa7ae8");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a5fa7ae8"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4a13b3b3"));
        assert_eq!(tracker.get_user_referrals("user_test_a5fa7ae8"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4a13b3b3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_48() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_75642038");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_75642038");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_75642038"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ce34a0c6"));
        assert_eq!(tracker.get_user_referrals("user_test_75642038"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ce34a0c6").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_49() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_123d04a3");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_123d04a3");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_123d04a3"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_756cb857"));
        assert_eq!(tracker.get_user_referrals("user_test_123d04a3"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_756cb857").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_50() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a4b6caa0");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a4b6caa0");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a4b6caa0"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_0d3c5db0"));
        assert_eq!(tracker.get_user_referrals("user_test_a4b6caa0"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_0d3c5db0").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_51() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_391c2c76");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_391c2c76");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_391c2c76"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_48dd2593"));
        assert_eq!(tracker.get_user_referrals("user_test_391c2c76"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_48dd2593").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_52() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ac35aa45");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ac35aa45");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ac35aa45"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_447a175e"));
        assert_eq!(tracker.get_user_referrals("user_test_ac35aa45"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_447a175e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_53() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_6984562f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_6984562f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_6984562f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a280bc13"));
        assert_eq!(tracker.get_user_referrals("user_test_6984562f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a280bc13").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_54() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e84bd0c3");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e84bd0c3");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e84bd0c3"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f571b36e"));
        assert_eq!(tracker.get_user_referrals("user_test_e84bd0c3"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f571b36e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_55() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_13da707e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_13da707e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_13da707e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b6d0875e"));
        assert_eq!(tracker.get_user_referrals("user_test_13da707e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b6d0875e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_56() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_30c46d18");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_30c46d18");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_30c46d18"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bd1bd278"));
        assert_eq!(tracker.get_user_referrals("user_test_30c46d18"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bd1bd278").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_57() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_00121472");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_00121472");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_00121472"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_810eed6d"));
        assert_eq!(tracker.get_user_referrals("user_test_00121472"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_810eed6d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_58() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_c58f9c74");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_c58f9c74");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_c58f9c74"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7faf53d0"));
        assert_eq!(tracker.get_user_referrals("user_test_c58f9c74"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7faf53d0").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_59() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_1829b409");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_1829b409");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_1829b409"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_94f19d41"));
        assert_eq!(tracker.get_user_referrals("user_test_1829b409"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_94f19d41").unwrap(), 1);
    }

    #[test]
    fn test_referral_tracker_edge_case_0() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_43347b4e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_43347b4e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_43347b4e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_69ef52f1"));
        assert_eq!(tracker.get_user_referrals("user_test_43347b4e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_69ef52f1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_1() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_228cad4f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_228cad4f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_228cad4f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_75f802d5"));
        assert_eq!(tracker.get_user_referrals("user_test_228cad4f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_75f802d5").unwrap(), 1);
    }

    #[test]
    fn test_referral_tracker_edge_case_0() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_91fc6f4d");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_91fc6f4d");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_91fc6f4d"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_98d4a25b"));
        assert_eq!(tracker.get_user_referrals("user_test_91fc6f4d"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_98d4a25b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_1() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_26105b55");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_26105b55");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_26105b55"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a9c3be82"));
        assert_eq!(tracker.get_user_referrals("user_test_26105b55"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a9c3be82").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_2() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_2595345c");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_2595345c");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_2595345c"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_3fd6b157"));
        assert_eq!(tracker.get_user_referrals("user_test_2595345c"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_3fd6b157").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_3() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_3fb916bb");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_3fb916bb");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_3fb916bb"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_81d71d6a"));
        assert_eq!(tracker.get_user_referrals("user_test_3fb916bb"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_81d71d6a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_4() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e7bb4408");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e7bb4408");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e7bb4408"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_600e9996"));
        assert_eq!(tracker.get_user_referrals("user_test_e7bb4408"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_600e9996").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_5() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7e7b3cd7");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7e7b3cd7");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7e7b3cd7"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_79897095"));
        assert_eq!(tracker.get_user_referrals("user_test_7e7b3cd7"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_79897095").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_6() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_371466ea");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_371466ea");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_371466ea"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_70358232"));
        assert_eq!(tracker.get_user_referrals("user_test_371466ea"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_70358232").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_7() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_35f96613");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_35f96613");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_35f96613"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_5bf65350"));
        assert_eq!(tracker.get_user_referrals("user_test_35f96613"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_5bf65350").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_8() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_5e4aaec0");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_5e4aaec0");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_5e4aaec0"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_950412b7"));
        assert_eq!(tracker.get_user_referrals("user_test_5e4aaec0"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_950412b7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_9() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e974f871");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e974f871");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e974f871"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_20f03019"));
        assert_eq!(tracker.get_user_referrals("user_test_e974f871"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_20f03019").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_10() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ed641053");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ed641053");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ed641053"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e6b6dccd"));
        assert_eq!(tracker.get_user_referrals("user_test_ed641053"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e6b6dccd").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_11() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_02486a75");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_02486a75");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_02486a75"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f5f37f41"));
        assert_eq!(tracker.get_user_referrals("user_test_02486a75"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f5f37f41").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_12() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_557f1c46");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_557f1c46");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_557f1c46"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_aa19b156"));
        assert_eq!(tracker.get_user_referrals("user_test_557f1c46"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_aa19b156").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_13() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ef03d489");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ef03d489");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ef03d489"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_d4813797"));
        assert_eq!(tracker.get_user_referrals("user_test_ef03d489"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_d4813797").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_14() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_6d21bd08");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_6d21bd08");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_6d21bd08"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_82f251dd"));
        assert_eq!(tracker.get_user_referrals("user_test_6d21bd08"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_82f251dd").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_15() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_c4ce39a1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_c4ce39a1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_c4ce39a1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_d5845814"));
        assert_eq!(tracker.get_user_referrals("user_test_c4ce39a1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_d5845814").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_16() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_3406a684");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_3406a684");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_3406a684"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c83a09c6"));
        assert_eq!(tracker.get_user_referrals("user_test_3406a684"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c83a09c6").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_17() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_cd2681f1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_cd2681f1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_cd2681f1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_57eef1c5"));
        assert_eq!(tracker.get_user_referrals("user_test_cd2681f1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_57eef1c5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_18() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_fc3e5ac5");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_fc3e5ac5");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_fc3e5ac5"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f4197f90"));
        assert_eq!(tracker.get_user_referrals("user_test_fc3e5ac5"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f4197f90").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_19() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_93a53f85");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_93a53f85");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_93a53f85"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4a1078a2"));
        assert_eq!(tracker.get_user_referrals("user_test_93a53f85"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4a1078a2").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_20() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7c3b5dc9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7c3b5dc9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7c3b5dc9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_65ec4a98"));
        assert_eq!(tracker.get_user_referrals("user_test_7c3b5dc9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_65ec4a98").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_21() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d7b7b372");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d7b7b372");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d7b7b372"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_096493ed"));
        assert_eq!(tracker.get_user_referrals("user_test_d7b7b372"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_096493ed").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_22() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_57e1e22a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_57e1e22a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_57e1e22a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f856471d"));
        assert_eq!(tracker.get_user_referrals("user_test_57e1e22a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f856471d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_23() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e1cf806b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e1cf806b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e1cf806b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b8687d52"));
        assert_eq!(tracker.get_user_referrals("user_test_e1cf806b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b8687d52").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_24() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a85978d8");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a85978d8");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a85978d8"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_dcda014d"));
        assert_eq!(tracker.get_user_referrals("user_test_a85978d8"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_dcda014d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_25() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_1efbb6a9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_1efbb6a9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_1efbb6a9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_32e450fb"));
        assert_eq!(tracker.get_user_referrals("user_test_1efbb6a9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_32e450fb").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_26() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_98bc1d04");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_98bc1d04");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_98bc1d04"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_77906825"));
        assert_eq!(tracker.get_user_referrals("user_test_98bc1d04"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_77906825").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_27() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_74f26b76");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_74f26b76");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_74f26b76"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_39679289"));
        assert_eq!(tracker.get_user_referrals("user_test_74f26b76"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_39679289").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_28() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_42637e17");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_42637e17");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_42637e17"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_633e13f0"));
        assert_eq!(tracker.get_user_referrals("user_test_42637e17"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_633e13f0").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_29() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f408041d");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f408041d");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f408041d"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f6afab63"));
        assert_eq!(tracker.get_user_referrals("user_test_f408041d"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f6afab63").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_30() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_08532400");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_08532400");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_08532400"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_2fd319be"));
        assert_eq!(tracker.get_user_referrals("user_test_08532400"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_2fd319be").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_31() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bc339c4d");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bc339c4d");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bc339c4d"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_251667db"));
        assert_eq!(tracker.get_user_referrals("user_test_bc339c4d"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_251667db").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_32() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_38bbeb75");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_38bbeb75");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_38bbeb75"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_9ef27f1a"));
        assert_eq!(tracker.get_user_referrals("user_test_38bbeb75"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_9ef27f1a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_33() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_27e86e1a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_27e86e1a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_27e86e1a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4371c8e7"));
        assert_eq!(tracker.get_user_referrals("user_test_27e86e1a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4371c8e7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_34() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_74dac593");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_74dac593");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_74dac593"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ad2ef132"));
        assert_eq!(tracker.get_user_referrals("user_test_74dac593"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ad2ef132").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_35() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_8d5051a4");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_8d5051a4");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_8d5051a4"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7d6cee1b"));
        assert_eq!(tracker.get_user_referrals("user_test_8d5051a4"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7d6cee1b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_36() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_608febb0");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_608febb0");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_608febb0"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_8e7145d1"));
        assert_eq!(tracker.get_user_referrals("user_test_608febb0"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_8e7145d1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_37() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_3ef13830");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_3ef13830");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_3ef13830"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_3a9a6b17"));
        assert_eq!(tracker.get_user_referrals("user_test_3ef13830"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_3a9a6b17").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_38() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d5d8ee9e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d5d8ee9e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d5d8ee9e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ca9a2adc"));
        assert_eq!(tracker.get_user_referrals("user_test_d5d8ee9e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ca9a2adc").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_39() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_1a5264ce");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_1a5264ce");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_1a5264ce"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c678359b"));
        assert_eq!(tracker.get_user_referrals("user_test_1a5264ce"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c678359b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_40() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7b3f838e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7b3f838e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7b3f838e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_eb713117"));
        assert_eq!(tracker.get_user_referrals("user_test_7b3f838e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_eb713117").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_41() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d841eb95");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d841eb95");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d841eb95"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bcfbbf2c"));
        assert_eq!(tracker.get_user_referrals("user_test_d841eb95"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bcfbbf2c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_42() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_32aca9ab");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_32aca9ab");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_32aca9ab"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_39aa140c"));
        assert_eq!(tracker.get_user_referrals("user_test_32aca9ab"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_39aa140c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_43() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_c942c028");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_c942c028");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_c942c028"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_3ca774aa"));
        assert_eq!(tracker.get_user_referrals("user_test_c942c028"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_3ca774aa").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_44() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_50c90fb2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_50c90fb2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_50c90fb2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_446f17a7"));
        assert_eq!(tracker.get_user_referrals("user_test_50c90fb2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_446f17a7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_45() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_88b87f54");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_88b87f54");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_88b87f54"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f7b9d53e"));
        assert_eq!(tracker.get_user_referrals("user_test_88b87f54"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f7b9d53e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_46() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_27d4f5bc");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_27d4f5bc");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_27d4f5bc"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_34e30744"));
        assert_eq!(tracker.get_user_referrals("user_test_27d4f5bc"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_34e30744").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_47() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f59735c8");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f59735c8");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f59735c8"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_fe02ec77"));
        assert_eq!(tracker.get_user_referrals("user_test_f59735c8"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_fe02ec77").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_48() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_8c904aa2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_8c904aa2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_8c904aa2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_5fb5e222"));
        assert_eq!(tracker.get_user_referrals("user_test_8c904aa2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_5fb5e222").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_49() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_20c438ce");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_20c438ce");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_20c438ce"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_9bfacc10"));
        assert_eq!(tracker.get_user_referrals("user_test_20c438ce"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_9bfacc10").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_50() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d73061c1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d73061c1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d73061c1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_16f76ad6"));
        assert_eq!(tracker.get_user_referrals("user_test_d73061c1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_16f76ad6").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_51() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_680877ac");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_680877ac");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_680877ac"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_61578406"));
        assert_eq!(tracker.get_user_referrals("user_test_680877ac"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_61578406").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_52() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_11ac8686");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_11ac8686");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_11ac8686"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_255f0591"));
        assert_eq!(tracker.get_user_referrals("user_test_11ac8686"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_255f0591").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_53() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0543a7e1");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0543a7e1");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0543a7e1"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_fe2c4fa6"));
        assert_eq!(tracker.get_user_referrals("user_test_0543a7e1"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_fe2c4fa6").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_54() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_667765ed");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_667765ed");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_667765ed"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_0a9663ce"));
        assert_eq!(tracker.get_user_referrals("user_test_667765ed"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_0a9663ce").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_55() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_2a8b6803");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_2a8b6803");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_2a8b6803"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7c60d974"));
        assert_eq!(tracker.get_user_referrals("user_test_2a8b6803"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7c60d974").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_56() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_6e40b22f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_6e40b22f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_6e40b22f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_53ab6451"));
        assert_eq!(tracker.get_user_referrals("user_test_6e40b22f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_53ab6451").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_57() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_79a8f6ad");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_79a8f6ad");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_79a8f6ad"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_8c825c3c"));
        assert_eq!(tracker.get_user_referrals("user_test_79a8f6ad"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_8c825c3c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_58() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_93a4e0ea");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_93a4e0ea");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_93a4e0ea"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_d5a64d0e"));
        assert_eq!(tracker.get_user_referrals("user_test_93a4e0ea"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_d5a64d0e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_59() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a7c47063");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a7c47063");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a7c47063"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e2269459"));
        assert_eq!(tracker.get_user_referrals("user_test_a7c47063"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e2269459").unwrap(), 1);
    }

    #[test]
    fn test_referral_tracker_edge_case_0() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bc4969fa");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bc4969fa");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bc4969fa"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_48d7a4e3"));
        assert_eq!(tracker.get_user_referrals("user_test_bc4969fa"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_48d7a4e3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_1() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_6e2654b2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_6e2654b2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_6e2654b2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a9612a12"));
        assert_eq!(tracker.get_user_referrals("user_test_6e2654b2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a9612a12").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_2() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_fe7ac698");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_fe7ac698");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_fe7ac698"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_1188729b"));
        assert_eq!(tracker.get_user_referrals("user_test_fe7ac698"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_1188729b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_3() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_64a84585");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_64a84585");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_64a84585"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c8bc6f73"));
        assert_eq!(tracker.get_user_referrals("user_test_64a84585"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c8bc6f73").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_4() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a19f0dd9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a19f0dd9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a19f0dd9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ee425a6f"));
        assert_eq!(tracker.get_user_referrals("user_test_a19f0dd9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ee425a6f").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_5() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_5cc8df32");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_5cc8df32");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_5cc8df32"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_ad42625c"));
        assert_eq!(tracker.get_user_referrals("user_test_5cc8df32"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_ad42625c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_6() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_24e8125b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_24e8125b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_24e8125b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_6344a872"));
        assert_eq!(tracker.get_user_referrals("user_test_24e8125b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_6344a872").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_7() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_b092b184");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_b092b184");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_b092b184"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_224c86e6"));
        assert_eq!(tracker.get_user_referrals("user_test_b092b184"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_224c86e6").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_8() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_12b1c3c5");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_12b1c3c5");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_12b1c3c5"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_9b2c717c"));
        assert_eq!(tracker.get_user_referrals("user_test_12b1c3c5"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_9b2c717c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_9() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f82a907a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f82a907a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f82a907a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_93ab2a5b"));
        assert_eq!(tracker.get_user_referrals("user_test_f82a907a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_93ab2a5b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_10() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a03a6b4b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a03a6b4b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a03a6b4b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_8c1c69a1"));
        assert_eq!(tracker.get_user_referrals("user_test_a03a6b4b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_8c1c69a1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_11() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_240f8eed");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_240f8eed");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_240f8eed"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7f834ce5"));
        assert_eq!(tracker.get_user_referrals("user_test_240f8eed"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7f834ce5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_12() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_63171b0f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_63171b0f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_63171b0f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b4844a61"));
        assert_eq!(tracker.get_user_referrals("user_test_63171b0f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b4844a61").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_13() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_20db2a63");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_20db2a63");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_20db2a63"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_91f51eb4"));
        assert_eq!(tracker.get_user_referrals("user_test_20db2a63"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_91f51eb4").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_14() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7bfa2ebe");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7bfa2ebe");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7bfa2ebe"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_6dc154cf"));
        assert_eq!(tracker.get_user_referrals("user_test_7bfa2ebe"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_6dc154cf").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_15() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_89c3328a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_89c3328a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_89c3328a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a19d5097"));
        assert_eq!(tracker.get_user_referrals("user_test_89c3328a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a19d5097").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_16() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e1ffcccf");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e1ffcccf");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e1ffcccf"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a327d1e7"));
        assert_eq!(tracker.get_user_referrals("user_test_e1ffcccf"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a327d1e7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_17() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a8b9c1e9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a8b9c1e9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a8b9c1e9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_423e80a7"));
        assert_eq!(tracker.get_user_referrals("user_test_a8b9c1e9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_423e80a7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_18() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bea74124");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bea74124");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bea74124"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_2920c503"));
        assert_eq!(tracker.get_user_referrals("user_test_bea74124"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_2920c503").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_19() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_ebb87bfe");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_ebb87bfe");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_ebb87bfe"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_543265fb"));
        assert_eq!(tracker.get_user_referrals("user_test_ebb87bfe"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_543265fb").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_20() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f46735f9");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f46735f9");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f46735f9"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b4a4f3e3"));
        assert_eq!(tracker.get_user_referrals("user_test_f46735f9"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b4a4f3e3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_21() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_c6ba44f2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_c6ba44f2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_c6ba44f2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7714902d"));
        assert_eq!(tracker.get_user_referrals("user_test_c6ba44f2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7714902d").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_22() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_99d862ff");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_99d862ff");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_99d862ff"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_c1bcd86e"));
        assert_eq!(tracker.get_user_referrals("user_test_99d862ff"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_c1bcd86e").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_23() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_e284addf");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_e284addf");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_e284addf"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_2db650df"));
        assert_eq!(tracker.get_user_referrals("user_test_e284addf"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_2db650df").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_24() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_48861918");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_48861918");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_48861918"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_aa1ec995"));
        assert_eq!(tracker.get_user_referrals("user_test_48861918"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_aa1ec995").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_25() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a0a06e60");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a0a06e60");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a0a06e60"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4f502ce1"));
        assert_eq!(tracker.get_user_referrals("user_test_a0a06e60"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4f502ce1").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_26() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_8f79c226");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_8f79c226");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_8f79c226"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_9d07162b"));
        assert_eq!(tracker.get_user_referrals("user_test_8f79c226"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_9d07162b").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_27() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_85b12513");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_85b12513");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_85b12513"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_3e656cff"));
        assert_eq!(tracker.get_user_referrals("user_test_85b12513"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_3e656cff").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_28() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_0f4e585e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_0f4e585e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_0f4e585e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_1277ad78"));
        assert_eq!(tracker.get_user_referrals("user_test_0f4e585e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_1277ad78").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_29() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_190c5f2b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_190c5f2b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_190c5f2b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_405dab86"));
        assert_eq!(tracker.get_user_referrals("user_test_190c5f2b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_405dab86").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_30() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_37cf5fdc");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_37cf5fdc");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_37cf5fdc"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a1f0eab5"));
        assert_eq!(tracker.get_user_referrals("user_test_37cf5fdc"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a1f0eab5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_31() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bc5a5884");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bc5a5884");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bc5a5884"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_f2c4d71f"));
        assert_eq!(tracker.get_user_referrals("user_test_bc5a5884"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_f2c4d71f").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_32() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_70684aae");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_70684aae");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_70684aae"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b8ac24ab"));
        assert_eq!(tracker.get_user_referrals("user_test_70684aae"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b8ac24ab").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_33() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_001c252e");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_001c252e");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_001c252e"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_797a01b7"));
        assert_eq!(tracker.get_user_referrals("user_test_001c252e"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_797a01b7").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_34() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f1b33894");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f1b33894");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f1b33894"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_d5a368e3"));
        assert_eq!(tracker.get_user_referrals("user_test_f1b33894"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_d5a368e3").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_35() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_4be32fa2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_4be32fa2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_4be32fa2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_785f01e5"));
        assert_eq!(tracker.get_user_referrals("user_test_4be32fa2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_785f01e5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_36() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_58f41256");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_58f41256");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_58f41256"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_fc7ae869"));
        assert_eq!(tracker.get_user_referrals("user_test_58f41256"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_fc7ae869").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_37() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_90034418");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_90034418");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_90034418"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_6becbdcc"));
        assert_eq!(tracker.get_user_referrals("user_test_90034418"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_6becbdcc").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_38() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_db8f8467");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_db8f8467");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_db8f8467"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_45085c89"));
        assert_eq!(tracker.get_user_referrals("user_test_db8f8467"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_45085c89").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_39() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_72ecbd43");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_72ecbd43");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_72ecbd43"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_7d22ed86"));
        assert_eq!(tracker.get_user_referrals("user_test_72ecbd43"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_7d22ed86").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_40() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_f0cc3f70");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_f0cc3f70");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_f0cc3f70"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bf8ac3ca"));
        assert_eq!(tracker.get_user_referrals("user_test_f0cc3f70"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bf8ac3ca").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_41() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_3473e933");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_3473e933");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_3473e933"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b711ebd8"));
        assert_eq!(tracker.get_user_referrals("user_test_3473e933"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b711ebd8").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_42() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_7394af52");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_7394af52");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_7394af52"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_42b77c96"));
        assert_eq!(tracker.get_user_referrals("user_test_7394af52"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_42b77c96").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_43() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bd2c899f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bd2c899f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bd2c899f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_31d9e1ac"));
        assert_eq!(tracker.get_user_referrals("user_test_bd2c899f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_31d9e1ac").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_44() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_2f556143");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_2f556143");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_2f556143"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4d645bea"));
        assert_eq!(tracker.get_user_referrals("user_test_2f556143"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4d645bea").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_45() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_07671e5a");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_07671e5a");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_07671e5a"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_32e28942"));
        assert_eq!(tracker.get_user_referrals("user_test_07671e5a"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_32e28942").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_46() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_bfb69228");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_bfb69228");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_bfb69228"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e9e42026"));
        assert_eq!(tracker.get_user_referrals("user_test_bfb69228"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e9e42026").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_47() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_49765a09");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_49765a09");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_49765a09"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_4af0d8c2"));
        assert_eq!(tracker.get_user_referrals("user_test_49765a09"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_4af0d8c2").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_48() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_13a7e3e2");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_13a7e3e2");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_13a7e3e2"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_05de7f80"));
        assert_eq!(tracker.get_user_referrals("user_test_13a7e3e2"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_05de7f80").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_49() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_81ef9508");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_81ef9508");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_81ef9508"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_aa9cfcff"));
        assert_eq!(tracker.get_user_referrals("user_test_81ef9508"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_aa9cfcff").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_50() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a6dd7391");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a6dd7391");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a6dd7391"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_249d6aff"));
        assert_eq!(tracker.get_user_referrals("user_test_a6dd7391"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_249d6aff").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_51() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_a991115f");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_a991115f");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_a991115f"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_bcc5fecd"));
        assert_eq!(tracker.get_user_referrals("user_test_a991115f"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_bcc5fecd").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_52() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_4c2e0dcd");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_4c2e0dcd");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_4c2e0dcd"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_73c83d09"));
        assert_eq!(tracker.get_user_referrals("user_test_4c2e0dcd"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_73c83d09").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_53() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_693f9fda");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_693f9fda");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_693f9fda"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_e8d3097c"));
        assert_eq!(tracker.get_user_referrals("user_test_693f9fda"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_e8d3097c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_54() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_35d31675");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_35d31675");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_35d31675"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_566940e5"));
        assert_eq!(tracker.get_user_referrals("user_test_35d31675"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_566940e5").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_55() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_4f4ca549");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_4f4ca549");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_4f4ca549"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_1bf8b059"));
        assert_eq!(tracker.get_user_referrals("user_test_4f4ca549"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_1bf8b059").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_56() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_96fa857b");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_96fa857b");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_96fa857b"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_77a7cf4c"));
        assert_eq!(tracker.get_user_referrals("user_test_96fa857b"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_77a7cf4c").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_57() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_08b0b783");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_08b0b783");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_08b0b783"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_056a4ddb"));
        assert_eq!(tracker.get_user_referrals("user_test_08b0b783"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_056a4ddb").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_58() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_68e246c7");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_68e246c7");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_68e246c7"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_a5b5af4a"));
        assert_eq!(tracker.get_user_referrals("user_test_68e246c7"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_a5b5af4a").unwrap(), 1);
    }


    #[test]
    fn test_referral_tracker_edge_case_59() {
        let tracker = ReferralTracker::new();
        let code = tracker.generate_referral_code("user_test_d2dd5f63");
        assert_eq!(code.len(), 8);

        let code2 = tracker.generate_referral_code("user_test_d2dd5f63");
        assert_eq!(code, code2);

        assert!(tracker.record_referral(&code));
        assert_eq!(tracker.get_user_referrals("user_test_d2dd5f63"), 1);
        assert_eq!(tracker.get_total_referrals(), 1);

        assert!(tracker.record_referral_with_channel(&code, "channel_b1d1a315"));
        assert_eq!(tracker.get_user_referrals("user_test_d2dd5f63"), 2);

        let stats = tracker.get_channel_stats();
        assert_eq!(*stats.get("channel_b1d1a315").unwrap(), 1);
    }
