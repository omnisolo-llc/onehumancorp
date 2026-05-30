use super::client::StripeClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
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
                name: "Stripe Payment".to_string(),
                category: "finance".to_string(),
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

    pub async fn create_checkout_session(&self, price_id: &str, customer_id: &str, amount_usd: f64) -> Result<String, String> {
        self.client.create_checkout_session(price_id, customer_id, amount_usd).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_provider_new() {
        let provider = StripeProvider::new("test_key".to_string());
        assert_eq!(provider.metadata.id, "stripe");
        assert_eq!(provider.metadata.category, "finance");
    }

    #[test]
    fn test_stripe_provider_into() {
        let provider = StripeProvider::new("test_key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "stripe");
    }
}
impl StripeProvider {
    pub async fn process_checkout(&self, price_id: &str, customer_id: &str, amount_usd: f64) -> Result<String, String> {
        self.create_checkout_session(price_id, customer_id, amount_usd).await
    }
}
