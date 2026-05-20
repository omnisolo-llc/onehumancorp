use super::client::{CalendlyClientWrapper, RealCalendlyClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalendlyProvider {
    client: Arc<dyn CalendlyClientWrapper>,
    metadata: ProviderMetadata,
}

impl CalendlyProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealCalendlyClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.calendly.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalendlyClientWrapper>) -> Self {
        Self {
            client,
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

    pub async fn get_event_types(&self) -> Result<String, String> {
        self.client.get_event_types().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockCalendlyClient {
        called: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl CalendlyClientWrapper for MockCalendlyClient {
        async fn get_event_types(&self) -> Result<String, String> {
            self.called.fetch_add(1, Ordering::SeqCst);
            Ok("[]".to_string())
        }
    }

    #[tokio::test]
    async fn test_calendly_provider_integration() {
        let called = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockCalendlyClient { called: called.clone() });
        let provider = CalendlyProvider::with_client(mock);

        provider.get_event_types().await.unwrap();
        assert_eq!(called.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_calendly_provider_new() {
        let provider = CalendlyProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "calendly");
    }

    #[test]
    fn test_calendly_provider_to_integration_provider() {
        let provider = CalendlyProvider::new("key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "calendly");
    }
}
