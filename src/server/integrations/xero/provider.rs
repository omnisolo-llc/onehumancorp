use super::client::XeroClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct XeroProvider {
    client: Arc<XeroClient>,
    metadata: ProviderMetadata,
}

impl XeroProvider {
    pub fn new(access_token: String, tenant_id: String) -> Self {
        let client = XeroClient::new(access_token, tenant_id);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "xero".to_string(),
                name: "Xero".to_string(),
                category: "accounting".to_string(),
                base_url: "https://api.xero.com/api.xro/2.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<XeroClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "xero".to_string(),
                name: "Xero".to_string(),
                category: "accounting".to_string(),
                base_url: "https://api.xero.com/api.xro/2.0".to_string(),
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
            },
        }
    }

    pub async fn get_contacts(&self) -> Result<Vec<super::client::XeroContact>, String> {
        self.client.get_contacts().await
    }

    pub async fn create_contact(
        &self,
        name: &str,
        email: &str,
    ) -> Result<super::client::XeroContact, String> {
        self.client.create_contact(name, email).await
    }

    pub async fn get_invoices(&self) -> Result<Vec<super::client::XeroInvoice>, String> {
        self.client.get_invoices().await
    }

    pub async fn create_invoice(
        &self,
        contact_id: &str,
        line_items: &[super::client::XeroLineItem],
    ) -> Result<super::client::XeroInvoice, String> {
        self.client.create_invoice(contact_id, line_items).await
    }

    pub async fn get_accounts(&self) -> Result<Vec<super::client::XeroAccount>, String> {
        self.client.get_accounts().await
    }

    pub async fn get_contacts_modified_since(
        &self,
        since: &str,
    ) -> Result<Vec<super::client::XeroContact>, String> {
        self.client.get_contacts_modified_since(since).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xero_provider_new() {
        let provider = XeroProvider::new("test-token".to_string(), "tenant-123".to_string());
        assert_eq!(provider.metadata.id, "xero");
        assert_eq!(provider.metadata.category, "accounting");
    }

    #[test]
    fn test_xero_provider_to_integration_provider() {
        let provider = XeroProvider::new("test-token".to_string(), "tenant-123".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "xero");
    }

    #[test]
    fn test_xero_provider_with_client() {
        let client = Arc::new(XeroClient::new(
            "test-token".to_string(),
            "tenant-123".to_string(),
        ));
        let provider = XeroProvider::with_client(client);
        assert_eq!(provider.metadata.id, "xero");
    }
}
