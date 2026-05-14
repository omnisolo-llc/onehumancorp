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

    #[tokio::test]
    async fn test_storage_quota_matrix() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let test_cases = vec![
                    // (tier, current_used, delta, expect_allowed, expect_soft_limit)
                    (PlanTier::Free, 0, 104857600),
                    (PlanTier::Starter, 1048576, 104857600),
                    (PlanTier::Pro, 2097152, 104857600),
                    (PlanTier::Business, 3145728, 104857600),
                    (PlanTier::Free, 4194304, 104857600),
                    (PlanTier::Starter, 5242880, 104857600),
                    (PlanTier::Pro, 6291456, 104857600),
                    (PlanTier::Business, 7340032, 104857600),
                    (PlanTier::Free, 8388608, 104857600),
                    (PlanTier::Starter, 9437184, 104857600),
                    (PlanTier::Pro, 10485760, 104857600),
                    (PlanTier::Business, 11534336, 104857600),
                    (PlanTier::Free, 12582912, 104857600),
                    (PlanTier::Starter, 13631488, 104857600),
                    (PlanTier::Pro, 14680064, 104857600),
                    (PlanTier::Business, 15728640, 104857600),
                    (PlanTier::Free, 16777216, 104857600),
                    (PlanTier::Starter, 17825792, 104857600),
                    (PlanTier::Pro, 18874368, 104857600),
                    (PlanTier::Business, 19922944, 104857600),
                    (PlanTier::Free, 20971520, 104857600),
                    (PlanTier::Starter, 22020096, 104857600),
                    (PlanTier::Pro, 23068672, 104857600),
                    (PlanTier::Business, 24117248, 104857600),
                    (PlanTier::Free, 25165824, 104857600),
                    (PlanTier::Starter, 26214400, 104857600),
                    (PlanTier::Pro, 27262976, 104857600),
                    (PlanTier::Business, 28311552, 104857600),
                    (PlanTier::Free, 29360128, 104857600),
                    (PlanTier::Starter, 30408704, 104857600),
                    (PlanTier::Pro, 31457280, 104857600),
                    (PlanTier::Business, 32505856, 104857600),
                    (PlanTier::Free, 33554432, 104857600),
                    (PlanTier::Starter, 34603008, 104857600),
                    (PlanTier::Pro, 35651584, 104857600),
                    (PlanTier::Business, 36700160, 104857600),
                    (PlanTier::Free, 37748736, 104857600),
                    (PlanTier::Starter, 38797312, 104857600),
                    (PlanTier::Pro, 39845888, 104857600),
                    (PlanTier::Business, 40894464, 104857600),
                    (PlanTier::Free, 41943040, 104857600),
                    (PlanTier::Starter, 42991616, 104857600),
                    (PlanTier::Pro, 44040192, 104857600),
                    (PlanTier::Business, 45088768, 104857600),
                    (PlanTier::Free, 46137344, 104857600),
                    (PlanTier::Starter, 47185920, 104857600),
                    (PlanTier::Pro, 48234496, 104857600),
                    (PlanTier::Business, 49283072, 104857600),
                    (PlanTier::Free, 50331648, 104857600),
                    (PlanTier::Starter, 51380224, 104857600),
                    (PlanTier::Pro, 52428800, 104857600),
                    (PlanTier::Business, 53477376, 104857600),
                    (PlanTier::Free, 54525952, 104857600),
                    (PlanTier::Starter, 55574528, 104857600),
                    (PlanTier::Pro, 56623104, 104857600),
                    (PlanTier::Business, 57671680, 104857600),
                    (PlanTier::Free, 58720256, 104857600),
                    (PlanTier::Starter, 59768832, 104857600),
                    (PlanTier::Pro, 60817408, 104857600),
                    (PlanTier::Business, 61865984, 104857600),
                    (PlanTier::Free, 62914560, 104857600),
                    (PlanTier::Starter, 63963136, 104857600),
                    (PlanTier::Pro, 65011712, 104857600),
                    (PlanTier::Business, 66060288, 104857600),
                    (PlanTier::Free, 67108864, 104857600),
                    (PlanTier::Starter, 68157440, 104857600),
                    (PlanTier::Pro, 69206016, 104857600),
                    (PlanTier::Business, 70254592, 104857600),
                    (PlanTier::Free, 71303168, 104857600),
                    (PlanTier::Starter, 72351744, 104857600),
                    (PlanTier::Pro, 73400320, 104857600),
                    (PlanTier::Business, 74448896, 104857600),
                    (PlanTier::Free, 75497472, 104857600),
                    (PlanTier::Starter, 76546048, 104857600),
                    (PlanTier::Pro, 77594624, 104857600),
                    (PlanTier::Business, 78643200, 104857600),
                    (PlanTier::Free, 79691776, 104857600),
                    (PlanTier::Starter, 80740352, 104857600),
                    (PlanTier::Pro, 81788928, 104857600),
                    (PlanTier::Business, 82837504, 104857600),
                    (PlanTier::Free, 83886080, 104857600),
                    (PlanTier::Starter, 84934656, 104857600),
                    (PlanTier::Pro, 85983232, 104857600),
                    (PlanTier::Business, 87031808, 104857600),
                    (PlanTier::Free, 88080384, 104857600),
                    (PlanTier::Starter, 89128960, 104857600),
                    (PlanTier::Pro, 90177536, 104857600),
                    (PlanTier::Business, 91226112, 104857600),
                    (PlanTier::Free, 92274688, 104857600),
                    (PlanTier::Starter, 93323264, 104857600),
                    (PlanTier::Pro, 94371840, 104857600),
                    (PlanTier::Business, 95420416, 104857600),
                    (PlanTier::Free, 96468992, 104857600),
                    (PlanTier::Starter, 97517568, 104857600),
                    (PlanTier::Pro, 98566144, 104857600),
                    (PlanTier::Business, 99614720, 104857600),
                    (PlanTier::Free, 100663296, 104857600),
                    (PlanTier::Starter, 101711872, 104857600),
                    (PlanTier::Pro, 102760448, 104857600),
                    (PlanTier::Business, 103809024, 104857600),
                ];

                for (i, (tier, used, delta)) in test_cases.into_iter().enumerate() {
                    let tenant_id = format!("test-matrix-{}", i);
                    let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                    let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);
                    let _ : () = redis::AsyncCommands::del(&mut conn, &storage_key).await.unwrap_or(());

                    limiter.set_tenant_tier(&tenant_id, tier.clone()).await.unwrap();
                    if used > 0 {
                        let _ : () = redis::AsyncCommands::set(&mut conn, &storage_key, used).await.unwrap_or(());
                    }

                    let status = limiter.check_storage_quota(&tenant_id, delta).await.unwrap();

                    let limit_bytes = tier.storage_limit_mb().map(|mb| (mb as i64) * 1024 * 1024);
                    let expect_soft_limit = limit_bytes.map_or(false, |l| used + delta > l);

                    assert!(status.is_allowed);
                    assert_eq!(status.soft_limit_reached, expect_soft_limit);
                }
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
