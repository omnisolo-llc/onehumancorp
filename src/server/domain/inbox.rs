use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub channel: String,
    pub ai_paused: bool,
    pub last_human_interaction: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String, // 'HUMAN_OWNER', 'AI_AGENT', 'CUSTOMER'
    pub content: String,
    pub requires_escalation: bool,
}

impl Conversation {
    /// Applies the logic invariant: `ai_paused` must be set to TRUE if sender is human
    pub fn process_new_message(&mut self, message: &Message, current_timestamp: i64) {
        if message.sender_type == "HUMAN_OWNER" {
            self.ai_paused = true;
            self.last_human_interaction = Some(current_timestamp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_ai_paused_on_human_message() {
        let mut conv = Conversation {
            id: "conv-123".to_string(),
            tenant_id: "tenant-abc".to_string(),
            channel: "IG".to_string(),
            ai_paused: false,
            last_human_interaction: None,
        };

        let msg_human = Message {
            id: "msg-1".to_string(),
            tenant_id: "tenant-abc".to_string(),
            conversation_id: "conv-123".to_string(),
            sender_type: "HUMAN_OWNER".to_string(),
            content: "Yes we do vegan cakes!".to_string(),
            requires_escalation: false,
        };

        let current_time = 1680000000;
        conv.process_new_message(&msg_human, current_time);

        assert!(conv.ai_paused, "AI should be paused when a human owner replies");
        assert_eq!(conv.last_human_interaction, Some(current_time));
    }

    #[test]
    fn test_conversation_ai_not_paused_on_customer_message() {
        let mut conv = Conversation {
            id: "conv-123".to_string(),
            tenant_id: "tenant-abc".to_string(),
            channel: "WhatsApp".to_string(),
            ai_paused: false,
            last_human_interaction: None,
        };

        let msg_customer = Message {
            id: "msg-2".to_string(),
            tenant_id: "tenant-abc".to_string(),
            conversation_id: "conv-123".to_string(),
            sender_type: "CUSTOMER".to_string(),
            content: "How much is it?".to_string(),
            requires_escalation: false,
        };

        let current_time = 1680000050;
        conv.process_new_message(&msg_customer, current_time);

        assert!(!conv.ai_paused, "AI should not be paused when a customer replies");
        assert_eq!(conv.last_human_interaction, None);
    }
}
