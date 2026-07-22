use super::client::QuickBooksClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct QuickBooksProvider {
    client: Arc<QuickBooksClient>,
    metadata: ProviderMetadata,
}

impl QuickBooksProvider {
    pub fn new(access_token: String, realm_id: String) -> Self {
        let client = QuickBooksClient::new(access_token, realm_id.clone());
        let base_url = format!(
            "https://quickbooks.api.intuit.com/v3/company/{}",
            realm_id
        );

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "quickbooks".to_string(),
                name: "QuickBooks".to_string(),
                category: "accounting".to_string(),
                base_url,
            },
        }
    }

    pub fn with_client(client: Arc<QuickBooksClient>, base_url: String) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "quickbooks".to_string(),
                name: "QuickBooks".to_string(),
                category: "accounting".to_string(),
                base_url,
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

    pub async fn get_customers(
        &self,
        max_results: u32,
    ) -> Result<Vec<super::client::QuickBooksCustomer>, String> {
        self.client.get_customers(max_results).await
    }

    pub async fn create_customer(
        &self,
        name: &str,
        email: &str,
        phone: &str,
        company: &str,
    ) -> Result<super::client::QuickBooksCustomer, String> {
        self.client
            .create_customer(name, email, phone, company)
            .await
    }

    pub async fn get_invoices(
        &self,
        max_results: u32,
    ) -> Result<Vec<super::client::QuickBooksInvoice>, String> {
        self.client.get_invoices(max_results).await
    }

    pub async fn create_invoice(
        &self,
        customer_id: &str,
        line_items: &[super::client::QuickBooksLineItem],
    ) -> Result<super::client::QuickBooksInvoice, String> {
        self.client.create_invoice(customer_id, line_items).await
    }

    pub async fn get_products(
        &self,
        max_results: u32,
    ) -> Result<Vec<super::client::QuickBooksProduct>, String> {
        self.client.get_products(max_results).await
    }

    pub async fn get_accounts(&self) -> Result<Vec<super::client::QuickBooksAccount>, String> {
        self.client.get_accounts().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quickbooks_provider_new() {
        let provider =
            QuickBooksProvider::new("test-token".to_string(), "realm-123".to_string());
        assert_eq!(provider.metadata.id, "quickbooks");
        assert_eq!(provider.metadata.category, "accounting");
    }

    #[test]
    fn test_quickbooks_provider_to_integration_provider() {
        let provider =
            QuickBooksProvider::new("test-token".to_string(), "realm-123".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "quickbooks");
    }

    #[test]
    fn test_quickbooks_provider_with_client() {
        let client = Arc::new(QuickBooksClient::new(
            "test-token".to_string(),
            "realm-123".to_string(),
        ));
        let provider = QuickBooksProvider::with_client(
            client,
            "https://quickbooks.api.intuit.com/v3/company/realm-123".to_string(),
        );
        assert_eq!(provider.metadata.id, "quickbooks");
    }
}
