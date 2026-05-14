use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{DailyCoClientWrapper, RealDailyCoClient};

pub struct DailyCoProvider {
    pub client: Arc<dyn DailyCoClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl DailyCoProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealDailyCoClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "dailyco".to_string(),
                name: "Daily.co Integration".to_string(),
                category: "video_conferencing".to_string(),
                base_url: "https://api.daily.co/v1".to_string(),
            },
        }
    }
}
