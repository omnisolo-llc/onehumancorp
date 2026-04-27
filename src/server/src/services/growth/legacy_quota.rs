use crate::analytics::Tracker;
use crate::services::growth::legacy_repo::ReferralRepository;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct QuotaService {
    tracker: Arc<Tracker>,
    repo: Arc<ReferralRepository>,
    usage: RwLock<HashMap<String, i32>>,
    limit: i32,
}

impl QuotaService {
    pub fn new(tracker: Arc<Tracker>, repo: Arc<ReferralRepository>, limit: i32) -> Self {
        QuotaService {
            tracker,
            repo,
            usage: RwLock::new(HashMap::new()),
            limit,
        }
    }

    pub fn check_quota(&self, tenant_id: &str) -> Result<bool, String> {
        if tenant_id.is_empty() {
            return Err("invalid tenant ID".to_string());
        }

        let usage = {
            let usage_map = self.usage.read().map_err(|e| e.to_string())?;
            *usage_map.get(tenant_id).unwrap_or(&0)
        };

        let mut current_limit = self.limit;
        if let Ok(stats) = self.repo.get_stats(tenant_id) {
             current_limit += (stats.signups * 50) as i32;
        }

        if usage >= current_limit {
            let mut props = HashMap::new();
            props.insert("tenant_id".to_string(), tenant_id.to_string());
            self.tracker.track_event("quota_exceeded", props);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn increment_usage(&self, tenant_id: &str) -> Result<(), String> {
        if tenant_id.is_empty() {
            return Err("invalid tenant ID".to_string());
        }

        let mut usage_map = self.usage.write().map_err(|e| e.to_string())?;
        let count = usage_map.entry(tenant_id.to_string()).or_insert(0);
        *count += 1;

        let mut props = HashMap::new();
        props.insert("tenant_id".to_string(), tenant_id.to_string());
        self.tracker.track_event("quota_usage_incremented", props);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analytics::Tracker;
    use crate::services::growth::legacy_repo::{ReferralRepository, GrowthReferral};
    use std::sync::Arc;

    #[test]
    fn test_quota_service_dynamic_limit() {
        let tracker = Arc::new(Tracker::new());
        let repo = Arc::new(ReferralRepository::new());
        let service = QuotaService::new(tracker, repo.clone(), 2);
        let tenant_id = "tenant-dynamic";

        // Baseline limit is 2. Let's use 2.
        service.increment_usage(tenant_id).unwrap();
        service.increment_usage(tenant_id).unwrap();

        // Now usage is 2, limit is 2 -> exceeded.
        let allowed = service.check_quota(tenant_id).unwrap();
        assert!(!allowed);

        // Add a SIGNED_UP referral for this tenant
        let referral = GrowthReferral {
            id: "ref-dyn-1".to_string(),
            inviter_id: tenant_id.to_string(),
            invitee_email: "dyn@example.com".to_string(),
            status: "SIGNED_UP".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        repo.save_referral(referral).unwrap();

        // Now dynamic limit should be 2 + 50 = 52.
        let allowed = service.check_quota(tenant_id).unwrap();
        assert!(allowed);

        // Let's use up to 51
        for _ in 0..49 {
            service.increment_usage(tenant_id).unwrap();
        }

        let allowed = service.check_quota(tenant_id).unwrap();
        assert!(allowed);

        // Usage 52 -> exceeded
        service.increment_usage(tenant_id).unwrap();
        let allowed = service.check_quota(tenant_id).unwrap();
        assert!(!allowed);
    }
}
