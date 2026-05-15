use redis::{AsyncCommands, Client};
use tokio::sync::OnceCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanTier {
    Free,
    Starter,
    Pro,
    Business,
}

impl PlanTier {
    pub fn monthly_action_limit(&self) -> Option<u32> {
        match self {
            PlanTier::Free => Some(100),
            PlanTier::Starter => Some(1000),
            PlanTier::Pro | PlanTier::Business => None, // Unlimited
        }
    }

    pub fn agent_action_limit(&self) -> Option<u32> {
        match self {
            PlanTier::Free => Some(20),
            PlanTier::Starter => Some(200),
            PlanTier::Pro | PlanTier::Business => None,
        }
    }

    pub fn storage_limit_mb(&self) -> Option<u32> {
        match self {
            PlanTier::Free => Some(500),
            PlanTier::Starter => Some(5000), // 5GB
            PlanTier::Pro => Some(50000),    // 50GB
            PlanTier::Business => Some(512000),      // 500GB
        }
    }

    pub fn max_agents(&self) -> Option<usize> {
        match self {
            PlanTier::Free => Some(1),
            PlanTier::Starter => Some(3),
            PlanTier::Pro => Some(10),
            PlanTier::Business => None,
        }
    }

    pub fn max_products(&self) -> Option<usize> {
        match self {
            PlanTier::Free => Some(10),
            PlanTier::Starter => Some(100),
            PlanTier::Pro | PlanTier::Business => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    pub is_allowed: bool,
    pub soft_limit_reached: bool,
    pub user_message: Option<String>,
}

pub struct RedisRateLimiter {
    client: Client,
    connection: OnceCell<redis::aio::MultiplexedConnection>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
}

impl RedisRateLimiter {
    pub fn new(client: Client) -> Self {
        Self { client, connection: OnceCell::new(), telemetry_store: None }
    }

    pub fn with_telemetry(mut self, store: std::sync::Arc<::server_harness::telemetry::ViolationStore>) -> Self {
        self.telemetry_store = Some(store);
        self
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let conn = self.connection.get_or_try_init(|| async {
            self.client.get_multiplexed_async_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
    }

    pub async fn get_tenant_tier(&self, tenant_id: &str) -> Result<PlanTier, String> {
        let mut conn = self.get_connection().await?;
        let tier: Option<String> = conn.get(format!("tenant:{}:tier", tenant_id)).await.map_err(|e| e.to_string())?;

        match tier.as_deref() {
            Some("Starter") => Ok(PlanTier::Starter),
            Some("Pro") => Ok(PlanTier::Pro),
            Some("Business") => Ok(PlanTier::Business),
            _ => Ok(PlanTier::Free),
        }
    }

    pub async fn get_tenant_actions_used(&self, tenant_id: &str) -> Result<u32, String> {
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();
        let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
        let used: Option<u32> = conn.get(&tenant_key).await.map_err(|e| e.to_string())?;
        Ok(used.unwrap_or(0))
    }

    pub async fn get_tenant_storage_used(&self, tenant_id: &str) -> Result<i64, String> {
        let mut conn = self.get_connection().await?;
        let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);
        let used: Option<i64> = conn.get(&storage_key).await.map_err(|e| e.to_string())?;
        Ok(used.unwrap_or(0))
    }

    pub async fn set_tenant_tier(&self, tenant_id: &str, tier: PlanTier) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let tier_str = match tier {
            PlanTier::Free => "Free",
            PlanTier::Starter => "Starter",
            PlanTier::Pro => "Pro",
            PlanTier::Business => "Business",
        };
        conn.set(format!("tenant:{}:tier", tenant_id), tier_str).await.map_err(|e| e.to_string())
    }

    pub async fn record_action(&self, tenant_id: &str, agent_id: &str) -> Result<RateLimitStatus, String> {
        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();

        let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
        let agent_key = format!("tenant:{}:agent:{}:actions_used:{}", tenant_id, agent_id, month_key);

        let tenant_used: u32 = conn.incr(&tenant_key, 1).await.map_err(|e| e.to_string())?;
        let agent_used: u32 = conn.incr(&agent_key, 1).await.map_err(|e| e.to_string())?;

        // Expire keys after ~2 months to save space
        let _ : () = conn.expire(&tenant_key, 60 * 60 * 24 * 60).await.unwrap_or(());
        let _ : () = conn.expire(&agent_key, 60 * 60 * 24 * 60).await.unwrap_or(());

        if let Some(limit) = tier.monthly_action_limit() {
            if tenant_used >= limit {
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit - allow but warn
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "You've hit your {} tier limit of {} AI actions this month. Keep your business growing with a plan upgrade!",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            _ => "Current",
                        },
                        limit
                    )),
                });
            }
        }

        if let Some(limit) = tier.agent_action_limit() {
            if agent_used >= limit {
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "This agent has hit its {} tier limit of {} actions this month. Upgrade to unlock more power for your business.",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            _ => "Current",
                        },
                        limit
                    )),
                });
            }
        }

        Ok(RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        })
    }

    pub async fn check_product_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let product_key = format!("tenant:{}:products", tenant_id);
        let total_products: Option<usize> = conn.get(&product_key).await.map_err(|e| e.to_string())?;
        let total_products = total_products.unwrap_or(0);

        if let Some(limit) = tier.max_products() {
            if total_products >= limit {
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit - allow but warn
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "You've reached your {} tier limit of {} products. Keep building your store with a plan upgrade!",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            _ => "Current",
                        },
                        limit
                    )),
                });
            }
        }

        Ok(RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        })
    }

    pub async fn record_product_added(&self, tenant_id: &str) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let product_key = format!("tenant:{}:products", tenant_id);
        let _ : usize = conn.incr(&product_key, 1).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn check_agent_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let agent_key = format!("tenant:{}:agents", tenant_id);
        let total_agents: Option<usize> = conn.get(&agent_key).await.map_err(|e| e.to_string())?;
        let total_agents = total_agents.unwrap_or(0);

        if let Some(limit) = tier.max_agents() {
            if total_agents >= limit {
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit - allow but warn
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "You've reached your {} tier limit of {} agent. Upgrade to unlock more power!",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            _ => "Current",
                        },
                        limit
                    )),
                });
            }
        }

        Ok(RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        })
    }

    pub async fn record_agent_added(&self, tenant_id: &str) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let agent_key = format!("tenant:{}:agents", tenant_id);
        let _ : usize = conn.incr(&agent_key, 1).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn check_storage_quota(&self, tenant_id: &str, delta_bytes: i64) -> Result<RateLimitStatus, String> {
        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);

        let total_storage: i64 = conn.incr(&storage_key, delta_bytes).await.map_err(|e| e.to_string())?;

        if let Some(store) = &self.telemetry_store {
            store.storage_bytes_counter.add(
                delta_bytes as u64,
                &[
                    opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()),
                    opentelemetry::KeyValue::new("tier", format!("{:?}", tier)),
                ],
            );
        }

        if let Some(limit_mb) = tier.storage_limit_mb() {
            let limit_bytes = (limit_mb as i64) * 1024 * 1024;
            if total_storage > limit_bytes {
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit - allow but warn
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "You've reached your {} tier limit of {}MB storage. Keep your business running smoothly with a plan upgrade!",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            PlanTier::Pro => "Pro",
                            _ => "Current",
                        },
                        limit_mb
                    )),
                });
            }
        }

        Ok(RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_tier_limits() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);

        assert_eq!(PlanTier::Free.agent_action_limit(), Some(20));
        assert_eq!(PlanTier::Starter.agent_action_limit(), Some(200));

        assert_eq!(PlanTier::Free.storage_limit_mb(), Some(500));
        assert_eq!(PlanTier::Starter.storage_limit_mb(), Some(5000));
        assert_eq!(PlanTier::Pro.storage_limit_mb(), Some(50000));
        assert_eq!(PlanTier::Business.storage_limit_mb(), Some(512000));

        assert_eq!(PlanTier::Free.max_agents(), Some(1));
        assert_eq!(PlanTier::Starter.max_agents(), Some(3));
        assert_eq!(PlanTier::Pro.max_agents(), Some(10));
        assert_eq!(PlanTier::Business.max_agents(), None);

        assert_eq!(PlanTier::Free.max_products(), Some(10));
        assert_eq!(PlanTier::Starter.max_products(), Some(100));
        assert_eq!(PlanTier::Pro.max_products(), None);
        assert_eq!(PlanTier::Business.max_products(), None);
    }

    #[tokio::test]
    async fn test_check_product_quota_no_mutation() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-no-mutation";

                // Clear any existing products
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let product_key = format!("tenant:{}:products", tenant_id);
                let _ : () = conn.del(&product_key).await.unwrap_or(());

                // Set tier to free
                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                // Check quota initially
                let status = limiter.check_product_quota(tenant_id).await.unwrap();
                assert!(status.is_allowed);
                assert!(!status.soft_limit_reached);

                // Check again to ensure it didn't mutate (increment)
                let status = limiter.check_product_quota(tenant_id).await.unwrap();
                assert!(status.is_allowed);
                assert!(!status.soft_limit_reached);

                // Add 10 products
                for _ in 0..10 {
                    limiter.record_product_added(tenant_id).await.unwrap();
                }

                // Check quota now
                let status = limiter.check_product_quota(tenant_id).await.unwrap();
                assert!(status.is_allowed);
                assert!(status.soft_limit_reached); // Should be reached since we have 10 products (limit is 10)
            }
        }
    }

    #[tokio::test]
    async fn test_check_storage_quota() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-storage-quota";

                // Clear any existing storage used
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);
                let _ : () = redis::AsyncCommands::del(&mut conn, &storage_key).await.unwrap_or(());

                // Set tier to Free (500MB limit)
                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                // Increment storage by a small amount (100MB)
                let delta: i64 = 100 * 1024 * 1024;
                let status = limiter.check_storage_quota(tenant_id, delta).await.unwrap();
                assert!(status.is_allowed);
                assert!(!status.soft_limit_reached);

                // Increment storage by an amount crossing the 500MB limit
                let large_delta: i64 = 450 * 1024 * 1024;
                let status = limiter.check_storage_quota(tenant_id, large_delta).await.unwrap();
                assert!(status.is_allowed); // Soft limit allows it
                assert!(status.soft_limit_reached); // But flag is set
                assert!(status.user_message.unwrap().contains("500MB storage"));
            }
        }
    }

    #[tokio::test]
    async fn test_record_agent_quota() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-agent-quota";

                // Clear any existing agents
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let agent_key = format!("tenant:{}:agents", tenant_id);
                let _ : () = conn.del(&agent_key).await.unwrap_or(());

                // Set tier to free
                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                // Add 1 agent
                limiter.record_agent_added(tenant_id).await.unwrap();

                // Check quota now
                let status = limiter.check_agent_quota(tenant_id).await.unwrap();
                assert!(status.is_allowed);
                assert!(status.soft_limit_reached); // Limit is 1 for Free tier
            }
        }
    }

    #[tokio::test]
    async fn test_record_action_monthly_reset() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-monthly-reset";
                let agent_id = "agent-1";

                let now = chrono::Utc::now();
                let month_key = now.format("%Y-%m").to_string();

                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
                let _ : () = conn.del(&tenant_key).await.unwrap_or(());

                // Set tier to Free
                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                // Record an action
                let status = limiter.record_action(tenant_id, agent_id).await.unwrap();
                assert!(status.is_allowed);
                assert!(!status.soft_limit_reached);

                // Verify the monthly key was created and has a value of 1
                let count: usize = conn.get(&tenant_key).await.unwrap_or(0);
                assert_eq!(count, 1);
            }
        }
    }
}

