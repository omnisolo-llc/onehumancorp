use super::client::HubSpotClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct HubSpotProvider {
    client: Arc<HubSpotClient>,
    metadata: ProviderMetadata,
}

impl HubSpotProvider {
    pub fn new(access_token: String) -> Self {
        let client = HubSpotClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "hubspot".to_string(),
                name: "HubSpot".to_string(),
                category: "crm".to_string(),
                base_url: "https://api.hubapi.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<HubSpotClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "hubspot".to_string(),
                name: "HubSpot".to_string(),
                category: "crm".to_string(),
                base_url: "https://api.hubapi.com".to_string(),
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
        limit: u32,
        after: Option<&str>,
    ) -> Result<(Vec<super::client::HubSpotContact>, Option<String>), String> {
        self.client.get_contacts(limit, after).await
    }

    pub async fn create_contact(
        &self,
        email: &str,
        first_name: &str,
        last_name: &str,
        company: &str,
        phone: &str,
    ) -> Result<super::client::HubSpotContact, String> {
        self.client
            .create_contact(email, first_name, last_name, company, phone)
            .await
    }

    pub async fn update_contact(
        &self,
        contact_id: &str,
        properties: &serde_json::Value,
    ) -> Result<(), String> {
        self.client.update_contact(contact_id, properties).await
    }

    pub async fn get_deals(
        &self,
        limit: u32,
        stage: Option<&str>,
    ) -> Result<Vec<super::client::HubSpotDeal>, String> {
        self.client.get_deals(limit, stage).await
    }

    pub async fn create_deal(
        &self,
        name: &str,
        stage: &str,
        amount: Option<f64>,
        close_date: Option<&str>,
    ) -> Result<super::client::HubSpotDeal, String> {
        self.client.create_deal(name, stage, amount, close_date).await
    }

    pub async fn update_deal(
        &self,
        deal_id: &str,
        properties: &serde_json::Value,
    ) -> Result<(), String> {
        self.client.update_deal(deal_id, properties).await
    }

    pub async fn get_companies(
        &self,
        limit: u32,
    ) -> Result<Vec<super::client::HubSpotCompany>, String> {
        self.client.get_companies(limit).await
    }

    pub async fn create_company(
        &self,
        name: &str,
        domain: &str,
    ) -> Result<super::client::HubSpotCompany, String> {
        self.client.create_company(name, domain).await
    }

    pub async fn search_contacts(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<super::client::HubSpotContact>, String> {
        self.client.search_contacts(query, limit).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hubspot_provider_new() {
        let provider = HubSpotProvider::new("test-token".to_string());
        assert_eq!(provider.metadata.id, "hubspot");
        assert_eq!(provider.metadata.category, "crm");
    }

    #[test]
    fn test_hubspot_provider_to_integration_provider() {
        let provider = HubSpotProvider::new("test-token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "hubspot");
    }

    #[test]
    fn test_hubspot_provider_with_client() {
        let client = Arc::new(HubSpotClient::new("test-token".to_string()));
        let provider = HubSpotProvider::with_client(client);
        assert_eq!(provider.metadata.id, "hubspot");
    }
}
