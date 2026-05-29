use super::client::CalComClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalComProvider {
    _client: Arc<CalComClient>,
    metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(access_token: String) -> Self {
        let client = CalComClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id,
                name: self.metadata.name,
                category: self.metadata.category,
                base_url: self.metadata.base_url,
            }
        }
    }
}

impl CalComProvider {
    pub async fn get_booking_link(&self, event_type: &str) -> Result<String, String> {
        self._client.get_booking_link(event_type).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_com_provider_new() {
        let provider = CalComProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "cal_com");
    }

    #[test]
    fn test_cal_com_provider_into() {
        let provider = CalComProvider::new("test_token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "cal_com");
    }

    #[tokio::test]
    async fn test_cal_com_provider_get_booking_link() {
        let provider = CalComProvider::new("test_token".to_string());
        let result = provider.get_booking_link("test").await;
        assert!(result.is_ok());
    }
}
