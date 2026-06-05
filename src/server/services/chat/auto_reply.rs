use crate::domain::chat::conversation::{Conversation, ConversationStatus};
use crate::domain::chat::message::{Message, MessageRole};
use chrono::Utc;
use uuid::Uuid;

pub struct AutoReplyEngine;

impl AutoReplyEngine {
    pub async fn process_conversation(
        conversation: &mut Conversation,
        latest_message: &Message,
    ) -> Option<Message> {
        if conversation.human_takeover {
            return None; // AI is paused for this thread
        }

        if latest_message.role != MessageRole::Customer {
            return None;
        }

        // Simulate LLM processing and confidence scoring
        // In a real implementation, this would call AgentRouter -> CSAgent / SalesAgent
        let (reply_content, confidence, is_draft) = Self::simulate_llm_eval(&latest_message.content);

        let ai_message = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            organization_id: conversation.organization_id.clone(),
            role: MessageRole::AiAgent,
            content: reply_content,
            is_draft,
            confidence_score: Some(confidence),
            created_at: Utc::now(),
        };

        if !is_draft {
            conversation.ai_handled = true;
            conversation.status = ConversationStatus::Resolved; // Simplified logic
        } else {
            conversation.status = ConversationStatus::RequiresAttention;
        }
        conversation.updated_at = Utc::now();

        Some(ai_message)
    }

    fn simulate_llm_eval(content: &str) -> (String, f64, bool) {
        if content.to_lowercase().contains("vegan") {
            (
                "Yes! We offer vegan chocolate and vanilla cakes. You can order and pay your deposit here: https://checkout.stripe.com/...".to_string(),
                0.95,
                false, // Auto-send
            )
        } else if content.to_lowercase().contains("plumbing") {
            (
                "I can help with plumbing fixes. Could you describe the issue? Generally, my rate is $80/hr.".to_string(),
                0.85,
                false, // Auto-send
            )
        } else {
            (
                "I'm not completely sure about that. Let me have the owner get back to you shortly.".to_string(),
                0.40,
                true, // Draft for review
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::chat::channel::ChannelType;

    #[tokio::test]
    async fn test_auto_reply_vegan_match() {
        let mut conversation = Conversation {
            id: Uuid::new_v4().to_string(),
            organization_id: "org_1".to_string(),
            customer_id: None,
            channel_type: ChannelType::InstagramDm,
            channel_identifier: "testuser".to_string(),
            status: ConversationStatus::Open,
            ai_handled: false,
            human_takeover: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let message = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            organization_id: "org_1".to_string(),
            role: MessageRole::Customer,
            content: "Do you have vegan options?".to_string(),
            is_draft: false,
            confidence_score: None,
            created_at: Utc::now(),
        };

        let reply_opt = AutoReplyEngine::process_conversation(&mut conversation, &message).await;
        assert!(reply_opt.is_some());
        let reply = reply_opt.unwrap();
        assert_eq!(reply.role, MessageRole::AiAgent);
        assert!(!reply.is_draft);
        assert!(reply.content.contains("vegan"));
        assert!(conversation.ai_handled);
        assert_eq!(conversation.status, ConversationStatus::Resolved);
    }
}
