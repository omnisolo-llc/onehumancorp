use redis::{AsyncCommands, Client};
use tokio::sync::OnceCell;
use dashmap::DashMap;
use std::time::{Instant, Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanTier {
    Free,
    Starter,
    Pro,
    Business,
}

impl std::fmt::Display for PlanTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanTier::Free => write!(f, "Free"),
            PlanTier::Starter => write!(f, "Starter"),
            PlanTier::Pro => write!(f, "Pro"),
            PlanTier::Business => write!(f, "Business"),
        }
    }
}

impl std::str::FromStr for PlanTier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "starter" => Ok(PlanTier::Starter),
            "pro" => Ok(PlanTier::Pro),
            "business" => Ok(PlanTier::Business),
            "free" => Ok(PlanTier::Free),
            _ => Err(format!("Unknown tier: {}", s)),
        }
    }
}

impl PlanTier {
    pub fn monthly_action_limit(&self) -> Option<u32> {
        let env_var = match self {
            PlanTier::Free => "OHC_FREE_TIER_ACTIONS",
            PlanTier::Starter => "OHC_STARTER_TIER_ACTIONS",
            _ => "",
        };
        if !env_var.is_empty()
            && let Some(v) = std::env::var(env_var).ok().and_then(|s| s.parse::<u32>().ok()) {
                return Some(v);
            }

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
        let env_var = match self {
            PlanTier::Free => "OHC_FREE_TIER_STORAGE_MB",
            PlanTier::Starter => "OHC_STARTER_TIER_STORAGE_MB",
            PlanTier::Pro => "OHC_PRO_TIER_STORAGE_MB",
            PlanTier::Business => "OHC_BUSINESS_TIER_STORAGE_MB",
        };
        if !env_var.is_empty()
            && let Some(v) = std::env::var(env_var).ok().and_then(|s| s.parse::<u32>().ok()) {
                return Some(v);
            }

        match self {
            PlanTier::Free => Some(500), // 500MB
            PlanTier::Starter => Some(5120), // 5GB
            PlanTier::Pro => Some(51200),    // 50GB
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

    pub fn base_price(&self) -> f64 {
        match self {
            PlanTier::Free => 0.0,
            PlanTier::Starter => 29.0,
            PlanTier::Pro => 79.0,
            PlanTier::Business => 299.0,
        }
    }

    pub fn get_prompt_cache_ttl(&self) -> std::time::Duration {
        match self {
            PlanTier::Free => std::time::Duration::from_secs(60 * 60), // 1 hour
            PlanTier::Starter => std::time::Duration::from_secs(24 * 60 * 60), // 24 hours
            PlanTier::Pro => std::time::Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            PlanTier::Business => std::time::Duration::from_secs(30 * 24 * 60 * 60), // 30 days
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
    db_pool: Option<sqlx::PgPool>,
    tier_cache: DashMap<String, (PlanTier, Instant)>,
}

impl RedisRateLimiter {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            connection: OnceCell::new(),
            telemetry_store: None,
            db_pool: None,
            tier_cache: DashMap::new(),
        }
    }

    pub fn with_db(mut self, pool: sqlx::PgPool) -> Self {
        self.db_pool = Some(pool);
        self
    }

    pub fn with_telemetry(mut self, store: std::sync::Arc<::server_harness::telemetry::ViolationStore>) -> Self {
        self.telemetry_store = Some(store);
        self
    }

    pub async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let conn = self.connection.get_or_try_init(|| async {
            self.client.get_multiplexed_async_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
    }

    pub async fn get_tenant_tier(&self, tenant_id: &str) -> Result<PlanTier, String> {
        if let Some(entry) = self.tier_cache.get(tenant_id)
            && entry.1.elapsed() < Duration::from_secs(300) {
                return Ok(entry.0.clone());
            }

        let mut conn = self.get_connection().await?;
        let redis_key = format!("tenant:{}:tier", tenant_id);
        let mut tier: Option<String> = conn.get(&redis_key).await.map_err(|e| e.to_string())?;

        if tier.is_none()
            && let Some(pool) = &self.db_pool {
                use sqlx::Row;
                if let Ok(record) = sqlx::query("SELECT plan_tier as tier FROM tenants WHERE id = $1")
                    .bind(tenant_id)
                    .fetch_one(pool)
                    .await
                    && let Ok(t) = record.try_get::<Option<String>, _>("tier") {
                        tier = t;
                        if let Some(ref t_str) = tier {
                            // Cache for 24 hours
                            let _ : () = conn.set_ex(&redis_key, t_str, 24 * 60 * 60).await.unwrap_or(());
                        }
                    }
            }

        let tier_val = match tier {
            Some(t) => t.parse::<PlanTier>().unwrap_or(PlanTier::Free),
            None => PlanTier::Free,
        };

        self.tier_cache.insert(tenant_id.to_string(), (tier_val.clone(), Instant::now()));

        Ok(tier_val)
    }

    pub async fn get_tenant_actions_used(&self, tenant_id: &str) -> Result<u32, String> {
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();
        let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
        let used: Option<u32> = conn.get(&tenant_key).await.map_err(|e| e.to_string())?;
        Ok(used.unwrap_or(0))
    }

    pub async fn get_agent_actions_used(&self, tenant_id: &str, agent_id: &str) -> Result<u32, String> {
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();
        let agent_key = format!("tenant:{}:agent:{}:actions_used:{}", tenant_id, agent_id, month_key);
        let used: Option<u32> = conn.get(&agent_key).await.map_err(|e| e.to_string())?;
        Ok(used.unwrap_or(0))
    }

    pub async fn get_tenant_storage_used(&self, tenant_id: &str) -> Result<i64, String> {
        let mut conn = self.get_connection().await?;
        let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);
        let used: Option<i64> = conn.get(&storage_key).await.map_err(|e| e.to_string())?;
        let mut used_bytes = used.unwrap_or(0);
        if used_bytes < 0 {
            used_bytes = 0;
            let _ : () = conn.set(&storage_key, 0).await.unwrap_or(());
        }
        Ok(used_bytes)
    }

    pub async fn set_tenant_tier(&self, tenant_id: &str, tier: PlanTier) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let tier_str = tier.to_string();
        let _ : () = conn.set(format!("tenant:{}:tier", tenant_id), tier_str).await.map_err(|e| e.to_string())?;
        self.tier_cache.insert(tenant_id.to_string(), (tier.clone(), Instant::now()));
        Ok(())
    }

    pub async fn record_token_usage(&self, tenant_id: &str, model: &str, tokens: i64) -> Result<(), String> {
        if tokens <= 0 {
            return Ok(());
        }
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();

        tracing::info!("💰 Miser telemetry: Recording {} tokens for tenant: {} model: {}", tokens, tenant_id, model); // pii-safe // pii-safe

        let tenant_key = format!("tenant:{}:tokens_used:{}", tenant_id, month_key);
        let model_key = format!("tenant:{}:tokens_used:{}:{}", tenant_id, model, month_key);

        let _ : () = redis::AsyncCommands::incr(&mut conn, &tenant_key, tokens).await.unwrap_or(());
        let _ : () = redis::AsyncCommands::incr(&mut conn, &model_key, tokens).await.unwrap_or(());

        let cost_cents = crate::calculator::calculate_cost_cents(model, tokens, 0, 0);

        if let Some(store) = &self.telemetry_store {
            store.mission_cost_cents.add(
                cost_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string()), opentelemetry::KeyValue::new("model", model.to_string())],
            );
        }

