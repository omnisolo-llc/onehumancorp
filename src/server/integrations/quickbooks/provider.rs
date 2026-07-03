use super::client::{QuickBooksClient, QBOInvoice};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct QuickBooksProvider {
    pub client: Arc<RwLock<QuickBooksClient>>,
    metadata: ProviderMetadata,
}

impl QuickBooksProvider {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        let client = QuickBooksClient::new(access_token, refresh_token);

        Self {
            client: Arc::new(RwLock::new(client)),
            metadata: ProviderMetadata {
                id: "quickbooks".to_string(),
                name: "QuickBooks Online".to_string(),
                category: "accounting".to_string(),
                base_url: "https://sandbox-quickbooks.api.intuit.com/v3".to_string(),
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

    pub async fn sync_invoice(&self, company_id: &str, invoice: QBOInvoice) -> Result<QBOInvoice, String> {
        let mut client = self.client.write().await;
        client.create_invoice(company_id, invoice).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickbooks_provider_new() {
        let provider = QuickBooksProvider::new("test_token".to_string(), "refresh".to_string());
        assert_eq!(provider.metadata.id, "quickbooks");
        assert_eq!(provider.metadata.category, "accounting");
    }

    #[test]
    fn test_quickbooks_provider_into() {
        let provider = QuickBooksProvider::new("test_token".to_string(), "refresh".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "quickbooks");
    }
}
