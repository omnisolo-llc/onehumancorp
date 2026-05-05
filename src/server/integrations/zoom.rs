pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct ZoomProvider {}

    impl ZoomProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "zoom".to_string(),
                    name: "Zoom".to_string(),
                    category: "Video Conferencing".to_string(),
                    base_url: "https://zoom.us".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://zoom.us/oauth/authorize?response_type=code&client_id=MOCK_CLIENT_ID&redirect_uri=MOCK_REDIRECT_URI".to_string())
        }

        pub async fn generate_meeting_link(&self, _booking_details: &str) -> Result<String, String> {
            Ok("https://zoom.us/j/1234567890?pwd=MOCK_PASSWORD".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_zoom_provider_metadata() {
        let provider = ZoomProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "zoom");
        assert_eq!(provider.metadata.name, "Zoom");
        assert_eq!(provider.metadata.category, "Video Conferencing");
        assert_eq!(provider.metadata.base_url, "https://zoom.us");
    }

    #[tokio::test]
    async fn test_zoom_auth_flow() {
        let provider = ZoomProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("oauth/authorize"));
    }

    #[tokio::test]
    async fn test_zoom_generate_link() {
        let provider = ZoomProvider::new();
        let link = provider.generate_meeting_link("dummy").await.unwrap();
        assert!(link.contains("zoom.us/j/"));
    }
}
