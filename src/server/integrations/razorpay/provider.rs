use super::client::RazorpayClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct RazorpayProvider {
    _client: Arc<RazorpayClient>,
    metadata: ProviderMetadata,
}

impl RazorpayProvider {
    pub fn new(api_key: String) -> Self {
        let client = RazorpayClient::new(api_key);

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

    pub async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        self._client.create_payment(amount, description, payer_email).await
    }
}
