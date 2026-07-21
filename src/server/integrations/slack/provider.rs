use super::client::SlackClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct SlackProvider {
    client: Arc<SlackClient>,
    metadata: ProviderMetadata,
}

impl SlackProvider {
    pub fn new(bot_token: String) -> Self {
        let client = SlackClient::new(bot_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "slack".to_string(),
                name: "Slack".to_string(),
                category: "messaging".to_string(),
                base_url: "https://slack.com/api".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<SlackClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "slack".to_string(),
                name: "Slack".to_string(),
                category: "messaging".to_string(),
                base_url: "https://slack.com/api".to_string(),
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

    pub async fn send_message(&self, channel: &str, text: &str) -> Result<(), String> {
        self.client.send_message(channel, text).await?;
        Ok(())
    }

    pub async fn send_block_message(&self, channel: &str, blocks: &[serde_json::Value]) -> Result<(), String> {
        self.client.send_block_message(channel, blocks).await?;
        Ok(())
    }

    pub async fn upload_file(&self, channel: &str, filename: &str, content: &[u8]) -> Result<(), String> {
        self.client.upload_file(channel, filename, content).await
    }

    pub async fn list_channels(&self) -> Result<Vec<super::client::SlackChannel>, String> {
        self.client.list_channels().await
    }

    pub async fn get_channel_history(&self, channel_id: &str, limit: u32) -> Result<Vec<super::client::SlackMessage>, String> {
        self.client.get_channel_history(channel_id, limit).await
    }

    pub async fn create_channel(&self, name: &str, is_private: bool) -> Result<super::client::SlackChannel, String> {
        self.client.create_channel(name, is_private).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slack_provider_new() {
        let provider = SlackProvider::new("xoxb-test-token".to_string());
        assert_eq!(provider.metadata.id, "slack");
        assert_eq!(provider.metadata.category, "messaging");
    }

    #[test]
    fn test_slack_provider_to_integration_provider() {
        let provider = SlackProvider::new("xoxb-test-token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "slack");
    }

    #[test]
    fn test_slack_provider_with_client() {
        let client = Arc::new(SlackClient::new("xoxb-test".to_string()));
        let provider = SlackProvider::with_client(client);
        assert_eq!(provider.metadata.id, "slack");
    }
}
