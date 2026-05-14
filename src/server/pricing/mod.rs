pub use ::server_harness as harness;

pub mod budget;
pub mod cache;
pub mod calculator;
pub mod compression;
pub mod prompt_caching;
pub mod rate_limit;

pub struct PricingAnalytics {
    pub total_savings_cents: std::sync::atomic::AtomicI64,
    pub total_requests: std::sync::atomic::AtomicU64,
}

impl PricingAnalytics {
    pub fn new() -> Self {
        Self {
            total_savings_cents: std::sync::atomic::AtomicI64::new(0),
            total_requests: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn record_request(&self, savings_cents: i64) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.total_savings_cents.fetch_add(savings_cents, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> (u64, i64) {
        (
            self.total_requests.load(std::sync::atomic::Ordering::Relaxed),
            self.total_savings_cents.load(std::sync::atomic::Ordering::Relaxed)
        )
    }
}

pub struct BillingEngine {
    pub subscription_active: bool,
    pub next_billing_date: chrono::DateTime<chrono::Utc>,
    pub cached_savings_total: f64,
}

impl BillingEngine {
    pub fn new() -> Self {
        Self {
            subscription_active: true,
            next_billing_date: chrono::Utc::now() + chrono::Duration::days(30),
            cached_savings_total: 0.0,
        }
    }

    pub fn apply_discount_code(&mut self, code: &str) -> Result<f64, String> {
        match code {
            "WELCOME20" => Ok(0.20),
            "YEARLY_SUB" => Ok(0.15),
            "BETA_TESTER" => Ok(0.50),
            _ => Err("Invalid discount code".to_string()),
        }
    }

    pub fn calculate_prorated_upgrade(&self, current_tier_cost: f64, next_tier_cost: f64, days_used: i64) -> f64 {
        let daily_current_rate = current_tier_cost / 30.0;
        let daily_next_rate = next_tier_cost / 30.0;

        let days_remaining = 30 - days_used;
        if days_remaining <= 0 {
            return next_tier_cost;
        }

        let unused_credit = daily_current_rate * days_remaining as f64;
        let new_charge = daily_next_rate * days_remaining as f64;

        let due_now = new_charge - unused_credit;
        if due_now < 0.0 {
            0.0
        } else {
            (due_now * 100.0).round() / 100.0
        }
    }
}

#[cfg(test)]
mod billing_engine_tests {
    use super::*;

    #[test]
    fn test_discount_codes() {
        let mut engine = BillingEngine::new();
        assert_eq!(engine.apply_discount_code("WELCOME20").unwrap(), 0.20);
        assert!(engine.apply_discount_code("FAKE").is_err());
    }

    #[test]
    fn test_proration() {
        let engine = BillingEngine::new();
        // Upgrade from $10 to $20 after 15 days
        // Unused credit: $5. New charge for remaining 15 days: $10
        // Due now: $5
        let due = engine.calculate_prorated_upgrade(10.0, 20.0, 15);
        assert_eq!(due, 5.00);

        // Upgrade after 29 days
        let due_late = engine.calculate_prorated_upgrade(10.0, 20.0, 29);
        // remaining = 1. Credit = 0.333. New = 0.666. Due = 0.33
        assert_eq!(due_late, 0.33);
    }
}
