use super::client::StripeClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct StripeProvider {
    client: Arc<StripeClient>,
    metadata: ProviderMetadata,
}

impl StripeProvider {
    pub fn new(api_key: String) -> Self {
        let client = StripeClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "stripe".to_string(),
                name: "Stripe Connect".to_string(),
                category: "payment".to_string(),
                base_url: "https://api.stripe.com/v1".to_string(),
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

    pub async fn create_payment_link(&self, amount_usd: f64, description: &str, customer_id: &str) -> Result<String, String> {
        self.client.create_checkout_session(description, customer_id, amount_usd).await
    }
}
