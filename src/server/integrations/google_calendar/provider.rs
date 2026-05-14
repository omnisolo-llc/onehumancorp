use super::client::{GoogleCalendarClientWrapper, RealGoogleCalendarClient, CalendarEvent};
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
                base_url: "https://www.googleapis.com/calendar".to_string(),
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
                base_url: "https://www.googleapis.com/calendar".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_event(&self, calendar_id: &str, event: CalendarEvent) -> Result<CalendarEvent, String> {
        self.client.create_event(calendar_id, event).await
    }

    pub async fn list_events(&self, calendar_id: &str) -> Result<Vec<CalendarEvent>, String> {
        self.client.list_events(calendar_id).await
    }
}