        // Expire keys after ~2 months to save space
        let _ : () = redis::AsyncCommands::expire(&mut conn, &tenant_key, 60 * 60 * 24 * 60).await.unwrap_or(());
        let _ : () = redis::AsyncCommands::expire(&mut conn, &model_key, 60 * 60 * 24 * 60).await.unwrap_or(());

        Ok(())
    }

    pub async fn get_token_usage(&self, tenant_id: &str) -> Result<i64, String> {
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();
        let tenant_key = format!("tenant:{}:tokens_used:{}", tenant_id, month_key);

        let count: i64 = redis::AsyncCommands::get(&mut conn, &tenant_key).await.unwrap_or(0);
        Ok(count)
    }

    pub async fn record_action(&self, tenant_id: &str, agent_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(store) = &self.telemetry_store {
            store.rate_limit_checks_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
        }

        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let now = chrono::Utc::now();
        let month_key = now.format("%Y-%m").to_string();

        let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
        tracing::info!("💰 Miser telemetry: Recording action for tenant: {} agent: {}", tenant_id, agent_id); // pii-safe
        let agent_key = format!("tenant:{}:agent:{}:actions_used:{}", tenant_id, agent_id, month_key);

        let tenant_used: u32 = conn.incr(&tenant_key, 1).await.map_err(|e| e.to_string())?;
        let agent_used: u32 = conn.incr(&agent_key, 1).await.map_err(|e| e.to_string())?;

        // Expire keys after ~2 months to save space
        let _ : () = conn.expire(&tenant_key, 60 * 60 * 24 * 60).await.unwrap_or(());
        let _ : () = conn.expire(&agent_key, 60 * 60 * 24 * 60).await.unwrap_or(());

        if let Some(limit) = tier.monthly_action_limit()
            && tenant_used >= limit {
                if let Some(store) = &self.telemetry_store {
                    store.rate_limit_exceeded_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
                }
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit per requirements
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

        if let Some(limit) = tier.agent_action_limit()
            && agent_used >= limit {
                if let Some(store) = &self.telemetry_store {
                    store.rate_limit_exceeded_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
                }
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit per requirements
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

        Ok(RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        })
    }

    pub async fn check_product_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        tracing::info!("💰 Miser telemetry: Checking product quota for tenant: {}", tenant_id); // pii-safe
        if let Some(store) = &self.telemetry_store {
            store.rate_limit_checks_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
        }

        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let product_key = format!("tenant:{}:products", tenant_id);
        let total_products: Option<usize> = conn.get(&product_key).await.map_err(|e| e.to_string())?;
        let total_products = total_products.unwrap_or(0);

        if let Some(limit) = tier.max_products()
            && total_products >= limit {
                if let Some(store) = &self.telemetry_store {
                    store.rate_limit_exceeded_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
                }
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit per requirements
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
        tracing::info!("💰 Miser telemetry: Checking agent quota for tenant: {}", tenant_id); // pii-safe
        if let Some(store) = &self.telemetry_store {
            store.rate_limit_checks_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
        }

        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let agent_key = format!("tenant:{}:agents", tenant_id);
        let total_agents: Option<usize> = conn.get(&agent_key).await.map_err(|e| e.to_string())?;
        let total_agents = total_agents.unwrap_or(0);

        if let Some(limit) = tier.max_agents()
            && total_agents >= limit {
                if let Some(store) = &self.telemetry_store {
                    store.rate_limit_exceeded_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
                }
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit per requirements
                    soft_limit_reached: true,
                    user_message: Some(format!(
                        "You've reached your {} tier limit of {} agents. Upgrade to unlock more power!",
                        match tier {
                            PlanTier::Free => "Free",
                            PlanTier::Starter => "Starter",
                            _ => "Current",
                        },
                        limit
                    )),
                });
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
        tracing::info!("💰 Miser telemetry: Checking storage quota for tenant: {} with delta: {}", tenant_id, delta_bytes); // pii-safe
        if let Some(store) = &self.telemetry_store {
            store.rate_limit_checks_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
        }

        let mut conn = self.get_connection().await?;
        let tier = self.get_tenant_tier(tenant_id).await?;

        let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);

        let mut total_storage: i64 = if delta_bytes == 0 {
            let used: Option<i64> = conn.get(&storage_key).await.map_err(|e| e.to_string())?;
            used.unwrap_or(0)
        } else {
            conn.incr(&storage_key, delta_bytes).await.map_err(|e| e.to_string())?
        };

        if total_storage < 0 {
            let _ : () = conn.set(&storage_key, 0).await.unwrap_or(());
            total_storage = 0;
        }

        if let Some(store) = &self.telemetry_store
            && delta_bytes > 0 {
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
                if let Some(store) = &self.telemetry_store {
                    store.rate_limit_exceeded_total.add(1, &[opentelemetry::KeyValue::new("tenant_id", tenant_id.to_string())]);
                }
                return Ok(RateLimitStatus {
                    is_allowed: true, // Soft limit per requirements
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
        assert_eq!(PlanTier::Starter.storage_limit_mb(), Some(5120));
        assert_eq!(PlanTier::Pro.storage_limit_mb(), Some(51200));
        assert_eq!(PlanTier::Business.storage_limit_mb(), Some(512000));

        assert_eq!(PlanTier::Free.max_agents(), Some(1));
        assert_eq!(PlanTier::Starter.max_agents(), Some(3));
        assert_eq!(PlanTier::Pro.max_agents(), Some(10));
        assert_eq!(PlanTier::Business.max_agents(), None);

        assert_eq!(PlanTier::Free.max_products(), Some(10));
        assert_eq!(PlanTier::Starter.max_products(), Some(100));
        assert_eq!(PlanTier::Pro.max_products(), None);
        assert_eq!(PlanTier::Business.max_products(), None);

        assert_eq!(PlanTier::Free.base_price(), 0.0);
        assert_eq!(PlanTier::Starter.base_price(), 29.0);
        assert_eq!(PlanTier::Pro.base_price(), 79.0);
        assert_eq!(PlanTier::Business.base_price(), 299.0);
    }

    #[test]
    fn test_plan_tier_edge_cases() {
        // Create an "unknown" tier simulation if we were to parse from string
        // Though PlanTier is an enum, we just verify its methods hold true for all variants
        let tiers = vec![PlanTier::Free, PlanTier::Starter, PlanTier::Pro, PlanTier::Business];
        for tier in tiers {
            // Verify base_price is never negative
            assert!(tier.base_price() >= 0.0);

            // Verify that if storage limit is provided, it's non-zero (or handle zero)
            if let Some(limit) = tier.storage_limit_mb() {
                assert!(limit > 0);
            }

            // Max products should be greater than 0 if present
            if let Some(products) = tier.max_products() {
                assert!(products > 0);
            }
        }
    }

    #[test]
    fn test_plan_tier_display_and_from_str() {
        use std::str::FromStr;

        // Test Display
        assert_eq!(PlanTier::Free.to_string(), "Free");
        assert_eq!(PlanTier::Starter.to_string(), "Starter");
        assert_eq!(PlanTier::Pro.to_string(), "Pro");
        assert_eq!(PlanTier::Business.to_string(), "Business");

        // Test FromStr
        assert_eq!(PlanTier::from_str("free").unwrap(), PlanTier::Free);
        assert_eq!(PlanTier::from_str("Free").unwrap(), PlanTier::Free);
        assert_eq!(PlanTier::from_str("starter").unwrap(), PlanTier::Starter);
        assert_eq!(PlanTier::from_str("Starter").unwrap(), PlanTier::Starter);
        assert_eq!(PlanTier::from_str("pro").unwrap(), PlanTier::Pro);
        assert_eq!(PlanTier::from_str("Pro").unwrap(), PlanTier::Pro);
        assert_eq!(PlanTier::from_str("business").unwrap(), PlanTier::Business);
        assert_eq!(PlanTier::from_str("Business").unwrap(), PlanTier::Business);

        // Test Invalid
        assert!(PlanTier::from_str("unknown").is_err());
        assert_eq!(PlanTier::from_str("unknown").unwrap_err(), "Unknown tier: unknown");
    }

    #[tokio::test]
    async fn test_redis_rate_limiter_connection_failure() {
        // Provide an invalid Redis URL to test connection fallback/error
        if let Ok(client) = redis::Client::open("redis://invalid_host:9999") {
            let limiter = RedisRateLimiter::new(client);
            let res = limiter.check_agent_quota("test-tenant").await;
            // It should fail gracefully returning an Err string
            assert!(res.is_err());
        }
    }

    #[tokio::test]
    async fn test_check_product_quota_no_mutation() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-no-mutation";

                // Clear any existing products
                let mut conn = limiter.get_connection().await.unwrap();
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

    #[tokio::test]
    async fn test_check_storage_quota_no_mutation() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-storage-no-mutation";

                let mut conn = limiter.get_connection().await.unwrap();
                let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);
                let _ : () = redis::AsyncCommands::del(&mut conn, &storage_key).await.unwrap_or(());

                let status = limiter.check_storage_quota(tenant_id, 0).await.unwrap();
                assert!(status.is_allowed);
            }
    }

    #[tokio::test]
    async fn test_check_storage_quota() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-storage-quota";

                // Clear any existing storage used
                let mut conn = limiter.get_connection().await.unwrap();
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
                let large_delta: i64 = 500 * 1024 * 1024;
                let status = limiter.check_storage_quota(tenant_id, large_delta).await.unwrap();
                assert!(status.is_allowed);
                assert!(status.soft_limit_reached); // But flag is set
                assert!(status.user_message.unwrap().contains("500MB storage"));
            }
    }

    #[tokio::test]
    async fn test_record_agent_quota() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-agent-quota";

                // Clear any existing agents
                let mut conn = limiter.get_connection().await.unwrap();
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

    #[tokio::test]
    async fn test_record_action_monthly_reset() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-monthly-reset";
                let agent_id = "agent-1";

                let now = chrono::Utc::now();
                let month_key = now.format("%Y-%m").to_string();

                let mut conn = limiter.get_connection().await.unwrap();
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

    #[tokio::test]
    async fn test_agent_action_limit() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-agent-action";
                let agent_id = "agent-limit";

                let mut conn = limiter.get_connection().await.unwrap();
                let now = chrono::Utc::now();
                let month_key = now.format("%Y-%m").to_string();
                let agent_key = format!("tenant:{}:agent:{}:actions_used:{}", tenant_id, agent_id, month_key);
                let _ : () = conn.del(&agent_key).await.unwrap_or(());

                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                for _ in 0..20 {
                    let _ = limiter.record_action(tenant_id, agent_id).await;
                }
                let status = limiter.record_action(tenant_id, agent_id).await.unwrap();
                assert!(status.soft_limit_reached);
            }
    }

    #[tokio::test]
    async fn test_record_token_usage() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-tokens";

                let mut conn = limiter.get_connection().await.unwrap();
                let now = chrono::Utc::now();
                let month_key = now.format("%Y-%m").to_string();
                let tenant_key = format!("tenant:{}:tokens_used:{}", tenant_id, month_key);
                let _ : () = redis::AsyncCommands::del(&mut conn, &tenant_key).await.unwrap_or(());

                let usage = limiter.get_token_usage(tenant_id).await.unwrap();
                assert_eq!(usage, 0);

                limiter.record_token_usage(tenant_id, "gpt-4o", 1500).await.unwrap();
                limiter.record_token_usage(tenant_id, "gpt-4o", 500).await.unwrap();

                let usage = limiter.get_token_usage(tenant_id).await.unwrap();
                assert_eq!(usage, 2000);
            }
    }

    #[tokio::test]
    async fn test_rate_limit_status_is_always_allowed_soft_limit() {
        if let Ok(redis_url) = std::env::var("REDIS_URL")
            && let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-soft-limits";
                let agent_id = "agent-1";

                let mut conn = limiter.get_connection().await.unwrap();
                let now = chrono::Utc::now();
                let month_key = now.format("%Y-%m").to_string();
                let tenant_key = format!("tenant:{}:actions_used:{}", tenant_id, month_key);
                let _ : () = conn.del(&tenant_key).await.unwrap_or(());

                limiter.set_tenant_tier(tenant_id, PlanTier::Free).await.unwrap();

                // exceed limit
                for _ in 0..100 {
                    let _ = limiter.record_action(tenant_id, agent_id).await;
                }
                let status = limiter.record_action(tenant_id, agent_id).await.unwrap();

                assert!(status.is_allowed);
                assert!(status.soft_limit_reached);
            }
    }

    #[tokio::test]
    async fn test_tier_cache_hit() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-cache-hit";

                // Clear any existing tier in redis
                let mut conn = limiter.get_connection().await.unwrap();
                let redis_key = format!("tenant:{}:tier", tenant_id);
                let _ : () = redis::AsyncCommands::del(&mut conn, &redis_key).await.unwrap_or(());

                // Fetch once to populate cache (fallback to Free)
                let tier1 = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(tier1, PlanTier::Free);

                // Now modify redis directly
                let _ : () = redis::AsyncCommands::set(&mut conn, &redis_key, "Starter").await.unwrap_or(());

                // Fetch again, should still be Free because of memory cache
                let tier2 = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(tier2, PlanTier::Free);
            }
        }
    }

    #[tokio::test]
    async fn test_tier_cache_expiration() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-cache-exp";

                // Clear any existing tier in redis
                let mut conn = limiter.get_connection().await.unwrap();
                let redis_key = format!("tenant:{}:tier", tenant_id);
                let _ : () = redis::AsyncCommands::del(&mut conn, &redis_key).await.unwrap_or(());

                // Insert into cache manually with expired time
                limiter.tier_cache.insert(
                    tenant_id.to_string(),
                    (PlanTier::Business, std::time::Instant::now() - std::time::Duration::from_secs(400))
                );

                // Fetch should bypass expired cache, read from redis (which is None -> Free)
                let tier = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(tier, PlanTier::Free);
            }
        }
    }

    #[tokio::test]
    async fn test_tier_cache_invalidation_on_set() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-cache-inv";

                let _ = limiter.set_tenant_tier(tenant_id, PlanTier::Starter).await;

                // Should hit cache
                let tier = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(tier, PlanTier::Starter);

                // Change via set_tenant_tier, which updates cache
                let _ = limiter.set_tenant_tier(tenant_id, PlanTier::Pro).await;

                let new_tier = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(new_tier, PlanTier::Pro);
            }
        }
    }

    #[tokio::test]
    async fn test_get_tenant_tier_db_fallback_cached() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-cache-db";

                let mut conn = limiter.get_connection().await.unwrap();
                let redis_key = format!("tenant:{}:tier", tenant_id);
                let _ : () = redis::AsyncCommands::del(&mut conn, &redis_key).await.unwrap_or(());

                // Assume no db_pool, tier defaults to Free
                let tier = limiter.get_tenant_tier(tenant_id).await.unwrap();
                assert_eq!(tier, PlanTier::Free);

                // Assert it's cached now
                assert!(limiter.tier_cache.contains_key(tenant_id));
            }
        }
    }

    #[tokio::test]
    async fn test_tier_cache_concurrency_safe() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
                let tenant_id = "test-tenant-cache-conc";

                let mut handles = vec![];
                for _ in 0..10 {
                    let lim = limiter.clone();
                    handles.push(tokio::spawn(async move {
                        let _ = lim.get_tenant_tier(tenant_id).await;
                    }));
                }

                for h in handles {
                    let _ = h.await;
                }

                // Ensure only one cached entry
                let entry = limiter.tier_cache.get(tenant_id);
                assert!(entry.is_some());
                assert_eq!(entry.unwrap().0, PlanTier::Free);
            }
        }
    }

    #[tokio::test]
    async fn test_get_tenant_storage_used_negative() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-storage-negative";

                let mut conn = limiter.get_connection().await.unwrap();
                let storage_key = format!("tenant:{}:storage_used_bytes", tenant_id);

                // Set negative storage manually
                let _ : () = redis::AsyncCommands::set(&mut conn, &storage_key, -100).await.unwrap_or(());

                // Getting storage used should fix the negative value to 0
                let used = limiter.get_tenant_storage_used(tenant_id).await.unwrap();
                assert_eq!(used, 0);
            }
        }
    }

    #[tokio::test]
    async fn test_record_token_usage_negative_or_zero() {
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                let limiter = RedisRateLimiter::new(client.clone());
                let tenant_id = "test-tenant-tokens-neg";

                // record 0 tokens
                let res = limiter.record_token_usage(tenant_id, "gpt-4o", 0).await;
                assert!(res.is_ok());

                // record negative tokens
                let res = limiter.record_token_usage(tenant_id, "gpt-4o", -10).await;
                assert!(res.is_ok());

                let usage = limiter.get_token_usage(tenant_id).await.unwrap();
                assert_eq!(usage, 0);
            }
        }
    }
}
