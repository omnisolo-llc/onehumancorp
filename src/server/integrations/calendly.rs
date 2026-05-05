pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct CalendlyProvider {}

    impl CalendlyProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "calendly".to_string(),
                    name: "Calendly".to_string(),
                    category: "Calendar & Scheduling".to_string(),
                    base_url: "https://calendly.com".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://auth.calendly.com/oauth/authorize?client_id=MOCK".to_string())
        }

        pub async fn get_available_slots(&self, _date: &str) -> Result<Vec<String>, String> {
            Ok(vec!["10:00 AM".to_string(), "02:00 PM".to_string()])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_calendly_provider_metadata() {
        let provider = CalendlyProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "calendly");
        assert_eq!(provider.metadata.name, "Calendly");
        assert_eq!(provider.metadata.category, "Calendar & Scheduling");
        assert_eq!(provider.metadata.base_url, "https://calendly.com");
    }

    #[tokio::test]
    async fn test_calendly_auth_flow() {
        let provider = CalendlyProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("oauth/authorize"));
    }

    #[tokio::test]
    async fn test_calendly_slots() {
        let provider = CalendlyProvider::new();
        let slots = provider.get_available_slots("dummy").await.unwrap();
        assert_eq!(slots.len(), 2);
    }
}
