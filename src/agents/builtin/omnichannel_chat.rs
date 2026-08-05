use crate::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message as LlmMessage, Role};
use std::sync::Arc;

/// Intent representing the classification of a user message.
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Support,
    Sales,
    Refund,
    Escalate,
    Unknown,
}

impl Intent {
    pub fn from_str(s: &str) -> Self {
        let s = s.to_lowercase();
        if s.contains("support") || s.contains("help") {
            Intent::Support
        } else if s.contains("sales") || s.contains("buy") || s.contains("pricing") {
            Intent::Sales
        } else if s.contains("refund") || s.contains("money back") {
            Intent::Refund
        } else if s.contains("escalate") || s.contains("human") || s.contains("manager") {
            Intent::Escalate
        } else {
            Intent::Unknown
        }
    }
}

/// The status of a conversation, tracking its lifecycle natively in Rust.
#[derive(Debug, Clone, PartialEq)]
pub enum ConversationStatus {
    Open,
    BotHandled,
    HumanHandled,
    Resolved,
}

/// A message within an omnichannel conversation.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub role: Role,
    pub is_draft: bool,
}

/// A rich representation of a conversation, preserving state natively.
#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    pub id: String,
    pub status: ConversationStatus,
    pub messages: Vec<Message>,
    pub intent: Option<Intent>,
}