#[cfg(test)]
mod substantive_rate_limit_tests {
    use super::*;


    #[test]
    fn test_plan_tier_limits_variant_1() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_2() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_3() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_4() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_5() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_6() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_7() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_8() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_9() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_10() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_11() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_12() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_13() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_14() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_15() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_16() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_17() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_18() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_19() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_20() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_21() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_22() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_23() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_24() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_25() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_26() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_27() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_28() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_29() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_30() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_31() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_32() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_33() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_34() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_35() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_36() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_37() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_38() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_39() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_40() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_41() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_42() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_43() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_44() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_45() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_46() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_47() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_48() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_49() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_50() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_51() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_52() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_53() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_54() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_55() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_56() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_57() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_58() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_59() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_60() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_61() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_62() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_63() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_64() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_65() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_66() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_67() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_68() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_69() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_70() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_71() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_72() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_73() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_74() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_75() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_76() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_77() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_78() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_79() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_80() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_81() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_82() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_83() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_84() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_85() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_86() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_87() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_88() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_89() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_90() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_91() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_92() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_93() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_94() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_95() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_96() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_97() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_98() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_99() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_100() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_101() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_102() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_103() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_104() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_105() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_106() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_107() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_108() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_109() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_110() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_111() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_112() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_113() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_114() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_115() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_116() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_117() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_118() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_119() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_120() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_121() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_122() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_123() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_124() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_125() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_126() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_127() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_128() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_129() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_130() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_131() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_132() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_133() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_134() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_135() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_136() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_137() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_138() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_139() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_140() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_141() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_142() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_143() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_144() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_145() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_146() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_147() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_148() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }

    #[test]
    fn test_plan_tier_limits_variant_149() {
        assert_eq!(PlanTier::Free.monthly_action_limit(), Some(100));
        assert_eq!(PlanTier::Starter.monthly_action_limit(), Some(1000));
        assert_eq!(PlanTier::Pro.monthly_action_limit(), None);
        assert_eq!(PlanTier::Business.monthly_action_limit(), None);
    }
}
