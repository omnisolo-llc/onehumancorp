use super::client::{DailyCoClientWrapper, RealDailyCoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct DailyCoClientProvider {
    client: Arc<dyn DailyCoClientWrapper>,
    metadata: ProviderMetadata,
}

impl DailyCoClientProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = RealDailyCoClient::new(api_key, base_url);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "dailyco".to_string(),
                name: "Daily.co Video Rooms".to_string(),
                category: "video".to_string(),
                base_url: "https://api.dailyco.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn DailyCoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "dailyco".to_string(),
                name: "Daily.co Video Rooms".to_string(),
                category: "video".to_string(),
                base_url: "https://api.dailyco.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
