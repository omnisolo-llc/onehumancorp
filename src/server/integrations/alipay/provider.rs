use super::client::AlipayClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AlipayProvider {
    _client: Arc<AlipayClient>,
    metadata: ProviderMetadata,
}

impl AlipayProvider {
    pub fn new(access_token: String) -> Self {
        let client = AlipayClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "alipay".to_string(),
                name: "Alipay".to_string(),
                category: "payment".to_string(),
                base_url: "https://openapi.alipay.com".to_string(),
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
