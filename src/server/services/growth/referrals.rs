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


// Functional extensions for referral tiers

pub struct AdvancedReferralTier_0 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_0 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_1 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_1 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_2 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_2 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_3 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_3 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_4 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_4 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_5 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_5 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_6 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_6 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_7 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_7 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_8 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_8 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_9 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_9 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_10 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_10 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_11 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_11 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_12 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_12 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_13 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_13 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_14 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_14 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_15 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_15 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_16 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_16 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_17 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_17 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_18 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_18 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_19 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_19 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_20 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_20 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_21 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_21 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_22 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_22 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_23 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_23 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}

pub struct AdvancedReferralTier_24 {
    pub minimum_referrals: i32,
    pub discount_percentage: f64,
    pub bonus_credits: i32,
    pub tier_name: String,
}

impl AdvancedReferralTier_24 {
    pub fn new(min: i32) -> Self {
        Self {
            minimum_referrals: min,
            discount_percentage: 0.15 + (min as f64 * 0.001),
            bonus_credits: min * 10,
            tier_name: format!("Tier-{}", min),
        }
    }

    pub fn qualifies(&self, current: i32) -> bool {
        current >= self.minimum_referrals
    }

    pub fn calculate_discount(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percentage)
    }

    pub fn apply_bonus(&self, mut current_credits: i32) -> i32 {
        current_credits += self.bonus_credits;
        current_credits
    }
}
