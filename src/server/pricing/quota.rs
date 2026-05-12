use crate::rate_limit::{PlanTier, RateLimitStatus};
use std::collections::HashMap;

pub struct QuotaEnforcer;

impl QuotaEnforcer {
    pub fn check_hard_limit(tier: &PlanTier, current_usage: u32) -> RateLimitStatus {
        let limit = match tier {
            PlanTier::Free => 100,
            PlanTier::Starter => 1000,
            PlanTier::Pro => 10000,
            PlanTier::Business => 100000,
        };

        if current_usage >= limit {
            RateLimitStatus {
                is_allowed: false,
                soft_limit_reached: true,
                user_message: Some(format!("Hard limit reached for your {} plan. Please upgrade to continue.", tier_name(tier))),
            }
        } else {
            RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            }
        }
    }

    pub fn calculate_forecast(usage_history: &[u32]) -> f64 {
        if usage_history.len() < 2 {
            return 0.0;
        }

        let mut growth_rates = Vec::new();
        for i in 1..usage_history.len() {
            let prev = usage_history[i-1] as f64;
            let current = usage_history[i] as f64;
            if prev > 0.0 {
                growth_rates.push(current / prev);
            }
        }

        if growth_rates.is_empty() {
            return *usage_history.last().unwrap_or(&0) as f64;
        }

        let avg_growth: f64 = growth_rates.iter().sum::<f64>() / growth_rates.len() as f64;
        (*usage_history.last().unwrap_or(&0) as f64) * avg_growth
    }
}

fn tier_name(tier: &PlanTier) -> &str {
    match tier {
        PlanTier::Free => "Free",
        PlanTier::Starter => "Starter",
        PlanTier::Pro => "Pro",
        PlanTier::Business => "Business",
    }
}

pub struct MultiTenantQuotaManager {
    quotas: HashMap<String, u32>,
}

impl MultiTenantQuotaManager {
    pub fn new() -> Self {
        Self { quotas: HashMap::new() }
    }

    pub fn record_usage(&mut self, tenant_id: &str, amount: u32) {
        let entry = self.quotas.entry(tenant_id.to_string()).or_insert(0);
        *entry += amount;
    }

    pub fn get_usage(&self, tenant_id: &str) -> u32 {
        *self.quotas.get(tenant_id).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_hard_limit() {
        let status = QuotaEnforcer::check_hard_limit(&PlanTier::Free, 101);
        assert!(!status.is_allowed);
        assert!(status.user_message.unwrap().contains("Hard limit"));
    }

    #[test]
    fn test_usage_forecast() {
        let history = vec![100, 110, 121]; // 10% growth
        let forecast = QuotaEnforcer::calculate_forecast(&history);
        assert!((forecast - 133.1).abs() < 0.1);
    }
}
