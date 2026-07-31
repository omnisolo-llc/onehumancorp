#![allow(clippy::empty_line_after_doc_comments)]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message};

/// Master Catalog: Chatwoot (Customer Support & Omnichannel Chat)
/// Replicating full omnichannel customer chat features natively in Rust:
/// AI auto-responder, copilot response drafting, intent classification, and human agent handoff.

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatIntent {
    Support,
    Sales,
    Billing,
    Handoff,
    General,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OmnichannelResponse {
    pub auto_reply: Option<String>,
    pub copilot_draft: Option<String>,
    pub intent: ChatIntent,
    pub handoff_required: bool,
}

pub struct OmnichannelChatEngine {
    pub llm: Arc<dyn LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn process_message(&self, message: &str, is_copilot_mode: bool) -> Result<OmnichannelResponse, String> {
        let system_prompt = "You are an omnichannel routing engine. Classify intent (Support, Sales, Billing, Handoff, General). Decide if human handoff is required. If copilot_mode is true, generate a copilot_draft instead of an auto_reply. Respond strictly in JSON format: {\"intent\": \"Support\", \"handoff_required\": false, \"auto_reply\": \"...\", \"copilot_draft\": null}";

        let req = ChatRequest {
            model: "claude-3-haiku-20240307".to_string(),
            messages: vec![

                Message::user(format!("copilot_mode: {}\nUser Message: {}", is_copilot_mode, message)),
            ],
            temperature: 0.0,
            system: system_prompt.to_string(),
            max_tokens: 1024,
            tools: vec![],
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        let text = resp.message.content;
        let parsed: OmnichannelResponse = serde_json::from_str(&text).map_err(|e| e.to_string())?;

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockChatwootLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockChatwootLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_omnichannel_chat_auto_responder() {
        let llm = Arc::new(MockChatwootLlm {
            response_text: r#"{"intent": "Support", "handoff_required": false, "auto_reply": "How can I help you today?", "copilot_draft": null}"#.to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm);
        let resp = engine.process_message("I need help with my account", false).await.unwrap();
        assert_eq!(resp.intent, ChatIntent::Support);
        assert!(!resp.handoff_required);
        assert_eq!(resp.auto_reply.unwrap(), "How can I help you today?");
        assert!(resp.copilot_draft.is_none());
    }

    #[tokio::test]
    async fn test_omnichannel_chat_copilot_draft() {
        let llm = Arc::new(MockChatwootLlm {
            response_text: r#"{"intent": "Sales", "handoff_required": true, "auto_reply": null, "copilot_draft": "I see you are interested in our premium plan."}"#.to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm);
        let resp = engine.process_message("I want to buy the premium plan", true).await.unwrap();
        assert_eq!(resp.intent, ChatIntent::Sales);
        assert!(resp.handoff_required);
        assert!(resp.auto_reply.is_none());
        assert_eq!(resp.copilot_draft.unwrap(), "I see you are interested in our premium plan.");
    }

    #[tokio::test]
    async fn test_omnichannel_chat_handoff() {
        let llm = Arc::new(MockChatwootLlm {
            response_text: r#"{"intent": "Handoff", "handoff_required": true, "auto_reply": "Connecting you to a human agent.", "copilot_draft": null}"#.to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm);
        let resp = engine.process_message("Let me speak to a human", false).await.unwrap();
        assert_eq!(resp.intent, ChatIntent::Handoff);
        assert!(resp.handoff_required);
    }
}
