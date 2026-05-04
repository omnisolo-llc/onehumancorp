// Billing module stub - provides Tracker struct used by hub.rs
#[allow(unused_imports)]
pub use crate::services::billing::auditor::CostAuditor;
use crate::pricing::rate_limit::{RedisRateLimiter, RateLimitStatus};
use crate::integrations::stripe::client::StripeClient;
use redis::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct Tracker {
    rate_limiter: Option<Arc<RedisRateLimiter>>,
    pub stripe_client: Option<Arc<StripeClient>>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker { rate_limiter: None, stripe_client: None }
    }

    pub fn new_with_redis(redis_url: &str) -> Self {
        let stripe_client = std::env::var("STRIPE_API_KEY")
            .ok()
            .map(|key| Arc::new(StripeClient::new(key)));
        if let Ok(client) = Client::open(redis_url) {
            Tracker {
                rate_limiter: Some(Arc::new(RedisRateLimiter::new(client))),
                stripe_client,
            }
        } else {
            Tracker { rate_limiter: None, stripe_client }
        }
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

    pub async fn create_checkout_session(&self, price_id: &str, customer_id: &str) -> Result<String, String> {
        if let Some(ref client) = self.stripe_client {
            client.create_checkout_session(price_id, customer_id).await
        } else {
            Err("Stripe client not configured".to_string())
        }
    }

    pub async fn list_invoices(&self, customer_id: &str) -> Result<Vec<crate::integrations::stripe::client::StripeInvoice>, String> {
        if let Some(ref client) = self.stripe_client {
            client.list_invoices(customer_id).await
        } else {
            Err("Stripe client not configured".to_string())
        }
    }

    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<crate::integrations::stripe::client::StripeSubscription, String> {
        if let Some(ref client) = self.stripe_client {
            client.cancel_subscription(subscription_id).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracker_stripe_methods_without_client() {
        let tracker = Tracker::new();

        let res = tracker.create_checkout_session("price_123", "cus_123").await;
        assert_eq!(res.unwrap_err(), "Stripe client not configured");

        let res = tracker.list_invoices("cus_123").await;
        assert_eq!(res.unwrap_err(), "Stripe client not configured");

        let res = tracker.cancel_subscription("sub_123").await;
        assert_eq!(res.unwrap_err(), "Stripe client not configured");
    }

    #[tokio::test]
    async fn test_tracker_stripe_methods_with_client() {
        let mut tracker = Tracker::new();
        tracker.stripe_client = Some(Arc::new(StripeClient::new("sk_test_123".to_string())));

        let session_url = tracker.create_checkout_session("price_123", "cus_123").await.unwrap();
        assert!(session_url.starts_with("https://checkout.stripe.com"));

        let invoices = tracker.list_invoices("cus_123").await.unwrap();
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].status, "paid");

        let sub = tracker.cancel_subscription("sub_123").await.unwrap();
        assert_eq!(sub.status, "canceled");
    }
}
