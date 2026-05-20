use super::client::{MailchimpClientWrapper, RealMailchimpClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MailchimpProvider {
    client: Arc<dyn MailchimpClientWrapper>,
    metadata: ProviderMetadata,
}

impl MailchimpProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealMailchimpClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp".to_string(),
                category: "email".to_string(),
                base_url: "https://api.mailchimp.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MailchimpClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp".to_string(),
                category: "email".to_string(),
                base_url: "https://api.mailchimp.com".to_string(),
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

    pub async fn add_customer(&self, list_id: &str, email: &str, tag: &str) -> Result<(), String> {
        self.client.add_customer(list_id, email, tag).await
    }

    pub async fn send_campaign(&self, campaign_id: &str) -> Result<(), String> {
        self.client.send_campaign(campaign_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockMailchimpClient {
        called: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MailchimpClientWrapper for MockMailchimpClient {
        async fn add_customer(&self, _list_id: &str, _email: &str, _tag: &str) -> Result<(), String> {
            self.called.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        async fn send_campaign(&self, _campaign_id: &str) -> Result<(), String> {
            self.called.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mailchimp_provider_integration() {
        let called = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockMailchimpClient { called: called.clone() });
        let provider = MailchimpProvider::with_client(mock);

        provider.add_customer("list", "test@test.com", "tag").await.unwrap();
        provider.send_campaign("camp").await.unwrap();
        assert_eq!(called.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_mailchimp_provider_new() {
        let provider = MailchimpProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "mailchimp");
    }

    #[test]
    fn test_mailchimp_provider_to_integration_provider() {
        let provider = MailchimpProvider::new("key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "mailchimp");
    }
}
