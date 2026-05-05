pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct ManychatProvider {}

    impl ManychatProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "manychat".to_string(),
                    name: "Manychat".to_string(),
                    category: "Social Media".to_string(),
                    base_url: "https://manychat.com".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://manychat.com/oauth/authorize?client_id=MOCK_CLIENT_ID".to_string())
        }

        pub async fn sync_messages(&self, _inbox_id: &str) -> Result<Vec<String>, String> {
            Ok(vec!["Mock message 1".to_string(), "Mock message 2".to_string()])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_manychat_provider_metadata() {
        let provider = ManychatProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.name, "Manychat");
        assert_eq!(provider.metadata.category, "Social Media");
        assert_eq!(provider.metadata.base_url, "https://manychat.com");
    }

    #[tokio::test]
    async fn test_manychat_auth_flow() {
        let provider = ManychatProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("oauth/authorize"));
    }

    #[tokio::test]
    async fn test_manychat_sync_messages() {
        let provider = ManychatProvider::new();
        let msgs = provider.sync_messages("dummy").await.unwrap();
        assert_eq!(msgs.len(), 2);
    }
}
