use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait ManyChatClientWrapper: Send + Sync {
    async fn send_message(&self, recipient_id: &str, message_text: &str) -> Result<(), String>;
}

pub struct RealManyChatClient {
    pub api_key: String,
}

impl RealManyChatClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl ManyChatClientWrapper for RealManyChatClient {
    async fn send_message(&self, recipient_id: &str, message_text: &str) -> Result<(), String> {
        // Mock ManyChat message send
        tracing::info!("Sending message to ManyChat recipient {}: {}", recipient_id, message_text);
        Ok(())
    }
}
