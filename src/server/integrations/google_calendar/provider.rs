use super::client::{GoogleCalendarClientWrapper, RealGoogleCalendarClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleCalendarProvider {
    client: Arc<dyn GoogleCalendarClientWrapper>,
    pub metadata: ProviderMetadata,
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
            },
        }
    }

    pub async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        self.client.create_event(summary, start_time, end_time).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockGoogleCalendarClient {
        created_events: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl GoogleCalendarClientWrapper for MockGoogleCalendarClient {
        async fn create_event(&self, _summary: &str, _start_time: &str, _end_time: &str) -> Result<String, String> {
            self.created_events.fetch_add(1, Ordering::SeqCst);
            Ok("mock_id".to_string())
        }
    }

    #[tokio::test]
    async fn test_google_calendar_provider_integration() {
        let created = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockGoogleCalendarClient { created_events: created.clone() });
        let provider = GoogleCalendarProvider::with_client(mock);

        provider.create_event("Summary", "start", "end").await.unwrap();
        assert_eq!(created.load(Ordering::SeqCst), 1);
    }
}
