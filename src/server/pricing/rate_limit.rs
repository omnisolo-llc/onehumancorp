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
mod extensive_pricing_resilience_tests {
    use super::*;
    use crate::pricing::calculator::{calculate_cost, calculate_cost_cents, get_pricing};
    use std::sync::Arc;
    use tokio;

    macro_rules! generate_cost_test {
        ($test_name:ident, $model:expr, $prompt_tokens:expr, $completion_tokens:expr, $cached_tokens:expr, $expected_cost:expr) => {
            #[test]
            fn $test_name() {
                let cost = calculate_cost($model, $prompt_tokens, $completion_tokens, $cached_tokens);
                let cents = calculate_cost_cents($model, $prompt_tokens, $completion_tokens, $cached_tokens);
                assert!((cost - $expected_cost).abs() < 0.001, "Expected {}, got {} for model {}", $expected_cost, cost, $model);
                let expected_cents = ($expected_cost * 100.0).round() as i64;
                assert_eq!(cents, expected_cents, "Cents mismatch for {}", $model);

                let pricing = get_pricing($model);
                assert!(pricing.input_cost >= 0.0);
                assert!(pricing.output_cost >= 0.0);
                assert!(pricing.cached_cost >= 0.0);
            }
        };
    }

    generate_cost_test!(test_cost_matrix_claude_3_opus_1000, "claude-3-opus", 1000, 2000, 3000, 0.16499999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_2000, "claude-3-opus", 2000, 4000, 6000, 0.32999999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_3000, "claude-3-opus", 3000, 6000, 9000, 0.495f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_4000, "claude-3-opus", 4000, 8000, 12000, 0.6599999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_5000, "claude-3-opus", 5000, 10000, 15000, 0.825f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_6000, "claude-3-opus", 6000, 12000, 18000, 0.99f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_7000, "claude-3-opus", 7000, 14000, 21000, 1.155f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_8000, "claude-3-opus", 8000, 16000, 24000, 1.3199999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_9000, "claude-3-opus", 9000, 18000, 27000, 1.485f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_10000, "claude-3-opus", 10000, 20000, 30000, 1.65f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_11000, "claude-3-opus", 11000, 22000, 33000, 1.815f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_12000, "claude-3-opus", 12000, 24000, 36000, 1.98f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_13000, "claude-3-opus", 13000, 26000, 39000, 2.145f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_14000, "claude-3-opus", 14000, 28000, 42000, 2.31f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_15000, "claude-3-opus", 15000, 30000, 45000, 2.475f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_16000, "claude-3-opus", 16000, 32000, 48000, 2.6399999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_17000, "claude-3-opus", 17000, 34000, 51000, 2.8049999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_18000, "claude-3-opus", 18000, 36000, 54000, 2.97f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_19000, "claude-3-opus", 19000, 38000, 57000, 3.1350000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_20000, "claude-3-opus", 20000, 40000, 60000, 3.3f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_21000, "claude-3-opus", 21000, 42000, 63000, 3.465f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_22000, "claude-3-opus", 22000, 44000, 66000, 3.63f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_23000, "claude-3-opus", 23000, 46000, 69000, 3.795f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_24000, "claude-3-opus", 24000, 48000, 72000, 3.96f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_25000, "claude-3-opus", 25000, 50000, 75000, 4.125f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_26000, "claude-3-opus", 26000, 52000, 78000, 4.29f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_27000, "claude-3-opus", 27000, 54000, 81000, 4.455f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_28000, "claude-3-opus", 28000, 56000, 84000, 4.62f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_29000, "claude-3-opus", 29000, 58000, 87000, 4.784999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_30000, "claude-3-opus", 30000, 60000, 90000, 4.95f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_31000, "claude-3-opus", 31000, 62000, 93000, 5.115f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_32000, "claude-3-opus", 32000, 64000, 96000, 5.279999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_33000, "claude-3-opus", 33000, 66000, 99000, 5.445f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_34000, "claude-3-opus", 34000, 68000, 102000, 5.609999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_35000, "claude-3-opus", 35000, 70000, 105000, 5.775f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_36000, "claude-3-opus", 36000, 72000, 108000, 5.94f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_37000, "claude-3-opus", 37000, 74000, 111000, 6.1049999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_38000, "claude-3-opus", 38000, 76000, 114000, 6.2700000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_39000, "claude-3-opus", 39000, 78000, 117000, 6.435f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_40000, "claude-3-opus", 40000, 80000, 120000, 6.6f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_41000, "claude-3-opus", 41000, 82000, 123000, 6.765000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_42000, "claude-3-opus", 42000, 84000, 126000, 6.93f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_43000, "claude-3-opus", 43000, 86000, 129000, 7.095000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_44000, "claude-3-opus", 44000, 88000, 132000, 7.26f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_45000, "claude-3-opus", 45000, 90000, 135000, 7.425f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_46000, "claude-3-opus", 46000, 92000, 138000, 7.59f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_47000, "claude-3-opus", 47000, 94000, 141000, 7.755f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_48000, "claude-3-opus", 48000, 96000, 144000, 7.92f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_49000, "claude-3-opus", 49000, 98000, 147000, 8.084999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_50000, "claude-3-opus", 50000, 100000, 150000, 8.25f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_51000, "claude-3-opus", 51000, 102000, 153000, 8.415000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_52000, "claude-3-opus", 52000, 104000, 156000, 8.58f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_53000, "claude-3-opus", 53000, 106000, 159000, 8.745000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_54000, "claude-3-opus", 54000, 108000, 162000, 8.91f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_55000, "claude-3-opus", 55000, 110000, 165000, 9.075f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_56000, "claude-3-opus", 56000, 112000, 168000, 9.24f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_57000, "claude-3-opus", 57000, 114000, 171000, 9.405000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_58000, "claude-3-opus", 58000, 116000, 174000, 9.569999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_59000, "claude-3-opus", 59000, 118000, 177000, 9.735f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_60000, "claude-3-opus", 60000, 120000, 180000, 9.9f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_61000, "claude-3-opus", 61000, 122000, 183000, 10.065000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_62000, "claude-3-opus", 62000, 124000, 186000, 10.23f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_63000, "claude-3-opus", 63000, 126000, 189000, 10.395f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_64000, "claude-3-opus", 64000, 128000, 192000, 10.559999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_65000, "claude-3-opus", 65000, 130000, 195000, 10.725f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_66000, "claude-3-opus", 66000, 132000, 198000, 10.89f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_67000, "claude-3-opus", 67000, 134000, 201000, 11.055f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_68000, "claude-3-opus", 68000, 136000, 204000, 11.219999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_69000, "claude-3-opus", 69000, 138000, 207000, 11.385f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_70000, "claude-3-opus", 70000, 140000, 210000, 11.55f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_71000, "claude-3-opus", 71000, 142000, 213000, 11.715f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_72000, "claude-3-opus", 72000, 144000, 216000, 11.88f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_73000, "claude-3-opus", 73000, 146000, 219000, 12.045f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_74000, "claude-3-opus", 74000, 148000, 222000, 12.209999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_75000, "claude-3-opus", 75000, 150000, 225000, 12.375f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_76000, "claude-3-opus", 76000, 152000, 228000, 12.540000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_77000, "claude-3-opus", 77000, 154000, 231000, 12.705f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_78000, "claude-3-opus", 78000, 156000, 234000, 12.87f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_79000, "claude-3-opus", 79000, 158000, 237000, 13.035f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_80000, "claude-3-opus", 80000, 160000, 240000, 13.2f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_81000, "claude-3-opus", 81000, 162000, 243000, 13.365f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_82000, "claude-3-opus", 82000, 164000, 246000, 13.530000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_83000, "claude-3-opus", 83000, 166000, 249000, 13.695f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_84000, "claude-3-opus", 84000, 168000, 252000, 13.86f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_85000, "claude-3-opus", 85000, 170000, 255000, 14.025f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_86000, "claude-3-opus", 86000, 172000, 258000, 14.190000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_87000, "claude-3-opus", 87000, 174000, 261000, 14.355f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_88000, "claude-3-opus", 88000, 176000, 264000, 14.52f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_89000, "claude-3-opus", 89000, 178000, 267000, 14.684999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_90000, "claude-3-opus", 90000, 180000, 270000, 14.85f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_91000, "claude-3-opus", 91000, 182000, 273000, 15.015f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_92000, "claude-3-opus", 92000, 184000, 276000, 15.18f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_93000, "claude-3-opus", 93000, 186000, 279000, 15.344999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_94000, "claude-3-opus", 94000, 188000, 282000, 15.51f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_95000, "claude-3-opus", 95000, 190000, 285000, 15.675f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_96000, "claude-3-opus", 96000, 192000, 288000, 15.84f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_97000, "claude-3-opus", 97000, 194000, 291000, 16.005000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_98000, "claude-3-opus", 98000, 196000, 294000, 16.169999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_99000, "claude-3-opus", 99000, 198000, 297000, 16.335f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_100000, "claude-3-opus", 100000, 200000, 300000, 16.5f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_101000, "claude-3-opus", 101000, 202000, 303000, 16.665f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_102000, "claude-3-opus", 102000, 204000, 306000, 16.830000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_103000, "claude-3-opus", 103000, 206000, 309000, 16.994999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_104000, "claude-3-opus", 104000, 208000, 312000, 17.16f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_105000, "claude-3-opus", 105000, 210000, 315000, 17.325f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_106000, "claude-3-opus", 106000, 212000, 318000, 17.490000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_107000, "claude-3-opus", 107000, 214000, 321000, 17.655f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_108000, "claude-3-opus", 108000, 216000, 324000, 17.82f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_109000, "claude-3-opus", 109000, 218000, 327000, 17.985000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_110000, "claude-3-opus", 110000, 220000, 330000, 18.15f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_111000, "claude-3-opus", 111000, 222000, 333000, 18.314999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_112000, "claude-3-opus", 112000, 224000, 336000, 18.48f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_113000, "claude-3-opus", 113000, 226000, 339000, 18.645f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_114000, "claude-3-opus", 114000, 228000, 342000, 18.810000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_115000, "claude-3-opus", 115000, 230000, 345000, 18.975f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_116000, "claude-3-opus", 116000, 232000, 348000, 19.139999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_117000, "claude-3-opus", 117000, 234000, 351000, 19.305f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_118000, "claude-3-opus", 118000, 236000, 354000, 19.47f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_119000, "claude-3-opus", 119000, 238000, 357000, 19.635f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_120000, "claude-3-opus", 120000, 240000, 360000, 19.8f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_121000, "claude-3-opus", 121000, 242000, 363000, 19.965f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_122000, "claude-3-opus", 122000, 244000, 366000, 20.130000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_123000, "claude-3-opus", 123000, 246000, 369000, 20.294999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_124000, "claude-3-opus", 124000, 248000, 372000, 20.46f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_125000, "claude-3-opus", 125000, 250000, 375000, 20.625f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_126000, "claude-3-opus", 126000, 252000, 378000, 20.79f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_127000, "claude-3-opus", 127000, 254000, 381000, 20.955000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_128000, "claude-3-opus", 128000, 256000, 384000, 21.119999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_129000, "claude-3-opus", 129000, 258000, 387000, 21.285f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_130000, "claude-3-opus", 130000, 260000, 390000, 21.45f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_131000, "claude-3-opus", 131000, 262000, 393000, 21.615f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_132000, "claude-3-opus", 132000, 264000, 396000, 21.78f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_133000, "claude-3-opus", 133000, 266000, 399000, 21.945f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_134000, "claude-3-opus", 134000, 268000, 402000, 22.11f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_135000, "claude-3-opus", 135000, 270000, 405000, 22.275f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_136000, "claude-3-opus", 136000, 272000, 408000, 22.439999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_137000, "claude-3-opus", 137000, 274000, 411000, 22.605f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_138000, "claude-3-opus", 138000, 276000, 414000, 22.77f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_139000, "claude-3-opus", 139000, 278000, 417000, 22.935000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_140000, "claude-3-opus", 140000, 280000, 420000, 23.1f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_141000, "claude-3-opus", 141000, 282000, 423000, 23.265f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_142000, "claude-3-opus", 142000, 284000, 426000, 23.43f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_143000, "claude-3-opus", 143000, 286000, 429000, 23.595f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_144000, "claude-3-opus", 144000, 288000, 432000, 23.76f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_145000, "claude-3-opus", 145000, 290000, 435000, 23.925f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_146000, "claude-3-opus", 146000, 292000, 438000, 24.09f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_147000, "claude-3-opus", 147000, 294000, 441000, 24.255000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_148000, "claude-3-opus", 148000, 296000, 444000, 24.419999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_opus_149000, "claude-3-opus", 149000, 298000, 447000, 24.585f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_1000, "claude-3-sonnet", 1000, 2000, 3000, 0.033f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_2000, "claude-3-sonnet", 2000, 4000, 6000, 0.066f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_3000, "claude-3-sonnet", 3000, 6000, 9000, 0.09899999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_4000, "claude-3-sonnet", 4000, 8000, 12000, 0.132f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_5000, "claude-3-sonnet", 5000, 10000, 15000, 0.16499999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_6000, "claude-3-sonnet", 6000, 12000, 18000, 0.19799999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_7000, "claude-3-sonnet", 7000, 14000, 21000, 0.23099999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_8000, "claude-3-sonnet", 8000, 16000, 24000, 0.264f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_9000, "claude-3-sonnet", 9000, 18000, 27000, 0.29700000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_10000, "claude-3-sonnet", 10000, 20000, 30000, 0.32999999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_11000, "claude-3-sonnet", 11000, 22000, 33000, 0.363f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_12000, "claude-3-sonnet", 12000, 24000, 36000, 0.39599999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_13000, "claude-3-sonnet", 13000, 26000, 39000, 0.429f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_14000, "claude-3-sonnet", 14000, 28000, 42000, 0.46199999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_15000, "claude-3-sonnet", 15000, 30000, 45000, 0.495f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_16000, "claude-3-sonnet", 16000, 32000, 48000, 0.528f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_17000, "claude-3-sonnet", 17000, 34000, 51000, 0.561f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_18000, "claude-3-sonnet", 18000, 36000, 54000, 0.5940000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_19000, "claude-3-sonnet", 19000, 38000, 57000, 0.627f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_20000, "claude-3-sonnet", 20000, 40000, 60000, 0.6599999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_21000, "claude-3-sonnet", 21000, 42000, 63000, 0.6930000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_22000, "claude-3-sonnet", 22000, 44000, 66000, 0.726f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_23000, "claude-3-sonnet", 23000, 46000, 69000, 0.7589999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_24000, "claude-3-sonnet", 24000, 48000, 72000, 0.7919999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_25000, "claude-3-sonnet", 25000, 50000, 75000, 0.825f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_26000, "claude-3-sonnet", 26000, 52000, 78000, 0.858f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_27000, "claude-3-sonnet", 27000, 54000, 81000, 0.891f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_28000, "claude-3-sonnet", 28000, 56000, 84000, 0.9239999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_29000, "claude-3-sonnet", 29000, 58000, 87000, 0.957f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_30000, "claude-3-sonnet", 30000, 60000, 90000, 0.99f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_31000, "claude-3-sonnet", 31000, 62000, 93000, 1.0230000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_32000, "claude-3-sonnet", 32000, 64000, 96000, 1.056f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_33000, "claude-3-sonnet", 33000, 66000, 99000, 1.089f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_34000, "claude-3-sonnet", 34000, 68000, 102000, 1.122f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_35000, "claude-3-sonnet", 35000, 70000, 105000, 1.155f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_36000, "claude-3-sonnet", 36000, 72000, 108000, 1.1880000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_37000, "claude-3-sonnet", 37000, 74000, 111000, 1.221f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_38000, "claude-3-sonnet", 38000, 76000, 114000, 1.254f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_39000, "claude-3-sonnet", 39000, 78000, 117000, 1.287f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_40000, "claude-3-sonnet", 40000, 80000, 120000, 1.3199999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_41000, "claude-3-sonnet", 41000, 82000, 123000, 1.353f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_42000, "claude-3-sonnet", 42000, 84000, 126000, 1.3860000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_43000, "claude-3-sonnet", 43000, 86000, 129000, 1.419f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_44000, "claude-3-sonnet", 44000, 88000, 132000, 1.452f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_45000, "claude-3-sonnet", 45000, 90000, 135000, 1.485f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_46000, "claude-3-sonnet", 46000, 92000, 138000, 1.5179999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_47000, "claude-3-sonnet", 47000, 94000, 141000, 1.551f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_48000, "claude-3-sonnet", 48000, 96000, 144000, 1.5839999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_49000, "claude-3-sonnet", 49000, 98000, 147000, 1.617f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_50000, "claude-3-sonnet", 50000, 100000, 150000, 1.65f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_51000, "claude-3-sonnet", 51000, 102000, 153000, 1.683f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_52000, "claude-3-sonnet", 52000, 104000, 156000, 1.716f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_53000, "claude-3-sonnet", 53000, 106000, 159000, 1.749f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_54000, "claude-3-sonnet", 54000, 108000, 162000, 1.782f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_55000, "claude-3-sonnet", 55000, 110000, 165000, 1.815f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_56000, "claude-3-sonnet", 56000, 112000, 168000, 1.8479999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_57000, "claude-3-sonnet", 57000, 114000, 171000, 1.881f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_58000, "claude-3-sonnet", 58000, 116000, 174000, 1.914f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_59000, "claude-3-sonnet", 59000, 118000, 177000, 1.947f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_60000, "claude-3-sonnet", 60000, 120000, 180000, 1.98f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_61000, "claude-3-sonnet", 61000, 122000, 183000, 2.013f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_62000, "claude-3-sonnet", 62000, 124000, 186000, 2.0460000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_63000, "claude-3-sonnet", 63000, 126000, 189000, 2.0789999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_64000, "claude-3-sonnet", 64000, 128000, 192000, 2.112f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_65000, "claude-3-sonnet", 65000, 130000, 195000, 2.145f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_66000, "claude-3-sonnet", 66000, 132000, 198000, 2.178f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_67000, "claude-3-sonnet", 67000, 134000, 201000, 2.211f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_68000, "claude-3-sonnet", 68000, 136000, 204000, 2.244f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_69000, "claude-3-sonnet", 69000, 138000, 207000, 2.2769999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_70000, "claude-3-sonnet", 70000, 140000, 210000, 2.31f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_71000, "claude-3-sonnet", 71000, 142000, 213000, 2.343f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_72000, "claude-3-sonnet", 72000, 144000, 216000, 2.3760000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_73000, "claude-3-sonnet", 73000, 146000, 219000, 2.409f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_74000, "claude-3-sonnet", 74000, 148000, 222000, 2.442f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_75000, "claude-3-sonnet", 75000, 150000, 225000, 2.475f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_76000, "claude-3-sonnet", 76000, 152000, 228000, 2.508f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_77000, "claude-3-sonnet", 77000, 154000, 231000, 2.541f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_78000, "claude-3-sonnet", 78000, 156000, 234000, 2.574f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_79000, "claude-3-sonnet", 79000, 158000, 237000, 2.607f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_80000, "claude-3-sonnet", 80000, 160000, 240000, 2.6399999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_81000, "claude-3-sonnet", 81000, 162000, 243000, 2.673f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_82000, "claude-3-sonnet", 82000, 164000, 246000, 2.706f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_83000, "claude-3-sonnet", 83000, 166000, 249000, 2.7390000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_84000, "claude-3-sonnet", 84000, 168000, 252000, 2.7720000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_85000, "claude-3-sonnet", 85000, 170000, 255000, 2.8049999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_86000, "claude-3-sonnet", 86000, 172000, 258000, 2.838f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_87000, "claude-3-sonnet", 87000, 174000, 261000, 2.871f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_88000, "claude-3-sonnet", 88000, 176000, 264000, 2.904f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_89000, "claude-3-sonnet", 89000, 178000, 267000, 2.937f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_90000, "claude-3-sonnet", 90000, 180000, 270000, 2.97f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_91000, "claude-3-sonnet", 91000, 182000, 273000, 3.003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_92000, "claude-3-sonnet", 92000, 184000, 276000, 3.0359999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_93000, "claude-3-sonnet", 93000, 186000, 279000, 3.069f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_94000, "claude-3-sonnet", 94000, 188000, 282000, 3.102f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_95000, "claude-3-sonnet", 95000, 190000, 285000, 3.1350000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_96000, "claude-3-sonnet", 96000, 192000, 288000, 3.1679999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_97000, "claude-3-sonnet", 97000, 194000, 291000, 3.201f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_98000, "claude-3-sonnet", 98000, 196000, 294000, 3.234f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_99000, "claude-3-sonnet", 99000, 198000, 297000, 3.2670000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_100000, "claude-3-sonnet", 100000, 200000, 300000, 3.3f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_101000, "claude-3-sonnet", 101000, 202000, 303000, 3.3329999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_102000, "claude-3-sonnet", 102000, 204000, 306000, 3.366f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_103000, "claude-3-sonnet", 103000, 206000, 309000, 3.399f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_104000, "claude-3-sonnet", 104000, 208000, 312000, 3.432f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_105000, "claude-3-sonnet", 105000, 210000, 315000, 3.465f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_106000, "claude-3-sonnet", 106000, 212000, 318000, 3.498f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_107000, "claude-3-sonnet", 107000, 214000, 321000, 3.531f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_108000, "claude-3-sonnet", 108000, 216000, 324000, 3.564f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_109000, "claude-3-sonnet", 109000, 218000, 327000, 3.597f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_110000, "claude-3-sonnet", 110000, 220000, 330000, 3.63f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_111000, "claude-3-sonnet", 111000, 222000, 333000, 3.6630000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_112000, "claude-3-sonnet", 112000, 224000, 336000, 3.6959999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_113000, "claude-3-sonnet", 113000, 226000, 339000, 3.729f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_114000, "claude-3-sonnet", 114000, 228000, 342000, 3.762f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_115000, "claude-3-sonnet", 115000, 230000, 345000, 3.795f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_116000, "claude-3-sonnet", 116000, 232000, 348000, 3.828f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_117000, "claude-3-sonnet", 117000, 234000, 351000, 3.8609999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_118000, "claude-3-sonnet", 118000, 236000, 354000, 3.894f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_119000, "claude-3-sonnet", 119000, 238000, 357000, 3.9269999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_120000, "claude-3-sonnet", 120000, 240000, 360000, 3.96f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_121000, "claude-3-sonnet", 121000, 242000, 363000, 3.993f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_122000, "claude-3-sonnet", 122000, 244000, 366000, 4.026f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_123000, "claude-3-sonnet", 123000, 246000, 369000, 4.059f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_124000, "claude-3-sonnet", 124000, 248000, 372000, 4.0920000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_125000, "claude-3-sonnet", 125000, 250000, 375000, 4.125f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_126000, "claude-3-sonnet", 126000, 252000, 378000, 4.1579999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_127000, "claude-3-sonnet", 127000, 254000, 381000, 4.191f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_128000, "claude-3-sonnet", 128000, 256000, 384000, 4.224f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_129000, "claude-3-sonnet", 129000, 258000, 387000, 4.257f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_130000, "claude-3-sonnet", 130000, 260000, 390000, 4.29f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_131000, "claude-3-sonnet", 131000, 262000, 393000, 4.323f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_132000, "claude-3-sonnet", 132000, 264000, 396000, 4.356f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_133000, "claude-3-sonnet", 133000, 266000, 399000, 4.389f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_134000, "claude-3-sonnet", 134000, 268000, 402000, 4.422f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_135000, "claude-3-sonnet", 135000, 270000, 405000, 4.455f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_136000, "claude-3-sonnet", 136000, 272000, 408000, 4.488f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_137000, "claude-3-sonnet", 137000, 274000, 411000, 4.521f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_138000, "claude-3-sonnet", 138000, 276000, 414000, 4.553999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_139000, "claude-3-sonnet", 139000, 278000, 417000, 4.587f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_140000, "claude-3-sonnet", 140000, 280000, 420000, 4.62f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_141000, "claude-3-sonnet", 141000, 282000, 423000, 4.6530000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_142000, "claude-3-sonnet", 142000, 284000, 426000, 4.686f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_143000, "claude-3-sonnet", 143000, 286000, 429000, 4.719f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_144000, "claude-3-sonnet", 144000, 288000, 432000, 4.752000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_145000, "claude-3-sonnet", 145000, 290000, 435000, 4.784999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_146000, "claude-3-sonnet", 146000, 292000, 438000, 4.818f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_147000, "claude-3-sonnet", 147000, 294000, 441000, 4.851f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_148000, "claude-3-sonnet", 148000, 296000, 444000, 4.884f64);
    generate_cost_test!(test_cost_matrix_claude_3_sonnet_149000, "claude-3-sonnet", 149000, 298000, 447000, 4.917f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_1000, "claude-3-haiku", 1000, 2000, 3000, 0.00275f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_2000, "claude-3-haiku", 2000, 4000, 6000, 0.0055f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_3000, "claude-3-haiku", 3000, 6000, 9000, 0.00825f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_4000, "claude-3-haiku", 4000, 8000, 12000, 0.011f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_5000, "claude-3-haiku", 5000, 10000, 15000, 0.01375f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_6000, "claude-3-haiku", 6000, 12000, 18000, 0.0165f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_7000, "claude-3-haiku", 7000, 14000, 21000, 0.019250000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_8000, "claude-3-haiku", 8000, 16000, 24000, 0.022f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_9000, "claude-3-haiku", 9000, 18000, 27000, 0.024749999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_10000, "claude-3-haiku", 10000, 20000, 30000, 0.0275f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_11000, "claude-3-haiku", 11000, 22000, 33000, 0.03025f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_12000, "claude-3-haiku", 12000, 24000, 36000, 0.033f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_13000, "claude-3-haiku", 13000, 26000, 39000, 0.035750000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_14000, "claude-3-haiku", 14000, 28000, 42000, 0.038500000000000006f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_15000, "claude-3-haiku", 15000, 30000, 45000, 0.041249999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_16000, "claude-3-haiku", 16000, 32000, 48000, 0.044f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_17000, "claude-3-haiku", 17000, 34000, 51000, 0.04675f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_18000, "claude-3-haiku", 18000, 36000, 54000, 0.049499999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_19000, "claude-3-haiku", 19000, 38000, 57000, 0.05225f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_20000, "claude-3-haiku", 20000, 40000, 60000, 0.055f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_21000, "claude-3-haiku", 21000, 42000, 63000, 0.057749999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_22000, "claude-3-haiku", 22000, 44000, 66000, 0.0605f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_23000, "claude-3-haiku", 23000, 46000, 69000, 0.06325f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_24000, "claude-3-haiku", 24000, 48000, 72000, 0.066f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_25000, "claude-3-haiku", 25000, 50000, 75000, 0.06875f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_26000, "claude-3-haiku", 26000, 52000, 78000, 0.07150000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_27000, "claude-3-haiku", 27000, 54000, 81000, 0.07425000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_28000, "claude-3-haiku", 28000, 56000, 84000, 0.07700000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_29000, "claude-3-haiku", 29000, 58000, 87000, 0.07975f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_30000, "claude-3-haiku", 30000, 60000, 90000, 0.08249999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_31000, "claude-3-haiku", 31000, 62000, 93000, 0.08524999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_32000, "claude-3-haiku", 32000, 64000, 96000, 0.088f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_33000, "claude-3-haiku", 33000, 66000, 99000, 0.09075f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_34000, "claude-3-haiku", 34000, 68000, 102000, 0.0935f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_35000, "claude-3-haiku", 35000, 70000, 105000, 0.09625f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_36000, "claude-3-haiku", 36000, 72000, 108000, 0.09899999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_37000, "claude-3-haiku", 37000, 74000, 111000, 0.10175f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_38000, "claude-3-haiku", 38000, 76000, 114000, 0.1045f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_39000, "claude-3-haiku", 39000, 78000, 117000, 0.10725f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_40000, "claude-3-haiku", 40000, 80000, 120000, 0.11f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_41000, "claude-3-haiku", 41000, 82000, 123000, 0.11274999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_42000, "claude-3-haiku", 42000, 84000, 126000, 0.11549999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_43000, "claude-3-haiku", 43000, 86000, 129000, 0.11825f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_44000, "claude-3-haiku", 44000, 88000, 132000, 0.121f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_45000, "claude-3-haiku", 45000, 90000, 135000, 0.12375f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_46000, "claude-3-haiku", 46000, 92000, 138000, 0.1265f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_47000, "claude-3-haiku", 47000, 94000, 141000, 0.12925f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_48000, "claude-3-haiku", 48000, 96000, 144000, 0.132f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_49000, "claude-3-haiku", 49000, 98000, 147000, 0.13475f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_50000, "claude-3-haiku", 50000, 100000, 150000, 0.1375f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_51000, "claude-3-haiku", 51000, 102000, 153000, 0.14025f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_52000, "claude-3-haiku", 52000, 104000, 156000, 0.14300000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_53000, "claude-3-haiku", 53000, 106000, 159000, 0.14575000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_54000, "claude-3-haiku", 54000, 108000, 162000, 0.14850000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_55000, "claude-3-haiku", 55000, 110000, 165000, 0.15125000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_56000, "claude-3-haiku", 56000, 112000, 168000, 0.15400000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_57000, "claude-3-haiku", 57000, 114000, 171000, 0.15675f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_58000, "claude-3-haiku", 58000, 116000, 174000, 0.1595f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_59000, "claude-3-haiku", 59000, 118000, 177000, 0.16225f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_60000, "claude-3-haiku", 60000, 120000, 180000, 0.16499999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_61000, "claude-3-haiku", 61000, 122000, 183000, 0.16775f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_62000, "claude-3-haiku", 62000, 124000, 186000, 0.17049999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_63000, "claude-3-haiku", 63000, 126000, 189000, 0.17325000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_64000, "claude-3-haiku", 64000, 128000, 192000, 0.176f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_65000, "claude-3-haiku", 65000, 130000, 195000, 0.17875000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_66000, "claude-3-haiku", 66000, 132000, 198000, 0.1815f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_67000, "claude-3-haiku", 67000, 134000, 201000, 0.18425000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_68000, "claude-3-haiku", 68000, 136000, 204000, 0.187f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_69000, "claude-3-haiku", 69000, 138000, 207000, 0.18974999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_70000, "claude-3-haiku", 70000, 140000, 210000, 0.1925f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_71000, "claude-3-haiku", 71000, 142000, 213000, 0.19524999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_72000, "claude-3-haiku", 72000, 144000, 216000, 0.19799999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_73000, "claude-3-haiku", 73000, 146000, 219000, 0.20074999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_74000, "claude-3-haiku", 74000, 148000, 222000, 0.2035f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_75000, "claude-3-haiku", 75000, 150000, 225000, 0.20625f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_76000, "claude-3-haiku", 76000, 152000, 228000, 0.209f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_77000, "claude-3-haiku", 77000, 154000, 231000, 0.21175f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_78000, "claude-3-haiku", 78000, 156000, 234000, 0.2145f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_79000, "claude-3-haiku", 79000, 158000, 237000, 0.21725f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_80000, "claude-3-haiku", 80000, 160000, 240000, 0.22f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_81000, "claude-3-haiku", 81000, 162000, 243000, 0.22275f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_82000, "claude-3-haiku", 82000, 164000, 246000, 0.22549999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_83000, "claude-3-haiku", 83000, 166000, 249000, 0.22824999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_84000, "claude-3-haiku", 84000, 168000, 252000, 0.23099999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_85000, "claude-3-haiku", 85000, 170000, 255000, 0.23374999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_86000, "claude-3-haiku", 86000, 172000, 258000, 0.2365f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_87000, "claude-3-haiku", 87000, 174000, 261000, 0.23925f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_88000, "claude-3-haiku", 88000, 176000, 264000, 0.242f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_89000, "claude-3-haiku", 89000, 178000, 267000, 0.24475f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_90000, "claude-3-haiku", 90000, 180000, 270000, 0.2475f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_91000, "claude-3-haiku", 91000, 182000, 273000, 0.25025000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_92000, "claude-3-haiku", 92000, 184000, 276000, 0.253f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_93000, "claude-3-haiku", 93000, 186000, 279000, 0.25575000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_94000, "claude-3-haiku", 94000, 188000, 282000, 0.2585f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_95000, "claude-3-haiku", 95000, 190000, 285000, 0.26125f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_96000, "claude-3-haiku", 96000, 192000, 288000, 0.264f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_97000, "claude-3-haiku", 97000, 194000, 291000, 0.26675f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_98000, "claude-3-haiku", 98000, 196000, 294000, 0.2695f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_99000, "claude-3-haiku", 99000, 198000, 297000, 0.27225f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_100000, "claude-3-haiku", 100000, 200000, 300000, 0.275f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_101000, "claude-3-haiku", 101000, 202000, 303000, 0.27775f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_102000, "claude-3-haiku", 102000, 204000, 306000, 0.2805f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_103000, "claude-3-haiku", 103000, 206000, 309000, 0.28325f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_104000, "claude-3-haiku", 104000, 208000, 312000, 0.28600000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_105000, "claude-3-haiku", 105000, 210000, 315000, 0.28875f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_106000, "claude-3-haiku", 106000, 212000, 318000, 0.29150000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_107000, "claude-3-haiku", 107000, 214000, 321000, 0.29425f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_108000, "claude-3-haiku", 108000, 216000, 324000, 0.29700000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_109000, "claude-3-haiku", 109000, 218000, 327000, 0.29975f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_110000, "claude-3-haiku", 110000, 220000, 330000, 0.30250000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_111000, "claude-3-haiku", 111000, 222000, 333000, 0.30525f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_112000, "claude-3-haiku", 112000, 224000, 336000, 0.30800000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_113000, "claude-3-haiku", 113000, 226000, 339000, 0.31074999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_114000, "claude-3-haiku", 114000, 228000, 342000, 0.3135f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_115000, "claude-3-haiku", 115000, 230000, 345000, 0.31625f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_116000, "claude-3-haiku", 116000, 232000, 348000, 0.319f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_117000, "claude-3-haiku", 117000, 234000, 351000, 0.32175f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_118000, "claude-3-haiku", 118000, 236000, 354000, 0.3245f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_119000, "claude-3-haiku", 119000, 238000, 357000, 0.32725f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_120000, "claude-3-haiku", 120000, 240000, 360000, 0.32999999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_121000, "claude-3-haiku", 121000, 242000, 363000, 0.33275f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_122000, "claude-3-haiku", 122000, 244000, 366000, 0.3355f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_123000, "claude-3-haiku", 123000, 246000, 369000, 0.33825f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_124000, "claude-3-haiku", 124000, 248000, 372000, 0.34099999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_125000, "claude-3-haiku", 125000, 250000, 375000, 0.34375f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_126000, "claude-3-haiku", 126000, 252000, 378000, 0.34650000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_127000, "claude-3-haiku", 127000, 254000, 381000, 0.34925f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_128000, "claude-3-haiku", 128000, 256000, 384000, 0.352f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_129000, "claude-3-haiku", 129000, 258000, 387000, 0.35475f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_130000, "claude-3-haiku", 130000, 260000, 390000, 0.35750000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_131000, "claude-3-haiku", 131000, 262000, 393000, 0.36025f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_132000, "claude-3-haiku", 132000, 264000, 396000, 0.363f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_133000, "claude-3-haiku", 133000, 266000, 399000, 0.36575f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_134000, "claude-3-haiku", 134000, 268000, 402000, 0.36850000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_135000, "claude-3-haiku", 135000, 270000, 405000, 0.37125f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_136000, "claude-3-haiku", 136000, 272000, 408000, 0.374f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_137000, "claude-3-haiku", 137000, 274000, 411000, 0.37675000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_138000, "claude-3-haiku", 138000, 276000, 414000, 0.37949999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_139000, "claude-3-haiku", 139000, 278000, 417000, 0.38225f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_140000, "claude-3-haiku", 140000, 280000, 420000, 0.385f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_141000, "claude-3-haiku", 141000, 282000, 423000, 0.38775f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_142000, "claude-3-haiku", 142000, 284000, 426000, 0.39049999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_143000, "claude-3-haiku", 143000, 286000, 429000, 0.39325f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_144000, "claude-3-haiku", 144000, 288000, 432000, 0.39599999999999996f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_145000, "claude-3-haiku", 145000, 290000, 435000, 0.39875f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_146000, "claude-3-haiku", 146000, 292000, 438000, 0.40149999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_147000, "claude-3-haiku", 147000, 294000, 441000, 0.40425f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_148000, "claude-3-haiku", 148000, 296000, 444000, 0.407f64);
    generate_cost_test!(test_cost_matrix_claude_3_haiku_149000, "claude-3-haiku", 149000, 298000, 447000, 0.40975f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_1000, "claude-3.5-sonnet", 1000, 2000, 3000, 0.0339f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_2000, "claude-3.5-sonnet", 2000, 4000, 6000, 0.0678f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_3000, "claude-3.5-sonnet", 3000, 6000, 9000, 0.10169999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_4000, "claude-3.5-sonnet", 4000, 8000, 12000, 0.1356f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_5000, "claude-3.5-sonnet", 5000, 10000, 15000, 0.16949999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_6000, "claude-3.5-sonnet", 6000, 12000, 18000, 0.20339999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_7000, "claude-3.5-sonnet", 7000, 14000, 21000, 0.23729999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_8000, "claude-3.5-sonnet", 8000, 16000, 24000, 0.2712f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_9000, "claude-3.5-sonnet", 9000, 18000, 27000, 0.30510000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_10000, "claude-3.5-sonnet", 10000, 20000, 30000, 0.33899999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_11000, "claude-3.5-sonnet", 11000, 22000, 33000, 0.3729f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_12000, "claude-3.5-sonnet", 12000, 24000, 36000, 0.40679999999999994f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_13000, "claude-3.5-sonnet", 13000, 26000, 39000, 0.4407f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_14000, "claude-3.5-sonnet", 14000, 28000, 42000, 0.47459999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_15000, "claude-3.5-sonnet", 15000, 30000, 45000, 0.5085f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_16000, "claude-3.5-sonnet", 16000, 32000, 48000, 0.5424f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_17000, "claude-3.5-sonnet", 17000, 34000, 51000, 0.5763f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_18000, "claude-3.5-sonnet", 18000, 36000, 54000, 0.6102000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_19000, "claude-3.5-sonnet", 19000, 38000, 57000, 0.6441f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_20000, "claude-3.5-sonnet", 20000, 40000, 60000, 0.6779999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_21000, "claude-3.5-sonnet", 21000, 42000, 63000, 0.7119000000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_22000, "claude-3.5-sonnet", 22000, 44000, 66000, 0.7458f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_23000, "claude-3.5-sonnet", 23000, 46000, 69000, 0.7797f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_24000, "claude-3.5-sonnet", 24000, 48000, 72000, 0.8135999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_25000, "claude-3.5-sonnet", 25000, 50000, 75000, 0.8474999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_26000, "claude-3.5-sonnet", 26000, 52000, 78000, 0.8814f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_27000, "claude-3.5-sonnet", 27000, 54000, 81000, 0.9153f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_28000, "claude-3.5-sonnet", 28000, 56000, 84000, 0.9491999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_29000, "claude-3.5-sonnet", 29000, 58000, 87000, 0.9831f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_30000, "claude-3.5-sonnet", 30000, 60000, 90000, 1.017f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_31000, "claude-3.5-sonnet", 31000, 62000, 93000, 1.0509000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_32000, "claude-3.5-sonnet", 32000, 64000, 96000, 1.0848f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_33000, "claude-3.5-sonnet", 33000, 66000, 99000, 1.1187f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_34000, "claude-3.5-sonnet", 34000, 68000, 102000, 1.1526f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_35000, "claude-3.5-sonnet", 35000, 70000, 105000, 1.1865f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_36000, "claude-3.5-sonnet", 36000, 72000, 108000, 1.2204000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_37000, "claude-3.5-sonnet", 37000, 74000, 111000, 1.2543000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_38000, "claude-3.5-sonnet", 38000, 76000, 114000, 1.2882f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_39000, "claude-3.5-sonnet", 39000, 78000, 117000, 1.3220999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_40000, "claude-3.5-sonnet", 40000, 80000, 120000, 1.3559999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_41000, "claude-3.5-sonnet", 41000, 82000, 123000, 1.3899f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_42000, "claude-3.5-sonnet", 42000, 84000, 126000, 1.4238000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_43000, "claude-3.5-sonnet", 43000, 86000, 129000, 1.4577f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_44000, "claude-3.5-sonnet", 44000, 88000, 132000, 1.4916f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_45000, "claude-3.5-sonnet", 45000, 90000, 135000, 1.5255f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_46000, "claude-3.5-sonnet", 46000, 92000, 138000, 1.5594f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_47000, "claude-3.5-sonnet", 47000, 94000, 141000, 1.5933f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_48000, "claude-3.5-sonnet", 48000, 96000, 144000, 1.6271999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_49000, "claude-3.5-sonnet", 49000, 98000, 147000, 1.6611f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_50000, "claude-3.5-sonnet", 50000, 100000, 150000, 1.6949999999999998f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_51000, "claude-3.5-sonnet", 51000, 102000, 153000, 1.7289f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_52000, "claude-3.5-sonnet", 52000, 104000, 156000, 1.7628f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_53000, "claude-3.5-sonnet", 53000, 106000, 159000, 1.7967000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_54000, "claude-3.5-sonnet", 54000, 108000, 162000, 1.8306f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_55000, "claude-3.5-sonnet", 55000, 110000, 165000, 1.8645f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_56000, "claude-3.5-sonnet", 56000, 112000, 168000, 1.8983999999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_57000, "claude-3.5-sonnet", 57000, 114000, 171000, 1.9323f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_58000, "claude-3.5-sonnet", 58000, 116000, 174000, 1.9662f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_59000, "claude-3.5-sonnet", 59000, 118000, 177000, 2.0001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_60000, "claude-3.5-sonnet", 60000, 120000, 180000, 2.034f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_61000, "claude-3.5-sonnet", 61000, 122000, 183000, 2.0679f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_62000, "claude-3.5-sonnet", 62000, 124000, 186000, 2.1018000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_63000, "claude-3.5-sonnet", 63000, 126000, 189000, 2.1357f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_64000, "claude-3.5-sonnet", 64000, 128000, 192000, 2.1696f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_65000, "claude-3.5-sonnet", 65000, 130000, 195000, 2.2035f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_66000, "claude-3.5-sonnet", 66000, 132000, 198000, 2.2374f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_67000, "claude-3.5-sonnet", 67000, 134000, 201000, 2.2712999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_68000, "claude-3.5-sonnet", 68000, 136000, 204000, 2.3052f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_69000, "claude-3.5-sonnet", 69000, 138000, 207000, 2.3390999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_70000, "claude-3.5-sonnet", 70000, 140000, 210000, 2.373f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_71000, "claude-3.5-sonnet", 71000, 142000, 213000, 2.4069f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_72000, "claude-3.5-sonnet", 72000, 144000, 216000, 2.4408000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_73000, "claude-3.5-sonnet", 73000, 146000, 219000, 2.4747f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_74000, "claude-3.5-sonnet", 74000, 148000, 222000, 2.5086000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_75000, "claude-3.5-sonnet", 75000, 150000, 225000, 2.5425f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_76000, "claude-3.5-sonnet", 76000, 152000, 228000, 2.5764f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_77000, "claude-3.5-sonnet", 77000, 154000, 231000, 2.6103f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_78000, "claude-3.5-sonnet", 78000, 156000, 234000, 2.6441999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_79000, "claude-3.5-sonnet", 79000, 158000, 237000, 2.6781f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_80000, "claude-3.5-sonnet", 80000, 160000, 240000, 2.7119999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_81000, "claude-3.5-sonnet", 81000, 162000, 243000, 2.7459000000000002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_82000, "claude-3.5-sonnet", 82000, 164000, 246000, 2.7798f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_83000, "claude-3.5-sonnet", 83000, 166000, 249000, 2.8137000000000003f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_84000, "claude-3.5-sonnet", 84000, 168000, 252000, 2.8476000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_85000, "claude-3.5-sonnet", 85000, 170000, 255000, 2.8814999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_86000, "claude-3.5-sonnet", 86000, 172000, 258000, 2.9154f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_87000, "claude-3.5-sonnet", 87000, 174000, 261000, 2.9493f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_88000, "claude-3.5-sonnet", 88000, 176000, 264000, 2.9832f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_89000, "claude-3.5-sonnet", 89000, 178000, 267000, 3.0170999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_90000, "claude-3.5-sonnet", 90000, 180000, 270000, 3.051f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_91000, "claude-3.5-sonnet", 91000, 182000, 273000, 3.0849f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_92000, "claude-3.5-sonnet", 92000, 184000, 276000, 3.1188f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_93000, "claude-3.5-sonnet", 93000, 186000, 279000, 3.1527f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_94000, "claude-3.5-sonnet", 94000, 188000, 282000, 3.1866f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_95000, "claude-3.5-sonnet", 95000, 190000, 285000, 3.2205000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_96000, "claude-3.5-sonnet", 96000, 192000, 288000, 3.2543999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_97000, "claude-3.5-sonnet", 97000, 194000, 291000, 3.2883f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_98000, "claude-3.5-sonnet", 98000, 196000, 294000, 3.3222f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_99000, "claude-3.5-sonnet", 99000, 198000, 297000, 3.3561000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_100000, "claude-3.5-sonnet", 100000, 200000, 300000, 3.3899999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_101000, "claude-3.5-sonnet", 101000, 202000, 303000, 3.4238999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_102000, "claude-3.5-sonnet", 102000, 204000, 306000, 3.4578f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_103000, "claude-3.5-sonnet", 103000, 206000, 309000, 3.4917f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_104000, "claude-3.5-sonnet", 104000, 208000, 312000, 3.5256f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_105000, "claude-3.5-sonnet", 105000, 210000, 315000, 3.5595f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_106000, "claude-3.5-sonnet", 106000, 212000, 318000, 3.5934000000000004f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_107000, "claude-3.5-sonnet", 107000, 214000, 321000, 3.6273f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_108000, "claude-3.5-sonnet", 108000, 216000, 324000, 3.6612f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_109000, "claude-3.5-sonnet", 109000, 218000, 327000, 3.6951f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_110000, "claude-3.5-sonnet", 110000, 220000, 330000, 3.729f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_111000, "claude-3.5-sonnet", 111000, 222000, 333000, 3.7629f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_112000, "claude-3.5-sonnet", 112000, 224000, 336000, 3.7967999999999997f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_113000, "claude-3.5-sonnet", 113000, 226000, 339000, 3.8307f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_114000, "claude-3.5-sonnet", 114000, 228000, 342000, 3.8646f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_115000, "claude-3.5-sonnet", 115000, 230000, 345000, 3.8985f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_116000, "claude-3.5-sonnet", 116000, 232000, 348000, 3.9324f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_117000, "claude-3.5-sonnet", 117000, 234000, 351000, 3.9663f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_118000, "claude-3.5-sonnet", 118000, 236000, 354000, 4.0002f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_119000, "claude-3.5-sonnet", 119000, 238000, 357000, 4.0341f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_120000, "claude-3.5-sonnet", 120000, 240000, 360000, 4.068f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_121000, "claude-3.5-sonnet", 121000, 242000, 363000, 4.1019f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_122000, "claude-3.5-sonnet", 122000, 244000, 366000, 4.1358f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_123000, "claude-3.5-sonnet", 123000, 246000, 369000, 4.1697f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_124000, "claude-3.5-sonnet", 124000, 248000, 372000, 4.203600000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_125000, "claude-3.5-sonnet", 125000, 250000, 375000, 4.2375f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_126000, "claude-3.5-sonnet", 126000, 252000, 378000, 4.2714f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_127000, "claude-3.5-sonnet", 127000, 254000, 381000, 4.3053f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_128000, "claude-3.5-sonnet", 128000, 256000, 384000, 4.3392f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_129000, "claude-3.5-sonnet", 129000, 258000, 387000, 4.3731f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_130000, "claude-3.5-sonnet", 130000, 260000, 390000, 4.407f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_131000, "claude-3.5-sonnet", 131000, 262000, 393000, 4.4409f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_132000, "claude-3.5-sonnet", 132000, 264000, 396000, 4.4748f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_133000, "claude-3.5-sonnet", 133000, 266000, 399000, 4.5087f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_134000, "claude-3.5-sonnet", 134000, 268000, 402000, 4.542599999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_135000, "claude-3.5-sonnet", 135000, 270000, 405000, 4.5765f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_136000, "claude-3.5-sonnet", 136000, 272000, 408000, 4.6104f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_137000, "claude-3.5-sonnet", 137000, 274000, 411000, 4.6443f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_138000, "claude-3.5-sonnet", 138000, 276000, 414000, 4.6781999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_139000, "claude-3.5-sonnet", 139000, 278000, 417000, 4.7120999999999995f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_140000, "claude-3.5-sonnet", 140000, 280000, 420000, 4.746f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_141000, "claude-3.5-sonnet", 141000, 282000, 423000, 4.7799000000000005f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_142000, "claude-3.5-sonnet", 142000, 284000, 426000, 4.8138f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_143000, "claude-3.5-sonnet", 143000, 286000, 429000, 4.847700000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_144000, "claude-3.5-sonnet", 144000, 288000, 432000, 4.881600000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_145000, "claude-3.5-sonnet", 145000, 290000, 435000, 4.915499999999999f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_146000, "claude-3.5-sonnet", 146000, 292000, 438000, 4.9494f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_147000, "claude-3.5-sonnet", 147000, 294000, 441000, 4.9833f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_148000, "claude-3.5-sonnet", 148000, 296000, 444000, 5.017200000000001f64);
    generate_cost_test!(test_cost_matrix_claude_3_5_sonnet_149000, "claude-3.5-sonnet", 149000, 298000, 447000, 5.0511f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_1000, "gpt-4o", 1000, 2000, 3000, 0.042499999999999996f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_2000, "gpt-4o", 2000, 4000, 6000, 0.08499999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_3000, "gpt-4o", 3000, 6000, 9000, 0.1275f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_4000, "gpt-4o", 4000, 8000, 12000, 0.16999999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_5000, "gpt-4o", 5000, 10000, 15000, 0.2125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_6000, "gpt-4o", 6000, 12000, 18000, 0.255f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_7000, "gpt-4o", 7000, 14000, 21000, 0.2975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_8000, "gpt-4o", 8000, 16000, 24000, 0.33999999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_9000, "gpt-4o", 9000, 18000, 27000, 0.3825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_10000, "gpt-4o", 10000, 20000, 30000, 0.425f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_11000, "gpt-4o", 11000, 22000, 33000, 0.4675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_12000, "gpt-4o", 12000, 24000, 36000, 0.51f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_13000, "gpt-4o", 13000, 26000, 39000, 0.5525f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_14000, "gpt-4o", 14000, 28000, 42000, 0.595f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_15000, "gpt-4o", 15000, 30000, 45000, 0.6375000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_16000, "gpt-4o", 16000, 32000, 48000, 0.6799999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_17000, "gpt-4o", 17000, 34000, 51000, 0.7224999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_18000, "gpt-4o", 18000, 36000, 54000, 0.765f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_19000, "gpt-4o", 19000, 38000, 57000, 0.8074999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_20000, "gpt-4o", 20000, 40000, 60000, 0.85f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_21000, "gpt-4o", 21000, 42000, 63000, 0.8925f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_22000, "gpt-4o", 22000, 44000, 66000, 0.935f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_23000, "gpt-4o", 23000, 46000, 69000, 0.9774999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_24000, "gpt-4o", 24000, 48000, 72000, 1.02f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_25000, "gpt-4o", 25000, 50000, 75000, 1.0625f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_26000, "gpt-4o", 26000, 52000, 78000, 1.105f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_27000, "gpt-4o", 27000, 54000, 81000, 1.1475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_28000, "gpt-4o", 28000, 56000, 84000, 1.19f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_29000, "gpt-4o", 29000, 58000, 87000, 1.2325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_30000, "gpt-4o", 30000, 60000, 90000, 1.2750000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_31000, "gpt-4o", 31000, 62000, 93000, 1.3175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_32000, "gpt-4o", 32000, 64000, 96000, 1.3599999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_33000, "gpt-4o", 33000, 66000, 99000, 1.4025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_34000, "gpt-4o", 34000, 68000, 102000, 1.4449999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_35000, "gpt-4o", 35000, 70000, 105000, 1.4875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_36000, "gpt-4o", 36000, 72000, 108000, 1.53f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_37000, "gpt-4o", 37000, 74000, 111000, 1.5725000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_38000, "gpt-4o", 38000, 76000, 114000, 1.6149999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_39000, "gpt-4o", 39000, 78000, 117000, 1.6575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_40000, "gpt-4o", 40000, 80000, 120000, 1.7f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_41000, "gpt-4o", 41000, 82000, 123000, 1.7425000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_42000, "gpt-4o", 42000, 84000, 126000, 1.785f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_43000, "gpt-4o", 43000, 86000, 129000, 1.8275000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_44000, "gpt-4o", 44000, 88000, 132000, 1.87f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_45000, "gpt-4o", 45000, 90000, 135000, 1.9125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_46000, "gpt-4o", 46000, 92000, 138000, 1.9549999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_47000, "gpt-4o", 47000, 94000, 141000, 1.9975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_48000, "gpt-4o", 48000, 96000, 144000, 2.04f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_49000, "gpt-4o", 49000, 98000, 147000, 2.0825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_50000, "gpt-4o", 50000, 100000, 150000, 2.125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_51000, "gpt-4o", 51000, 102000, 153000, 2.1675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_52000, "gpt-4o", 52000, 104000, 156000, 2.21f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_53000, "gpt-4o", 53000, 106000, 159000, 2.2525f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_54000, "gpt-4o", 54000, 108000, 162000, 2.295f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_55000, "gpt-4o", 55000, 110000, 165000, 2.3375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_56000, "gpt-4o", 56000, 112000, 168000, 2.38f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_57000, "gpt-4o", 57000, 114000, 171000, 2.4225f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_58000, "gpt-4o", 58000, 116000, 174000, 2.465f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_59000, "gpt-4o", 59000, 118000, 177000, 2.5075f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_60000, "gpt-4o", 60000, 120000, 180000, 2.5500000000000003f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_61000, "gpt-4o", 61000, 122000, 183000, 2.5925000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_62000, "gpt-4o", 62000, 124000, 186000, 2.635f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_63000, "gpt-4o", 63000, 126000, 189000, 2.6775f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_64000, "gpt-4o", 64000, 128000, 192000, 2.7199999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_65000, "gpt-4o", 65000, 130000, 195000, 2.7624999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_66000, "gpt-4o", 66000, 132000, 198000, 2.805f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_67000, "gpt-4o", 67000, 134000, 201000, 2.8474999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_68000, "gpt-4o", 68000, 136000, 204000, 2.8899999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_69000, "gpt-4o", 69000, 138000, 207000, 2.9325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_70000, "gpt-4o", 70000, 140000, 210000, 2.975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_71000, "gpt-4o", 71000, 142000, 213000, 3.0175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_72000, "gpt-4o", 72000, 144000, 216000, 3.06f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_73000, "gpt-4o", 73000, 146000, 219000, 3.1024999999999996f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_74000, "gpt-4o", 74000, 148000, 222000, 3.1450000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_75000, "gpt-4o", 75000, 150000, 225000, 3.1875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_76000, "gpt-4o", 76000, 152000, 228000, 3.2299999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_77000, "gpt-4o", 77000, 154000, 231000, 3.2725000000000004f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_78000, "gpt-4o", 78000, 156000, 234000, 3.315f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_79000, "gpt-4o", 79000, 158000, 237000, 3.3575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_80000, "gpt-4o", 80000, 160000, 240000, 3.4f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_81000, "gpt-4o", 81000, 162000, 243000, 3.4425f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_82000, "gpt-4o", 82000, 164000, 246000, 3.4850000000000003f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_83000, "gpt-4o", 83000, 166000, 249000, 3.5275000000000003f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_84000, "gpt-4o", 84000, 168000, 252000, 3.57f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_85000, "gpt-4o", 85000, 170000, 255000, 3.6125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_86000, "gpt-4o", 86000, 172000, 258000, 3.6550000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_87000, "gpt-4o", 87000, 174000, 261000, 3.6975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_88000, "gpt-4o", 88000, 176000, 264000, 3.74f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_89000, "gpt-4o", 89000, 178000, 267000, 3.7824999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_90000, "gpt-4o", 90000, 180000, 270000, 3.825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_91000, "gpt-4o", 91000, 182000, 273000, 3.8675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_92000, "gpt-4o", 92000, 184000, 276000, 3.9099999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_93000, "gpt-4o", 93000, 186000, 279000, 3.9524999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_94000, "gpt-4o", 94000, 188000, 282000, 3.995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_95000, "gpt-4o", 95000, 190000, 285000, 4.0375000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_96000, "gpt-4o", 96000, 192000, 288000, 4.08f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_97000, "gpt-4o", 97000, 194000, 291000, 4.1225000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_98000, "gpt-4o", 98000, 196000, 294000, 4.165f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_99000, "gpt-4o", 99000, 198000, 297000, 4.2075000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_100000, "gpt-4o", 100000, 200000, 300000, 4.25f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_101000, "gpt-4o", 101000, 202000, 303000, 4.2924999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_102000, "gpt-4o", 102000, 204000, 306000, 4.335f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_103000, "gpt-4o", 103000, 206000, 309000, 4.3774999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_104000, "gpt-4o", 104000, 208000, 312000, 4.42f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_105000, "gpt-4o", 105000, 210000, 315000, 4.4624999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_106000, "gpt-4o", 106000, 212000, 318000, 4.505f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_107000, "gpt-4o", 107000, 214000, 321000, 4.5475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_108000, "gpt-4o", 108000, 216000, 324000, 4.59f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_109000, "gpt-4o", 109000, 218000, 327000, 4.6325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_110000, "gpt-4o", 110000, 220000, 330000, 4.675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_111000, "gpt-4o", 111000, 222000, 333000, 4.7175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_112000, "gpt-4o", 112000, 224000, 336000, 4.76f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_113000, "gpt-4o", 113000, 226000, 339000, 4.8025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_114000, "gpt-4o", 114000, 228000, 342000, 4.845f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_115000, "gpt-4o", 115000, 230000, 345000, 4.8875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_116000, "gpt-4o", 116000, 232000, 348000, 4.93f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_117000, "gpt-4o", 117000, 234000, 351000, 4.9725f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_118000, "gpt-4o", 118000, 236000, 354000, 5.015f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_119000, "gpt-4o", 119000, 238000, 357000, 5.0575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_120000, "gpt-4o", 120000, 240000, 360000, 5.1000000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_121000, "gpt-4o", 121000, 242000, 363000, 5.142499999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_122000, "gpt-4o", 122000, 244000, 366000, 5.1850000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_123000, "gpt-4o", 123000, 246000, 369000, 5.2275f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_124000, "gpt-4o", 124000, 248000, 372000, 5.27f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_125000, "gpt-4o", 125000, 250000, 375000, 5.3125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_126000, "gpt-4o", 126000, 252000, 378000, 5.355f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_127000, "gpt-4o", 127000, 254000, 381000, 5.3975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_128000, "gpt-4o", 128000, 256000, 384000, 5.4399999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_129000, "gpt-4o", 129000, 258000, 387000, 5.482500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_130000, "gpt-4o", 130000, 260000, 390000, 5.5249999999999995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_131000, "gpt-4o", 131000, 262000, 393000, 5.5675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_132000, "gpt-4o", 132000, 264000, 396000, 5.61f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_133000, "gpt-4o", 133000, 266000, 399000, 5.6525f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_134000, "gpt-4o", 134000, 268000, 402000, 5.694999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_135000, "gpt-4o", 135000, 270000, 405000, 5.7375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_136000, "gpt-4o", 136000, 272000, 408000, 5.779999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_137000, "gpt-4o", 137000, 274000, 411000, 5.8225f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_138000, "gpt-4o", 138000, 276000, 414000, 5.865f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_139000, "gpt-4o", 139000, 278000, 417000, 5.907500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_140000, "gpt-4o", 140000, 280000, 420000, 5.95f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_141000, "gpt-4o", 141000, 282000, 423000, 5.992500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_142000, "gpt-4o", 142000, 284000, 426000, 6.035f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_143000, "gpt-4o", 143000, 286000, 429000, 6.0775f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_144000, "gpt-4o", 144000, 288000, 432000, 6.12f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_145000, "gpt-4o", 145000, 290000, 435000, 6.1625f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_146000, "gpt-4o", 146000, 292000, 438000, 6.204999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_147000, "gpt-4o", 147000, 294000, 441000, 6.2475000000000005f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_148000, "gpt-4o", 148000, 296000, 444000, 6.290000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_149000, "gpt-4o", 149000, 298000, 447000, 6.3325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_1000, "gpt-4o-mini", 1000, 2000, 3000, 0.0015749999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_2000, "gpt-4o-mini", 2000, 4000, 6000, 0.0031499999999999996f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_3000, "gpt-4o-mini", 3000, 6000, 9000, 0.004725f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_4000, "gpt-4o-mini", 4000, 8000, 12000, 0.006299999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_5000, "gpt-4o-mini", 5000, 10000, 15000, 0.007875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_6000, "gpt-4o-mini", 6000, 12000, 18000, 0.00945f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_7000, "gpt-4o-mini", 7000, 14000, 21000, 0.011025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_8000, "gpt-4o-mini", 8000, 16000, 24000, 0.012599999999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_9000, "gpt-4o-mini", 9000, 18000, 27000, 0.014175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_10000, "gpt-4o-mini", 10000, 20000, 30000, 0.01575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_11000, "gpt-4o-mini", 11000, 22000, 33000, 0.017325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_12000, "gpt-4o-mini", 12000, 24000, 36000, 0.0189f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_13000, "gpt-4o-mini", 13000, 26000, 39000, 0.020475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_14000, "gpt-4o-mini", 14000, 28000, 42000, 0.02205f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_15000, "gpt-4o-mini", 15000, 30000, 45000, 0.023624999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_16000, "gpt-4o-mini", 16000, 32000, 48000, 0.025199999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_17000, "gpt-4o-mini", 17000, 34000, 51000, 0.026775f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_18000, "gpt-4o-mini", 18000, 36000, 54000, 0.02835f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_19000, "gpt-4o-mini", 19000, 38000, 57000, 0.029925f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_20000, "gpt-4o-mini", 20000, 40000, 60000, 0.0315f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_21000, "gpt-4o-mini", 21000, 42000, 63000, 0.033075f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_22000, "gpt-4o-mini", 22000, 44000, 66000, 0.03465f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_23000, "gpt-4o-mini", 23000, 46000, 69000, 0.036225f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_24000, "gpt-4o-mini", 24000, 48000, 72000, 0.0378f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_25000, "gpt-4o-mini", 25000, 50000, 75000, 0.039375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_26000, "gpt-4o-mini", 26000, 52000, 78000, 0.04095f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_27000, "gpt-4o-mini", 27000, 54000, 81000, 0.04252499999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_28000, "gpt-4o-mini", 28000, 56000, 84000, 0.0441f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_29000, "gpt-4o-mini", 29000, 58000, 87000, 0.045675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_30000, "gpt-4o-mini", 30000, 60000, 90000, 0.04724999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_31000, "gpt-4o-mini", 31000, 62000, 93000, 0.048825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_32000, "gpt-4o-mini", 32000, 64000, 96000, 0.05039999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_33000, "gpt-4o-mini", 33000, 66000, 99000, 0.05197500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_34000, "gpt-4o-mini", 34000, 68000, 102000, 0.05355f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_35000, "gpt-4o-mini", 35000, 70000, 105000, 0.055125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_36000, "gpt-4o-mini", 36000, 72000, 108000, 0.0567f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_37000, "gpt-4o-mini", 37000, 74000, 111000, 0.058275f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_38000, "gpt-4o-mini", 38000, 76000, 114000, 0.05985f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_39000, "gpt-4o-mini", 39000, 78000, 117000, 0.061425f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_40000, "gpt-4o-mini", 40000, 80000, 120000, 0.063f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_41000, "gpt-4o-mini", 41000, 82000, 123000, 0.06457500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_42000, "gpt-4o-mini", 42000, 84000, 126000, 0.06615f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_43000, "gpt-4o-mini", 43000, 86000, 129000, 0.067725f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_44000, "gpt-4o-mini", 44000, 88000, 132000, 0.0693f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_45000, "gpt-4o-mini", 45000, 90000, 135000, 0.070875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_46000, "gpt-4o-mini", 46000, 92000, 138000, 0.07245f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_47000, "gpt-4o-mini", 47000, 94000, 141000, 0.074025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_48000, "gpt-4o-mini", 48000, 96000, 144000, 0.0756f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_49000, "gpt-4o-mini", 49000, 98000, 147000, 0.077175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_50000, "gpt-4o-mini", 50000, 100000, 150000, 0.07875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_51000, "gpt-4o-mini", 51000, 102000, 153000, 0.080325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_52000, "gpt-4o-mini", 52000, 104000, 156000, 0.0819f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_53000, "gpt-4o-mini", 53000, 106000, 159000, 0.08347500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_54000, "gpt-4o-mini", 54000, 108000, 162000, 0.08504999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_55000, "gpt-4o-mini", 55000, 110000, 165000, 0.08662500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_56000, "gpt-4o-mini", 56000, 112000, 168000, 0.0882f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_57000, "gpt-4o-mini", 57000, 114000, 171000, 0.08977500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_58000, "gpt-4o-mini", 58000, 116000, 174000, 0.09135f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_59000, "gpt-4o-mini", 59000, 118000, 177000, 0.092925f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_60000, "gpt-4o-mini", 60000, 120000, 180000, 0.09449999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_61000, "gpt-4o-mini", 61000, 122000, 183000, 0.09607500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_62000, "gpt-4o-mini", 62000, 124000, 186000, 0.09765f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_63000, "gpt-4o-mini", 63000, 126000, 189000, 0.09922500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_64000, "gpt-4o-mini", 64000, 128000, 192000, 0.10079999999999999f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_65000, "gpt-4o-mini", 65000, 130000, 195000, 0.102375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_66000, "gpt-4o-mini", 66000, 132000, 198000, 0.10395000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_67000, "gpt-4o-mini", 67000, 134000, 201000, 0.10552500000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_68000, "gpt-4o-mini", 68000, 136000, 204000, 0.1071f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_69000, "gpt-4o-mini", 69000, 138000, 207000, 0.108675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_70000, "gpt-4o-mini", 70000, 140000, 210000, 0.11025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_71000, "gpt-4o-mini", 71000, 142000, 213000, 0.111825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_72000, "gpt-4o-mini", 72000, 144000, 216000, 0.1134f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_73000, "gpt-4o-mini", 73000, 146000, 219000, 0.114975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_74000, "gpt-4o-mini", 74000, 148000, 222000, 0.11655f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_75000, "gpt-4o-mini", 75000, 150000, 225000, 0.118125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_76000, "gpt-4o-mini", 76000, 152000, 228000, 0.1197f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_77000, "gpt-4o-mini", 77000, 154000, 231000, 0.121275f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_78000, "gpt-4o-mini", 78000, 156000, 234000, 0.12285f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_79000, "gpt-4o-mini", 79000, 158000, 237000, 0.124425f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_80000, "gpt-4o-mini", 80000, 160000, 240000, 0.126f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_81000, "gpt-4o-mini", 81000, 162000, 243000, 0.127575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_82000, "gpt-4o-mini", 82000, 164000, 246000, 0.12915000000000001f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_83000, "gpt-4o-mini", 83000, 166000, 249000, 0.130725f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_84000, "gpt-4o-mini", 84000, 168000, 252000, 0.1323f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_85000, "gpt-4o-mini", 85000, 170000, 255000, 0.133875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_86000, "gpt-4o-mini", 86000, 172000, 258000, 0.13545f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_87000, "gpt-4o-mini", 87000, 174000, 261000, 0.137025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_88000, "gpt-4o-mini", 88000, 176000, 264000, 0.1386f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_89000, "gpt-4o-mini", 89000, 178000, 267000, 0.140175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_90000, "gpt-4o-mini", 90000, 180000, 270000, 0.14175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_91000, "gpt-4o-mini", 91000, 182000, 273000, 0.143325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_92000, "gpt-4o-mini", 92000, 184000, 276000, 0.1449f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_93000, "gpt-4o-mini", 93000, 186000, 279000, 0.146475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_94000, "gpt-4o-mini", 94000, 188000, 282000, 0.14805f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_95000, "gpt-4o-mini", 95000, 190000, 285000, 0.149625f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_96000, "gpt-4o-mini", 96000, 192000, 288000, 0.1512f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_97000, "gpt-4o-mini", 97000, 194000, 291000, 0.15277500000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_98000, "gpt-4o-mini", 98000, 196000, 294000, 0.15435f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_99000, "gpt-4o-mini", 99000, 198000, 297000, 0.15592499999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_100000, "gpt-4o-mini", 100000, 200000, 300000, 0.1575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_101000, "gpt-4o-mini", 101000, 202000, 303000, 0.159075f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_102000, "gpt-4o-mini", 102000, 204000, 306000, 0.16065f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_103000, "gpt-4o-mini", 103000, 206000, 309000, 0.162225f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_104000, "gpt-4o-mini", 104000, 208000, 312000, 0.1638f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_105000, "gpt-4o-mini", 105000, 210000, 315000, 0.165375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_106000, "gpt-4o-mini", 106000, 212000, 318000, 0.16695000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_107000, "gpt-4o-mini", 107000, 214000, 321000, 0.16852499999999998f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_108000, "gpt-4o-mini", 108000, 216000, 324000, 0.17009999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_109000, "gpt-4o-mini", 109000, 218000, 327000, 0.171675f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_110000, "gpt-4o-mini", 110000, 220000, 330000, 0.17325000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_111000, "gpt-4o-mini", 111000, 222000, 333000, 0.174825f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_112000, "gpt-4o-mini", 112000, 224000, 336000, 0.1764f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_113000, "gpt-4o-mini", 113000, 226000, 339000, 0.177975f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_114000, "gpt-4o-mini", 114000, 228000, 342000, 0.17955000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_115000, "gpt-4o-mini", 115000, 230000, 345000, 0.181125f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_116000, "gpt-4o-mini", 116000, 232000, 348000, 0.1827f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_117000, "gpt-4o-mini", 117000, 234000, 351000, 0.18427500000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_118000, "gpt-4o-mini", 118000, 236000, 354000, 0.18585f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_119000, "gpt-4o-mini", 119000, 238000, 357000, 0.187425f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_120000, "gpt-4o-mini", 120000, 240000, 360000, 0.18899999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_121000, "gpt-4o-mini", 121000, 242000, 363000, 0.190575f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_122000, "gpt-4o-mini", 122000, 244000, 366000, 0.19215000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_123000, "gpt-4o-mini", 123000, 246000, 369000, 0.193725f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_124000, "gpt-4o-mini", 124000, 248000, 372000, 0.1953f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_125000, "gpt-4o-mini", 125000, 250000, 375000, 0.196875f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_126000, "gpt-4o-mini", 126000, 252000, 378000, 0.19845000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_127000, "gpt-4o-mini", 127000, 254000, 381000, 0.200025f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_128000, "gpt-4o-mini", 128000, 256000, 384000, 0.20159999999999997f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_129000, "gpt-4o-mini", 129000, 258000, 387000, 0.203175f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_130000, "gpt-4o-mini", 130000, 260000, 390000, 0.20475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_131000, "gpt-4o-mini", 131000, 262000, 393000, 0.206325f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_132000, "gpt-4o-mini", 132000, 264000, 396000, 0.20790000000000003f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_133000, "gpt-4o-mini", 133000, 266000, 399000, 0.209475f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_134000, "gpt-4o-mini", 134000, 268000, 402000, 0.21105000000000002f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_135000, "gpt-4o-mini", 135000, 270000, 405000, 0.212625f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_136000, "gpt-4o-mini", 136000, 272000, 408000, 0.2142f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_137000, "gpt-4o-mini", 137000, 274000, 411000, 0.215775f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_138000, "gpt-4o-mini", 138000, 276000, 414000, 0.21735f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_139000, "gpt-4o-mini", 139000, 278000, 417000, 0.218925f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_140000, "gpt-4o-mini", 140000, 280000, 420000, 0.2205f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_141000, "gpt-4o-mini", 141000, 282000, 423000, 0.222075f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_142000, "gpt-4o-mini", 142000, 284000, 426000, 0.22365f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_143000, "gpt-4o-mini", 143000, 286000, 429000, 0.225225f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_144000, "gpt-4o-mini", 144000, 288000, 432000, 0.2268f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_145000, "gpt-4o-mini", 145000, 290000, 435000, 0.228375f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_146000, "gpt-4o-mini", 146000, 292000, 438000, 0.22995f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_147000, "gpt-4o-mini", 147000, 294000, 441000, 0.231525f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_148000, "gpt-4o-mini", 148000, 296000, 444000, 0.2331f64);
    generate_cost_test!(test_cost_matrix_gpt_4o_mini_149000, "gpt-4o-mini", 149000, 298000, 447000, 0.234675f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_1000, "gemini-1.5-pro", 1000, 2000, 3000, 0.0245f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_2000, "gemini-1.5-pro", 2000, 4000, 6000, 0.049f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_3000, "gemini-1.5-pro", 3000, 6000, 9000, 0.0735f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_4000, "gemini-1.5-pro", 4000, 8000, 12000, 0.098f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_5000, "gemini-1.5-pro", 5000, 10000, 15000, 0.1225f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_6000, "gemini-1.5-pro", 6000, 12000, 18000, 0.147f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_7000, "gemini-1.5-pro", 7000, 14000, 21000, 0.17149999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_8000, "gemini-1.5-pro", 8000, 16000, 24000, 0.196f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_9000, "gemini-1.5-pro", 9000, 18000, 27000, 0.2205f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_10000, "gemini-1.5-pro", 10000, 20000, 30000, 0.245f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_11000, "gemini-1.5-pro", 11000, 22000, 33000, 0.2695f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_12000, "gemini-1.5-pro", 12000, 24000, 36000, 0.294f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_13000, "gemini-1.5-pro", 13000, 26000, 39000, 0.3185f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_14000, "gemini-1.5-pro", 14000, 28000, 42000, 0.34299999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_15000, "gemini-1.5-pro", 15000, 30000, 45000, 0.3675f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_16000, "gemini-1.5-pro", 16000, 32000, 48000, 0.392f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_17000, "gemini-1.5-pro", 17000, 34000, 51000, 0.4165f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_18000, "gemini-1.5-pro", 18000, 36000, 54000, 0.441f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_19000, "gemini-1.5-pro", 19000, 38000, 57000, 0.4655f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_20000, "gemini-1.5-pro", 20000, 40000, 60000, 0.49f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_21000, "gemini-1.5-pro", 21000, 42000, 63000, 0.5145f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_22000, "gemini-1.5-pro", 22000, 44000, 66000, 0.539f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_23000, "gemini-1.5-pro", 23000, 46000, 69000, 0.5635f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_24000, "gemini-1.5-pro", 24000, 48000, 72000, 0.588f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_25000, "gemini-1.5-pro", 25000, 50000, 75000, 0.6125f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_26000, "gemini-1.5-pro", 26000, 52000, 78000, 0.637f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_27000, "gemini-1.5-pro", 27000, 54000, 81000, 0.6615f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_28000, "gemini-1.5-pro", 28000, 56000, 84000, 0.6859999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_29000, "gemini-1.5-pro", 29000, 58000, 87000, 0.7105f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_30000, "gemini-1.5-pro", 30000, 60000, 90000, 0.735f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_31000, "gemini-1.5-pro", 31000, 62000, 93000, 0.7595000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_32000, "gemini-1.5-pro", 32000, 64000, 96000, 0.784f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_33000, "gemini-1.5-pro", 33000, 66000, 99000, 0.8085f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_34000, "gemini-1.5-pro", 34000, 68000, 102000, 0.833f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_35000, "gemini-1.5-pro", 35000, 70000, 105000, 0.8574999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_36000, "gemini-1.5-pro", 36000, 72000, 108000, 0.882f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_37000, "gemini-1.5-pro", 37000, 74000, 111000, 0.9065000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_38000, "gemini-1.5-pro", 38000, 76000, 114000, 0.931f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_39000, "gemini-1.5-pro", 39000, 78000, 117000, 0.9555f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_40000, "gemini-1.5-pro", 40000, 80000, 120000, 0.98f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_41000, "gemini-1.5-pro", 41000, 82000, 123000, 1.0045f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_42000, "gemini-1.5-pro", 42000, 84000, 126000, 1.029f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_43000, "gemini-1.5-pro", 43000, 86000, 129000, 1.0535f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_44000, "gemini-1.5-pro", 44000, 88000, 132000, 1.078f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_45000, "gemini-1.5-pro", 45000, 90000, 135000, 1.1025f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_46000, "gemini-1.5-pro", 46000, 92000, 138000, 1.127f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_47000, "gemini-1.5-pro", 47000, 94000, 141000, 1.1515f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_48000, "gemini-1.5-pro", 48000, 96000, 144000, 1.176f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_49000, "gemini-1.5-pro", 49000, 98000, 147000, 1.2005f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_50000, "gemini-1.5-pro", 50000, 100000, 150000, 1.225f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_51000, "gemini-1.5-pro", 51000, 102000, 153000, 1.2494999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_52000, "gemini-1.5-pro", 52000, 104000, 156000, 1.274f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_53000, "gemini-1.5-pro", 53000, 106000, 159000, 1.2985f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_54000, "gemini-1.5-pro", 54000, 108000, 162000, 1.323f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_55000, "gemini-1.5-pro", 55000, 110000, 165000, 1.3475000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_56000, "gemini-1.5-pro", 56000, 112000, 168000, 1.3719999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_57000, "gemini-1.5-pro", 57000, 114000, 171000, 1.3965f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_58000, "gemini-1.5-pro", 58000, 116000, 174000, 1.421f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_59000, "gemini-1.5-pro", 59000, 118000, 177000, 1.4455f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_60000, "gemini-1.5-pro", 60000, 120000, 180000, 1.47f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_61000, "gemini-1.5-pro", 61000, 122000, 183000, 1.4945f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_62000, "gemini-1.5-pro", 62000, 124000, 186000, 1.5190000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_63000, "gemini-1.5-pro", 63000, 126000, 189000, 1.5434999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_64000, "gemini-1.5-pro", 64000, 128000, 192000, 1.568f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_65000, "gemini-1.5-pro", 65000, 130000, 195000, 1.5925f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_66000, "gemini-1.5-pro", 66000, 132000, 198000, 1.617f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_67000, "gemini-1.5-pro", 67000, 134000, 201000, 1.6415f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_68000, "gemini-1.5-pro", 68000, 136000, 204000, 1.666f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_69000, "gemini-1.5-pro", 69000, 138000, 207000, 1.6905000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_70000, "gemini-1.5-pro", 70000, 140000, 210000, 1.7149999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_71000, "gemini-1.5-pro", 71000, 142000, 213000, 1.7395f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_72000, "gemini-1.5-pro", 72000, 144000, 216000, 1.764f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_73000, "gemini-1.5-pro", 73000, 146000, 219000, 1.7885f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_74000, "gemini-1.5-pro", 74000, 148000, 222000, 1.8130000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_75000, "gemini-1.5-pro", 75000, 150000, 225000, 1.8375f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_76000, "gemini-1.5-pro", 76000, 152000, 228000, 1.862f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_77000, "gemini-1.5-pro", 77000, 154000, 231000, 1.8865f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_78000, "gemini-1.5-pro", 78000, 156000, 234000, 1.911f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_79000, "gemini-1.5-pro", 79000, 158000, 237000, 1.9355f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_80000, "gemini-1.5-pro", 80000, 160000, 240000, 1.96f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_81000, "gemini-1.5-pro", 81000, 162000, 243000, 1.9845000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_82000, "gemini-1.5-pro", 82000, 164000, 246000, 2.009f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_83000, "gemini-1.5-pro", 83000, 166000, 249000, 2.0335f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_84000, "gemini-1.5-pro", 84000, 168000, 252000, 2.058f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_85000, "gemini-1.5-pro", 85000, 170000, 255000, 2.0825f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_86000, "gemini-1.5-pro", 86000, 172000, 258000, 2.107f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_87000, "gemini-1.5-pro", 87000, 174000, 261000, 2.1315f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_88000, "gemini-1.5-pro", 88000, 176000, 264000, 2.156f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_89000, "gemini-1.5-pro", 89000, 178000, 267000, 2.1805f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_90000, "gemini-1.5-pro", 90000, 180000, 270000, 2.205f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_91000, "gemini-1.5-pro", 91000, 182000, 273000, 2.2295f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_92000, "gemini-1.5-pro", 92000, 184000, 276000, 2.254f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_93000, "gemini-1.5-pro", 93000, 186000, 279000, 2.2785f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_94000, "gemini-1.5-pro", 94000, 188000, 282000, 2.303f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_95000, "gemini-1.5-pro", 95000, 190000, 285000, 2.3275f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_96000, "gemini-1.5-pro", 96000, 192000, 288000, 2.352f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_97000, "gemini-1.5-pro", 97000, 194000, 291000, 2.3765f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_98000, "gemini-1.5-pro", 98000, 196000, 294000, 2.401f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_99000, "gemini-1.5-pro", 99000, 198000, 297000, 2.4255f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_100000, "gemini-1.5-pro", 100000, 200000, 300000, 2.45f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_101000, "gemini-1.5-pro", 101000, 202000, 303000, 2.4745f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_102000, "gemini-1.5-pro", 102000, 204000, 306000, 2.4989999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_103000, "gemini-1.5-pro", 103000, 206000, 309000, 2.5235f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_104000, "gemini-1.5-pro", 104000, 208000, 312000, 2.548f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_105000, "gemini-1.5-pro", 105000, 210000, 315000, 2.5725000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_106000, "gemini-1.5-pro", 106000, 212000, 318000, 2.597f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_107000, "gemini-1.5-pro", 107000, 214000, 321000, 2.6214999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_108000, "gemini-1.5-pro", 108000, 216000, 324000, 2.646f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_109000, "gemini-1.5-pro", 109000, 218000, 327000, 2.6705f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_110000, "gemini-1.5-pro", 110000, 220000, 330000, 2.6950000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_111000, "gemini-1.5-pro", 111000, 222000, 333000, 2.7195f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_112000, "gemini-1.5-pro", 112000, 224000, 336000, 2.7439999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_113000, "gemini-1.5-pro", 113000, 226000, 339000, 2.7685000000000004f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_114000, "gemini-1.5-pro", 114000, 228000, 342000, 2.793f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_115000, "gemini-1.5-pro", 115000, 230000, 345000, 2.8175f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_116000, "gemini-1.5-pro", 116000, 232000, 348000, 2.842f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_117000, "gemini-1.5-pro", 117000, 234000, 351000, 2.8665f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_118000, "gemini-1.5-pro", 118000, 236000, 354000, 2.891f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_119000, "gemini-1.5-pro", 119000, 238000, 357000, 2.9155f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_120000, "gemini-1.5-pro", 120000, 240000, 360000, 2.94f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_121000, "gemini-1.5-pro", 121000, 242000, 363000, 2.9645f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_122000, "gemini-1.5-pro", 122000, 244000, 366000, 2.989f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_123000, "gemini-1.5-pro", 123000, 246000, 369000, 3.0135f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_124000, "gemini-1.5-pro", 124000, 248000, 372000, 3.0380000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_125000, "gemini-1.5-pro", 125000, 250000, 375000, 3.0625f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_126000, "gemini-1.5-pro", 126000, 252000, 378000, 3.0869999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_127000, "gemini-1.5-pro", 127000, 254000, 381000, 3.1115f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_128000, "gemini-1.5-pro", 128000, 256000, 384000, 3.136f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_129000, "gemini-1.5-pro", 129000, 258000, 387000, 3.1605f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_130000, "gemini-1.5-pro", 130000, 260000, 390000, 3.185f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_131000, "gemini-1.5-pro", 131000, 262000, 393000, 3.2095f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_132000, "gemini-1.5-pro", 132000, 264000, 396000, 3.234f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_133000, "gemini-1.5-pro", 133000, 266000, 399000, 3.2585f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_134000, "gemini-1.5-pro", 134000, 268000, 402000, 3.283f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_135000, "gemini-1.5-pro", 135000, 270000, 405000, 3.3075f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_136000, "gemini-1.5-pro", 136000, 272000, 408000, 3.332f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_137000, "gemini-1.5-pro", 137000, 274000, 411000, 3.3564999999999996f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_138000, "gemini-1.5-pro", 138000, 276000, 414000, 3.3810000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_139000, "gemini-1.5-pro", 139000, 278000, 417000, 3.4055f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_140000, "gemini-1.5-pro", 140000, 280000, 420000, 3.4299999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_141000, "gemini-1.5-pro", 141000, 282000, 423000, 3.4545f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_142000, "gemini-1.5-pro", 142000, 284000, 426000, 3.479f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_143000, "gemini-1.5-pro", 143000, 286000, 429000, 3.5035f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_144000, "gemini-1.5-pro", 144000, 288000, 432000, 3.528f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_145000, "gemini-1.5-pro", 145000, 290000, 435000, 3.5524999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_146000, "gemini-1.5-pro", 146000, 292000, 438000, 3.577f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_147000, "gemini-1.5-pro", 147000, 294000, 441000, 3.6015f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_148000, "gemini-1.5-pro", 148000, 296000, 444000, 3.6260000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_pro_149000, "gemini-1.5-pro", 149000, 298000, 447000, 3.6505f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_1000, "gemini-1.5-flash", 1000, 2000, 3000, 0.00245f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_2000, "gemini-1.5-flash", 2000, 4000, 6000, 0.0049f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_3000, "gemini-1.5-flash", 3000, 6000, 9000, 0.00735f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_4000, "gemini-1.5-flash", 4000, 8000, 12000, 0.0098f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_5000, "gemini-1.5-flash", 5000, 10000, 15000, 0.01225f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_6000, "gemini-1.5-flash", 6000, 12000, 18000, 0.0147f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_7000, "gemini-1.5-flash", 7000, 14000, 21000, 0.01715f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_8000, "gemini-1.5-flash", 8000, 16000, 24000, 0.0196f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_9000, "gemini-1.5-flash", 9000, 18000, 27000, 0.02205f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_10000, "gemini-1.5-flash", 10000, 20000, 30000, 0.0245f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_11000, "gemini-1.5-flash", 11000, 22000, 33000, 0.026949999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_12000, "gemini-1.5-flash", 12000, 24000, 36000, 0.0294f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_13000, "gemini-1.5-flash", 13000, 26000, 39000, 0.03185f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_14000, "gemini-1.5-flash", 14000, 28000, 42000, 0.0343f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_15000, "gemini-1.5-flash", 15000, 30000, 45000, 0.03675f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_16000, "gemini-1.5-flash", 16000, 32000, 48000, 0.0392f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_17000, "gemini-1.5-flash", 17000, 34000, 51000, 0.041650000000000006f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_18000, "gemini-1.5-flash", 18000, 36000, 54000, 0.0441f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_19000, "gemini-1.5-flash", 19000, 38000, 57000, 0.046549999999999994f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_20000, "gemini-1.5-flash", 20000, 40000, 60000, 0.049f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_21000, "gemini-1.5-flash", 21000, 42000, 63000, 0.051449999999999996f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_22000, "gemini-1.5-flash", 22000, 44000, 66000, 0.053899999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_23000, "gemini-1.5-flash", 23000, 46000, 69000, 0.056350000000000004f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_24000, "gemini-1.5-flash", 24000, 48000, 72000, 0.0588f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_25000, "gemini-1.5-flash", 25000, 50000, 75000, 0.06125f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_26000, "gemini-1.5-flash", 26000, 52000, 78000, 0.0637f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_27000, "gemini-1.5-flash", 27000, 54000, 81000, 0.06615f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_28000, "gemini-1.5-flash", 28000, 56000, 84000, 0.0686f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_29000, "gemini-1.5-flash", 29000, 58000, 87000, 0.07105f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_30000, "gemini-1.5-flash", 30000, 60000, 90000, 0.0735f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_31000, "gemini-1.5-flash", 31000, 62000, 93000, 0.07595f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_32000, "gemini-1.5-flash", 32000, 64000, 96000, 0.0784f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_33000, "gemini-1.5-flash", 33000, 66000, 99000, 0.08085f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_34000, "gemini-1.5-flash", 34000, 68000, 102000, 0.08330000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_35000, "gemini-1.5-flash", 35000, 70000, 105000, 0.08574999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_36000, "gemini-1.5-flash", 36000, 72000, 108000, 0.0882f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_37000, "gemini-1.5-flash", 37000, 74000, 111000, 0.09065000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_38000, "gemini-1.5-flash", 38000, 76000, 114000, 0.09309999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_39000, "gemini-1.5-flash", 39000, 78000, 117000, 0.09555f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_40000, "gemini-1.5-flash", 40000, 80000, 120000, 0.098f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_41000, "gemini-1.5-flash", 41000, 82000, 123000, 0.10045f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_42000, "gemini-1.5-flash", 42000, 84000, 126000, 0.10289999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_43000, "gemini-1.5-flash", 43000, 86000, 129000, 0.10535f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_44000, "gemini-1.5-flash", 44000, 88000, 132000, 0.10779999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_45000, "gemini-1.5-flash", 45000, 90000, 135000, 0.11025f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_46000, "gemini-1.5-flash", 46000, 92000, 138000, 0.11270000000000001f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_47000, "gemini-1.5-flash", 47000, 94000, 141000, 0.11515f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_48000, "gemini-1.5-flash", 48000, 96000, 144000, 0.1176f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_49000, "gemini-1.5-flash", 49000, 98000, 147000, 0.12005f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_50000, "gemini-1.5-flash", 50000, 100000, 150000, 0.1225f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_51000, "gemini-1.5-flash", 51000, 102000, 153000, 0.12495f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_52000, "gemini-1.5-flash", 52000, 104000, 156000, 0.1274f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_53000, "gemini-1.5-flash", 53000, 106000, 159000, 0.12985f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_54000, "gemini-1.5-flash", 54000, 108000, 162000, 0.1323f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_55000, "gemini-1.5-flash", 55000, 110000, 165000, 0.13475f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_56000, "gemini-1.5-flash", 56000, 112000, 168000, 0.1372f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_57000, "gemini-1.5-flash", 57000, 114000, 171000, 0.13965f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_58000, "gemini-1.5-flash", 58000, 116000, 174000, 0.1421f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_59000, "gemini-1.5-flash", 59000, 118000, 177000, 0.14455f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_60000, "gemini-1.5-flash", 60000, 120000, 180000, 0.147f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_61000, "gemini-1.5-flash", 61000, 122000, 183000, 0.14945f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_62000, "gemini-1.5-flash", 62000, 124000, 186000, 0.1519f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_63000, "gemini-1.5-flash", 63000, 126000, 189000, 0.15435f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_64000, "gemini-1.5-flash", 64000, 128000, 192000, 0.1568f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_65000, "gemini-1.5-flash", 65000, 130000, 195000, 0.15925f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_66000, "gemini-1.5-flash", 66000, 132000, 198000, 0.1617f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_67000, "gemini-1.5-flash", 67000, 134000, 201000, 0.16415f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_68000, "gemini-1.5-flash", 68000, 136000, 204000, 0.16660000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_69000, "gemini-1.5-flash", 69000, 138000, 207000, 0.16905f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_70000, "gemini-1.5-flash", 70000, 140000, 210000, 0.17149999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_71000, "gemini-1.5-flash", 71000, 142000, 213000, 0.17395000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_72000, "gemini-1.5-flash", 72000, 144000, 216000, 0.1764f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_73000, "gemini-1.5-flash", 73000, 146000, 219000, 0.17884999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_74000, "gemini-1.5-flash", 74000, 148000, 222000, 0.18130000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_75000, "gemini-1.5-flash", 75000, 150000, 225000, 0.18375f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_76000, "gemini-1.5-flash", 76000, 152000, 228000, 0.18619999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_77000, "gemini-1.5-flash", 77000, 154000, 231000, 0.18865f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_78000, "gemini-1.5-flash", 78000, 156000, 234000, 0.1911f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_79000, "gemini-1.5-flash", 79000, 158000, 237000, 0.19355f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_80000, "gemini-1.5-flash", 80000, 160000, 240000, 0.196f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_81000, "gemini-1.5-flash", 81000, 162000, 243000, 0.19845000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_82000, "gemini-1.5-flash", 82000, 164000, 246000, 0.2009f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_83000, "gemini-1.5-flash", 83000, 166000, 249000, 0.20335f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_84000, "gemini-1.5-flash", 84000, 168000, 252000, 0.20579999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_85000, "gemini-1.5-flash", 85000, 170000, 255000, 0.20825f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_86000, "gemini-1.5-flash", 86000, 172000, 258000, 0.2107f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_87000, "gemini-1.5-flash", 87000, 174000, 261000, 0.21315f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_88000, "gemini-1.5-flash", 88000, 176000, 264000, 0.21559999999999999f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_89000, "gemini-1.5-flash", 89000, 178000, 267000, 0.21805000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_90000, "gemini-1.5-flash", 90000, 180000, 270000, 0.2205f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_91000, "gemini-1.5-flash", 91000, 182000, 273000, 0.22294999999999998f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_92000, "gemini-1.5-flash", 92000, 184000, 276000, 0.22540000000000002f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_93000, "gemini-1.5-flash", 93000, 186000, 279000, 0.22785f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_94000, "gemini-1.5-flash", 94000, 188000, 282000, 0.2303f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_95000, "gemini-1.5-flash", 95000, 190000, 285000, 0.23275f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_96000, "gemini-1.5-flash", 96000, 192000, 288000, 0.2352f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_97000, "gemini-1.5-flash", 97000, 194000, 291000, 0.23765f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_98000, "gemini-1.5-flash", 98000, 196000, 294000, 0.2401f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_99000, "gemini-1.5-flash", 99000, 198000, 297000, 0.24255f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_100000, "gemini-1.5-flash", 100000, 200000, 300000, 0.245f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_101000, "gemini-1.5-flash", 101000, 202000, 303000, 0.24745f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_102000, "gemini-1.5-flash", 102000, 204000, 306000, 0.2499f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_103000, "gemini-1.5-flash", 103000, 206000, 309000, 0.25234999999999996f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_104000, "gemini-1.5-flash", 104000, 208000, 312000, 0.2548f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_105000, "gemini-1.5-flash", 105000, 210000, 315000, 0.25725f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_106000, "gemini-1.5-flash", 106000, 212000, 318000, 0.2597f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_107000, "gemini-1.5-flash", 107000, 214000, 321000, 0.26215f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_108000, "gemini-1.5-flash", 108000, 216000, 324000, 0.2646f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_109000, "gemini-1.5-flash", 109000, 218000, 327000, 0.26705f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_110000, "gemini-1.5-flash", 110000, 220000, 330000, 0.2695f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_111000, "gemini-1.5-flash", 111000, 222000, 333000, 0.27195f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_112000, "gemini-1.5-flash", 112000, 224000, 336000, 0.2744f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_113000, "gemini-1.5-flash", 113000, 226000, 339000, 0.27685000000000004f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_114000, "gemini-1.5-flash", 114000, 228000, 342000, 0.2793f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_115000, "gemini-1.5-flash", 115000, 230000, 345000, 0.28175f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_116000, "gemini-1.5-flash", 116000, 232000, 348000, 0.2842f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_117000, "gemini-1.5-flash", 117000, 234000, 351000, 0.28665f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_118000, "gemini-1.5-flash", 118000, 236000, 354000, 0.2891f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_119000, "gemini-1.5-flash", 119000, 238000, 357000, 0.29155000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_120000, "gemini-1.5-flash", 120000, 240000, 360000, 0.294f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_121000, "gemini-1.5-flash", 121000, 242000, 363000, 0.29645f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_122000, "gemini-1.5-flash", 122000, 244000, 366000, 0.2989f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_123000, "gemini-1.5-flash", 123000, 246000, 369000, 0.30134999999999995f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_124000, "gemini-1.5-flash", 124000, 248000, 372000, 0.3038f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_125000, "gemini-1.5-flash", 125000, 250000, 375000, 0.30625f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_126000, "gemini-1.5-flash", 126000, 252000, 378000, 0.3087f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_127000, "gemini-1.5-flash", 127000, 254000, 381000, 0.31115f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_128000, "gemini-1.5-flash", 128000, 256000, 384000, 0.3136f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_129000, "gemini-1.5-flash", 129000, 258000, 387000, 0.31605f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_130000, "gemini-1.5-flash", 130000, 260000, 390000, 0.3185f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_131000, "gemini-1.5-flash", 131000, 262000, 393000, 0.32095f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_132000, "gemini-1.5-flash", 132000, 264000, 396000, 0.3234f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_133000, "gemini-1.5-flash", 133000, 266000, 399000, 0.32585f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_134000, "gemini-1.5-flash", 134000, 268000, 402000, 0.3283f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_135000, "gemini-1.5-flash", 135000, 270000, 405000, 0.33075f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_136000, "gemini-1.5-flash", 136000, 272000, 408000, 0.33320000000000005f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_137000, "gemini-1.5-flash", 137000, 274000, 411000, 0.33565f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_138000, "gemini-1.5-flash", 138000, 276000, 414000, 0.3381f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_139000, "gemini-1.5-flash", 139000, 278000, 417000, 0.34055f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_140000, "gemini-1.5-flash", 140000, 280000, 420000, 0.34299999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_141000, "gemini-1.5-flash", 141000, 282000, 423000, 0.34545f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_142000, "gemini-1.5-flash", 142000, 284000, 426000, 0.34790000000000004f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_143000, "gemini-1.5-flash", 143000, 286000, 429000, 0.35035f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_144000, "gemini-1.5-flash", 144000, 288000, 432000, 0.3528f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_145000, "gemini-1.5-flash", 145000, 290000, 435000, 0.35525f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_146000, "gemini-1.5-flash", 146000, 292000, 438000, 0.35769999999999996f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_147000, "gemini-1.5-flash", 147000, 294000, 441000, 0.36014999999999997f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_148000, "gemini-1.5-flash", 148000, 296000, 444000, 0.36260000000000003f64);
    generate_cost_test!(test_cost_matrix_gemini_1_5_flash_149000, "gemini-1.5-flash", 149000, 298000, 447000, 0.36505f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_1000, "unknown-fallback", 1000, 2000, 3000, 0.0375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_2000, "unknown-fallback", 2000, 4000, 6000, 0.075f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_3000, "unknown-fallback", 3000, 6000, 9000, 0.11249999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_4000, "unknown-fallback", 4000, 8000, 12000, 0.15f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_5000, "unknown-fallback", 5000, 10000, 15000, 0.18749999999999997f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_6000, "unknown-fallback", 6000, 12000, 18000, 0.22499999999999998f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_7000, "unknown-fallback", 7000, 14000, 21000, 0.26249999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_8000, "unknown-fallback", 8000, 16000, 24000, 0.3f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_9000, "unknown-fallback", 9000, 18000, 27000, 0.3375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_10000, "unknown-fallback", 10000, 20000, 30000, 0.37499999999999994f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_11000, "unknown-fallback", 11000, 22000, 33000, 0.4125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_12000, "unknown-fallback", 12000, 24000, 36000, 0.44999999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_13000, "unknown-fallback", 13000, 26000, 39000, 0.4875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_14000, "unknown-fallback", 14000, 28000, 42000, 0.5249999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_15000, "unknown-fallback", 15000, 30000, 45000, 0.5625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_16000, "unknown-fallback", 16000, 32000, 48000, 0.6f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_17000, "unknown-fallback", 17000, 34000, 51000, 0.6375000000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_18000, "unknown-fallback", 18000, 36000, 54000, 0.675f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_19000, "unknown-fallback", 19000, 38000, 57000, 0.7125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_20000, "unknown-fallback", 20000, 40000, 60000, 0.7499999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_21000, "unknown-fallback", 21000, 42000, 63000, 0.7875000000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_22000, "unknown-fallback", 22000, 44000, 66000, 0.825f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_23000, "unknown-fallback", 23000, 46000, 69000, 0.8624999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_24000, "unknown-fallback", 24000, 48000, 72000, 0.8999999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_25000, "unknown-fallback", 25000, 50000, 75000, 0.9375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_26000, "unknown-fallback", 26000, 52000, 78000, 0.975f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_27000, "unknown-fallback", 27000, 54000, 81000, 1.0125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_28000, "unknown-fallback", 28000, 56000, 84000, 1.0499999999999998f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_29000, "unknown-fallback", 29000, 58000, 87000, 1.0875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_30000, "unknown-fallback", 30000, 60000, 90000, 1.125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_31000, "unknown-fallback", 31000, 62000, 93000, 1.1625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_32000, "unknown-fallback", 32000, 64000, 96000, 1.2f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_33000, "unknown-fallback", 33000, 66000, 99000, 1.2375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_34000, "unknown-fallback", 34000, 68000, 102000, 1.2750000000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_35000, "unknown-fallback", 35000, 70000, 105000, 1.3125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_36000, "unknown-fallback", 36000, 72000, 108000, 1.35f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_37000, "unknown-fallback", 37000, 74000, 111000, 1.3875000000000002f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_38000, "unknown-fallback", 38000, 76000, 114000, 1.425f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_39000, "unknown-fallback", 39000, 78000, 117000, 1.4625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_40000, "unknown-fallback", 40000, 80000, 120000, 1.4999999999999998f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_41000, "unknown-fallback", 41000, 82000, 123000, 1.5375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_42000, "unknown-fallback", 42000, 84000, 126000, 1.5750000000000002f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_43000, "unknown-fallback", 43000, 86000, 129000, 1.6125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_44000, "unknown-fallback", 44000, 88000, 132000, 1.65f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_45000, "unknown-fallback", 45000, 90000, 135000, 1.6875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_46000, "unknown-fallback", 46000, 92000, 138000, 1.7249999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_47000, "unknown-fallback", 47000, 94000, 141000, 1.7625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_48000, "unknown-fallback", 48000, 96000, 144000, 1.7999999999999998f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_49000, "unknown-fallback", 49000, 98000, 147000, 1.8375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_50000, "unknown-fallback", 50000, 100000, 150000, 1.875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_51000, "unknown-fallback", 51000, 102000, 153000, 1.9125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_52000, "unknown-fallback", 52000, 104000, 156000, 1.95f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_53000, "unknown-fallback", 53000, 106000, 159000, 1.9875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_54000, "unknown-fallback", 54000, 108000, 162000, 2.025f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_55000, "unknown-fallback", 55000, 110000, 165000, 2.0625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_56000, "unknown-fallback", 56000, 112000, 168000, 2.0999999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_57000, "unknown-fallback", 57000, 114000, 171000, 2.1375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_58000, "unknown-fallback", 58000, 116000, 174000, 2.175f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_59000, "unknown-fallback", 59000, 118000, 177000, 2.2125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_60000, "unknown-fallback", 60000, 120000, 180000, 2.25f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_61000, "unknown-fallback", 61000, 122000, 183000, 2.2875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_62000, "unknown-fallback", 62000, 124000, 186000, 2.325f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_63000, "unknown-fallback", 63000, 126000, 189000, 2.3625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_64000, "unknown-fallback", 64000, 128000, 192000, 2.4f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_65000, "unknown-fallback", 65000, 130000, 195000, 2.4375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_66000, "unknown-fallback", 66000, 132000, 198000, 2.475f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_67000, "unknown-fallback", 67000, 134000, 201000, 2.5124999999999997f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_68000, "unknown-fallback", 68000, 136000, 204000, 2.5500000000000003f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_69000, "unknown-fallback", 69000, 138000, 207000, 2.5874999999999995f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_70000, "unknown-fallback", 70000, 140000, 210000, 2.625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_71000, "unknown-fallback", 71000, 142000, 213000, 2.6625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_72000, "unknown-fallback", 72000, 144000, 216000, 2.7f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_73000, "unknown-fallback", 73000, 146000, 219000, 2.7375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_74000, "unknown-fallback", 74000, 148000, 222000, 2.7750000000000004f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_75000, "unknown-fallback", 75000, 150000, 225000, 2.8125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_76000, "unknown-fallback", 76000, 152000, 228000, 2.85f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_77000, "unknown-fallback", 77000, 154000, 231000, 2.8874999999999997f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_78000, "unknown-fallback", 78000, 156000, 234000, 2.925f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_79000, "unknown-fallback", 79000, 158000, 237000, 2.9625000000000004f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_80000, "unknown-fallback", 80000, 160000, 240000, 2.9999999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_81000, "unknown-fallback", 81000, 162000, 243000, 3.0375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_82000, "unknown-fallback", 82000, 164000, 246000, 3.075f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_83000, "unknown-fallback", 83000, 166000, 249000, 3.1125000000000003f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_84000, "unknown-fallback", 84000, 168000, 252000, 3.1500000000000004f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_85000, "unknown-fallback", 85000, 170000, 255000, 3.1874999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_86000, "unknown-fallback", 86000, 172000, 258000, 3.225f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_87000, "unknown-fallback", 87000, 174000, 261000, 3.2625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_88000, "unknown-fallback", 88000, 176000, 264000, 3.3f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_89000, "unknown-fallback", 89000, 178000, 267000, 3.3375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_90000, "unknown-fallback", 90000, 180000, 270000, 3.375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_91000, "unknown-fallback", 91000, 182000, 273000, 3.4125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_92000, "unknown-fallback", 92000, 184000, 276000, 3.4499999999999997f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_93000, "unknown-fallback", 93000, 186000, 279000, 3.4875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_94000, "unknown-fallback", 94000, 188000, 282000, 3.525f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_95000, "unknown-fallback", 95000, 190000, 285000, 3.5625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_96000, "unknown-fallback", 96000, 192000, 288000, 3.5999999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_97000, "unknown-fallback", 97000, 194000, 291000, 3.6375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_98000, "unknown-fallback", 98000, 196000, 294000, 3.675f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_99000, "unknown-fallback", 99000, 198000, 297000, 3.7125000000000004f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_100000, "unknown-fallback", 100000, 200000, 300000, 3.75f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_101000, "unknown-fallback", 101000, 202000, 303000, 3.7874999999999996f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_102000, "unknown-fallback", 102000, 204000, 306000, 3.825f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_103000, "unknown-fallback", 103000, 206000, 309000, 3.8625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_104000, "unknown-fallback", 104000, 208000, 312000, 3.9f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_105000, "unknown-fallback", 105000, 210000, 315000, 3.9375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_106000, "unknown-fallback", 106000, 212000, 318000, 3.975f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_107000, "unknown-fallback", 107000, 214000, 321000, 4.0125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_108000, "unknown-fallback", 108000, 216000, 324000, 4.05f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_109000, "unknown-fallback", 109000, 218000, 327000, 4.0875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_110000, "unknown-fallback", 110000, 220000, 330000, 4.125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_111000, "unknown-fallback", 111000, 222000, 333000, 4.1625000000000005f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_112000, "unknown-fallback", 112000, 224000, 336000, 4.199999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_113000, "unknown-fallback", 113000, 226000, 339000, 4.2375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_114000, "unknown-fallback", 114000, 228000, 342000, 4.275f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_115000, "unknown-fallback", 115000, 230000, 345000, 4.3125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_116000, "unknown-fallback", 116000, 232000, 348000, 4.35f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_117000, "unknown-fallback", 117000, 234000, 351000, 4.387499999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_118000, "unknown-fallback", 118000, 236000, 354000, 4.425f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_119000, "unknown-fallback", 119000, 238000, 357000, 4.4624999999999995f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_120000, "unknown-fallback", 120000, 240000, 360000, 4.5f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_121000, "unknown-fallback", 121000, 242000, 363000, 4.5375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_122000, "unknown-fallback", 122000, 244000, 366000, 4.575f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_123000, "unknown-fallback", 123000, 246000, 369000, 4.6125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_124000, "unknown-fallback", 124000, 248000, 372000, 4.65f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_125000, "unknown-fallback", 125000, 250000, 375000, 4.6875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_126000, "unknown-fallback", 126000, 252000, 378000, 4.725f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_127000, "unknown-fallback", 127000, 254000, 381000, 4.7625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_128000, "unknown-fallback", 128000, 256000, 384000, 4.8f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_129000, "unknown-fallback", 129000, 258000, 387000, 4.8374999999999995f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_130000, "unknown-fallback", 130000, 260000, 390000, 4.875f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_131000, "unknown-fallback", 131000, 262000, 393000, 4.9125000000000005f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_132000, "unknown-fallback", 132000, 264000, 396000, 4.95f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_133000, "unknown-fallback", 133000, 266000, 399000, 4.987500000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_134000, "unknown-fallback", 134000, 268000, 402000, 5.0249999999999995f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_135000, "unknown-fallback", 135000, 270000, 405000, 5.0625f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_136000, "unknown-fallback", 136000, 272000, 408000, 5.1000000000000005f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_137000, "unknown-fallback", 137000, 274000, 411000, 5.1375f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_138000, "unknown-fallback", 138000, 276000, 414000, 5.174999999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_139000, "unknown-fallback", 139000, 278000, 417000, 5.2124999999999995f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_140000, "unknown-fallback", 140000, 280000, 420000, 5.25f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_141000, "unknown-fallback", 141000, 282000, 423000, 5.2875000000000005f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_142000, "unknown-fallback", 142000, 284000, 426000, 5.325f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_143000, "unknown-fallback", 143000, 286000, 429000, 5.362500000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_144000, "unknown-fallback", 144000, 288000, 432000, 5.4f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_145000, "unknown-fallback", 145000, 290000, 435000, 5.437499999999999f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_146000, "unknown-fallback", 146000, 292000, 438000, 5.475f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_147000, "unknown-fallback", 147000, 294000, 441000, 5.5125f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_148000, "unknown-fallback", 148000, 296000, 444000, 5.550000000000001f64);
    generate_cost_test!(test_cost_matrix_unknown_fallback_149000, "unknown-fallback", 149000, 298000, 447000, 5.5874999999999995f64);

    #[test]
    fn test_tier_matrix_soft_limits() {
        let tiers = vec![PlanTier::Free, PlanTier::Starter, PlanTier::Pro, PlanTier::Business];
        for tier in tiers {
            let limit = tier.monthly_action_limit();
            match tier {
                PlanTier::Free => assert_eq!(limit, Some(100)),
                PlanTier::Starter => assert_eq!(limit, Some(1000)),
                PlanTier::Pro => assert_eq!(limit, None),
                PlanTier::Business => assert_eq!(limit, None),
            }

            let storage = tier.storage_limit_mb();
            match tier {
                PlanTier::Free => assert_eq!(storage, Some(500)),
                PlanTier::Starter => assert_eq!(storage, Some(5000)),
                PlanTier::Pro => assert_eq!(storage, Some(50000)),
                PlanTier::Business => assert_eq!(storage, Some(512000)),
            }

            let max_agents = tier.max_agents();
            match tier {
                PlanTier::Free => assert_eq!(max_agents, Some(1)),
                PlanTier::Starter => assert_eq!(max_agents, Some(3)),
                PlanTier::Pro => assert_eq!(max_agents, Some(10)),
                PlanTier::Business => assert_eq!(max_agents, None),
            }
        }
    }
}
