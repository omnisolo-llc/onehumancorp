pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct MailerliteProvider {}

    impl MailerliteProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "mailerlite".to_string(),
                    name: "MailerLite".to_string(),
                    category: "Email Marketing".to_string(),
                    base_url: "https://mailerlite.com".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://app.mailerlite.com/oauth/authorize?client_id=MOCK".to_string())
        }

        pub async fn draft_campaign(&self, _content: &str) -> Result<String, String> {
            Ok("draft_123".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_mailerlite_provider_metadata() {
        let provider = MailerliteProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "mailerlite");
        assert_eq!(provider.metadata.name, "MailerLite");
        assert_eq!(provider.metadata.category, "Email Marketing");
        assert_eq!(provider.metadata.base_url, "https://mailerlite.com");
    }

    #[tokio::test]
    async fn test_mailerlite_auth_flow() {
        let provider = MailerliteProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("oauth/authorize"));
    }

    #[tokio::test]
    async fn test_mailerlite_draft() {
        let provider = MailerliteProvider::new();
        let draft = provider.draft_campaign("dummy").await.unwrap();
        assert_eq!(draft, "draft_123");
    }
}