impl Conversation {
    pub fn new(id: String) -> Self {
        Self {
            id,
            status: ConversationStatus::Open,
            messages: Vec::new(),
            intent: None,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

/// The engine replicating the core features natively in Rust.
pub struct OmnichannelChatEngine {
    llm: Arc<dyn LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    /// Process an incoming message, classify intent, and decide the next action.
    pub async fn process_incoming_message(
        &self,
        conv: &mut Conversation,
        user_message: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg_id = format!("msg_{}", conv.messages.len() + 1);
        conv.add_message(Message {
            id: msg_id,
            content: user_message.clone(),
            role: Role::User,
            is_draft: false,
        });

        // Intent Classification
        let intent = self.classify_intent(&user_message).await?;
        conv.intent = Some(intent.clone());

        match intent {
            Intent::Escalate | Intent::Refund => {
                // High-touch intents get handed off to human
                self.handoff_to_human(conv);
                // Also draft a copilot response for the human
                self.draft_copilot_response(
                    conv,
                    "This requires human review. Suggested response...",
                )
                .await?;
            }
            _ => {
                // Auto-respond for standard intents
                self.auto_responder(conv).await?;
                conv.status = ConversationStatus::BotHandled;
            }
        }

        Ok(())
    }

    /// Uses the LLM client to parse user intent from text.
    pub async fn classify_intent(
        &self,
        text: &str,
    ) -> Result<Intent, Box<dyn std::error::Error + Send + Sync>> {
        let req = ChatRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            system: "Classify the following text into one of these intents: Support, Sales, Refund, Escalate, Unknown. Reply with ONLY the intent name.".to_string(),
            messages: vec![LlmMessage::user(text.to_string())],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        let res = self.llm.chat(req).await?;
        let content = res.message.content.trim();
        Ok(Intent::from_str(content))
    }

    /// Generates and adds a bot message directly.
    pub async fn auto_responder(
        &self,
        conv: &mut Conversation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let history = conv
            .messages
            .iter()
            .filter(|m| !m.is_draft)
            .map(|m| format!("{:?}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n");

        let req = ChatRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            system: "You are an AI assistant. Reply to the user's latest message based on the conversation history.".to_string(),
            messages: vec![LlmMessage::user(history)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        let res = self.llm.chat(req).await?;

        let msg_id = format!("msg_{}", conv.messages.len() + 1);
        conv.add_message(Message {
            id: msg_id,
            content: res.message.content,
            role: Role::Assistant,
            is_draft: false,
        });

        Ok(())
    }

    /// Generates a draft response for a human agent (is_draft = true).
    pub async fn draft_copilot_response(
        &self,
        conv: &mut Conversation,
        draft_content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg_id = format!("msg_{}_draft", conv.messages.len() + 1);
        conv.add_message(Message {
            id: msg_id,
            content: draft_content.to_string(),
            role: Role::Assistant,
            is_draft: true,
        });

        Ok(())
    }

    /// Updates the ConversationStatus to HumanHandled.
    pub fn handoff_to_human(&self, conv: &mut Conversation) {
        conv.status = ConversationStatus::HumanHandled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message as CoreMessage, Usage};
    use std::sync::Mutex;

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
            let mut resps = self.responses.lock().unwrap();
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Mocked response".to_string()
            };

            Ok(ChatResponse {
                message: CoreMessage::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let llm = Arc::new(MockLlmClient::new(vec!["Support".to_string()]));
        let engine = OmnichannelChatEngine::new(llm);
        let intent = engine
            .classify_intent("Help me with my account")
            .await
            .unwrap();
        assert_eq!(intent, Intent::Support);

        let llm = Arc::new(MockLlmClient::new(vec!["Sales".to_string()]));
        let engine = OmnichannelChatEngine::new(llm);
        let intent = engine
            .classify_intent("How much does this cost?")
            .await
            .unwrap();
        assert_eq!(intent, Intent::Sales);
    }

    #[tokio::test]
    async fn test_auto_responder() {
        let llm = Arc::new(MockLlmClient::new(vec!["Here is your answer".to_string()]));
        let engine = OmnichannelChatEngine::new(llm);
        let mut conv = Conversation::new("conv_1".to_string());

        let res = engine.auto_responder(&mut conv).await;
        assert!(res.is_ok());

        assert_eq!(conv.messages.len(), 1);
        let msg = &conv.messages[0];
        assert_eq!(msg.content, "Here is your answer");
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.is_draft, false);
    }

    #[tokio::test]
    async fn test_draft_copilot_response() {
        let llm = Arc::new(MockLlmClient::new(vec![]));
        let engine = OmnichannelChatEngine::new(llm);
        let mut conv = Conversation::new("conv_1".to_string());

        let res = engine
            .draft_copilot_response(&mut conv, "Draft response text")
            .await;
        assert!(res.is_ok());

        assert_eq!(conv.messages.len(), 1);
        let msg = &conv.messages[0];
        assert_eq!(msg.content, "Draft response text");
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.is_draft, true);
    }

    #[tokio::test]
    async fn test_handoff_to_human() {
        let llm = Arc::new(MockLlmClient::new(vec![]));
        let engine = OmnichannelChatEngine::new(llm);
        let mut conv = Conversation::new("conv_1".to_string());

        engine.handoff_to_human(&mut conv);
        assert_eq!(conv.status, ConversationStatus::HumanHandled);
    }

    #[tokio::test]
    async fn test_process_incoming_message_support() {
        let llm = Arc::new(MockLlmClient::new(vec![
            "Support".to_string(),       // For intent classification
            "Auto response".to_string(), // For auto responder
        ]));
        let engine = OmnichannelChatEngine::new(llm);
        let mut conv = Conversation::new("conv_1".to_string());

        let res = engine
            .process_incoming_message(&mut conv, "Help me".to_string())
            .await;
        assert!(res.is_ok());

        assert_eq!(conv.intent, Some(Intent::Support));
        assert_eq!(conv.status, ConversationStatus::BotHandled);
        assert_eq!(conv.messages.len(), 2);

        assert_eq!(conv.messages[0].role, Role::User);
        assert_eq!(conv.messages[0].content, "Help me");

        assert_eq!(conv.messages[1].role, Role::Assistant);
        assert_eq!(conv.messages[1].content, "Auto response");
        assert_eq!(conv.messages[1].is_draft, false);
    }

    #[tokio::test]
    async fn test_process_incoming_message_escalate() {
        let llm = Arc::new(MockLlmClient::new(vec![
            "Escalate".to_string(), // For intent classification
        ]));
        let engine = OmnichannelChatEngine::new(llm);
        let mut conv = Conversation::new("conv_1".to_string());

        let res = engine
            .process_incoming_message(&mut conv, "Talk to human".to_string())
            .await;
        assert!(res.is_ok());

        assert_eq!(conv.intent, Some(Intent::Escalate));
        assert_eq!(conv.status, ConversationStatus::HumanHandled);
        assert_eq!(conv.messages.len(), 2);

        assert_eq!(conv.messages[0].role, Role::User);
        assert_eq!(conv.messages[0].content, "Talk to human");

        assert_eq!(conv.messages[1].role, Role::Assistant);
        assert_eq!(
            conv.messages[1].content,
            "This requires human review. Suggested response..."
        );
        assert_eq!(conv.messages[1].is_draft, true);
    }
}
