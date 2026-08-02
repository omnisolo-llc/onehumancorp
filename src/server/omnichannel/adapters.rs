use uuid::Uuid;
use async_trait::async_trait;

pub struct MessagePayload {
    pub content: String,
    pub sender_id: String, // e.g., phone number or email depending on channel
    pub channel_identifier: String,
}

#[async_trait]
pub trait ChannelAdapter {
    async fn receive_message(&self, payload: MessagePayload, tenant_id: Uuid) -> Result<(), String>;
    async fn send_message(&self, content: &str, recipient_id: &str, tenant_id: Uuid) -> Result<(), String>;
}

pub struct ApiChannelAdapter;

#[async_trait]
impl ChannelAdapter for ApiChannelAdapter {
    async fn receive_message(&self, payload: MessagePayload, tenant_id: Uuid) -> Result<(), String> {
        // Implementation for receiving simulated webhook messages
        println!("Received message on API channel for tenant {}: {:?}", tenant_id, payload.content);
        Ok(())
    }

    async fn send_message(&self, content: &str, recipient_id: &str, tenant_id: Uuid) -> Result<(), String> {
        // Implementation for sending simulated messages
        println!("Sending message on API channel for tenant {} to {}: {}", tenant_id, recipient_id, content);
        Ok(())
    }
}
