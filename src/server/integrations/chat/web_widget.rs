use crate::models::Message;

pub async fn handle_web_widget_message(tenant_id: &str, content: &str) -> Message {
    Message {
        id: "msg_123".to_string(), // In real implementation, this comes from DB
        conversation_id: "conv_123".to_string(),
        tenant_id: tenant_id.to_string(),
        content: content.to_string(),
        sender_type: "user".to_string(),
    }
}
