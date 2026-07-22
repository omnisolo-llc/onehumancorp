use super::client::CalendlyClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalendlyProvider {
    _client: Arc<CalendlyClient>,
    metadata: ProviderMetadata,
}

impl CalendlyProvider {
    pub fn new(api_key: String) -> Self {
        let client = CalendlyClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.calendly.com".to_string(),
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

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        self._client.fetch_event_types().await
    }
<<<<<<< HEAD

    pub async fn create_event(
        &self,
        event_type_uri: &str,
        start_time: &str,
        invitee_email: &str,
    ) -> Result<String, String> {
        self._client
            .create_event(event_type_uri, start_time, invitee_email)
            .await
    }

    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        self._client.get_free_busy(time_min, time_max).await
    }
=======
>>>>>>> 97cc191c1 (perf: tokio RwLock, Redis pool, SSE streaming, unified WS, backpressure, React hooks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendly_provider_new() {
        let provider = CalendlyProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "calendly");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[test]
    fn test_calendly_provider_into() {
        let provider = CalendlyProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "calendly");
    }
}
