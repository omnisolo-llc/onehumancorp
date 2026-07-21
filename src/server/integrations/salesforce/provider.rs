use super::client::SalesforceClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct SalesforceProvider {
    client: Arc<SalesforceClient>,
    metadata: ProviderMetadata,
}

impl SalesforceProvider {
    pub fn new(instance_url: String, access_token: String) -> Self {
        let client = SalesforceClient::new(instance_url.clone(), access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "salesforce".to_string(),
                name: "Salesforce".to_string(),
                category: "crm".to_string(),
                base_url: instance_url,
            },
        }
    }

    pub fn with_client(client: Arc<SalesforceClient>, instance_url: String) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "salesforce".to_string(),
                name: "Salesforce".to_string(),
                category: "crm".to_string(),
                base_url: instance_url,
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

    pub async fn get_contacts(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<super::client::SalesforceRecord>, String> {
        self.client.get_contacts(query, limit).await
    }

    pub async fn create_contact(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        phone: &str,
        company: &str,
    ) -> Result<super::client::SalesforceRecord, String> {
        self.client
            .create_contact(first_name, last_name, email, phone, company)
            .await
    }

    pub async fn update_contact(
        &self,
        id: &str,
        fields: &serde_json::Value,
    ) -> Result<(), String> {
        self.client.update_contact(id, fields).await
    }

    pub async fn get_opportunities(
        &self,
        stage: Option<&str>,
        limit: u32,
    ) -> Result<Vec<super::client::SalesforceRecord>, String> {
        self.client.get_opportunities(stage, limit).await
    }

    pub async fn create_opportunity(
        &self,
        name: &str,
        account_id: &str,
        amount: f64,
        stage: &str,
        close_date: &str,
    ) -> Result<super::client::SalesforceRecord, String> {
        self.client
            .create_opportunity(name, account_id, amount, stage, close_date)
            .await
    }

    pub async fn get_accounts(
        &self,
        query: Option<&str>,
        limit: u32,
    ) -> Result<Vec<super::client::SalesforceRecord>, String> {
        self.client.get_accounts(query, limit).await
    }

    pub async fn search(
        &self,
        search_term: &str,
    ) -> Result<Vec<super::client::SalesforceRecord>, String> {
        self.client.search(search_term).await
    }

    pub async fn describe_object(
        &self,
        object_name: &str,
    ) -> Result<serde_json::Value, String> {
        self.client.describe_object(object_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salesforce_provider_new() {
        let provider = SalesforceProvider::new(
            "https://yourorg.salesforce.com".to_string(),
            "test-token".to_string(),
        );
        assert_eq!(provider.metadata.id, "salesforce");
        assert_eq!(provider.metadata.category, "crm");
    }

    #[test]
    fn test_salesforce_provider_to_integration_provider() {
        let provider = SalesforceProvider::new(
            "https://yourorg.salesforce.com".to_string(),
            "test-token".to_string(),
        );
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "salesforce");
    }

    #[test]
    fn test_salesforce_provider_with_client() {
        let client = Arc::new(SalesforceClient::new(
            "https://yourorg.salesforce.com".to_string(),
            "test-token".to_string(),
        ));
        let provider = SalesforceProvider::with_client(
            client,
            "https://yourorg.salesforce.com".to_string(),
        );
        assert_eq!(provider.metadata.id, "salesforce");
    }
}
