use super::client::ShippoClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    _client: Arc<ShippoClient>,
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_key: String) -> Self {
        let client = ShippoClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Logistics".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
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

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        self._client.fetch_rates(weight, dimensions).await
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        self._client.purchase_label(rate_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shippo_provider_new() {
        let provider = ShippoProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "shippo");
        assert_eq!(provider.metadata.category, "shipping");
    }

    #[test]
    fn test_shippo_provider_into() {
        let provider = ShippoProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "shippo");
    }
}

impl ShippoProvider {
    pub async fn generate_and_email_label(&self, rate_id: &str, _email: &str) -> Result<String, String> {
        let label_url = self.purchase_label(rate_id).await?;
        // Mock emailing tracking numbers to the customer
        Ok(label_url)
    }
}
