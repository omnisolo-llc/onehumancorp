use super::client::{TaxJarClient, TaxJarParams, TaxRate};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TaxJarProvider {
    _client: Arc<TaxJarClient>,
    metadata: ProviderMetadata,
}

impl TaxJarProvider {
    pub fn new(api_key: String) -> Self {
        let client = TaxJarClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "taxjar".to_string(),
                name: "TaxJar".to_string(),
                category: "finance".to_string(),
                base_url: "https://api.taxjar.com/v2".to_string(),
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

    pub async fn calculate_tax(&self, params: TaxJarParams<'_>) -> Result<TaxRate, String> {
        self._client.calculate_tax(params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taxjar_provider_new() {
        let provider = TaxJarProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "taxjar");
        assert_eq!(provider.metadata.category, "finance");
    }

    #[test]
    fn test_taxjar_provider_into() {
        let provider = TaxJarProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "taxjar");
    }
}
