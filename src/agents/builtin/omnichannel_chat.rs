#![allow(clippy::empty_line_after_doc_comments)]
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
use ohc_builtin_agent_llm::LlmClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// OmnichannelChatEngine replaces the external Chatwoot dependency.
/// It provides native Rust implementations for Intent Classification,
/// Auto-responder, Copilot Response Drafting, and Human Agent Handoff.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChatIntent {
    /// Routine query that the AI can confidently answer automatically
    Routine,
    /// Complex query, complaint, or explicit request for a human
    RequiresHuman,
    /// Out of scope or unparseable intent
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResult {
    pub intent: ChatIntent,
    pub confidence: f32,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationState {
    /// AI is fully handling the conversation
    BotManaged,
    /// AI is only drafting responses for human review (Copilot)
    CopilotDrafting,
    /// Handed off to a human, AI should not reply automatically
    HumanAgentAssigned,
}

#[derive(Debug, Clone)]
pub struct OmnichannelContext {
    pub session_id: String,
    pub state: ConversationState,
    pub messages: Vec<Message>,
}

impl OmnichannelContext {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            state: ConversationState::BotManaged,
            messages: Vec::new(),
        }
    }
}

pub struct OmnichannelChatEngine {
    pub llm: Arc<dyn LlmClient>,
    pub model: String,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    /// Process an incoming message and update the context accordingly.
    pub async fn process_incoming_message(
        &self,
        context: &mut OmnichannelContext,
        user_message: &str,
    ) -> Result<Option<String>, String> {
        context.messages.push(Message::user(user_message.to_string()));

        if let ConversationState::HumanAgentAssigned = context.state {
            // Human is handling it, AI should do nothing.
            return Ok(None);
        }

        let intent_result = self.intent_classification(user_message).await?;

        if intent_result.intent == ChatIntent::RequiresHuman {
            self.handoff_to_human(context);
            return Ok(Some(
                "I've transferred this conversation to a human agent. They will be with you shortly."
                    .to_string(),
            ));
        }

        match context.state {
            ConversationState::BotManaged => {
                let response = self.auto_responder(context, user_message).await?;
                context.messages.push(Message::assistant(response.clone()));
                Ok(Some(response))
            }
            ConversationState::CopilotDrafting => {
                let draft = self.draft_copilot_response(context, user_message).await?;
                // In Copilot mode, the message is NOT sent to the user automatically.
                Ok(Some(format!("[DRAFT FOR REVIEW] {}", draft)))
            }
            _ => Ok(None),
        }
    }

    /// Classify the intent of the user's message using the LLM.
    pub async fn intent_classification(&self, message: &str) -> Result<IntentResult, String> {
        let system_prompt = "You are an Intent Classifier for an omnichannel customer support chat.
Classify the user's message into one of these intents:
- 'Routine': Simple queries, FAQs, status checks.
- 'RequiresHuman': Complaints, complex technical issues, or explicit requests for a human agent.
- 'Unknown': Gibberish or completely out of scope.

Respond ONLY with a JSON object in this format:
{\"intent\": \"Routine|RequiresHuman|Unknown\", \"confidence\": 0.0-1.0, \"reasoning\": \"string\"}";

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(message.to_string())],
            tools: vec![],
            max_tokens: 200,
            temperature: 0.0,
        };

        let response = self
            .llm
            .chat(req)
            .await
            .map_err(|e| format!("LLM intent classification failed: {}", e))?;

        let text = response.message.content.trim();
        let json_text = if text.starts_with("```json") {
            text.trim_start_matches("```json").trim_end_matches("```").trim()
        } else {
            text
        };

        #[derive(Deserialize)]
        struct RawIntent {
            intent: String,
            confidence: f32,
            reasoning: String,
        }

        let parsed: RawIntent = serde_json::from_str(json_text)
            .map_err(|e| format!("Failed to parse JSON intent: {}", e))?;

        let intent = match parsed.intent.as_str() {
            "Routine" => ChatIntent::Routine,
            "RequiresHuman" => ChatIntent::RequiresHuman,
            _ => ChatIntent::Unknown,
        };

