mod models;
use std::sync::Arc;
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::types::{ChatRequest, Message as LlmMessage, Role};
use crate::models::{Conversation, Message, MessageType};

pub struct Copilot {
    llm: Arc<dyn LlmClient>,
}

impl Copilot {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn draft_response(
        &self,
        _conversation: &Conversation,
        history: &[Message],
    ) -> Result<String, String> {
        let mut prompt = String::from("You are an AI assistant helping a human support agent draft a response to a customer. Write a professional, empathetic response based on the conversation history.\n\nHistory:\n");
        for msg in history.iter().rev().take(10).rev() {
            let sender = if msg.message_type == MessageType::Incoming { "Customer" } else { "Agent" };
            prompt.push_str(&format!("{}: {}\n", sender, msg.content));
        }
        prompt.push_str("\nSuggested draft:");

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
            temperature: 0.5,
            max_tokens: 250,
            system: "".to_string(),

        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content.trim().to_string())
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
    async fn test_copilot_draft() {
        let llm = Arc::new(MockLlm { response: "I'm sorry you're facing this issue. Let me check your account.".to_string() });
        let copilot = Copilot::new(llm);
        let conv = Conversation {
            id: "conv1".to_string(),
            inbox_id: "inbox1".to_string(),
            contact_id: "contact1".to_string(),
            assignee_id: Some("human".to_string()),
            status: ConversationStatus::Open,
            is_bot_active: false,
        };
        let msg = Message {
            id: "msg1".to_string(),
            conversation_id: "conv1".to_string(),
            content: "My login is broken".to_string(),
            message_type: MessageType::Incoming,
            sender_id: None,
            private: false,
            created_at: 0,
        };

        let draft = copilot.draft_response(&conv, &[msg]).await.unwrap();
        assert_eq!(draft, "I'm sorry you're facing this issue. Let me check your account.");
    }
}
