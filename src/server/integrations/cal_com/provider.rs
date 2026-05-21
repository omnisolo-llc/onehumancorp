use super::client::CalComClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalComProvider {
    _client: Arc<CalComClient>,
    metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(access_token: String) -> Self {
        let client = CalComClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
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
