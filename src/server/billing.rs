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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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
                Err(e) => {
                    tracing::warn!("RateLimiter error: {}. Failing open to avoid blocking users.", e);
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

pub fn process_complex_refund(amount_cents: i64, reason: &str, days_since_purchase: u32) -> Result<i64, String> {
    if days_since_purchase > 30 {
        return Err("Refund window expired".to_string());
    }

    let mut refund_amount = amount_cents;

    // Prorate refund based on days used if it's a subscription
    if reason == "subscription_cancel" {
        let daily_rate = amount_cents as f64 / 30.0;
        let used_amount = (daily_rate * days_since_purchase as f64).round() as i64;
        refund_amount = amount_cents.saturating_sub(used_amount);
    }

    // Deduction for processing fees if not a fault of service
    if reason == "user_error" || reason == "changed_mind" {
        let fee = (amount_cents as f64 * 0.03) as i64; // 3% fee
        refund_amount = refund_amount.saturating_sub(fee);
    }

    Ok(refund_amount)
}

pub struct InvoiceGenerator {
    pub company_name: String,
    pub tax_rate: f64,
}

impl InvoiceGenerator {
    pub fn new(company_name: &str, tax_rate: f64) -> Self {
        Self {
            company_name: company_name.to_string(),
            tax_rate,
        }
    }

    pub fn generate_html_invoice(&self, items: Vec<(&str, i64)>, customer_name: &str) -> String {
        let mut html = format!("<html><body><h1>Invoice from {}</h1>", self.company_name);
        html.push_str(&format!("<h2>Bill To: {}</h2>", customer_name));
        html.push_str("<table><tr><th>Item</th><th>Amount (cents)</th></tr>");

        let mut subtotal = 0;
        for (name, amount) in items {
            html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", name, amount));
            subtotal += amount;
        }

        let tax = (subtotal as f64 * self.tax_rate).round() as i64;
        let total = subtotal + tax;

        html.push_str("</table>");
        html.push_str(&format!("<p>Subtotal: {}</p>", subtotal));
        html.push_str(&format!("<p>Tax: {}</p>", tax));
        html.push_str(&format!("<h3>Total: {}</h3>", total));
        html.push_str("</body></html>");

        html
    }
}

#[cfg(test)]
mod billing_logic_tests {
    use super::*;

    #[test]
    fn test_process_complex_refund() {
        assert_eq!(process_complex_refund(10000, "defective", 10).unwrap(), 10000);
        assert_eq!(process_complex_refund(10000, "subscription_cancel", 15).unwrap(), 5000);
        assert_eq!(process_complex_refund(10000, "changed_mind", 5).unwrap(), 9700);
        assert!(process_complex_refund(10000, "defective", 35).is_err());
    }

    #[test]
    fn test_invoice_generator() {
        let generator = InvoiceGenerator::new("TestCorp", 0.1);
        let items = vec![("Item A", 5000), ("Item B", 5000)];
        let html = generator.generate_html_invoice(items, "John Doe");

        assert!(html.contains("Invoice from TestCorp"));
        assert!(html.contains("John Doe"));
        assert!(html.contains("Subtotal: 10000"));
        assert!(html.contains("Tax: 1000"));
        assert!(html.contains("Total: 11000"));
    }
}
