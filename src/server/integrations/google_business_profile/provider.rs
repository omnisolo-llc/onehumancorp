use super::client::{GoogleBusinessProfileClientWrapper, RealGoogleBusinessProfileClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleBusinessProfileProvider {
    client: Arc<dyn GoogleBusinessProfileClientWrapper>,
    metadata: ProviderMetadata,
}

impl GoogleBusinessProfileProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealGoogleBusinessProfileClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_business_profile".to_string(),
                name: "Google Business Profile".to_string(),
                category: "marketing".to_string(),
                base_url: "https://mybusiness.googleapis.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn GoogleBusinessProfileClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_business_profile".to_string(),
                name: "Google Business Profile".to_string(),
                category: "marketing".to_string(),
                base_url: "https://mybusiness.googleapis.com".to_string(),
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

    pub async fn update_hours(&self, location_id: &str, hours: &serde_json::Value) -> Result<String, String> {
        self.client.update_hours(location_id, hours).await
    }

    pub async fn fetch_reviews(&self, location_id: &str) -> Result<String, String> {
        self.client.fetch_reviews(location_id).await
    }

    pub async fn reply_to_review(&self, location_id: &str, review_id: &str, reply: &str) -> Result<String, String> {
        self.client.reply_to_review(location_id, review_id, reply).await
    }
}
