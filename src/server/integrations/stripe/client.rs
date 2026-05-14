use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub current_period_end: i64,
}

pub struct StripeClient {
    pub api_key: String,
}

impl StripeClient {
    pub fn new(api_key: String) -> Self {
        StripeClient { api_key }
    }

    pub async fn create_checkout_session(&self, _price_id: &str, customer_id: &str, amount_usd: f64) -> Result<String, String> {
        let pool = crate::db::get_pool();
        let _ = ::server_telemetry::record_api_call_cost(
            &pool,
            customer_id, // assume customer_id is a proxy for organization_id
            "stripe_checkout_session",
            0.10 // mock cost for api orchestration
        ).await;

        // Use PaymentRouter to optimize method
        let pm = crate::integrations::stripe::routing::PaymentRouter::optimize_payment_method(amount_usd);

        match pm {
            crate::integrations::stripe::routing::PaymentMethod::Ach => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_ach...".to_string())
            },
            crate::integrations::stripe::routing::PaymentMethod::CreditCard => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_...".to_string())
            }
        }
    }

    pub async fn cancel_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        Ok(StripeSubscription {
            id: "sub_test_...".to_string(),
            status: "canceled".to_string(),
            current_period_end: 1714560000,
        })
    }
}
