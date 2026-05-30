use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use super::client::{OutlookClientWrapper, RealOutlookClient};
use std::sync::Arc;

pub struct OutlookProvider {
    client: Arc<dyn OutlookClientWrapper>,
    metadata: ProviderMetadata,
}

impl OutlookProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealOutlookClient::new(access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "outlook".to_string(),
                name: "Microsoft Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn OutlookClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "outlook".to_string(),
                name: "Microsoft Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com".to_string(),
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

    pub async fn sync_calendar(&self) -> Result<(), String> {
        self.client.sync_calendar().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockOutlookClient;

    #[async_trait]
    impl OutlookClientWrapper for MockOutlookClient {
        async fn sync_calendar(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_outlook_provider_metadata() {
        let provider = OutlookProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "outlook");
    }

    #[tokio::test]
    async fn test_outlook_sync() {
        let provider = OutlookProvider::with_client(Arc::new(MockOutlookClient));
        assert!(provider.sync_calendar().await.is_ok());
    }
}
