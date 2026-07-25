mod models;
use std::sync::Arc;
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::types::{ChatRequest, Message as LlmMessage, Role};
use crate::models::{Conversation, Message, MessageType};

pub struct AutoResponder {
    llm: Arc<dyn LlmClient>,
}

impl AutoResponder {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn generate_reply(
        &self,
        conversation: &Conversation,
        history: &[Message],
    ) -> Result<Option<Message>, String> {
        if !conversation.is_bot_active {
            return Ok(None); // Do not auto-respond if a human has taken over
        }

        let mut prompt = String::from("You are an AI support agent. Reply to the customer's last message concisely and politely.\n\nHistory:\n");
        for msg in history.iter().rev().take(5).rev() { // take last 5
            let sender = if msg.message_type == MessageType::Incoming { "Customer" } else { "Agent" };
            prompt.push_str(&format!("{}: {}\n", sender, msg.content));
        }
        prompt.push_str("Agent:");

        let req = ChatRequest {
            messages: vec![LlmMessage {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            model: "default".to_string(),
            temperature: 0.7,
            max_tokens: 150,
            system: "".to_string(),

        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        Ok(Some(Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation.id.clone(),
            content: resp.message.content.trim().to_string(),
            message_type: MessageType::Outgoing,
            sender_id: Some("ai_auto_responder".to_string()),
            private: false,
            created_at: chrono::Utc::now().timestamp(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ConversationStatus;
    use ohc_builtin_agent::types::{ChatResponse, Usage};

    struct MockLlm {
        response: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: LlmMessage::assistant(&self.response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_auto_responder_active() {
        let llm = Arc::new(MockLlm { response: "Hello, I can help!".to_string() });
        let responder = AutoResponder::new(llm);
        let conv = Conversation {
            id: "conv1".to_string(),
            inbox_id: "inbox1".to_string(),
            contact_id: "contact1".to_string(),
            assignee_id: None,
            status: ConversationStatus::Open,
            is_bot_active: true,
        };
        let msg = Message {
            id: "msg1".to_string(),
            conversation_id: "conv1".to_string(),
            content: "Hi".to_string(),
            message_type: MessageType::Incoming,
            sender_id: None,
            private: false,
            created_at: 0,
        };

        let reply = responder.generate_reply(&conv, &[msg]).await.unwrap().unwrap();
        assert_eq!(reply.content, "Hello, I can help!");
        assert_eq!(reply.sender_id.unwrap(), "ai_auto_responder");
    }

    #[tokio::test]
    async fn test_auto_responder_inactive() {
        let llm = Arc::new(MockLlm { response: "Hello".to_string() });
        let responder = AutoResponder::new(llm);
        let conv = Conversation {
            id: "conv1".to_string(),
            inbox_id: "inbox1".to_string(),
            contact_id: "contact1".to_string(),
            assignee_id: Some("agent1".to_string()),
            status: ConversationStatus::Open,
            is_bot_active: false, // human took over
        };

        let reply = responder.generate_reply(&conv, &[]).await.unwrap();
        assert!(reply.is_none());
    }
}
