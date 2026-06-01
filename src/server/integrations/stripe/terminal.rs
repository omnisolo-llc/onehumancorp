use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalConnectionToken {
    pub secret: String,
}

pub struct StripeTerminalClient {
    pub api_key: String,
}

impl StripeTerminalClient {
    pub fn new(api_key: String) -> Self {
        StripeTerminalClient { api_key }
    }

    /// Mocks creating a Stripe Terminal connection token
    pub async fn create_connection_token(&self, merchant_id: &str) -> Result<TerminalConnectionToken, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            merchant_id,
            "stripe_terminal_connection_token",
            0.01 // cost for API call
        ).await;

        Ok(TerminalConnectionToken {
            secret: format!("tct_mock_secret_{}", merchant_id),
        })
    }

    /// Mocks capturing a Stripe Terminal payment intent
    pub async fn capture_payment(&self, payment_intent_id: &str, merchant_id: &str) -> Result<bool, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            merchant_id,
            "stripe_terminal_capture_payment",
            0.05
        ).await;

        if payment_intent_id.is_empty() {
            return Err("Invalid payment intent ID".to_string());
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_connection_token() {
        let client = StripeTerminalClient::new("sk_test_123".to_string());
        let token = client.create_connection_token("merchant_1").await.unwrap();
        assert_eq!(token.secret, "tct_mock_secret_merchant_1");
    }

    #[tokio::test]
    async fn test_capture_payment() {
        let client = StripeTerminalClient::new("sk_test_123".to_string());
        assert!(client.capture_payment("pi_123", "merchant_1").await.unwrap());
        assert!(client.capture_payment("", "merchant_1").await.is_err());
    }
}
