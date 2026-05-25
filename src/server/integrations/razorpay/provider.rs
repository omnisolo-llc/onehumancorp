use super::client::RazorpayClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct RazorpayProvider {
    _client: Arc<RazorpayClient>,
    metadata: ProviderMetadata,
}

impl RazorpayProvider {
    pub fn new(key_id: String, key_secret: String) -> Self {
        let client = RazorpayClient::new(key_id, key_secret);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "razorpay".to_string(),
                name: "Razorpay".to_string(),
                category: "payment".to_string(),
                base_url: "https://api.razorpay.com/v1".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            }
        }
    }

    pub async fn create_payment_link(&self, amount: f64, description: &str, customer_email: &str) -> Result<String, String> {
        self._client.create_payment_link(amount, description, customer_email).await
    }

    pub async fn fetch_payment(&self, payment_id: &str) -> Result<String, String> {
        self._client.fetch_payment(payment_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_razorpay_provider_new() {
        let provider = RazorpayProvider::new("test_id".to_string(), "test_secret".to_string());
        assert_eq!(provider.metadata.id, "razorpay");
        assert_eq!(provider.metadata.category, "payment");
    }

    #[test]
    fn test_razorpay_provider_into() {
        let provider = RazorpayProvider::new("test_id".to_string(), "test_secret".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "razorpay");
    }

    #[tokio::test]
    async fn test_razorpay_provider_create_payment_link() {
        let provider = RazorpayProvider::new("test_id".to_string(), "test_secret".to_string());
        let result = provider.create_payment_link(100.0, "Test payment", "test@example.com").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_razorpay_provider_fetch_payment() {
        let provider = RazorpayProvider::new("test_id".to_string(), "test_secret".to_string());
        let result = provider.fetch_payment("pay_123").await;
        assert!(result.is_ok());
    }
}
