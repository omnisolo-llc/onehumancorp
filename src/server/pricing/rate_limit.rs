use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum Tier {
    Free,
    Starter,
    Pro,
}

impl Tier {
    pub fn action_limit(&self) -> Option<u32> {
        match self {
            Tier::Free => Some(100),
            Tier::Starter => Some(1000),
            Tier::Pro => None, // Unlimited
        }
    }
}

pub struct RateLimitManager {
    tenant_usage: Mutex<HashMap<String, TenantUsage>>,
}

struct TenantUsage {
    tier: Tier,
    actions_used: u32,
    window_start: Instant,
}

impl RateLimitManager {
    pub fn new() -> Self {
        RateLimitManager {
            tenant_usage: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_tenant_tier(&self, tenant_id: &str, tier: Tier) {
        let mut usage = self.tenant_usage.lock().unwrap();
        let entry = usage.entry(tenant_id.to_string()).or_insert_with(|| TenantUsage {
            tier: tier.clone(),
            actions_used: 0,
            window_start: Instant::now(),
        });
        entry.tier = tier;
    }

    pub fn record_action(&self, tenant_id: &str) -> Result<bool, String> {
        let mut usage = self.tenant_usage.lock().unwrap();
        let mut entry = usage.entry(tenant_id.to_string()).or_insert_with(|| TenantUsage {
            tier: Tier::Free,
            actions_used: 0,
            window_start: Instant::now(),
        });

        // Reset window if more than 30 days have passed (simplified month calculation)
        if entry.window_start.elapsed() > Duration::from_secs(30 * 24 * 60 * 60) {
            entry.actions_used = 0;
            entry.window_start = Instant::now();
        }

        if let Some(limit) = entry.tier.action_limit() {
            if entry.actions_used >= limit {
                return Err(format!("Friendly upgrade prompt: You have reached your {} tier limit of {} actions. Please upgrade your plan to continue.", match entry.tier { Tier::Free => "Free", Tier::Starter => "Starter", _ => "" }, limit));
            }
        }

        entry.actions_used += 1;
        Ok(true)
    }

    pub fn get_usage(&self, tenant_id: &str) -> (u32, Option<u32>) {
        let usage = self.tenant_usage.lock().unwrap();
        if let Some(entry) = usage.get(tenant_id) {
            (entry.actions_used, entry.tier.action_limit())
        } else {
            (0, Tier::Free.action_limit())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_free_tier() {
        let manager = RateLimitManager::new();
        manager.set_tenant_tier("tenant1", Tier::Free);

        for _ in 0..100 {
            assert!(manager.record_action("tenant1").is_ok());
        }

        let err = manager.record_action("tenant1").unwrap_err();
        assert!(err.contains("Friendly upgrade prompt"));
    }

    #[test]
    fn test_rate_limit_starter_tier() {
        let manager = RateLimitManager::new();
        manager.set_tenant_tier("tenant2", Tier::Starter);

        for _ in 0..1000 {
            assert!(manager.record_action("tenant2").is_ok());
        }

        let err = manager.record_action("tenant2").unwrap_err();
        assert!(err.contains("Friendly upgrade prompt"));
    }

    #[test]
    fn test_rate_limit_pro_tier() {
        let manager = RateLimitManager::new();
        manager.set_tenant_tier("tenant3", Tier::Pro);

        for _ in 0..1500 {
            assert!(manager.record_action("tenant3").is_ok());
        }
    }
}
