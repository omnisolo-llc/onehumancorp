use super::client::ZendeskClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ZendeskProvider {
    _client: Arc<ZendeskClient>,
    metadata: ProviderMetadata,
}

impl ZendeskProvider {
    pub fn new(subdomain: String, email: String, api_token: String) -> Self {
        let client = ZendeskClient::new(subdomain, email, api_token);
        let base_url = client.base_url.clone();

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "zendesk".to_string(),
                name: "Zendesk Support".to_string(),
                category: "support".to_string(),
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

    pub async fn create_ticket(
        &self,
        subject: &str,
        description: &str,
        priority: &str,
        requester_email: &str,
    ) -> Result<super::client::ZendeskTicket, String> {
        self._client
            .create_ticket(subject, description, priority, requester_email)
            .await
    }

    pub async fn get_ticket(
        &self,
        ticket_id: u64,
    ) -> Result<super::client::ZendeskTicket, String> {
        self._client.get_ticket(ticket_id).await
    }

    pub async fn update_ticket(
        &self,
        ticket_id: u64,
        comment: Option<&str>,
        status: Option<&str>,
        priority: Option<&str>,
    ) -> Result<super::client::ZendeskTicket, String> {
        self._client
            .update_ticket(ticket_id, comment, status, priority)
            .await
    }

    pub async fn list_tickets(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<super::client::ZendeskTicket>, String> {
        self._client.list_tickets(status, limit).await
    }

    pub async fn search_tickets(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<super::client::ZendeskTicket>, String> {
        self._client.search_tickets(query, limit).await
    }

    pub async fn get_ticket_comments(
        &self,
        ticket_id: u64,
    ) -> Result<Vec<super::client::ZendeskComment>, String> {
        self._client.get_ticket_comments(ticket_id).await
    }

    pub async fn add_comment(
        &self,
        ticket_id: u64,
        body: &str,
        author_id: u64,
    ) -> Result<(), String> {
        self._client.add_comment(ticket_id, body, author_id).await
    }

    pub async fn create_user(
        &self,
        name: &str,
        email: &str,
        role: &str,
    ) -> Result<super::client::ZendeskUser, String> {
        self._client.create_user(name, email, role).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zendesk_provider_metadata() {
        let provider =
            ZendeskProvider::new("mycompany".into(), "admin@my.com".into(), "token123".into());
        assert_eq!(provider.to_integration_provider().metadata.id, "zendesk");
        assert_eq!(
            provider.to_integration_provider().metadata.category,
            "support"
        );
    }

    #[test]
    fn test_zendesk_provider_base_url() {
        let provider =
            ZendeskProvider::new("mycompany".into(), "admin@my.com".into(), "token123".into());
        assert_eq!(
            provider.to_integration_provider().metadata.base_url,
            "https://mycompany.zendesk.com"
        );
    }

    #[test]
    fn test_zendesk_provider_into() {
        let provider =
            ZendeskProvider::new("acme".into(), "user@acme.com".into(), "abc".into());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "zendesk");
        assert_eq!(integration.metadata.name, "Zendesk Support");
    }
}
