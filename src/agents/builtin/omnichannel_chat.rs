/// Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// Replicates Chatwoot bot/webhook protocols and features natively in Rust.
/// Features: AI auto-responder, copilot response drafting, intent classification, and human agent handoff.

use ohc_builtin_agent_core::types::{ChatRequest, Message};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::llm::LlmClient;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatIntent {
    Sales,
    Support,
    General,
    Urgent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatStatus {
    BotHandled,
    HumanHandoff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatState {
    pub current_intent: Option<ChatIntent>,
    pub status: ChatStatus,
    pub copilot_draft: Option<String>,
    pub final_response: Option<String>,
}

impl Default for ChatState {
    fn default() -> Self {
        Self {
            current_intent: None,
            status: ChatStatus::BotHandled,
            copilot_draft: None,
            final_response: None,
        }
    }
}

pub struct OmnichannelChatEngine {
    llm: Arc<dyn LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    /// Classify the user intent to mimic Chatwoot's intent classification routing.
    pub async fn classify_intent(&self, message: &str) -> Result<ChatIntent, String> {
        let req = ChatRequest {
            messages: vec![Message::user(format!(
                "Classify the following user message into exactly one of these intents: Sales, Support, General, Urgent. Reply with JUST the word, nothing else.\nMessage: {}",
                message
            ))],
            system: "You are an intent classifier for an omnichannel chat engine.".to_string(),
            model: "default".to_string(),
            tools: vec![],
            max_tokens: 0,
            temperature: 0.0,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let text = response.message.content.trim().to_lowercase();

        if text.contains("urgent") {
            Ok(ChatIntent::Urgent)
        } else if text.contains("sales") {
            Ok(ChatIntent::Sales)
        } else if text.contains("support") {
            Ok(ChatIntent::Support)
        } else {
            Ok(ChatIntent::General)
        }
    }

    /// Automatically respond if the intent allows it.
    pub async fn auto_respond(&self, message: &str, intent: &ChatIntent) -> Result<ChatState, String> {
        if *intent == ChatIntent::Urgent {
            return self.handoff_to_human(message).await;
        }

        let req = ChatRequest {
            messages: vec![Message::user(message.to_string())],
            system: format!(
                "You are an AI auto-responder for an omnichannel chat system. The user intent is {:?}. Provide a helpful, concise response.",
                intent
            ),
            model: "default".to_string(),
            tools: vec![],
            max_tokens: 0,
            temperature: 0.0,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        Ok(ChatState {
            current_intent: Some(intent.clone()),
            status: ChatStatus::BotHandled,
            copilot_draft: None,
            final_response: Some(response.message.content),
        })
    }

    /// Draft a response for human review instead of sending it directly (Copilot mode).
    pub async fn draft_copilot_response(&self, message: &str, intent: &ChatIntent) -> Result<ChatState, String> {
        let req = ChatRequest {
            messages: vec![Message::user(message.to_string())],
            system: format!(
                "You are an AI copilot drafting a response for a human agent. The user intent is {:?}. Draft a professional and helpful reply.",
                intent
            ),
            model: "default".to_string(),
            tools: vec![],
            max_tokens: 0,
            temperature: 0.0,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        Ok(ChatState {
            current_intent: Some(intent.clone()),
            status: ChatStatus::HumanHandoff, // A drafted response expects human action
            copilot_draft: Some(response.message.content),
            final_response: None,
        })
    }

    /// Escalate the conversation to a human agent, halting bot interaction.
    pub async fn handoff_to_human(&self, _message: &str) -> Result<ChatState, String> {
        Ok(ChatState {
            current_intent: Some(ChatIntent::Urgent),
            status: ChatStatus::HumanHandoff,
            copilot_draft: None,
            final_response: Some("I am transferring you to a human agent who can assist you further.".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockOmniLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockOmniLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.response_text.clone()),
                response_id: Some("test".to_string()),
                stop_reason: "stop".to_string(),
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_omnichannel_classify_intent() {
        let llm = Arc::new(MockOmniLlm { response_text: "Sales".to_string() });
        let engine = OmnichannelChatEngine::new(llm);
        let intent = engine.classify_intent("How much is this?").await.unwrap();
        assert_eq!(intent, ChatIntent::Sales);
    }

    #[tokio::test]
    async fn test_omnichannel_auto_respond() {
        let llm = Arc::new(MockOmniLlm { response_text: "Here is your info.".to_string() });
        let engine = OmnichannelChatEngine::new(llm);
        let state = engine.auto_respond("Hello", &ChatIntent::Support).await.unwrap();
        assert_eq!(state.status, ChatStatus::BotHandled);
        assert_eq!(state.final_response.unwrap(), "Here is your info.");
        assert_eq!(state.current_intent.unwrap(), ChatIntent::Support);
    }

    #[tokio::test]
    async fn test_omnichannel_urgent_auto_handoff() {
        let llm = Arc::new(MockOmniLlm { response_text: "Should not be called".to_string() });
        let engine = OmnichannelChatEngine::new(llm);
        let state = engine.auto_respond("HELP!", &ChatIntent::Urgent).await.unwrap();
        assert_eq!(state.status, ChatStatus::HumanHandoff);
        assert!(state.final_response.unwrap().contains("transferring you to a human"));
    }

    #[tokio::test]
    async fn test_omnichannel_draft_copilot() {
        let llm = Arc::new(MockOmniLlm { response_text: "Draft response".to_string() });
        let engine = OmnichannelChatEngine::new(llm);
        let state = engine.draft_copilot_response("Question", &ChatIntent::General).await.unwrap();
        assert_eq!(state.status, ChatStatus::HumanHandoff);
        assert_eq!(state.copilot_draft.unwrap(), "Draft response");
        assert!(state.final_response.is_none());
    }
}
