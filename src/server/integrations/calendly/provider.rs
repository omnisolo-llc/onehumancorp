use super::client::{CalendlyClientWrapper, RealCalendlyClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalendlyProvider {
    _client: Arc<dyn CalendlyClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl CalendlyProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealCalendlyClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.calendly.com".to_string(),
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

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        self._client.fetch_event_types().await
    }

    pub async fn create_webhook(&self, url: &str) -> Result<(), String> {
        self._client.create_webhook(url).await
    }
}
