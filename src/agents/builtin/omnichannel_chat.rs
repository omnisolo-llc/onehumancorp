use crate::llm::LlmClient;
use crate::types::{ChatRequest, Message};
use std::sync::Arc;

/// Intent types for native Rust omnichannel chat routing.
#[derive(Debug, PartialEq, Clone)]
pub enum ChatIntent {
    Support,
    Sales,
    Billing,
    General,
    Unknown,
}

impl std::str::FromStr for ChatIntent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "support" => Ok(ChatIntent::Support),
            "sales" => Ok(ChatIntent::Sales),
            "billing" => Ok(ChatIntent::Billing),
            "general" => Ok(ChatIntent::General),
            _ => Ok(ChatIntent::Unknown),
        }
    }
}

/// The state of an ongoing chat conversation.
#[derive(Debug, PartialEq, Clone)]
pub enum ChatStatus {
    AiHandled,
    HumanHandoffRequested,
}

/// A native Rust Omnichannel Chat Engine to replace third-party Chatwoot integrations.
pub struct OmnichannelChatEngine {
    llm: Arc<dyn LlmClient>,
    model: String,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>, model: &str) -> Self {
        Self {
            llm,
            model: model.to_string(),
        }
    }

    /// Native AI auto-responder: Evaluates if a message can be handled automatically and replies.
    pub async fn ai_auto_responder(&self, context: &[Message]) -> Result<String, String> {
        let mut messages = vec![Message::system(
            "You are a helpful native AI auto-responder. Answer the customer's query \
            directly and concisely based on the conversation context. If the query requires \
            human assistance, explicitly state 'HUMAN_HANDOFF_REQUIRED'."
        )];
        messages.extend_from_slice(context);

        let req = ChatRequest {
            model: self.model.clone(),
            system: "".to_string(), // Injected in messages
            messages,
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.7,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content)
    }

    /// Copilot response drafting: Drafts a suggested response for a human agent.
    pub async fn draft_copilot_response(&self, context: &[Message]) -> Result<String, String> {
        let mut messages = vec![Message::system(
            "You are an AI Copilot assisting a human support agent. Based on the customer's \
            latest message and context, draft a polite, professional, and accurate response \
            that the human agent can review, edit, and send."
        )];
        messages.extend_from_slice(context);

        let req = ChatRequest {
            model: self.model.clone(),
            system: "".to_string(),
            messages,
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.5, // Lower temperature for more consistent drafting
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content)
    }

    /// Intent classification: Classifies the customer's intent using an LLM.
    pub async fn classify_intent(&self, message: &str) -> Result<ChatIntent, String> {
        let prompt = format!(
            "Classify the following customer message into one of these intents: Support, Sales, Billing, General.\n\n\
            Message: {}\n\n\
            Respond ONLY with the intent name.",
            message
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an expert intent classifier. Respond ONLY with a single word representing the intent.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let content = resp.message.content.trim();
        content.parse::<ChatIntent>().map_err(|_| "Failed to parse intent".to_string())
    }

    /// Human agent handoff: Pauses AI processing and flags for human escalation.
    pub fn handoff_to_human(&self, current_status: ChatStatus) -> ChatStatus {
        match current_status {
            ChatStatus::AiHandled => ChatStatus::HumanHandoffRequested,
            ChatStatus::HumanHandoffRequested => ChatStatus::HumanHandoffRequested,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;
    use async_trait::async_trait;

    struct MockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    impl MockLlmClient {
        fn new(responses: Vec<String>) -> Self {
            Self {
                responses: Mutex::new(responses),
            }
        }
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Mock response".to_string()
            };
            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_ai_auto_responder() {
        let llm = Arc::new(MockLlmClient::new(vec!["Here is your automated reply.".to_string()]));
        let engine = OmnichannelChatEngine::new(llm, "test-model");

        let context = vec![Message::user("How do I reset my password?")];
        let response = engine.ai_auto_responder(&context).await.unwrap();

        assert_eq!(response, "Here is your automated reply.");
    }

    #[tokio::test]
    async fn test_draft_copilot_response() {
        let llm = Arc::new(MockLlmClient::new(vec!["Draft: I can help with that...".to_string()]));
        let engine = OmnichannelChatEngine::new(llm, "test-model");

        let context = vec![Message::user("I need a refund.")];
        let response = engine.draft_copilot_response(&context).await.unwrap();

        assert_eq!(response, "Draft: I can help with that...");
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let llm = Arc::new(MockLlmClient::new(vec!["Billing".to_string()]));
        let engine = OmnichannelChatEngine::new(llm, "test-model");

        let intent = engine.classify_intent("Can I get a copy of my invoice?").await.unwrap();
        assert_eq!(intent, ChatIntent::Billing);
    }

    #[tokio::test]
    async fn test_classify_intent_unknown() {
        let llm = Arc::new(MockLlmClient::new(vec!["RandomText".to_string()]));
        let engine = OmnichannelChatEngine::new(llm, "test-model");

        let intent = engine.classify_intent("What is the meaning of life?").await.unwrap();
        assert_eq!(intent, ChatIntent::Unknown);
    }

    #[test]
    fn test_handoff_to_human() {
        let llm = Arc::new(MockLlmClient::new(vec![]));
        let engine = OmnichannelChatEngine::new(llm, "test-model");

        assert_eq!(engine.handoff_to_human(ChatStatus::AiHandled), ChatStatus::HumanHandoffRequested);
        assert_eq!(engine.handoff_to_human(ChatStatus::HumanHandoffRequested), ChatStatus::HumanHandoffRequested);
    }
}
