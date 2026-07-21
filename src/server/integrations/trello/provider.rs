use super::client::{TrelloClient, TrelloBoard, TrelloList, TrelloCard, TrelloLabel};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TrelloProvider {
    _client: Arc<TrelloClient>,
    metadata: ProviderMetadata,
}

impl TrelloProvider {
    pub fn new(api_key: String, token: String) -> Self {
        let client = TrelloClient::new(api_key, token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "trello".to_string(),
                name: "Trello".to_string(),
                category: "project-management".to_string(),
                base_url: "https://api.trello.com/1".to_string(),
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

    pub async fn get_boards(&self) -> Result<Vec<TrelloBoard>, String> {
        self._client.get_boards().await
    }

    pub async fn get_lists(&self, board_id: &str) -> Result<Vec<TrelloList>, String> {
        self._client.get_lists(board_id).await
    }

    pub async fn get_cards(&self, list_id: &str) -> Result<Vec<TrelloCard>, String> {
        self._client.get_cards(list_id).await
    }

    pub async fn create_card(
        &self,
        list_id: &str,
        name: &str,
        description: Option<&str>,
        due_date: Option<&str>,
        label_ids: &[String],
    ) -> Result<TrelloCard, String> {
        self._client
            .create_card(list_id, name, description, due_date, label_ids)
            .await
    }

    pub async fn update_card(
        &self,
        card_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<TrelloCard, String> {
        self._client
            .update_card(card_id, name, description, due_date)
            .await
    }

    pub async fn move_card(&self, card_id: &str, list_id: &str) -> Result<TrelloCard, String> {
        self._client.move_card(card_id, list_id).await
    }

    pub async fn delete_card(&self, card_id: &str) -> Result<(), String> {
        self._client.delete_card(card_id).await
    }

    pub async fn get_labels(&self, board_id: &str) -> Result<Vec<TrelloLabel>, String> {
        self._client.get_labels(board_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trello_provider_new() {
        let provider = TrelloProvider::new("key".to_string(), "token".to_string());
        assert_eq!(provider.metadata.id, "trello");
        assert_eq!(provider.metadata.category, "project-management");
        assert_eq!(provider.metadata.name, "Trello");
    }

    #[test]
    fn test_trello_provider_into() {
        let provider = TrelloProvider::new("key".to_string(), "token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "trello");
        assert_eq!(integration.metadata.category, "project-management");
    }
}
