use serde_json::Value;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn handle_webhook(&self, payload: Value) -> Result<(), String>;
    async fn send_message(&self, contact_identifier: &str, content: &str) -> Result<(), String>;
}

pub struct WebWidgetAdapter;

#[async_trait]
impl ChannelAdapter for WebWidgetAdapter {
    async fn handle_webhook(&self, _payload: Value) -> Result<(), String> {
        // Dummy implementation for web widget
        Ok(())
    }

    async fn send_message(&self, _contact_identifier: &str, _content: &str) -> Result<(), String> {
        // Dummy implementation for web widget
        Ok(())
    }
}
