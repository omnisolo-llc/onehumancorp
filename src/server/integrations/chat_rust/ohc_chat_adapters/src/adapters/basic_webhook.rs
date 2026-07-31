use ohc_chat_core::models::{Message, Conversation, Inbox, Contact};
use uuid::Uuid;

pub struct BasicWebhookAdapter;

impl BasicWebhookAdapter {
    pub fn ingest_message(payload: &str) -> Message {
        println!("Ingesting dummy webhook message: {}", payload);
        Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            content: payload.to_string(),
            sender_type: "customer".to_string(),
            sender_id: None,
            channel_message_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
