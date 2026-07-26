#![allow(clippy::all)]
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
/// Master Catalog Harness Pattern: Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// External Chatwoot dependencies are 100% RETIRED. The builtin AI agent microservice connects
/// directly via high-performance Rust IPC/gRPC to OHC's native Rust Chat Engine.
/// This module audits Chatwoot bot/webhook protocols and replicates matching native
/// AI auto-responder, copilot response drafting, intent classification, and human agent handoff features in Rust.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Intent {
    SalesInquiry,
    SupportRequest,
    Spam,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatwootMessageEvent {
    pub event: String, // e.g. "message_created"
    pub conversation_id: String,
    pub message_id: String,
    pub content: String,
    pub sender_type: String, // "Contact" or "User"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OmnichannelAction {
    AutoRespond(String),
    DraftCopilotResponse(String),
    HumanHandoff(String),
    NoAction,
}

pub struct ChatwootProtocolReplicator;

impl ChatwootProtocolReplicator {
    pub fn parse_webhook(payload: &str) -> Result<ChatwootMessageEvent, String> {
        serde_json::from_str(payload).map_err(|e| e.to_string())
    }
}

pub struct OmnichannelChatEngine {
    pub llm: Arc<dyn ohc_builtin_agent_llm::LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn ohc_builtin_agent_llm::LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn classify_intent(&self, content: &str) -> Result<Intent, String> {
        let system_prompt = "You are an intent classification engine. Classify the user message into one of: SalesInquiry, SupportRequest, Spam, Unknown. Reply ONLY with the classification.";
        let req = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(content.to_string())],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let res = resp.message.content.trim();

        match res {
            "SalesInquiry" => Ok(Intent::SalesInquiry),
            "SupportRequest" => Ok(Intent::SupportRequest),
            "Spam" => Ok(Intent::Spam),
            _ => Ok(Intent::Unknown),
        }
    }

    pub async fn draft_copilot_response(&self, content: &str) -> Result<String, String> {
        let system_prompt = "You are an AI copilot drafting a response for a customer support agent. Keep it professional, concise, and helpful.";
        let req = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(content.to_string())],
            tools: vec![],
            max_tokens: 250,
            temperature: 0.0,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content.trim().to_string())
    }

    pub async fn process_incoming_message(&self, event: &ChatwootMessageEvent) -> Result<OmnichannelAction, String> {
        if event.sender_type != "Contact" {
            return Ok(OmnichannelAction::NoAction);
        }

        let intent = self.classify_intent(&event.content).await?;

        match intent {
            Intent::Spam => Ok(OmnichannelAction::NoAction),
            Intent::SalesInquiry => {
                // Auto-responder for simple sales inquiries
                let draft = self.draft_copilot_response(&event.content).await?;
                Ok(OmnichannelAction::AutoRespond(draft))
            }
            Intent::SupportRequest | Intent::Unknown => {
                // Escalate to human and draft a copilot response
                let draft = self.draft_copilot_response(&event.content).await?;
                Ok(OmnichannelAction::HumanHandoff(format!("Draft prepared for review: {}", draft)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;
    use ohc_builtin_agent_core::types::{Role, Usage, ChatResponse};

    struct MockOmniLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl ohc_builtin_agent_llm::LlmClient for MockOmniLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let resp_text = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Unknown".to_string()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: resp_text,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_parse_webhook() {
        let payload = r#"{
            "event": "message_created",
            "conversation_id": "conv-123",
            "message_id": "msg-456",
            "content": "How much does it cost?",
            "sender_type": "Contact"
        }"#;

        let event = ChatwootProtocolReplicator::parse_webhook(payload).unwrap();
        assert_eq!(event.event, "message_created");
        assert_eq!(event.content, "How much does it cost?");
        assert_eq!(event.sender_type, "Contact");
    }

    #[tokio::test]
    async fn test_omnichannel_sales_autorespond() {
        let llm = Arc::new(MockOmniLlm {
            // First call is intent, second is draft
            responses: Mutex::new(vec!["SalesInquiry".to_string(), "It costs $10.".to_string()]),
        });
        let engine = OmnichannelChatEngine::new(llm);

        let event = ChatwootMessageEvent {
            event: "message_created".to_string(),
            conversation_id: "c1".to_string(),
            message_id: "m1".to_string(),
            content: "Pricing?".to_string(),
            sender_type: "Contact".to_string(),
        };

        let action = engine.process_incoming_message(&event).await.unwrap();
        match action {
            OmnichannelAction::AutoRespond(msg) => {
                assert_eq!(msg, "It costs $10.");
            }
            _ => panic!("Expected AutoRespond"),
        }
    }

    #[tokio::test]
    async fn test_omnichannel_support_handoff() {
        let llm = Arc::new(MockOmniLlm {
            // First call is intent, second is draft
            responses: Mutex::new(vec!["SupportRequest".to_string(), "I will help you.".to_string()]),
        });
        let engine = OmnichannelChatEngine::new(llm);

        let event = ChatwootMessageEvent {
            event: "message_created".to_string(),
            conversation_id: "c1".to_string(),
            message_id: "m1".to_string(),
            content: "It is broken".to_string(),
            sender_type: "Contact".to_string(),
        };

        let action = engine.process_incoming_message(&event).await.unwrap();
        match action {
            OmnichannelAction::HumanHandoff(msg) => {
                assert!(msg.contains("I will help you."));
            }
            _ => panic!("Expected HumanHandoff"),
        }
    }

    #[tokio::test]
    async fn test_omnichannel_spam_no_action() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec!["Spam".to_string()]),
        });
        let engine = OmnichannelChatEngine::new(llm);

        let event = ChatwootMessageEvent {
            event: "message_created".to_string(),
            conversation_id: "c1".to_string(),
            message_id: "m1".to_string(),
            content: "Buy pills".to_string(),
            sender_type: "Contact".to_string(),
        };

        let action = engine.process_incoming_message(&event).await.unwrap();
        assert_eq!(action, OmnichannelAction::NoAction);
    }

    #[tokio::test]
    async fn test_omnichannel_ignore_agent_messages() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec![]),
        });
        let engine = OmnichannelChatEngine::new(llm);

        let event = ChatwootMessageEvent {
            event: "message_created".to_string(),
            conversation_id: "c1".to_string(),
            message_id: "m1".to_string(),
            content: "Buy pills".to_string(),
            sender_type: "User".to_string(), // Agent message
        };

        let action = engine.process_incoming_message(&event).await.unwrap();
        assert_eq!(action, OmnichannelAction::NoAction);
    }
}
