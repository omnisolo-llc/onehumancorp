use super::client::{GoogleBusinessClientWrapper, RealGoogleBusinessClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleBusinessProvider {
    client: Arc<dyn GoogleBusinessClientWrapper>,
    metadata: ProviderMetadata,
}

impl GoogleBusinessProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealGoogleBusinessClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_business".to_string(),
                name: "Google Business".to_string(),
                category: "seo".to_string(),
                base_url: "https://mybusiness.googleapis.com/v4".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn GoogleBusinessClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_business".to_string(),
                name: "Google Business".to_string(),
                category: "seo".to_string(),
                base_url: "https://mybusiness.googleapis.com/v4".to_string(),
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

    pub async fn sync_menu(&self, menu_data: &str) -> Result<String, String> {
        self.client.sync_menu(menu_data).await
    }

    pub async fn sync_hours(&self, hours_data: &str) -> Result<String, String> {
        self.client.sync_hours(hours_data).await
    }
}
