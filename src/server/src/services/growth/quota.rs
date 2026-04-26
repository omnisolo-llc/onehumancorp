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

    pub fn calculate_quota(&self, successful_referrals: i32) -> i32 {
        self.base_quota + (successful_referrals * self.bonus_per_referral)
    }

    pub fn check_limit(&self, used: i32, successful_referrals: i32) -> bool {
        let limit = self.calculate_quota(successful_referrals);
        used < limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_tracker() {
        let tracker = QuotaTracker::new(100, 50);

        assert_eq!(tracker.calculate_quota(0), 100);
        assert_eq!(tracker.calculate_quota(2), 200);
    }

    #[test]
    fn test_quota_tracker_check_limit() {
        let tracker = QuotaTracker::new(100, 50);

        // User has used 50, quota is 100 (0 referrals). Under limit.
        assert!(tracker.check_limit(50, 0));

        // User has used 150, quota is 100 (0 referrals). Over limit.
        assert!(!tracker.check_limit(150, 0));

        // User has used 150, quota is 200 (2 referrals). Under limit.
        assert!(tracker.check_limit(150, 2));
    }
}
