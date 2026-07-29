/// Master Catalog C. Chat system Retirement & Custom Rust Omnichannel Chat System Standard
///
/// Native Rust Implementation: OHC implements its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside onehumancorp/mono.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::llm::LlmClient;
use crate::types::{ChatRequest, Message as LlmMessage};
use crate::agent::AgentRunConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub contact_id: String,
    pub status: ConversationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    BotActive,
    HumanHandoff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub sender_type: SenderType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SenderType {
    Customer,
    Agent,
    Bot,
}

pub struct ChatEngine {
    llm: Arc<dyn LlmClient>,
    cfg: AgentRunConfig,
}

impl ChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>, cfg: AgentRunConfig) -> Self {
        Self { llm, cfg }
    }

    /// Auto-responds using the real underlying LLM Harness.
    pub async fn auto_respond(&self, msg: &Message) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let req = ChatRequest {
            model: self.cfg.model.clone(),
            system: "You are an omnichannel AI auto-responder. Draft a response to the customer. Classify intent.".to_string(),
            messages: vec![
                LlmMessage::user(msg.content.clone())
            ],
            tools: vec![],
            max_tokens: self.cfg.max_tokens,
            temperature: self.cfg.temperature,
        };

        let response = self.llm.chat(req).await?;

        Ok(Message {
            id: format!("reply_{}", msg.id),
            conversation_id: msg.conversation_id.clone(),
            content: response.message.content,
            sender_type: SenderType::Bot,
        })
    }

    pub fn classify_intent(&self, msg: &Message) -> String {
        // More robust checking can be done via LLM as well in the future.
        let content = msg.content.to_lowercase();
        if content.contains("agent") || content.contains("human") || content.contains("operator") {
            "handoff_requested".to_string()
        } else if content.contains("refund") || content.contains("cancel") {
            "billing_inquiry".to_string()
        } else {
            "general_inquiry".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use async_trait::async_trait;
    use crate::types::{ChatResponse, Usage};

    struct MockLlmClient {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().unwrap();
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: LlmMessage::assistant("Hello, how can I help you today?"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_auto_respond_with_llm() {
        let llm = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });
        let engine = ChatEngine::new(llm, AgentRunConfig::default());
        let msg = Message {
            id: "msg1".to_string(),
            conversation_id: "conv1".to_string(),
            content: "Hello".to_string(),
            sender_type: SenderType::Customer,
        };

        let response = engine.auto_respond(&msg).await.unwrap();
        assert_eq!(response.sender_type, SenderType::Bot);
        assert_eq!(response.content, "Hello, how can I help you today?");
    }

    #[test]
    fn test_classify_intent() {
        let llm = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });
        let engine = ChatEngine::new(llm, AgentRunConfig::default());

        let msg_human = Message {
            id: "msg1".to_string(),
            conversation_id: "conv1".to_string(),
            content: "I want to talk to a human operator".to_string(),
            sender_type: SenderType::Customer,
        };

        let msg_billing = Message {
            id: "msg2".to_string(),
            conversation_id: "conv1".to_string(),
            content: "I need a refund".to_string(),
            sender_type: SenderType::Customer,
        };

        let msg_general = Message {
            id: "msg3".to_string(),
            conversation_id: "conv1".to_string(),
            content: "What are your hours?".to_string(),
            sender_type: SenderType::Customer,
        };

        assert_eq!(engine.classify_intent(&msg_human), "handoff_requested");
        assert_eq!(engine.classify_intent(&msg_billing), "billing_inquiry");
        assert_eq!(engine.classify_intent(&msg_general), "general_inquiry");
    }
}
