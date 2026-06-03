use super::client::PrintfulClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct PrintfulProvider {
    _client: Arc<PrintfulClient>,
    metadata: ProviderMetadata,
}

impl PrintfulProvider {
    pub fn new(api_key: String) -> Self {
        let client = PrintfulClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "printful".to_string(),
                name: "Printful Print-On-Demand".to_string(),
                category: "merchandising".to_string(),
                base_url: "https://api.printful.com".to_string(),
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

    pub async fn fetch_catalog(&self) -> Result<Vec<String>, String> {
        self._client.fetch_catalog().await
    }

    pub async fn generate_mockup(&self, product_id: &str, design_url: &str) -> Result<String, String> {
        self._client.generate_mockup(product_id, design_url).await
    }

    pub async fn create_order(&self, product_id: &str, design_url: &str, address: &str) -> Result<String, String> {
        self._client.create_order(product_id, design_url, address).await
    }

    pub async fn handle_webhook(&self, payload: &str) -> Result<(), String> {
        self._client.handle_webhook(payload).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printful_provider_new() {
        let provider = PrintfulProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "printful");
        assert_eq!(provider.metadata.category, "merchandising");
    }

    #[test]
    fn test_printful_provider_into() {
        let provider = PrintfulProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "printful");
    }
}

impl PrintfulProvider {
    pub async fn generate_mockup_and_list_product(&self, product_id: &str, design_url: &str) -> Result<String, String> {
        let mockup_url = self.generate_mockup(product_id, design_url).await?;
        // Mock listing product on storefront
        Ok(mockup_url)
    }
}
