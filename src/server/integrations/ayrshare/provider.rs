use super::client::{AyrshareClientWrapper, RealAyrshareClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use serde_json::Value;

pub struct AyrshareProvider {
    client: Arc<dyn AyrshareClientWrapper>,
    metadata: ProviderMetadata,
}

impl AyrshareProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealAyrshareClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social Media".to_string(),
                category: "social_media".to_string(),
                base_url: "https://app.ayrshare.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn AyrshareClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social Media".to_string(),
                category: "social_media".to_string(),
                base_url: "https://app.ayrshare.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<Value, String> {
        self.client.post_message(post, platforms).await
    }

    pub async fn get_messages(&self) -> Result<Value, String> {
        self.client.get_messages().await
    }

    pub async fn reply_message(&self, message_id: &str, reply: &str) -> Result<Value, String> {
        self.client.reply_message(message_id, reply).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use serde_json::json;

    struct MockAyrshareClient {
        posts: Arc<AtomicUsize>,
        fetches: Arc<AtomicUsize>,
        replies: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl AyrshareClientWrapper for MockAyrshareClient {
        async fn post_message(&self, _post: &str, _platforms: Vec<&str>) -> Result<Value, String> {
            self.posts.fetch_add(1, Ordering::SeqCst);
            Ok(json!({"status": "success"}))
        }

        async fn get_messages(&self) -> Result<Value, String> {
            self.fetches.fetch_add(1, Ordering::SeqCst);
            Ok(json!([{"id": "msg1", "text": "hello"}]))
        }

        async fn reply_message(&self, _message_id: &str, _reply: &str) -> Result<Value, String> {
            self.replies.fetch_add(1, Ordering::SeqCst);
            Ok(json!({"status": "success"}))
        }
    }

    #[tokio::test]
    async fn test_ayrshare_provider_integration() {
        let posts = Arc::new(AtomicUsize::new(0));
        let fetches = Arc::new(AtomicUsize::new(0));
        let replies = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockAyrshareClient {
            posts: posts.clone(),
            fetches: fetches.clone(),
            replies: replies.clone(),
        });
        let provider = AyrshareProvider::with_client(mock);

        provider.post_message("Test post", vec!["twitter"]).await.unwrap();
        assert_eq!(posts.load(Ordering::SeqCst), 1);

        provider.get_messages().await.unwrap();
        assert_eq!(fetches.load(Ordering::SeqCst), 1);

        provider.reply_message("msg1", "reply text").await.unwrap();
        assert_eq!(replies.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_ayrshare_provider_new() {
        let provider = AyrshareProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "ayrshare");
        assert_eq!(provider.metadata.category, "social_media");
    }

    #[test]
    fn test_ayrshare_provider_into() {
        let provider = AyrshareProvider::new("api_key".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "ayrshare");
    }
}
