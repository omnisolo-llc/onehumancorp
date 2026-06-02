use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionToken {
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntent {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub capture_method: String,
}

impl super::client::StripeClient {
    pub async fn create_terminal_connection_token(&self) -> Result<ConnectionToken, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "stripe_create_terminal_connection_token",
            0.05
        ).await;
        Ok(ConnectionToken {
            secret: "tctc_test_1234567890".to_string(),
        })
    }

    pub async fn create_terminal_payment_intent(&self, amount: i64, currency: &str) -> Result<PaymentIntent, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "stripe_create_terminal_payment_intent",
            0.05
        ).await;
        Ok(PaymentIntent {
            id: "pi_test_12345".to_string(),
            amount,
            currency: currency.to_string(),
            status: "requires_payment_method".to_string(),
            capture_method: "manual".to_string(), // Typical for terminal
        })
    }
}
