use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChannelAdapter {
    async fn send_message(&self, recipient_id: &str, content: &str) -> Result<(), String>;
    async fn receive_webhook(&self, payload: Value) -> Result<(), String>;
}
