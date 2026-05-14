pub struct QuotaTracker {
    pub base_quota: i32,
    pub bonus_per_referral: i32,
}

impl QuotaTracker {
    pub fn new(base: i32, bonus: i32) -> Self {
        QuotaTracker {
            base_quota: base,
            bonus_per_referral: bonus,
        }
    }

    pub fn calculate_quota(&self, tier: &str, successful_referrals: i32) -> i32 {
        let tier_base = match tier {
            "starter" => 100,
            "pro" => 1000,
            "business" => 10000,
            _ => 50, // free tier
        };
        tier_base + (successful_referrals * self.bonus_per_referral)
    }

    pub fn check_limit(&self, tier: &str, used: i32, successful_referrals: i32) -> bool {
        let limit = self.calculate_quota(tier, successful_referrals);
        used < limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_tracker() {
        let tracker = QuotaTracker::new(50, 50);

        assert_eq!(tracker.calculate_quota("free", 0), 50);
        assert_eq!(tracker.calculate_quota("free", 2), 150);
    }

    #[test]
    fn test_quota_tracker_check_limit() {
        let tracker = QuotaTracker::new(50, 50);

        // User has used 50, quota is 50 (0 referrals). Over limit (used < limit).
        assert!(!tracker.check_limit("free", 50, 0));

        // User has used 30, quota is 50 (0 referrals). Under limit.
        assert!(tracker.check_limit("free", 30, 0));

        // User has used 150, quota is 150 (2 referrals). Over limit.
        assert!(!tracker.check_limit("free", 150, 2));
    }
}
