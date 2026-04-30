use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq)]
pub enum PlanTier {
    Free,
    Starter,
    Pro,
    Business,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuotaResult {
    pub allowed: bool,
    pub soft_limited: bool,
    pub message: Option<String>,
}

#[async_trait::async_trait]
pub trait QuotaStore: Send + Sync {
    async fn set_tier(&self, tenant_id: &str, tier: &str) -> Result<(), String>;
    async fn get_tier(&self, tenant_id: &str) -> Result<String, String>;
    async fn increment_ai_action(&self, tenant_id: &str) -> Result<u64, String>;
    async fn get_storage_usage(&self, tenant_id: &str) -> Result<u64, String>;
    async fn add_storage_usage(&self, tenant_id: &str, bytes: u64) -> Result<(), String>;
}

pub struct DbQuotaStore {
    pool: sqlx::PgPool,
}

impl DbQuotaStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl QuotaStore for DbQuotaStore {
    async fn set_tier(&self, tenant_id: &str, tier: &str) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO tenant_quotas (tenant_id, tier) VALUES ($1, $2)
             ON CONFLICT (tenant_id) DO UPDATE SET tier = $2"
        )
        .bind(tenant_id)
        .bind(tier)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_tier(&self, tenant_id: &str) -> Result<String, String> {
        let row = sqlx::query("SELECT tier FROM tenant_quotas WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        use sqlx::Row;
        if let Some(r) = row {
            Ok(r.get("tier"))
        } else {
            Ok("free".to_string())
        }
    }

    async fn increment_ai_action(&self, tenant_id: &str) -> Result<u64, String> {
        let row = sqlx::query(
            "INSERT INTO tenant_quotas (tenant_id, tier, ai_action_usage)
             VALUES ($1, 'free', 1)
             ON CONFLICT (tenant_id) DO UPDATE SET ai_action_usage = tenant_quotas.ai_action_usage + 1
             RETURNING ai_action_usage"
        )
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        use sqlx::Row;
        let count: i64 = row.get("ai_action_usage");
        Ok(count as u64)
    }

    async fn get_storage_usage(&self, tenant_id: &str) -> Result<u64, String> {
        let row = sqlx::query("SELECT storage_usage_bytes FROM tenant_quotas WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        use sqlx::Row;
        if let Some(r) = row {
            let val: i64 = r.get("storage_usage_bytes");
            Ok(val as u64)
        } else {
            Ok(0)
        }
    }

    async fn add_storage_usage(&self, tenant_id: &str, bytes: u64) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO tenant_quotas (tenant_id, tier, storage_usage_bytes)
             VALUES ($1, 'free', $2)
             ON CONFLICT (tenant_id) DO UPDATE SET storage_usage_bytes = tenant_quotas.storage_usage_bytes + $2"
        )
        .bind(tenant_id)
        .bind(bytes as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct TenantQuotaManager {
    store: Box<dyn QuotaStore>,
}

impl TenantQuotaManager {
    pub fn new(store: Box<dyn QuotaStore>) -> Self {
        Self { store }
    }

    pub async fn set_tier(&self, tenant_id: &str, tier: PlanTier) -> Result<(), String> {
        let tier_str = match tier {
            PlanTier::Free => "free",
            PlanTier::Starter => "starter",
            PlanTier::Pro => "pro",
            PlanTier::Business => "business",
        };
        self.store.set_tier(tenant_id, tier_str).await
    }

    pub async fn get_tier(&self, tenant_id: &str) -> Result<PlanTier, String> {
        let tier_str = self.store.get_tier(tenant_id).await?;
        match tier_str.as_str() {
            "starter" => Ok(PlanTier::Starter),
            "pro" => Ok(PlanTier::Pro),
            "business" => Ok(PlanTier::Business),
            _ => Ok(PlanTier::Free),
        }
    }

    pub fn get_ai_limit(&self, tier: &PlanTier) -> Option<u64> {
        match tier {
            PlanTier::Free => Some(100),
            PlanTier::Starter => Some(1000),
            PlanTier::Pro | PlanTier::Business => None,
        }
    }

    pub fn get_storage_limit_bytes(&self, tier: &PlanTier) -> Option<u64> {
        match tier {
            PlanTier::Free => Some(500 * 1024 * 1024), // 500 MB
            PlanTier::Starter => Some(5 * 1024 * 1024 * 1024), // 5 GB
            PlanTier::Pro => Some(50 * 1024 * 1024 * 1024), // 50 GB
            PlanTier::Business => None,
        }
    }

    pub async fn record_ai_action(&self, tenant_id: &str) -> Result<QuotaResult, String> {
        let tier = self.get_tier(tenant_id).await?;
        let limit = self.get_ai_limit(&tier);

        let count = self.store.increment_ai_action(tenant_id).await?;

        if let Some(limit) = limit {
            if count > limit {
                return Ok(QuotaResult {
                    allowed: true, // soft limits
                    soft_limited: true,
                    message: Some(format!("You've reached the AI action limit for the {:?} tier ({} actions). We are still processing your request, but please upgrade your plan soon.", tier, limit)),
                });
            }
        }

        Ok(QuotaResult {
            allowed: true,
            soft_limited: false,
            message: None,
        })
    }

    pub async fn record_storage_upload(&self, tenant_id: &str, bytes: u64) -> Result<QuotaResult, String> {
        let tier = self.get_tier(tenant_id).await?;
        let limit = self.get_storage_limit_bytes(&tier);

        let current = self.store.get_storage_usage(tenant_id).await?;

        if let Some(limit) = limit {
            if current + bytes > limit {
                return Ok(QuotaResult {
                    allowed: false, // hard limits
                    soft_limited: false,
                    message: Some(format!("Storage limit exceeded for {:?} tier. Please upgrade to upload more files.", tier)),
                });
            }
        }

        self.store.add_storage_usage(tenant_id, bytes).await?;

        Ok(QuotaResult {
            allowed: true,
            soft_limited: false,
            message: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockQuotaStore {
        tiers: RwLock<HashMap<String, String>>,
        ai_usage: RwLock<HashMap<String, u64>>,
        storage_usage: RwLock<HashMap<String, u64>>,
    }

    impl MockQuotaStore {
        fn new() -> Self {
            Self {
                tiers: RwLock::new(HashMap::new()),
                ai_usage: RwLock::new(HashMap::new()),
                storage_usage: RwLock::new(HashMap::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl QuotaStore for MockQuotaStore {
        async fn set_tier(&self, tenant_id: &str, tier: &str) -> Result<(), String> {
            self.tiers.write().unwrap().insert(tenant_id.to_string(), tier.to_string());
            Ok(())
        }

        async fn get_tier(&self, tenant_id: &str) -> Result<String, String> {
            Ok(self.tiers.read().unwrap().get(tenant_id).unwrap_or(&"free".to_string()).clone())
        }

        async fn increment_ai_action(&self, tenant_id: &str) -> Result<u64, String> {
            let mut usage = self.ai_usage.write().unwrap();
            let count = usage.entry(tenant_id.to_string()).or_insert(0);
            *count += 1;
            Ok(*count)
        }

        async fn get_storage_usage(&self, tenant_id: &str) -> Result<u64, String> {
            let usage = self.storage_usage.read().unwrap();
            Ok(*usage.get(tenant_id).unwrap_or(&0))
        }

        async fn add_storage_usage(&self, tenant_id: &str, bytes: u64) -> Result<(), String> {
            let mut usage = self.storage_usage.write().unwrap();
            let count = usage.entry(tenant_id.to_string()).or_insert(0);
            *count += bytes;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_ai_action_soft_limit() {
        let store = Box::new(MockQuotaStore::new());
        let manager = TenantQuotaManager::new(store);

        manager.set_tier("tenant_free", PlanTier::Free).await.unwrap();

        // 100 limit for free tier
        for _ in 0..100 {
            let res = manager.record_ai_action("tenant_free").await.unwrap();
            assert!(res.allowed);
            assert!(!res.soft_limited);
        }

        let res = manager.record_ai_action("tenant_free").await.unwrap();
        assert!(res.allowed);
        assert!(res.soft_limited);
        assert!(res.message.unwrap().contains("Free"));

        manager.set_tier("tenant_pro", PlanTier::Pro).await.unwrap();
        for _ in 0..1500 {
            let res = manager.record_ai_action("tenant_pro").await.unwrap();
            assert!(res.allowed);
            assert!(!res.soft_limited);
        }
    }

    #[tokio::test]
    async fn test_storage_hard_limit() {
        let store = Box::new(MockQuotaStore::new());
        let manager = TenantQuotaManager::new(store);

        manager.set_tier("tenant_starter", PlanTier::Starter).await.unwrap();

        let res = manager.record_storage_upload("tenant_starter", 4 * 1024 * 1024 * 1024).await.unwrap();
        assert!(res.allowed);
        assert!(!res.soft_limited);

        let res = manager.record_storage_upload("tenant_starter", 2 * 1024 * 1024 * 1024).await.unwrap();
        assert!(!res.allowed);
        assert!(!res.soft_limited);
        assert!(res.message.unwrap().contains("Starter"));

        manager.set_tier("tenant_biz", PlanTier::Business).await.unwrap();
        let res = manager.record_storage_upload("tenant_biz", 100 * 1024 * 1024 * 1024).await.unwrap(); // 100GB
        assert!(res.allowed);
    }
}
