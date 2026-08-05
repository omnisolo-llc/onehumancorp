use async_trait::async_trait;

#[async_trait]
pub trait ChannelAdapter {
    async fn send_message(&self, recipient_id: &str, message: &str) -> Result<(), String>;
}

pub struct WebWidgetAdapter {
    pub connected_clients: std::sync::Arc<std::collections::HashMap<String, String>>,
}

#[async_trait]
impl ChannelAdapter for WebWidgetAdapter {
    async fn send_message(&self, recipient_id: &str, message: &str) -> Result<(), String> {
        tracing::info!("WebWidgetAdapter sending message to {}: {}", recipient_id, message);
        Ok(())
    }
}
