use crate::domain::chat::conversation::{Conversation, ConversationStatus};
use crate::domain::chat::message::{Message, MessageRole};
use crate::domain::chat::channel::ChannelType;
use chrono::Utc;
use uuid::Uuid;

pub struct IngestionService;

impl IngestionService {
    pub fn handle_incoming_message(
        organization_id: String,
        channel_type: ChannelType,
        channel_identifier: String,
        content: String,
        customer_id: Option<String>,
    ) -> (Conversation, Message) {
        // In a real implementation, this would look up an existing conversation.
        // For now, we simulate creating a new one or appending.

        let conversation = Conversation {
            id: Uuid::new_v4().to_string(),
            organization_id: organization_id.clone(),
            customer_id,
            channel_type,
            channel_identifier,
            status: ConversationStatus::Open,
            ai_handled: false,
            human_takeover: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let message = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            organization_id,
            role: MessageRole::Customer,
            content,
            is_draft: false,
            confidence_score: None,
            created_at: Utc::now(),
        };

        (conversation, message)
    }
}
