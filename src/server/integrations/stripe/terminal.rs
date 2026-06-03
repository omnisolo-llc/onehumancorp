use serde::{Deserialize, Serialize};

pub struct StripeTerminal {
    pub api_key: String,
}

impl StripeTerminal {
    pub fn new(api_key: String) -> Self {
        StripeTerminal { api_key }
    }

    pub async fn create_connection_token(&self, tenant_id: &str) -> Result<String, String> {
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

    pub async fn create_payment_intent(&self, tenant_id: &str, amount_usd: f64) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "stripe_terminal_payment_intent",
            0.10 // mock cost for api orchestration
        ).await;

        // In a real implementation, this would make an HTTP POST to Stripe's /v1/payment_intents
        // endpoint with payment_method_types=["card_present"].
        // Since we're mocking external APIs, we return a mock payment intent string here.
        let amount_cents = (amount_usd * 100.0) as i64;
        let mock_intent = format!("pi_mock_intent_{}_{}", tenant_id, amount_cents);

        Ok(mock_intent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_connection_token() {
        let terminal = StripeTerminal::new("sk_test_123".to_string());
        let result = terminal.create_connection_token("test_tenant").await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, "tss_mock_token_for_test_tenant");
    }

    #[tokio::test]
    async fn test_create_payment_intent() {
        let terminal = StripeTerminal::new("sk_test_123".to_string());
        let result = terminal.create_payment_intent("test_tenant", 15.50).await;
        assert!(result.is_ok());
        let intent = result.unwrap();
        assert_eq!(intent, "pi_mock_intent_test_tenant_1550");
    }
}
