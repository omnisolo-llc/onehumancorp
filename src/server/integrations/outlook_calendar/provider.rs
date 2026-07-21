use super::client::OutlookCalendarClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct OutlookCalendarProvider {
    client: Arc<OutlookCalendarClient>,
    metadata: ProviderMetadata,
}

impl OutlookCalendarProvider {
    pub fn new(access_token: String) -> Self {
        let client = OutlookCalendarClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "outlook_calendar".to_string(),
                name: "Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com/v1.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<OutlookCalendarClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "outlook_calendar".to_string(),
                name: "Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com/v1.0".to_string(),
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

    pub async fn get_events(
        &self,
        calendar_id: Option<&str>,
        start: &str,
        end: &str,
    ) -> Result<Vec<super::client::OutlookEvent>, String> {
        self.client.get_events(calendar_id, start, end).await
    }

    pub async fn create_event(
        &self,
        subject: &str,
        body: &str,
        start: &str,
        end: &str,
        attendees: &[String],
        is_online_meeting: bool,
    ) -> Result<super::client::OutlookEvent, String> {
        self.client
            .create_event(subject, body, start, end, attendees, is_online_meeting)
            .await
    }

    pub async fn update_event(
        &self,
        event_id: &str,
        subject: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<super::client::OutlookEvent, String> {
        self.client.update_event(event_id, subject, start, end).await
    }

    pub async fn delete_event(&self, event_id: &str) -> Result<(), String> {
        self.client.delete_event(event_id).await
    }

    pub async fn get_free_busy(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Vec<super::client::FreeBusySlot>, String> {
        self.client.get_free_busy(start, end).await
    }

    pub async fn get_calendars(&self) -> Result<Vec<super::client::OutlookCalendar>, String> {
        self.client.get_calendars().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outlook_calendar_provider_new() {
        let provider = OutlookCalendarProvider::new("ms_token".to_string());
        assert_eq!(provider.metadata.id, "outlook_calendar");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[test]
    fn test_outlook_calendar_provider_to_integration_provider() {
        let provider = OutlookCalendarProvider::new("ms_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "outlook_calendar");
    }

    #[test]
    fn test_outlook_calendar_provider_with_client() {
        let client = Arc::new(OutlookCalendarClient::new("ms_token".to_string()));
        let provider = OutlookCalendarProvider::with_client(client);
        assert_eq!(provider.metadata.id, "outlook_calendar");
    }
}
