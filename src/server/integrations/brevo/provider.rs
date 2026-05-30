use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use super::client::{BrevoClientWrapper, RealBrevoClient};
use std::sync::Arc;

pub struct BrevoProvider {
    client: Arc<dyn BrevoClientWrapper>,
    metadata: ProviderMetadata,
}

impl BrevoProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealBrevoClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "brevo".to_string(),
                name: "Brevo Email Marketing".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.brevo.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn BrevoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "brevo".to_string(),
                name: "Brevo Email Marketing".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.brevo.com".to_string(),
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

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        self.client.send_email(to, subject, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockBrevoClient;

    #[async_trait]
    impl BrevoClientWrapper for MockBrevoClient {
        async fn send_email(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_brevo_provider_metadata() {
        let provider = BrevoProvider::new("dummy_key".to_string());
        assert_eq!(provider.metadata.id, "brevo");
    }

    #[tokio::test]
    async fn test_brevo_send_email() {
        let provider = BrevoProvider::with_client(Arc::new(MockBrevoClient));
        assert!(provider.send_email("test@example.com", "Test", "Content").await.is_ok());
    }
}
