use super::models::Message;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn send_message(&self, message: &Message) -> Result<(), String>;
}

pub struct LocalApiAdapter {
    // Basic mock adapter for testing
    pub sent_messages: std::sync::Arc<tokio::sync::Mutex<Vec<Message>>>,
}

impl LocalApiAdapter {
    pub fn new() -> Self {
        Self {
            sent_messages: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl ChannelAdapter for LocalApiAdapter {
    async fn send_message(&self, message: &Message) -> Result<(), String> {
        let mut messages = self.sent_messages.lock().await;
        messages.push(message.clone());
        Ok(())
    }
}
