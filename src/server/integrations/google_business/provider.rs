use super::client::{GoogleBusinessClientWrapper, RealGoogleBusinessClient};
use server_integrations_core::{IntegrationProvider, ProviderMetadata};
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
                name: "Google Business Profile".to_string(),
                category: "marketing".to_string(),
                base_url: "https://mybusiness.googleapis.com/v4".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn GoogleBusinessClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_business".to_string(),
                name: "Google Business Profile".to_string(),
                category: "marketing".to_string(),
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

    pub async fn sync_hours(&self, hours: &str) -> Result<String, String> {
        self.client.sync_hours(hours).await
    }

    pub async fn sync_catalog(&self, catalog: &str) -> Result<String, String> {
        self.client.sync_catalog(catalog).await
    }

    pub async fn submit_review_reply(&self, review_id: &str, reply: &str) -> Result<String, String> {
        self.client.submit_review_reply(review_id, reply).await
    }
}
