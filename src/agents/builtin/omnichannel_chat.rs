use crate::types::{ChatRequest, ChatResponse, Message, ToolError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// LlmClientForChat abstracts the LLM needed for omnichannel operations.
#[async_trait]
pub trait LlmClientForChat: Send + Sync {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Intent {
    Support,
    Sales,
    General,
    EscalateToHuman,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub sender: String,
}

/// Native Rust Omnichannel Chat Engine
/// Replaces external Chatwoot dependency by implementing:
/// 1. Intent Classification
/// 2. Copilot response drafting
/// 3. Auto Responder
/// 4. Human Agent Handoff
pub struct OmnichannelChatEngine {
    pub llm: Arc<dyn LlmClientForChat>,
    pub model: String,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClientForChat>, model: String) -> Self {
        Self { llm, model }
    }

    /// 1. Intent Classification
    pub async fn classify_intent(
        &self,
        message: &str,
    ) -> Result<Intent, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = "You are an intent classifier for a customer chat system. \
                             Respond with ONLY ONE word representing the intent: \
                             Support, Sales, General, or Escalate. \
                             If the user is very angry or explicitly asks for a human, output Escalate.";
        let req = ChatRequest {
            model: self.model.clone(),
            system: String::new(),
            messages: vec![
                Message::system(system_prompt.to_string()),
                Message::user(message.to_string()),
            ],
            temperature: 0.0,
            max_tokens: 100,
            tools: vec![],
        };

        let response = self.llm.chat(req).await?;
        let output = response.message.content.trim().to_lowercase();

        match output.as_str() {
            "support" => Ok(Intent::Support),
            "sales" => Ok(Intent::Sales),
            "general" => Ok(Intent::General),
            "escalate" => Ok(Intent::EscalateToHuman),
            _ => Ok(Intent::Unknown),
        }
    }

    /// 2. Copilot Drafting
    pub async fn draft_response(
        &self,
        history: &[ChatMessage],
        context_data: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut messages = vec![Message::system(format!(
            "You are an AI Copilot for customer support. \
             Draft a polite, helpful response based on the chat history. \
             Here is some internal context: {}",
            context_data
        ))];

        for chat in history {
            if chat.sender == "agent" {
                messages.push(Message::assistant(chat.content.clone()));
            } else {
                messages.push(Message::user(chat.content.clone()));
            }
        }

        let req = ChatRequest {
            model: self.model.clone(),
            system: String::new(),
            messages,
            temperature: 0.0,
            max_tokens: 100,
            tools: vec![],
        };

        let response = self.llm.chat(req).await?;
        Ok(response.message.content)
    }

    /// 3 & 4. Auto Responder with Human Agent Handoff
    pub async fn auto_respond(
        &self,
        message: &str,
    ) -> Result<String, ToolError> {
        let intent = self.classify_intent(message).await.unwrap_or(Intent::Unknown);

        if intent == Intent::EscalateToHuman {
            return Err(ToolError::HandoffRequested(
                "Human intervention required based on intent classification.".to_string(),
            ));
        }

        let history = vec![ChatMessage {
            content: message.to_string(),
            sender: "customer".to_string(),
        }];

        // Generate auto reply based on intent
        let context = match intent {
            Intent::Support => "Help the user troubleshoot their issue.",
            Intent::Sales => "Highlight our pricing tiers and offer a demo.",
            _ => "Answer the general query politely.",
        };

        match self.draft_response(&history, context).await {
            Ok(draft) => Ok(draft),
            Err(e) => Err(ToolError::Unexpected(format!("Failed to draft response: {}", e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Usage;

    struct MockLlm {
        response_text: String,
    }

    #[async_trait]
    impl LlmClientForChat for MockLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.response_text.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_classify_intent_support() {
        let llm = Arc::new(MockLlm {
            response_text: "Support".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let intent = engine.classify_intent("My app is crashing").await.unwrap();
        assert_eq!(intent, Intent::Support);
    }

    #[tokio::test]
    async fn test_classify_intent_escalate() {
        let llm = Arc::new(MockLlm {
            response_text: "Escalate".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let intent = engine.classify_intent("I demand to speak to a manager!").await.unwrap();
        assert_eq!(intent, Intent::EscalateToHuman);
    }

    #[tokio::test]
    async fn test_draft_response() {
        let llm = Arc::new(MockLlm {
            response_text: "Here is your drafted reply.".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let history = vec![ChatMessage {
            content: "Hi".to_string(),
            sender: "customer".to_string(),
        }];
        let draft = engine.draft_response(&history, "Be nice").await.unwrap();
        assert_eq!(draft, "Here is your drafted reply.");
    }

    #[tokio::test]
    async fn test_auto_respond_success() {
        let llm = Arc::new(MockLlm {
            response_text: "Support".to_string(), // Classify as support
        });
        // In this mock, the drafted response will also be "Support" since MockLlm returns a fixed response.
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let response = engine.auto_respond("Help me").await.unwrap();
        assert_eq!(response, "Support");
    }

    #[tokio::test]
    async fn test_auto_respond_handoff() {
        let llm = Arc::new(MockLlm {
            response_text: "Escalate".to_string(), // Classify as escalate
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let result = engine.auto_respond("I'm angry").await;

        match result {
            Err(ToolError::HandoffRequested(msg)) => {
                assert!(msg.contains("Human intervention required"));
            }
            _ => panic!("Expected HandoffRequested error"),
        }
    }
}