        Ok(IntentResult {
            intent,
            confidence: parsed.confidence,
            reasoning: parsed.reasoning,
        })
    }

    /// Generate an automatic response to a routine query.
    pub async fn auto_responder(
        &self,
        context: &OmnichannelContext,
        _current_message: &str,
    ) -> Result<String, String> {
        let system_prompt = "You are a helpful customer support AI. Provide a concise and polite answer.";

        // We take the last 5 messages to provide some context, but keep it brief.
        let recent_messages: Vec<Message> = context.messages.iter().rev().take(5).cloned().collect();
        let mut messages = recent_messages;
        messages.reverse(); // put them back in chronological order

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages,
            tools: vec![],
            max_tokens: 500,
            temperature: 0.5,
        };

        let response = self
            .llm
            .chat(req)
            .await
            .map_err(|e| format!("Auto-responder failed: {}", e))?;

        Ok(response.message.content)
    }

    /// Draft a response for a human agent to review and send.
    pub async fn draft_copilot_response(
        &self,
        context: &OmnichannelContext,
        _current_message: &str,
    ) -> Result<String, String> {
        let system_prompt = "You are an AI Copilot assisting a human customer support agent. Draft a suggested reply for the agent to send to the customer. The tone should be professional and empathetic.";

        let recent_messages: Vec<Message> = context.messages.iter().rev().take(5).cloned().collect();
        let mut messages = recent_messages;
        messages.reverse();

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages,
            tools: vec![],
            max_tokens: 500,
            temperature: 0.5,
        };

        let response = self
            .llm
            .chat(req)
            .await
            .map_err(|e| format!("Copilot draft failed: {}", e))?;

        Ok(response.message.content)
    }

    /// Handoff the conversation to a human agent.
    pub fn handoff_to_human(&self, context: &mut OmnichannelContext) {
        context.state = ConversationState::HumanAgentAssigned;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockOmnichannelLlm {
        responses: Mutex<Vec<String>>,
    }

    impl MockOmnichannelLlm {
        fn new(responses: Vec<&str>) -> Self {
            Self {
                responses: Mutex::new(responses.into_iter().map(|s| s.to_string()).collect()),
            }
        }
    }

    #[async_trait::async_trait]
    impl LlmClient for MockOmnichannelLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default mock response".to_string()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                response_id: None,
                stop_reason: "stop".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_intent_classification_routine() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![
            r#"{"intent": "Routine", "confidence": 0.95, "reasoning": "Simple question"}"#
        ]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());

        let result = engine.intent_classification("What are your business hours?").await.unwrap();
        assert_eq!(result.intent, ChatIntent::Routine);
        assert_eq!(result.confidence, 0.95);
    }

    #[tokio::test]
    async fn test_intent_classification_requires_human() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![
            r#"{"intent": "RequiresHuman", "confidence": 0.88, "reasoning": "Explicit request for human"}"#
        ]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());

        let result = engine.intent_classification("I want to speak to a manager right now!").await.unwrap();
        assert_eq!(result.intent, ChatIntent::RequiresHuman);
    }

    #[tokio::test]
    async fn test_process_incoming_message_bot_managed_routine() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![
            r#"{"intent": "Routine", "confidence": 0.9, "reasoning": ""}"#,
            "We are open 9am to 5pm.",
        ]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let mut ctx = OmnichannelContext::new("session-1");

        let response = engine.process_incoming_message(&mut ctx, "hours?").await.unwrap();

        assert_eq!(response, Some("We are open 9am to 5pm.".to_string()));
        assert_eq!(ctx.messages.len(), 2);
        assert_eq!(ctx.messages[1].role, Role::Assistant);
        assert!(matches!(ctx.state, ConversationState::BotManaged));
    }

    #[tokio::test]
    async fn test_process_incoming_message_requires_human_handoff() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![
            r#"{"intent": "RequiresHuman", "confidence": 0.9, "reasoning": ""}"#,
        ]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let mut ctx = OmnichannelContext::new("session-2");

        let response = engine.process_incoming_message(&mut ctx, "human please").await.unwrap();

        assert_eq!(response.unwrap(), "I've transferred this conversation to a human agent. They will be with you shortly.");
        assert!(matches!(ctx.state, ConversationState::HumanAgentAssigned));
    }

    #[tokio::test]
    async fn test_process_incoming_message_copilot_drafting() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![
            r#"{"intent": "Routine", "confidence": 0.9, "reasoning": ""}"#,
            "Hello, here is a drafted response.",
        ]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let mut ctx = OmnichannelContext::new("session-3");
        ctx.state = ConversationState::CopilotDrafting;

        let response = engine.process_incoming_message(&mut ctx, "hello").await.unwrap();

        assert_eq!(response.unwrap(), "[DRAFT FOR REVIEW] Hello, here is a drafted response.");
        // Should not append to ctx.messages as assistant because it's a draft
        assert_eq!(ctx.messages.len(), 1); // Only the user message
    }

    #[tokio::test]
    async fn test_process_incoming_message_human_assigned_ignores() {
        let llm = Arc::new(MockOmnichannelLlm::new(vec![]));
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let mut ctx = OmnichannelContext::new("session-4");
        ctx.state = ConversationState::HumanAgentAssigned;

        let response = engine.process_incoming_message(&mut ctx, "hello").await.unwrap();

        assert_eq!(response, None);
        // User message still gets appended for transcript purposes
        assert_eq!(ctx.messages.len(), 1);
    }
}
