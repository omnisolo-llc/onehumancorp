use uuid::Uuid;
use std::sync::Arc;
use crate::models::{Conversation, Message, OutboxMessage};
use crate::repository::OmnichannelRepository;
use chrono::Utc;

pub struct OmnichannelService {
    repo: Arc<dyn OmnichannelRepository>,
}

impl OmnichannelService {
    pub fn new(repo: Arc<dyn OmnichannelRepository>) -> Self {
        Self { repo }
    }

    pub async fn handle_incoming_message(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        content: String,
    ) -> Result<Message, String> {
        // 1. Ensure conversation exists or create one
        // For simplicity, we just create a new one here if we don't have logic to find an open one.
        // In a real implementation, you'd lookup an existing open conversation for this contact/inbox.
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            contact_id,
            inbox_id,
            status: "open".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let _ = self.repo.create_conversation(tenant_id, conv.clone()).await?;

        // 2. Create the message
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conv.id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact_id),
            content,
            status: "delivered".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.create_message(tenant_id, msg.clone()).await
    }

    pub async fn draft_and_send_reply(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        channel_type: String,
    ) -> Result<Message, String> {
        // 1. Create the message as pending
        let mut msg = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            sender_type: "agent".to_string(), // Or bot depending on context
            sender_id: None,
            content,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        msg = self.repo.create_message(tenant_id, msg).await?;

        // 2. Enqueue in Outbox
        let outbox_msg = OutboxMessage {
            id: Uuid::new_v4(),
            tenant_id,
            message_id: msg.id,
            channel_type,
            payload: serde_json::json!({
                "content": msg.content,
                "conversation_id": msg.conversation_id.to_string(),
            }),
            status: "pending".to_string(),
            attempts: 0,
            last_attempt_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.repo.enqueue_outbox_message(tenant_id, outbox_msg).await?;

        Ok(msg)
    }
}
