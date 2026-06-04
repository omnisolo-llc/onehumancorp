use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentIntent {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
}

pub struct StripeTerminalClient {
    pub api_key: String,
}

impl StripeTerminalClient {
    pub fn new(api_key: String) -> Self {
        StripeTerminalClient { api_key }
    }

    pub async fn create_terminal_connection_token(&self, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "stripe_terminal_connection_token",
            0.05 // mock cost for api orchestration
        ).await;

        // In a real implementation, this would make an HTTP POST to Stripe's /v1/terminal/connection_tokens
        // endpoint. Since we're mocking external APIs, we return a mock token string here.
        // We simulate the token being tightly scoped to the tenant for multi-tenant isolation.
        let mock_token = format!("tss_mock_token_for_{}", tenant_id);

        Ok(mock_token)
    }

    pub async fn create_payment_intent(&self, tenant_id: &str, amount: i64, currency: &str) -> Result<PaymentIntent, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "stripe_terminal_payment_intent",
            0.05 // mock cost
        ).await;

        Ok(PaymentIntent {
            id: format!("pi_mock_{}", tenant_id),
            amount,
            currency: currency.to_string(),
            status: "requires_payment_method".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_terminal_connection_token() {
        let client = StripeTerminalClient::new("sk_test_123".to_string());
        let result = client.create_terminal_connection_token("test_tenant").await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, "tss_mock_token_for_test_tenant");
    }

    #[tokio::test]
    async fn test_create_payment_intent() {
        let client = StripeTerminalClient::new("sk_test_123".to_string());
        let result = client.create_payment_intent("test_tenant", 1000, "usd").await;
        assert!(result.is_ok());
        let intent = result.unwrap();
        assert_eq!(intent.amount, 1000);
        assert_eq!(intent.currency, "usd");
    }
}
