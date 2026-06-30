use crate::integrations::mercadopago::client::MercadoPagoClient;
use ::server_pricing::rate_limit::{RedisRateLimiter, RateLimitStatus};
use crate::integrations::stripe::client::StripeClient;
use redis::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tracker {
    rate_limiter: Option<Arc<RedisRateLimiter>>,
    pub stripe_client: Option<Arc<StripeClient>>,
    pub mercadopago_client: Option<Arc<MercadoPagoClient>>,
    pub auditor: Option<Arc<crate::services::billing::auditor::CostAuditor>>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker { rate_limiter: None, stripe_client: None, mercadopago_client: None, auditor: None }
    }

    pub fn new_with_redis(redis_url: &str) -> Self {
        let mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| Arc::new(MercadoPagoClient::new(token)));
        let stripe_client = std::env::var("STRIPE_API_KEY")
            .ok()
            .map(|key| Arc::new(StripeClient::new(key)));
        if let Ok(client) = Client::open(redis_url) {
            Tracker {
                rate_limiter: Some(Arc::new(RedisRateLimiter::new(client))),
                stripe_client,
                mercadopago_client: mercadopago_client.clone(),
                auditor: None,
            }
        } else {
            Tracker { rate_limiter: None, stripe_client, mercadopago_client, auditor: None }
        }
    }

    pub fn with_db(mut self, pool: sqlx::PgPool) -> Self {
        if let Some(limiter) = self.rate_limiter.take() {
            if let Ok(l) = Arc::try_unwrap(limiter) {
                self.rate_limiter = Some(Arc::new(l.with_db(pool)));
            } else {
            }
        }
        self
    }

    pub fn set_auditor(&mut self, auditor: Arc<crate::services::billing::auditor::CostAuditor>) {
        self.auditor = Some(auditor);
    }

    fn fail_open_status() -> RateLimitStatus {
        tracing::debug!("RateLimiter error. Failing open to avoid blocking users.");
        RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        }
    }

    fn allow_status() -> RateLimitStatus {
        RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        }
    }

    pub async fn track_storage_usage(&self, tenant_id: &str, delta_bytes: i64, agent_id: Option<&str>) -> Result<RateLimitStatus, String> {
        if let Some(auditor) = &self.auditor {
            if let Some(aid) = agent_id {
                auditor.record_agent_storage(aid, delta_bytes);
            }
        }
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.check_storage_quota(tenant_id, delta_bytes).await {
                Ok(status) => Ok(status),
                Err(_) => Ok(Self::fail_open_status()),
            }
        } else {
            Ok(Self::allow_status())
        }
    }

    pub async fn check_product_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.check_product_quota(tenant_id).await {
                Ok(status) => Ok(status),
                Err(_) => Ok(Self::fail_open_status()),
            }
        } else {
            Ok(Self::allow_status())
        }
    }

    pub async fn record_product_added(&self, tenant_id: &str) -> Result<(), String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.record_product_added(tenant_id).await {
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::debug!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    pub async fn check_rate_limit(&self, tenant_id: &str, agent_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.record_action(tenant_id, agent_id).await {
                Ok(status) => Ok(status),
                Err(_) => Ok(Self::fail_open_status()),
            }
        } else {
            Ok(Self::allow_status())
        }
    }

    pub async fn check_agent_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.check_agent_quota(tenant_id).await {
                Ok(status) => Ok(status),
                Err(_) => Ok(Self::fail_open_status()),
            }
        } else {
            Ok(Self::allow_status())
        }
    }

    pub async fn record_agent_added(&self, tenant_id: &str) -> Result<(), String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.record_agent_added(tenant_id).await {
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::debug!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    pub async fn get_tenant_tier(&self, tenant_id: &str) -> Result<::server_pricing::rate_limit::PlanTier, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.get_tenant_tier(tenant_id).await
        } else {
            Ok(::server_pricing::rate_limit::PlanTier::Free)
        }
    }

    pub async fn get_tenant_actions_used(&self, tenant_id: &str) -> Result<u32, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.get_tenant_actions_used(tenant_id).await
        } else {
            Ok(0)
        }
    }

    pub async fn get_agent_actions_used(&self, tenant_id: &str, agent_id: &str) -> Result<u32, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.get_agent_actions_used(tenant_id, agent_id).await
        } else {
            Ok(0)
        }
    }

    pub async fn get_tenant_storage_used(&self, tenant_id: &str) -> Result<i64, String> {
        if let Some(ref limiter) = self.rate_limiter {
            limiter.get_tenant_storage_used(tenant_id).await
        } else {
            Ok(0)
        }
    }

    pub async fn get_subscription(&self, subscription_id: &str) -> Result<crate::integrations::stripe::client::StripeSubscription, String> {
        if let Some(ref client) = self.stripe_client {
            client.get_subscription(subscription_id).await
        } else {
            Err("Stripe client not configured".to_string())
        }
    }

    pub fn get_tenant_tokens(&self, tenant_id: &str) -> i64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_tenant_tokens(tenant_id)
        } else {
            0
        }
    }

    pub fn get_tenant_cached_tokens(&self, tenant_id: &str) -> i64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_tenant_cached_tokens(tenant_id)
        } else {
            0
        }
    }

    pub fn get_tenant_cost_cents(&self, tenant_id: &str) -> i64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_tenant_cost_cents(tenant_id)
        } else {
            0
        }
    }

    pub fn get_tenant_llm_cost_cents(&self, tenant_id: &str) -> i64 {
        self.get_tenant_cost_cents(tenant_id)
    }

    pub fn get_tenant_payment_fees(&self, tenant_id: &str) -> f64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_tenant_payment_fees(tenant_id)
        } else {
            0.0
        }
    }

    pub fn get_tenant_bandwidth_savings(&self, tenant_id: &str) -> f64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_tenant_bandwidth_savings(tenant_id)
        } else {
            0.0
        }
    }

    pub fn get_storage_cost_cents(&self, bytes: i64) -> i64 {
        let cost_per_gb = if let Some(ref auditor) = self.auditor { auditor.get_cost_per_gb_month() } else { 0.10 };
        crate::pricing::calculator::calculate_storage_cost_cents(bytes, &crate::pricing::calculator::CostConfig { cost_per_gb_month: cost_per_gb, ..Default::default() })
    }

    pub fn get_total_cost_cents(&self) -> i64 {
        if let Some(ref auditor) = self.auditor {
            auditor.get_total_cost_cents()
        } else {
            0
        }
    }

    pub fn track_outbound_api_call(&self, tenant_id: &str, endpoint: &str) {
        tracing::info!("💰 Miser telemetry: Recording outbound API call for tenant: {}, endpoint: {}", tenant_id, endpoint); // pii-safe // pii-safe
        if let Some(ref auditor) = self.auditor {
            auditor.record_api_call(tenant_id, endpoint);
        }
    }

    pub fn track_email_send(&self, tenant_id: &str) {
        tracing::info!("💰 Miser telemetry: Recording communication metric for tenant: {}", tenant_id); // pii-safe // pii-safe
        if let Some(ref auditor) = self.auditor {
            auditor.record_email_send(tenant_id);
        }
    }

    pub fn record_bandwidth_compression(&self, tenant_id: &str, original_bytes: i64, compressed_bytes: i64) {
        if let Some(ref auditor) = self.auditor {
            auditor.record_bandwidth_compression(tenant_id, original_bytes, compressed_bytes);
        }
    }

    pub fn summary(&self, _scope: &str) -> TokenSummary {
        let total_tokens = if let Some(auditor) = &self.auditor {
            auditor.get_total_tokens()
        } else {
            0
        };
        let total_cached_tokens = if let Some(auditor) = &self.auditor {
            auditor.get_total_cached_tokens()
        } else {
            0
        };
        TokenSummary { total_tokens, total_cached_tokens }
    }
}

#[derive(Default)]
pub struct TokenSummary {
    pub total_tokens: i64,
    pub total_cached_tokens: i64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new()
    }
}
