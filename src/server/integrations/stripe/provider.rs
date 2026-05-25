use super::client::{StripeClient, StripeSubscription, StripeInvoice};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct StripeProvider {
    _client: Arc<StripeClient>,
    metadata: ProviderMetadata,
}

impl StripeProvider {
    pub fn new(api_key: String) -> Self {
        let client = StripeClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "stripe".to_string(),
                name: "Stripe".to_string(),
                category: "payment".to_string(),
                base_url: "https://api.stripe.com".to_string(),
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
}

impl StripeProvider {
    pub async fn create_checkout_session(&self, price_id: &str, customer_id: &str, amount_usd: f64) -> Result<String, String> {
        self._client.create_checkout_session(price_id, customer_id, amount_usd).await
    }

    pub async fn get_subscription(&self, subscription_id: &str) -> Result<StripeSubscription, String> {
        self._client.get_subscription(subscription_id).await
    }

    pub async fn list_invoices(&self, customer_id: &str) -> Result<Vec<StripeInvoice>, String> {
        self._client.list_invoices(customer_id).await
    }

    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<StripeSubscription, String> {
        self._client.cancel_subscription(subscription_id).await
    }
}
