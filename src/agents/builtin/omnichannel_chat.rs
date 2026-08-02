use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolError, Usage};
use crate::llm::LlmClient;

/// Represents the events emitted by the native omnichannel chat system (formerly Chatwoot webhooks).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    ConversationStatusChanged,
    ConversationUpdated,
    ConversationCreated,
    ContactCreated,
    ContactUpdated,
    MessageCreated,
    MessageUpdated,
    WebwidgetTriggered,
    InboxCreated,
    InboxUpdated,
    ConversationTypingOn,
    ConversationTypingOff,
}

/// Represents the payload of a chat event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventPayload {
    pub event: WebhookEvent,
    pub conversation_id: Option<u64>,
    pub message_id: Option<u64>,
    pub content: Option<String>,
    pub account_id: u64,
}

/// The omnichannel chat engine responsible for auto-responding, copilot drafting, intent classification, and human handoff.
pub struct OmnichannelChatEngine {
    pub account_id: u64,
    pub llm: Arc<dyn LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(account_id: u64, llm: Arc<dyn LlmClient>) -> Self {
        Self { account_id, llm }
    }

    /// Classifies the intent of a customer message to route it to the appropriate department.
    pub async fn intent_classification(&self, message: &str) -> Result<String, ToolError> {
        let prompt = format!(
            "Classify the intent of this user message into one of: 'Billing', 'Technical Support', 'General Inquiry'. Message: '{}'",
            message
        );
        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };
        let response = self.llm.chat(req).await.map_err(|e| ToolError::Transient(e.to_string()))?;
        let output = response.message.content;

        if output.contains("Billing") {
            Ok("Billing".to_string())
        } else if output.contains("Technical Support") {
            Ok("Technical Support".to_string())
        } else {
            Ok("General Inquiry".to_string())
        }
    }

    /// Drafts a copilot response for a human agent to review.
    pub async fn copilot_drafting(&self, message: &str) -> Result<String, ToolError> {
        let prompt = format!(
            "You are an AI assistant helping a human agent draft a response to a customer. Draft a polite and helpful response to: '{}'",
            message
        );
        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 150,
            temperature: 0.0,
        };
        let response = self.llm.chat(req).await.map_err(|e| ToolError::Transient(e.to_string()))?;
        Ok(response.message.content)
    }

    /// Generates an automatic response if the AI is highly confident.
    pub async fn auto_responder(&self, message: &str) -> Result<Option<String>, ToolError> {
        let prompt = format!(
            "You are an autonomous AI auto-responder. If you can confidently answer the user's message using common knowledge, provide the answer. Otherwise, reply exactly with 'UNKNOWN'. Message: '{}'",
            message
        );
        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 150,
            temperature: 0.0,
        };
        let response = self.llm.chat(req).await.map_err(|e| ToolError::Transient(e.to_string()))?;
        let output = response.message.content.trim();

        if output == "UNKNOWN" {
            Ok(None)
        } else {
            Ok(Some(output.to_string()))
        }
    }

    /// Evaluates whether the conversation needs to be escalated to a human.
    pub async fn human_handoff(&self, message: &str, ai_confidence: f32) -> Result<bool, ToolError> {
        if ai_confidence < 0.7 {
            return Ok(true);
        }

        let prompt = format!(
            "Analyze the following user message. Does the user explicitly request to speak to a human, manager, or indicate an urgent issue that an AI cannot handle? Reply 'YES' or 'NO'. Message: '{}'",
            message
        );
        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };
        let response = self.llm.chat(req).await.map_err(|e| ToolError::Transient(e.to_string()))?;

        Ok(response.message.content.contains("YES"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockLlm {
        response_content: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: self.response_content.clone(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage { input_tokens: 10, output_tokens: 10, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
                stop_reason: "stop".to_string(),
                response_id: Some("test".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_intent_classification() {
        let llm = Arc::new(MockLlm { response_content: "Billing".to_string() });
        let engine = OmnichannelChatEngine::new(1, llm);
        assert_eq!(engine.intent_classification("What is the pricing?").await.unwrap(), "Billing");
    }

    #[tokio::test]
    async fn test_copilot_drafting() {
        let llm = Arc::new(MockLlm { response_content: "Here is your draft".to_string() });
        let engine = OmnichannelChatEngine::new(1, llm);
        let draft = engine.copilot_drafting("help me").await.unwrap();
        assert_eq!(draft, "Here is your draft");
    }

    #[tokio::test]
    async fn test_auto_responder() {
        let llm = Arc::new(MockLlm { response_content: "UNKNOWN".to_string() });
        let engine = OmnichannelChatEngine::new(1, llm);
        assert_eq!(engine.auto_responder("complex question").await.unwrap(), None);

        let llm2 = Arc::new(MockLlm { response_content: "Hello!".to_string() });
        let engine2 = OmnichannelChatEngine::new(1, llm2);
        assert_eq!(engine2.auto_responder("Hi").await.unwrap(), Some("Hello!".to_string()));
    }

    #[tokio::test]
    async fn test_human_handoff() {
        let llm = Arc::new(MockLlm { response_content: "NO".to_string() });
        let engine = OmnichannelChatEngine::new(1, llm);
        // Low confidence still forces handoff
        assert!(engine.human_handoff("Some query", 0.5).await.unwrap());
        // High confidence and NO -> false
        assert!(!engine.human_handoff("Some query", 0.9).await.unwrap());

        let llm_yes = Arc::new(MockLlm { response_content: "YES".to_string() });
        let engine_yes = OmnichannelChatEngine::new(1, llm_yes);
        assert!(engine_yes.human_handoff("Human please", 0.9).await.unwrap());
    }
}
