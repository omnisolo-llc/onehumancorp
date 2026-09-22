use super::client::{GoogleCalendarClientWrapper, RealGoogleCalendarClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleCalendarProvider {
    pub client: Arc<dyn GoogleCalendarClientWrapper>,
    metadata: ProviderMetadata,
}

impl GoogleCalendarProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealGoogleCalendarClient::new(access_token, None, None, None);

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

    pub fn new_with_auth(
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        token_storage: Option<Arc<dyn super::client::TokenStorage>>,
    ) -> Self {
        let client =
            RealGoogleCalendarClient::new(access_token, refresh_token, expires_at, token_storage);

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
            },
        }
    }

    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        self.client.get_free_busy(time_min, time_max).await
    }

    pub async fn create_event(
        &self,
        summary: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<String, String> {
        self.client
            .create_event(summary, start_time, end_time)
            .await
    }

    pub async fn cancel_event(&self, event_id: &str) -> Result<(), String> {
        self.client.cancel_event(event_id).await
    }

    pub async fn list_events_sync(
        &self,
        sync_token: Option<&str>,
    ) -> Result<super::client::SyncEventsResponse, String> {
        self.client.list_events_sync(sync_token).await
    }
}
