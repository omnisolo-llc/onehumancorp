// Stub for WebSocket broadcasting
pub struct ChatWebSocket;

impl ChatWebSocket {
    pub async fn broadcast_message(_tenant_id: uuid::Uuid, _message: super::models::Message) {
        // Push message to connected Flutter clients for this tenant
    }
}
