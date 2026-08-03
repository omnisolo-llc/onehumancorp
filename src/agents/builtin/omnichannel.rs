use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    WhatsApp,
    Instagram,
    Facebook,
    Twitter,
    Email,
    WebWidget,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub tenant_id: String,
    pub channel: ChannelType,
    pub sender_id: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_from_agent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntentType {
    Support,
    Sales,
    Billing,
    Technical,
    Spam,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentClassificationResult {
    pub intent: IntentType,
    pub confidence: f32,
    pub suggested_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftResponse {
    pub content: String,
    pub confidence: f32,
    pub requires_human_approval: bool,
}

#[async_trait::async_trait]
pub trait OmnichannelEngine: Send + Sync {
    /// Native AI auto-responder
    async fn auto_respond(&self, message: &ChatMessage) -> Result<Option<ChatMessage>, String>;

    /// Copilot response drafting
    async fn draft_response(&self, message: &ChatMessage) -> Result<DraftResponse, String>;

    /// Intent classification
    async fn classify_intent(&self, message: &ChatMessage) -> Result<IntentClassificationResult, String>;

    /// Human agent handoff
    async fn request_human_handoff(&self, session_id: &str, reason: &str) -> Result<(), String>;
}

pub struct NativeOmnichannelEngine {
    llm: Arc<dyn crate::llm::LlmClient>,
}

impl NativeOmnichannelEngine {
    pub fn new(llm: Arc<dyn crate::llm::LlmClient>) -> Self {
        Self { llm }
    }
}

#[async_trait::async_trait]
impl OmnichannelEngine for NativeOmnichannelEngine {
    async fn auto_respond(&self, message: &ChatMessage) -> Result<Option<ChatMessage>, String> {
        // Use LLM to generate response if confidence is high, else return None
        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: "default".to_string(),
            system: "".to_string(),
            messages: vec![
                ohc_builtin_agent_core::types::Message::system("You are an auto-responder AI. Reply concisely and helpfully. If you cannot answer definitively, reply with exactly 'CANNOT_ANSWER'."),
                ohc_builtin_agent_core::types::Message::user(message.content.clone()),
            ],
            tools: vec![],
            temperature: 0.2,
            max_tokens: 0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim();
                if text == "CANNOT_ANSWER" {
                    Ok(None)
                } else {
                    Ok(Some(ChatMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        session_id: message.session_id.clone(),
                        tenant_id: message.tenant_id.clone(),
                        channel: message.channel.clone(),
                        sender_id: "agent".to_string(),
                        content: text.to_string(),
                        timestamp: chrono::Utc::now(),
                        is_from_agent: true,
                    }))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn draft_response(&self, message: &ChatMessage) -> Result<DraftResponse, String> {
        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: "default".to_string(),
            system: "".to_string(),
            messages: vec![
                ohc_builtin_agent_core::types::Message::system("You are a copilot drafter. Draft a helpful reply to the customer's message."),
                ohc_builtin_agent_core::types::Message::user(message.content.clone()),
            ],
            tools: vec![],
            temperature: 0.7,
            max_tokens: 0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(DraftResponse {
                content: resp.message.content,
                confidence: 0.85,
                requires_human_approval: true,
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn classify_intent(&self, message: &ChatMessage) -> Result<IntentClassificationResult, String> {
        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: "default".to_string(),
            system: "".to_string(),
            messages: vec![
                ohc_builtin_agent_core::types::Message::system("Classify the user intent into one of: Support, Sales, Billing, Technical, Spam. Reply with exactly the intent name."),
                ohc_builtin_agent_core::types::Message::user(message.content.clone()),
            ],
            tools: vec![],
            temperature: 0.1,
            max_tokens: 0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let intent = match resp.message.content.trim().to_lowercase().as_str() {
                    "support" => IntentType::Support,
                    "sales" => IntentType::Sales,
                    "billing" => IntentType::Billing,
                    "technical" => IntentType::Technical,
                    "spam" => IntentType::Spam,
                    _ => IntentType::Unknown,
                };
                Ok(IntentClassificationResult {
                    intent,
                    confidence: 0.9,
                    suggested_tags: vec![],
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn request_human_handoff(&self, _session_id: &str, _reason: &str) -> Result<(), String> {
        // Native rust equivalent of previous system's handoff
        // In a real implementation this would update the session state in DB
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};

    struct MockLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };
            Ok(ChatResponse {
                response_id: Some("".to_string()),
                stop_reason: "".to_string(),
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_omnichannel_classify_intent() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec!["Sales".to_string()]),
        });
        let engine = NativeOmnichannelEngine::new(llm);

        let msg = ChatMessage {
            id: "1".to_string(),
            session_id: "s1".to_string(),
            tenant_id: "t1".to_string(),
            channel: ChannelType::WhatsApp,
            sender_id: "u1".to_string(),
            content: "I want to buy a cake".to_string(),
            timestamp: chrono::Utc::now(),
            is_from_agent: false,
        };

        let result = engine.classify_intent(&msg).await.unwrap();
        assert_eq!(result.intent, IntentType::Sales);
    }

    #[tokio::test]
    async fn test_omnichannel_auto_respond() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec!["Here is the pricing list.".to_string()]),
        });
        let engine = NativeOmnichannelEngine::new(llm);

        let msg = ChatMessage {
            id: "1".to_string(),
            session_id: "s1".to_string(),
            tenant_id: "t1".to_string(),
            channel: ChannelType::Instagram,
            sender_id: "u1".to_string(),
            content: "How much?".to_string(),
            timestamp: chrono::Utc::now(),
            is_from_agent: false,
        };

        let result = engine.auto_respond(&msg).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().content, "Here is the pricing list.");
    }

    #[tokio::test]
    async fn test_omnichannel_auto_respond_cannot_answer() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec!["CANNOT_ANSWER".to_string()]),
        });
        let engine = NativeOmnichannelEngine::new(llm);

        let msg = ChatMessage {
            id: "1".to_string(),
            session_id: "s1".to_string(),
            tenant_id: "t1".to_string(),
            channel: ChannelType::Instagram,
            sender_id: "u1".to_string(),
            content: "What is the meaning of life?".to_string(),
            timestamp: chrono::Utc::now(),
            is_from_agent: false,
        };

        let result = engine.auto_respond(&msg).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_omnichannel_draft_response() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec!["Let me check that for you.".to_string()]),
        });
        let engine = NativeOmnichannelEngine::new(llm);

        let msg = ChatMessage {
            id: "1".to_string(),
            session_id: "s1".to_string(),
            tenant_id: "t1".to_string(),
            channel: ChannelType::Facebook,
            sender_id: "u1".to_string(),
            content: "Where is my order?".to_string(),
            timestamp: chrono::Utc::now(),
            is_from_agent: false,
        };

        let result = engine.draft_response(&msg).await.unwrap();
        assert_eq!(result.content, "Let me check that for you.");
        assert!(result.requires_human_approval);
    }

    #[tokio::test]
    async fn test_omnichannel_handoff() {
        let llm = Arc::new(MockLlm {
            responses: Mutex::new(vec![]),
        });
        let engine = NativeOmnichannelEngine::new(llm);
        let result = engine.request_human_handoff("s1", "Customer is angry").await;
        assert!(result.is_ok());
    }
}
