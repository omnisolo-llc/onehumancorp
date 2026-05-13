use super::client::{CalComClientWrapper, RealCalComClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalComProvider {
    client: Arc<dyn CalComClientWrapper>,
    metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = RealCalComClient::new(api_key, base_url);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Booking".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalComClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Booking".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn generate_booking_link(&self, user_email: &str, event_type: &str) -> Result<String, String> {
        self.client.generate_booking_link(user_email, event_type).await
    }
}
