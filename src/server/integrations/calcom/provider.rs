use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::CalComClient;
use std::sync::Arc;

pub struct CalComProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<CalComClient>>,
}

impl CalComProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
            client: None,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(CalComClient::new(api_key)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_booking_link(&self, event_type_id: &str, duration_mins: i32) -> Result<String, String> {
        if let Some(client) = &self.client {
            client.create_booking_link(event_type_id, duration_mins).await
        } else {
            Err("Cal.com client not initialized".to_string())
        }
    }

    pub async fn get_availability(&self, date_from: &str, date_to: &str) -> Result<Vec<String>, String> {
        if let Some(client) = &self.client {
            client.get_availability(date_from, date_to).await
        } else {
            Err("Cal.com client not initialized".to_string())
        }
    }
}
