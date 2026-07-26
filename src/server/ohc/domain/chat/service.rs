use super::models::{Conversation, Message};
use super::adapter::ChannelAdapter;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ChatService {
    adapter: Arc<dyn ChannelAdapter>,
}

impl ChatService {
    pub fn new(adapter: Arc<dyn ChannelAdapter>) -> Self {
        Self { adapter }
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Conversation, String> {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            contact_id,
            inbox_id,
            status: "open".to_string(),
        };
        // In a real implementation, this would save to the DB
        Ok(conversation)
    }

    pub async fn process_incoming_webhook(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_id: Uuid,
    ) -> Result<Message, String> {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            content,
            sender_type: "contact".to_string(),
            sender_id,
            created_at: Utc::now(),
        };
        // In a real implementation, this would save to the DB and trigger local events
        Ok(message)
    }

    pub async fn dispatch_outgoing_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_id: Uuid,
    ) -> Result<Message, String> {
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            content,
            sender_type: "agent".to_string(),
            sender_id,
            created_at: Utc::now(),
        };

        // In a real implementation, this would save to DB first
        self.adapter.send_message(&message).await?;
        Ok(message)
    }
}
