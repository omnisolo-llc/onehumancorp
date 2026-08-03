use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn send_message(
        &self,
        tenant_id: &str,
        recipient_id: &str,
        message: &str,
        config: Option<&Value>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct WebWidgetAdapter;

#[async_trait]
impl ChannelAdapter for WebWidgetAdapter {
    async fn send_message(
        &self,
        tenant_id: &str,
        recipient_id: &str,
        message: &str,
        _config: Option<&Value>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Implement WebSocket pushing logic here
        tracing::info!(
            "WebWidgetAdapter: Sending message to {} for tenant {}: {}",
            recipient_id,
            tenant_id,
            message
        );
        Ok(())
    }
}

pub struct EmailAdapter;

#[async_trait]
impl ChannelAdapter for EmailAdapter {
    async fn send_message(
        &self,
        tenant_id: &str,
        recipient_id: &str,
        message: &str,
        _config: Option<&Value>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Implement email sending logic here
        tracing::info!(
            "EmailAdapter: Sending email to {} for tenant {}: {}",
            recipient_id,
            tenant_id,
            message
        );
        Ok(())
    }
}
