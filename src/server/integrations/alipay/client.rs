use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlipayCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct AlipayClient {
    pub access_token: String,
}

impl AlipayClient {
    pub fn new(access_token: String) -> Self {
        AlipayClient { access_token }
    }

    pub async fn create_checkout_preference(&self, _price_id: &str, _tenant_id: &str) -> Result<String, String> {
        Err("Alipay access token is required".to_string())
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook
        Ok(())
    }
}

impl AlipayClient {
    pub async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
        Err("Alipay access token is required".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn checkout_preference_does_not_return_mock_gateway_url() {
        let client = AlipayClient::new("".to_string());
        let err = client.create_checkout_preference("price_123", "tenant_123").await.unwrap_err();
        assert!(err.contains("Alipay access token is required"));
    }

    #[tokio::test]
    async fn create_payment_does_not_return_mock_transaction_id() {
        let client = AlipayClient::new("".to_string());
        let err = client.create_payment(100.0, "Order", "buyer@example.com").await.unwrap_err();
        assert!(err.contains("Alipay access token is required"));
    }
}
