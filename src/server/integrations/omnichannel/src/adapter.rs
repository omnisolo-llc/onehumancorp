use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    /// Send a message through the channel integration
    async fn send_message(&self, recipient_id: &str, content: &str) -> Result<(), String>;

    /// Ingest an incoming webhook payload from the provider
    async fn ingest_webhook(&self, payload: Value) -> Result<(), String>;
}
