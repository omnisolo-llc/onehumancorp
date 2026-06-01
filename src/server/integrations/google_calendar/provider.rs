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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockGoogleCalendarClient;

    #[async_trait]
    impl GoogleCalendarClientWrapper for MockGoogleCalendarClient {
        async fn get_free_busy(&self, _time_min: &str, _time_max: &str) -> Result<String, String> {
            Ok("{\"freeBusy\": true}".to_string())
        }

        async fn create_event(&self, _summary: &str, _start_time: &str, _end_time: &str) -> Result<String, String> {
            Ok("mock_event_id".to_string())
        }
    }

    #[test]
    fn test_google_calendar_provider_new() {
        let provider = GoogleCalendarProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "google_calendar");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[test]
    fn test_google_calendar_provider_with_client() {
        let mock_client = Arc::new(MockGoogleCalendarClient);
        let provider = GoogleCalendarProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "google_calendar");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[test]
    fn test_google_calendar_provider_to_integration_provider() {
        let provider = GoogleCalendarProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "google_calendar");
    }

    #[tokio::test]
    async fn test_google_calendar_provider_get_free_busy() {
        let mock_client = Arc::new(MockGoogleCalendarClient);
        let provider = GoogleCalendarProvider::with_client(mock_client);
        let result = provider.get_free_busy("2024-01-01T00:00:00Z", "2024-01-02T00:00:00Z").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "{\"freeBusy\": true}");
    }

    #[tokio::test]
    async fn test_google_calendar_provider_create_event() {
        let mock_client = Arc::new(MockGoogleCalendarClient);
        let provider = GoogleCalendarProvider::with_client(mock_client);
        let result = provider.create_event("Meeting", "2024-01-01T10:00:00Z", "2024-01-01T11:00:00Z").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "mock_event_id");
    }
}
