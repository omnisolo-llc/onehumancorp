// Billing module stub - provides Tracker struct used by hub.rs
#[allow(unused_imports)]
pub use crate::services::billing::auditor::CostAuditor;
use crate::pricing::rate_limit::{RedisRateLimiter, RateLimitStatus};
use crate::integrations::stripe::client::StripeClient;
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tracker {
    rate_limiter: Option<Arc<RedisRateLimiter>>,
    pub stripe_client: Option<Arc<StripeClient>>,
    db_pool: Option<PgPool>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker { rate_limiter: None, stripe_client: None, db_pool: None }
    }

    pub fn set_db_pool(&mut self, pool: PgPool) {
        self.db_pool = Some(pool);
    }

    pub fn new_with_redis(redis_url: &str) -> Self {
        let stripe_client = std::env::var("STRIPE_API_KEY")
            .ok()
            .map(|key| Arc::new(StripeClient::new(key)));
        if let Ok(client) = Client::open(redis_url) {
            Tracker {
                rate_limiter: Some(Arc::new(RedisRateLimiter::new(client))),
                stripe_client,
                db_pool: None,
            }
        } else {
            Tracker { rate_limiter: None, stripe_client: None, db_pool: None }
        }
    }

    pub async fn record_token_usage(&self, tenant_id: &str, agent_id: &str, model: &str, prompt_tokens: i64, completion_tokens: i64, cost_usd: f64) -> Result<(), String> {
        if let Some(ref pool) = self.db_pool {
            sqlx::query(
                "INSERT INTO usage_events (agent_id, agent_role, organization_id, model, prompt_tokens, completion_tokens, cost_usd)
                 VALUES ($1, 'agent', $2, $3, $4, $5, $6)"
            )
            .bind(agent_id)
            .bind(tenant_id)
            .bind(model)
            .bind(prompt_tokens)
            .bind(completion_tokens)
            .bind(cost_usd)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn check_rate_limit(&self, tenant_id: &str, agent_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.record_action(tenant_id, agent_id).await
        } else {
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub async fn get_subscription(&self, subscription_id: &str) -> Result<crate::integrations::stripe::client::StripeSubscription, String> {
        if let Some(ref client) = self.stripe_client {
            client.get_subscription(subscription_id).await
        } else {
            Err("Stripe client not configured".to_string())
        }
    }

    pub async fn summary(&self, tenant_id: &str) -> TokenSummary {
        if let Some(ref pool) = self.db_pool {
            if let Ok(row) = sqlx::query_as::<_, (i64, f64)>("SELECT COALESCE(SUM(prompt_tokens + completion_tokens), 0), COALESCE(SUM(cost_usd), 0.0) FROM usage_events WHERE organization_id = $1")
                .bind(tenant_id)
                .fetch_one(pool)
                .await {
                return TokenSummary {
                    total_tokens: row.0,
                    total_cost_usd: row.1,
                };
            }
        }
        TokenSummary::default()
    }
}

#[derive(Default)]
pub struct TokenSummary {
    pub total_tokens: i64,
    pub total_cost_usd: f64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new()
    }
}
