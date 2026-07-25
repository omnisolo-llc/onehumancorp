mod models;
use crate::models::{Conversation, Message, MessageType};

pub struct HandoffManager;

impl HandoffManager {
    /// Escalate the conversation to a human agent and disable the bot.
    /// Returns the updated Conversation and an optional system message to add.
    pub fn execute_handoff(mut conversation: Conversation) -> (Conversation, Message) {
        conversation.is_bot_active = false;
        // Logic to assign to a human could be added here depending on routing rules.

        let system_message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            content: "You are being transferred to a human agent. Please hold on.".to_string(),
            message_type: MessageType::Outgoing,
            sender_id: Some("system".to_string()),
            private: false,
            created_at: chrono::Utc::now().timestamp(),
        };

        (conversation, system_message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ConversationStatus;

    #[test]
    fn test_handoff_execution() {
        let conv = Conversation {
            id: "conv1".to_string(),
            inbox_id: "inbox1".to_string(),
            contact_id: "contact1".to_string(),
            assignee_id: None,
            status: ConversationStatus::Open,
            is_bot_active: true,
        };

        let (updated_conv, msg) = HandoffManager::execute_handoff(conv);

        assert_eq!(updated_conv.is_bot_active, false);
        assert_eq!(msg.message_type, MessageType::Outgoing);
        assert!(msg.content.contains("transferred to a human agent"));
    }
}
