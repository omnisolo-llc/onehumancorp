use super::client::RazorpayClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct RazorpayProvider {
    _client: Arc<RazorpayClient>,
    pub metadata: ProviderMetadata,
}

impl RazorpayProvider {
    pub fn new(api_key: String, api_secret: String) -> Self {
        let client = RazorpayClient::new(api_key, api_secret);

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

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        self._client.create_checkout_preference(price_id, tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_razorpay_provider_new() {
        let provider = RazorpayProvider::new("key".to_string(), "secret".to_string());
        assert_eq!(provider.metadata.id, "razorpay");
        assert_eq!(provider.metadata.category, "payment");
    }

    #[test]
    fn test_razorpay_provider_into() {
        let provider = RazorpayProvider::new("key".to_string(), "secret".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "razorpay");
    }
}
