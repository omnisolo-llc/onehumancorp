use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChannelAdapter {
    async fn send_message(&self, tenant_id: &str, to: &str, content: &str) -> Result<(), String>;
    async fn handle_webhook(&self, payload: Value) -> Result<(), String>;
}
