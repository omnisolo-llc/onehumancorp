use super::client::CalendlyClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalendlyProvider {
    _client: Arc<CalendlyClient>,
    metadata: ProviderMetadata,
}

impl CalendlyProvider {
    pub fn new(api_key: String) -> Self {
        let client = CalendlyClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly".to_string(),
                category: "calendar".to_string(),
                base_url: "https://placeholder.url".to_string(),
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
