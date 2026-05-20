use crate::integrations::mercadopago::client::MercadoPagoClient;
// Billing module stub - provides Tracker struct used by hub.rs
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



    pub fn set_auditor(&mut self, auditor: Arc<crate::services::billing::auditor::CostAuditor>) {
        self.auditor = Some(auditor);
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
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(RateLimitStatus {
                        is_allowed: true,
                        soft_limit_reached: false,
                        user_message: None,
                    })
                }
            }
        } else {
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub async fn check_product_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.check_product_quota(tenant_id).await {
                Ok(status) => Ok(status),
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(RateLimitStatus {
                        is_allowed: true,
                        soft_limit_reached: false,
                        user_message: None,
                    })
                }
            }
        } else {
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub async fn record_product_added(&self, tenant_id: &str) -> Result<(), String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.record_product_added(tenant_id).await {
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
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
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(RateLimitStatus {
                        is_allowed: true,
                        soft_limit_reached: false,
                        user_message: None,
                    })
                }
            }
        } else {
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub async fn check_agent_quota(&self, tenant_id: &str) -> Result<RateLimitStatus, String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.check_agent_quota(tenant_id).await {
                Ok(status) => Ok(status),
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
                    Ok(RateLimitStatus {
                        is_allowed: true,
                        soft_limit_reached: false,
                        user_message: None,
                    })
                }
            }
        } else {
            Ok(RateLimitStatus {
                is_allowed: true,
                soft_limit_reached: false,
                user_message: None,
            })
        }
    }

    pub async fn record_agent_added(&self, tenant_id: &str) -> Result<(), String> {
        if let Some(ref limiter) = self.rate_limiter {
            match limiter.record_agent_added(tenant_id).await {
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::warn!("RateLimiter error. Failing open to avoid blocking users.");
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

    pub fn summary(&self, _scope: &str) -> TokenSummary {
        TokenSummary::default()
    }
}

#[derive(Default)]
pub struct TokenSummary {
    pub total_tokens: i64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new()
    }
}
