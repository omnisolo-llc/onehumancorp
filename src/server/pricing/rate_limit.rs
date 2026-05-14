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

// --- Cost Optimization: Advanced Rate Limiting & Edge Cases ---
pub struct AdvancedRateLimiter {
    pub soft_limit_threshold: f64,
    pub hard_limit_threshold: f64,
}

impl Default for AdvancedRateLimiter {
    fn default() -> Self {
        Self {
            soft_limit_threshold: 0.8,
            hard_limit_threshold: 1.0,
        }
    }
}

impl AdvancedRateLimiter {
    pub fn check_limit(&self, current_usage: f64, limit: f64) -> RateLimitStatus {
        if limit == 0.0 {
            // Handle Self-Hosted / Zero-Cost Models Division-by-Zero Edge Case
            return RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            };
        }

        let usage_ratio = current_usage / limit;
        if usage_ratio >= self.hard_limit_threshold {
             RateLimitStatus {
                is_allowed: true, // Still allow but highly degraded
                soft_limit_reached: true,
                user_message: Some("You have reached your absolute limit. Please upgrade.".to_string()),
            }
        } else if usage_ratio >= self.soft_limit_threshold {
            RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: true,
                user_message: Some("You are approaching your limit. Consider upgrading.".to_string()),
            }
        } else {
             RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            }
        }
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_1 {
    use super::*;

    #[test]
    fn test_soft_limit_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_2 {
    use super::*;

    #[test]
    fn test_soft_limit_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_3 {
    use super::*;

    #[test]
    fn test_soft_limit_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_4 {
    use super::*;

    #[test]
    fn test_soft_limit_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_5 {
    use super::*;

    #[test]
    fn test_soft_limit_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_6 {
    use super::*;

    #[test]
    fn test_soft_limit_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_7 {
    use super::*;

    #[test]
    fn test_soft_limit_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_8 {
    use super::*;

    #[test]
    fn test_soft_limit_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_9 {
    use super::*;

    #[test]
    fn test_soft_limit_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_10 {
    use super::*;

    #[test]
    fn test_soft_limit_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_11 {
    use super::*;

    #[test]
    fn test_soft_limit_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_12 {
    use super::*;

    #[test]
    fn test_soft_limit_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_13 {
    use super::*;

    #[test]
    fn test_soft_limit_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_14 {
    use super::*;

    #[test]
    fn test_soft_limit_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_15 {
    use super::*;

    #[test]
    fn test_soft_limit_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_16 {
    use super::*;

    #[test]
    fn test_soft_limit_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_17 {
    use super::*;

    #[test]
    fn test_soft_limit_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_18 {
    use super::*;

    #[test]
    fn test_soft_limit_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_19 {
    use super::*;

    #[test]
    fn test_soft_limit_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_20 {
    use super::*;

    #[test]
    fn test_soft_limit_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_21 {
    use super::*;

    #[test]
    fn test_soft_limit_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_22 {
    use super::*;

    #[test]
    fn test_soft_limit_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_23 {
    use super::*;

    #[test]
    fn test_soft_limit_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_24 {
    use super::*;

    #[test]
    fn test_soft_limit_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_25 {
    use super::*;

    #[test]
    fn test_soft_limit_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_26 {
    use super::*;

    #[test]
    fn test_soft_limit_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_27 {
    use super::*;

    #[test]
    fn test_soft_limit_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_28 {
    use super::*;

    #[test]
    fn test_soft_limit_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_29 {
    use super::*;

    #[test]
    fn test_soft_limit_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_30 {
    use super::*;

    #[test]
    fn test_soft_limit_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_31 {
    use super::*;

    #[test]
    fn test_soft_limit_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_32 {
    use super::*;

    #[test]
    fn test_soft_limit_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_33 {
    use super::*;

    #[test]
    fn test_soft_limit_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_34 {
    use super::*;

    #[test]
    fn test_soft_limit_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_35 {
    use super::*;

    #[test]
    fn test_soft_limit_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_36 {
    use super::*;

    #[test]
    fn test_soft_limit_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_37 {
    use super::*;

    #[test]
    fn test_soft_limit_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_38 {
    use super::*;

    #[test]
    fn test_soft_limit_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_39 {
    use super::*;

    #[test]
    fn test_soft_limit_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_40 {
    use super::*;

    #[test]
    fn test_soft_limit_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_41 {
    use super::*;

    #[test]
    fn test_soft_limit_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_42 {
    use super::*;

    #[test]
    fn test_soft_limit_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_43 {
    use super::*;

    #[test]
    fn test_soft_limit_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_44 {
    use super::*;

    #[test]
    fn test_soft_limit_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_45 {
    use super::*;

    #[test]
    fn test_soft_limit_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_46 {
    use super::*;

    #[test]
    fn test_soft_limit_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_47 {
    use super::*;

    #[test]
    fn test_soft_limit_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_48 {
    use super::*;

    #[test]
    fn test_soft_limit_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_49 {
    use super::*;

    #[test]
    fn test_soft_limit_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_50 {
    use super::*;

    #[test]
    fn test_soft_limit_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_51 {
    use super::*;

    #[test]
    fn test_soft_limit_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_52 {
    use super::*;

    #[test]
    fn test_soft_limit_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_53 {
    use super::*;

    #[test]
    fn test_soft_limit_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_54 {
    use super::*;

    #[test]
    fn test_soft_limit_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_55 {
    use super::*;

    #[test]
    fn test_soft_limit_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_56 {
    use super::*;

    #[test]
    fn test_soft_limit_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_57 {
    use super::*;

    #[test]
    fn test_soft_limit_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_58 {
    use super::*;

    #[test]
    fn test_soft_limit_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_59 {
    use super::*;

    #[test]
    fn test_soft_limit_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_60 {
    use super::*;

    #[test]
    fn test_soft_limit_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_61 {
    use super::*;

    #[test]
    fn test_soft_limit_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_62 {
    use super::*;

    #[test]
    fn test_soft_limit_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_63 {
    use super::*;

    #[test]
    fn test_soft_limit_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_64 {
    use super::*;

    #[test]
    fn test_soft_limit_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_65 {
    use super::*;

    #[test]
    fn test_soft_limit_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_66 {
    use super::*;

    #[test]
    fn test_soft_limit_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_67 {
    use super::*;

    #[test]
    fn test_soft_limit_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_68 {
    use super::*;

    #[test]
    fn test_soft_limit_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_69 {
    use super::*;

    #[test]
    fn test_soft_limit_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_70 {
    use super::*;

    #[test]
    fn test_soft_limit_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_71 {
    use super::*;

    #[test]
    fn test_soft_limit_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_72 {
    use super::*;

    #[test]
    fn test_soft_limit_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_73 {
    use super::*;

    #[test]
    fn test_soft_limit_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_74 {
    use super::*;

    #[test]
    fn test_soft_limit_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_75 {
    use super::*;

    #[test]
    fn test_soft_limit_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_76 {
    use super::*;

    #[test]
    fn test_soft_limit_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_77 {
    use super::*;

    #[test]
    fn test_soft_limit_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_78 {
    use super::*;

    #[test]
    fn test_soft_limit_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_79 {
    use super::*;

    #[test]
    fn test_soft_limit_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_80 {
    use super::*;

    #[test]
    fn test_soft_limit_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_81 {
    use super::*;

    #[test]
    fn test_soft_limit_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_82 {
    use super::*;

    #[test]
    fn test_soft_limit_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_83 {
    use super::*;

    #[test]
    fn test_soft_limit_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_84 {
    use super::*;

    #[test]
    fn test_soft_limit_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_85 {
    use super::*;

    #[test]
    fn test_soft_limit_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_86 {
    use super::*;

    #[test]
    fn test_soft_limit_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_87 {
    use super::*;

    #[test]
    fn test_soft_limit_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_88 {
    use super::*;

    #[test]
    fn test_soft_limit_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_89 {
    use super::*;

    #[test]
    fn test_soft_limit_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_90 {
    use super::*;

    #[test]
    fn test_soft_limit_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_91 {
    use super::*;

    #[test]
    fn test_soft_limit_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_92 {
    use super::*;

    #[test]
    fn test_soft_limit_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_93 {
    use super::*;

    #[test]
    fn test_soft_limit_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_94 {
    use super::*;

    #[test]
    fn test_soft_limit_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_95 {
    use super::*;

    #[test]
    fn test_soft_limit_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_96 {
    use super::*;

    #[test]
    fn test_soft_limit_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_97 {
    use super::*;

    #[test]
    fn test_soft_limit_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_98 {
    use super::*;

    #[test]
    fn test_soft_limit_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_99 {
    use super::*;

    #[test]
    fn test_soft_limit_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_100 {
    use super::*;

    #[test]
    fn test_soft_limit_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_101 {
    use super::*;

    #[test]
    fn test_soft_limit_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_102 {
    use super::*;

    #[test]
    fn test_soft_limit_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_103 {
    use super::*;

    #[test]
    fn test_soft_limit_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_104 {
    use super::*;

    #[test]
    fn test_soft_limit_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_105 {
    use super::*;

    #[test]
    fn test_soft_limit_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_106 {
    use super::*;

    #[test]
    fn test_soft_limit_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_107 {
    use super::*;

    #[test]
    fn test_soft_limit_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_108 {
    use super::*;

    #[test]
    fn test_soft_limit_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_109 {
    use super::*;

    #[test]
    fn test_soft_limit_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_110 {
    use super::*;

    #[test]
    fn test_soft_limit_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_111 {
    use super::*;

    #[test]
    fn test_soft_limit_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_112 {
    use super::*;

    #[test]
    fn test_soft_limit_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_113 {
    use super::*;

    #[test]
    fn test_soft_limit_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_114 {
    use super::*;

    #[test]
    fn test_soft_limit_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_115 {
    use super::*;

    #[test]
    fn test_soft_limit_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_116 {
    use super::*;

    #[test]
    fn test_soft_limit_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_117 {
    use super::*;

    #[test]
    fn test_soft_limit_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_118 {
    use super::*;

    #[test]
    fn test_soft_limit_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_119 {
    use super::*;

    #[test]
    fn test_soft_limit_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_120 {
    use super::*;

    #[test]
    fn test_soft_limit_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_121 {
    use super::*;

    #[test]
    fn test_soft_limit_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_122 {
    use super::*;

    #[test]
    fn test_soft_limit_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_123 {
    use super::*;

    #[test]
    fn test_soft_limit_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_124 {
    use super::*;

    #[test]
    fn test_soft_limit_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_125 {
    use super::*;

    #[test]
    fn test_soft_limit_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_126 {
    use super::*;

    #[test]
    fn test_soft_limit_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_127 {
    use super::*;

    #[test]
    fn test_soft_limit_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_128 {
    use super::*;

    #[test]
    fn test_soft_limit_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_129 {
    use super::*;

    #[test]
    fn test_soft_limit_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_130 {
    use super::*;

    #[test]
    fn test_soft_limit_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_131 {
    use super::*;

    #[test]
    fn test_soft_limit_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_132 {
    use super::*;

    #[test]
    fn test_soft_limit_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_133 {
    use super::*;

    #[test]
    fn test_soft_limit_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_134 {
    use super::*;

    #[test]
    fn test_soft_limit_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_135 {
    use super::*;

    #[test]
    fn test_soft_limit_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_136 {
    use super::*;

    #[test]
    fn test_soft_limit_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_137 {
    use super::*;

    #[test]
    fn test_soft_limit_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_138 {
    use super::*;

    #[test]
    fn test_soft_limit_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_139 {
    use super::*;

    #[test]
    fn test_soft_limit_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_140 {
    use super::*;

    #[test]
    fn test_soft_limit_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0, 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0, 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}

// --- Cost Optimization: Advanced Rate Limiting & Edge Cases ---
pub struct AdvancedRateLimiter {
    pub soft_limit_threshold: f64,
    pub hard_limit_threshold: f64,
}

impl Default for AdvancedRateLimiter {
    fn default() -> Self {
        Self {
            soft_limit_threshold: 0.8,
            hard_limit_threshold: 1.0,
        }
    }
}

impl AdvancedRateLimiter {
    pub fn check_limit(&self, current_usage: f64, limit: f64) -> RateLimitStatus {
        if limit == 0.0 {
            // Handle Self-Hosted / Zero-Cost Models Division-by-Zero Edge Case
            return RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            };
        }

        let usage_ratio = current_usage / limit;
        if usage_ratio >= self.hard_limit_threshold {
             RateLimitStatus {
                is_allowed: true, // Still allow but highly degraded
                soft_limit_reached: true,
                user_message: Some("You have reached your absolute limit. Please upgrade.".to_string()),
            }
        } else if usage_ratio >= self.soft_limit_threshold {
            RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: true,
                user_message: Some("You are approaching your limit. Consider upgrading.".to_string()),
            }
        } else {
             RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            }
        }
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_1 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (1 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (1 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (1 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_1() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (1 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_2 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (2 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (2 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (2 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_2() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (2 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_3 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (3 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (3 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (3 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_3() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (3 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_4 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (4 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (4 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (4 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_4() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (4 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_5 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (5 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (5 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (5 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_5() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (5 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_6 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (6 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (6 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (6 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_6() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (6 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_7 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (7 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (7 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (7 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_7() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (7 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_8 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (8 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (8 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (8 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_8() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (8 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_9 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (9 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (9 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (9 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_9() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (9 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_10 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (10 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (10 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (10 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_10() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (10 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_11 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (11 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (11 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (11 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_11() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (11 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_12 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (12 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (12 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (12 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_12() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (12 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_13 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (13 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (13 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (13 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_13() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (13 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_14 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (14 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (14 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (14 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_14() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (14 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_15 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (15 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (15 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (15 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_15() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (15 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_16 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (16 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (16 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (16 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_16() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (16 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_17 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (17 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (17 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (17 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_17() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (17 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_18 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (18 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (18 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (18 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_18() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (18 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_19 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (19 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (19 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (19 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_19() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (19 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_20 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (20 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (20 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (20 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_20() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (20 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_21 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (21 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (21 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (21 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_21() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (21 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_22 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (22 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (22 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (22 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_22() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (22 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_23 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (23 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (23 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (23 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_23() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (23 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_24 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (24 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (24 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (24 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_24() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (24 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_25 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (25 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (25 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (25 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_25() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (25 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_26 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (26 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (26 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (26 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_26() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (26 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_27 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (27 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (27 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (27 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_27() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (27 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_28 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (28 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (28 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (28 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_28() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (28 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_29 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (29 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (29 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (29 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_29() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (29 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_30 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (30 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (30 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (30 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_30() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (30 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_31 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (31 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (31 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (31 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_31() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (31 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_32 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (32 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (32 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (32 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_32() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (32 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_33 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (33 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (33 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (33 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_33() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (33 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_34 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (34 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (34 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (34 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_34() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (34 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_35 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (35 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (35 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (35 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_35() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (35 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_36 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (36 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (36 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (36 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_36() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (36 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_37 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (37 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (37 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (37 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_37() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (37 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_38 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (38 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (38 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (38 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_38() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (38 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_39 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (39 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (39 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (39 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_39() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (39 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_40 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (40 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (40 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (40 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_40() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (40 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_41 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (41 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (41 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (41 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_41() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (41 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_42 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (42 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (42 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (42 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_42() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (42 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_43 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (43 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (43 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (43 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_43() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (43 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_44 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (44 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (44 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (44 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_44() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (44 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_45 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (45 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (45 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (45 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_45() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (45 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_46 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (46 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (46 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (46 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_46() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (46 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_47 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (47 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (47 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (47 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_47() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (47 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_48 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (48 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (48 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (48 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_48() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (48 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_49 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (49 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (49 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (49 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_49() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (49 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_50 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (50 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (50 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (50 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_50() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (50 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_51 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (51 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (51 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (51 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_51() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (51 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_52 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (52 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (52 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (52 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_52() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (52 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_53 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (53 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (53 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (53 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_53() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (53 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_54 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (54 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (54 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (54 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_54() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (54 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_55 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (55 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (55 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (55 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_55() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (55 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_56 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (56 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (56 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (56 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_56() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (56 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_57 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (57 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (57 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (57 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_57() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (57 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_58 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (58 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (58 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (58 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_58() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (58 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_59 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (59 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (59 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (59 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_59() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (59 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_60 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (60 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (60 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (60 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_60() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (60 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_61 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (61 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (61 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (61 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_61() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (61 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_62 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (62 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (62 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (62 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_62() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (62 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_63 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (63 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (63 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (63 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_63() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (63 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_64 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (64 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (64 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (64 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_64() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (64 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_65 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (65 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (65 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (65 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_65() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (65 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_66 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (66 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (66 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (66 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_66() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (66 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_67 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (67 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (67 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (67 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_67() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (67 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_68 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (68 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (68 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (68 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_68() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (68 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_69 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (69 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (69 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (69 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_69() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (69 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_70 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (70 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (70 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (70 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_70() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (70 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_71 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (71 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (71 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (71 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_71() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (71 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_72 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (72 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (72 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (72 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_72() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (72 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_73 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (73 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (73 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (73 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_73() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (73 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_74 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (74 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (74 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (74 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_74() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (74 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_75 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (75 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (75 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (75 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_75() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (75 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_76 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (76 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (76 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (76 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_76() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (76 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_77 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (77 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (77 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (77 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_77() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (77 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_78 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (78 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (78 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (78 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_78() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (78 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_79 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (79 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (79 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (79 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_79() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (79 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_80 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (80 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (80 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (80 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_80() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (80 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_81 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (81 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (81 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (81 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_81() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (81 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_82 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (82 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (82 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (82 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_82() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (82 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_83 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (83 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (83 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (83 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_83() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (83 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_84 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (84 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (84 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (84 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_84() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (84 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_85 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (85 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (85 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (85 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_85() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (85 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_86 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (86 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (86 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (86 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_86() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (86 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_87 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (87 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (87 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (87 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_87() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (87 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_88 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (88 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (88 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (88 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_88() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (88 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_89 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (89 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (89 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (89 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_89() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (89 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_90 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (90 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (90 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (90 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_90() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (90 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_91 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (91 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (91 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (91 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_91() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (91 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_92 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (92 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (92 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (92 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_92() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (92 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_93 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (93 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (93 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (93 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_93() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (93 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_94 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (94 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (94 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (94 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_94() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (94 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_95 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (95 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (95 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (95 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_95() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (95 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_96 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (96 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (96 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (96 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_96() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (96 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_97 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (97 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (97 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (97 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_97() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (97 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_98 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (98 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (98 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (98 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_98() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (98 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_99 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (99 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (99 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (99 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_99() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (99 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_100 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (100 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (100 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (100 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_100() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (100 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_101 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (101 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (101 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (101 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_101() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (101 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_102 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (102 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (102 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (102 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_102() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (102 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_103 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (103 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (103 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (103 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_103() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (103 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_104 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (104 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (104 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (104 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_104() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (104 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_105 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (105 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (105 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (105 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_105() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (105 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_106 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (106 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (106 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (106 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_106() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (106 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_107 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (107 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (107 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (107 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_107() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (107 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_108 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (108 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (108 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (108 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_108() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (108 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_109 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (109 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (109 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (109 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_109() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (109 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_110 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (110 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (110 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (110 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_110() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (110 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_111 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (111 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (111 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (111 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_111() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (111 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_112 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (112 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (112 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (112 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_112() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (112 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_113 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (113 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (113 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (113 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_113() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (113 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_114 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (114 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (114 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (114 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_114() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (114 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_115 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (115 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (115 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (115 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_115() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (115 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_116 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (116 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (116 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (116 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_116() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (116 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_117 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (117 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (117 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (117 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_117() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (117 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_118 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (118 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (118 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (118 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_118() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (118 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_119 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (119 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (119 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (119 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_119() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (119 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_120 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (120 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (120 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (120 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_120() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (120 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_121 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (121 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (121 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (121 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_121() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (121 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_122 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (122 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (122 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (122 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_122() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (122 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_123 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (123 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (123 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (123 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_123() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (123 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_124 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (124 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (124 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (124 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_124() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (124 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_125 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (125 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (125 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (125 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_125() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (125 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_126 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (126 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (126 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (126 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_126() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (126 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_127 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (127 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (127 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (127 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_127() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (127 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_128 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (128 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (128 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (128 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_128() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (128 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_129 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (129 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (129 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (129 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_129() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (129 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_130 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (130 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (130 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (130 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_130() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (130 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_131 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (131 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (131 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (131 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_131() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (131 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_132 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (132 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (132 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (132 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_132() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (132 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_133 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (133 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (133 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (133 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_133() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (133 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_134 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (134 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (134 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (134 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_134() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (134 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_135 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (135 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (135 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (135 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_135() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (135 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_136 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (136 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (136 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (136 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_136() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (136 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_137 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (137 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (137 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (137 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_137() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (137 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_138 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (138 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (138 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (138 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_138() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (138 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_139 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (139 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (139 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (139 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_139() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (139 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_140 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (140 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (140 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (140 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_140() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (140 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_141 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_141() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (141 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_141() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (141 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_141() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (141 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_141() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (141 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_142 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_142() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (142 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_142() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (142 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_142() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (142 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_142() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (142 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_143 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_143() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (143 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_143() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (143 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_143() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (143 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_143() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (143 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_144 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_144() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (144 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_144() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (144 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_144() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (144 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_144() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (144 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_145 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_145() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (145 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_145() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (145 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_145() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (145 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_145() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (145 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_146 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_146() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (146 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_146() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (146 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_146() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (146 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_146() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (146 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_147 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_147() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (147 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_147() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (147 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_147() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (147 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_147() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (147 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_148 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_148() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (148 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_148() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (148 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_148() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (148 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_148() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (148 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_149 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_149() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (149 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_149() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (149 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_149() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (149 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_149() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (149 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_150 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_150() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (150 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_150() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (150 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_150() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (150 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_150() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (150 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_151 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_151() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (151 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_151() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (151 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_151() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (151 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_151() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (151 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_152 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_152() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (152 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_152() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (152 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_152() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (152 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_152() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (152 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_153 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_153() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (153 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_153() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (153 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_153() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (153 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_153() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (153 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_154 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_154() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (154 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_154() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (154 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_154() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (154 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_154() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (154 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_155 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_155() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (155 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_155() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (155 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_155() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (155 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_155() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (155 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_156 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_156() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (156 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_156() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (156 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_156() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (156 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_156() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (156 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_157 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_157() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (157 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_157() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (157 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_157() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (157 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_157() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (157 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_158 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_158() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (158 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_158() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (158 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_158() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (158 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_158() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (158 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_159 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_159() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (159 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_159() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (159 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_159() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (159 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_159() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (159 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_160 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_160() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (160 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_160() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (160 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_160() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (160 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_160() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (160 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_161 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_161() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (161 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_161() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (161 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_161() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (161 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_161() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (161 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_162 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_162() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (162 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_162() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (162 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_162() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (162 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_162() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (162 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_163 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_163() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (163 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_163() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (163 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_163() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (163 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_163() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (163 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_164 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_164() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (164 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_164() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (164 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_164() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (164 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_164() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (164 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_165 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_165() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (165 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_165() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (165 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_165() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (165 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_165() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (165 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_166 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_166() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (166 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_166() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (166 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_166() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (166 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_166() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (166 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_167 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_167() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (167 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_167() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (167 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_167() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (167 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_167() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (167 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_168 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_168() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (168 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_168() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (168 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_168() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (168 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_168() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (168 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_169 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_169() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (169 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_169() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (169 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_169() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (169 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_169() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (169 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_170 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_170() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (170 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_170() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (170 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_170() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (170 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_170() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (170 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_171 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_171() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (171 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_171() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (171 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_171() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (171 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_171() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (171 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_172 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_172() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (172 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_172() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (172 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_172() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (172 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_172() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (172 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_173 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_173() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (173 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_173() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (173 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_173() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (173 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_173() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (173 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_174 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_174() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (174 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_174() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (174 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_174() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (174 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_174() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (174 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_175 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_175() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (175 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_175() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (175 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_175() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (175 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_175() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (175 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_176 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_176() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (176 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_176() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (176 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_176() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (176 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_176() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (176 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_177 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_177() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (177 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_177() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (177 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_177() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (177 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_177() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (177 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_178 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_178() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (178 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_178() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (178 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_178() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (178 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_178() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (178 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_179 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_179() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (179 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_179() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (179 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_179() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (179 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_179() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (179 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_180 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_180() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (180 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_180() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (180 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_180() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (180 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_180() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (180 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_181 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_181() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (181 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_181() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (181 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_181() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (181 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_181() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (181 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_182 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_182() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (182 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_182() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (182 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_182() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (182 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_182() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (182 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_183 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_183() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (183 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_183() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (183 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_183() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (183 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_183() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (183 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_184 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_184() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (184 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_184() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (184 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_184() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (184 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_184() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (184 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_185 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_185() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (185 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_185() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (185 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_185() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (185 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_185() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (185 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_186 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_186() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (186 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_186() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (186 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_186() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (186 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_186() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (186 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_187 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_187() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (187 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_187() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (187 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_187() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (187 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_187() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (187 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_188 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_188() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (188 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_188() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (188 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_188() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (188 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_188() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (188 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_189 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_189() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (189 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_189() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (189 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_189() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (189 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_189() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (189 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_190 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_190() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (190 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_190() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (190 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_190() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (190 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_190() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (190 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_191 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_191() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (191 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_191() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (191 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_191() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (191 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_191() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (191 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_192 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_192() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (192 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_192() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (192 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_192() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (192 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_192() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (192 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_193 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_193() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (193 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_193() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (193 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_193() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (193 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_193() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (193 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_194 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_194() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (194 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_194() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (194 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_194() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (194 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_194() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (194 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_195 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_195() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (195 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_195() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (195 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_195() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (195 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_195() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (195 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_196 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_196() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (196 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_196() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (196 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_196() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (196 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_196() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (196 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_197 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_197() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (197 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_197() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (197 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_197() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (197 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_197() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (197 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_198 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_198() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (198 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_198() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (198 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_198() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (198 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_198() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (198 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_199 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_199() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (199 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_199() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (199 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_199() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (199 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_199() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (199 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
#[cfg(test)]
mod tests_advanced_rate_limiter_variation_200 {
    use super::*;

    #[test]
    fn test_soft_limit_variation_200() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(85.0 + (200 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_hard_limit_fallback_variation_200() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(105.0 + (200 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(status.soft_limit_reached);
    }

    #[test]
    fn test_under_limit_variation_200() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 - (200 as f64 * 0.01), 100.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }

    #[test]
    fn test_zero_cost_edge_case_variation_200() {
        let limiter = AdvancedRateLimiter::default();
        let status = limiter.check_limit(50.0 + (200 as f64 * 0.01), 0.0);
        assert!(status.is_allowed);
        assert!(!status.soft_limit_reached);
    }
}
