use super::client::{GoogleCalendarClientWrapper, RealGoogleCalendarClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleCalendarProvider {
    client: Arc<dyn GoogleCalendarClientWrapper>,
    metadata: ProviderMetadata,
}

impl GoogleCalendarProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealGoogleCalendarClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_calendar".to_string(),
                name: "Google Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://www.googleapis.com/calendar/v3".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn GoogleCalendarClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_calendar".to_string(),
                name: "Google Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://www.googleapis.com/calendar/v3".to_string(),
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

    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        self.client.get_free_busy(time_min, time_max).await
    }

    pub async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        self.client.create_event(summary, start_time, end_time).await
    }
}
