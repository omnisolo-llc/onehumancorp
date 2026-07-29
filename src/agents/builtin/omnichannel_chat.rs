use ohc_builtin_agent_core::types::{ChatRequest, Message};
use ohc_builtin_agent_llm::LlmClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPayload {
    pub id: String,
    pub status: String,
    pub inbox_id: String,
    pub account_id: String,
    pub assignee_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub id: String,
    pub content: String,
    pub message_type: String, // "incoming", "outgoing"
    pub conversation_id: String,
    pub sender_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreatedEvent {
    pub event: String, // "message_created"
    pub message: MessagePayload,
    pub conversation: ConversationPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStatusChangedEvent {
    pub event: String, // "conversation_status_changed"
    pub conversation: ConversationPayload,
    pub changed_attributes: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Intent {
    Support,
    Sales,
    Billing,
    Handoff,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BotAction {
    AutoResponse(String),
    DraftCopilot(String),
    HandoffToHuman,
    NoAction,
}

pub struct ChatEngine {
    pub llm_client: Arc<dyn LlmClient>,
    pub system_prompt: String,
    pub model: String,
}

impl ChatEngine {
    pub fn new(llm_client: Arc<dyn LlmClient>, model: String, system_prompt: String) -> Self {
        Self {
            llm_client,
            model,
            system_prompt,
        }
    }

    /// Entry point for message_created webhooks
    pub async fn handle_message_created(&self, event: &MessageCreatedEvent) -> Result<BotAction, Box<dyn std::error::Error + Send + Sync>> {
        // We only respond to incoming messages
        if event.message.message_type != "incoming" {
            return Ok(BotAction::NoAction);
        }

        let intent = self.classify_intent(&event.message.content).await?;

        match intent {
            Intent::Handoff => {
                self.handoff_to_human(event).await
            }
            Intent::Support | Intent::Sales | Intent::Billing => {
                // Determine whether to draft or auto-respond based on some logic.
                // For this harness, let's let the LLM decide or we can default to drafting a copilot response.
                self.draft_copilot_response(event).await
            }
            Intent::Unknown => {
                // If unknown, fallback to a standard auto-response or handoff.
                Ok(BotAction::HandoffToHuman)
            }
        }
    }

    /// Classify the intent of the user message
    pub async fn classify_intent(&self, message_content: &str) -> Result<Intent, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "Classify the intent of the following customer message into one of these categories: Support, Sales, Billing, Handoff, Unknown.\n\nMessage: {}\n\nRespond with ONLY the category name.",
            message_content
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an intent classification engine.".to_string(),
            messages: vec![Message::user(&prompt)],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        let response = self.llm_client.chat(req).await?;
        let output = response.message.content.trim().to_lowercase();

        if output.contains("support") {
            Ok(Intent::Support)
        } else if output.contains("sales") {
            Ok(Intent::Sales)
        } else if output.contains("billing") {
            Ok(Intent::Billing)
        } else if output.contains("handoff") {
            Ok(Intent::Handoff)
        } else {
            Ok(Intent::Unknown)
        }
    }

    /// Drafts a copilot response for the agent to review
    pub async fn draft_copilot_response(&self, event: &MessageCreatedEvent) -> Result<BotAction, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "Draft a helpful response for the following customer message.\n\nMessage: {}",
            event.message.content
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: self.system_prompt.clone(),
            messages: vec![Message::user(&prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        let response = self.llm_client.chat(req).await?;
        Ok(BotAction::DraftCopilot(response.message.content))
    }

    /// Auto response for simple queries
    pub async fn auto_respond(&self, event: &MessageCreatedEvent) -> Result<BotAction, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "Write a concise auto-response to the following customer message.\n\nMessage: {}",
            event.message.content
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: self.system_prompt.clone(),
            messages: vec![Message::user(&prompt)],
            tools: vec![],
            max_tokens: 200,
            temperature: 0.3,
        };

        let response = self.llm_client.chat(req).await?;
        Ok(BotAction::AutoResponse(response.message.content))
    }

    /// Hand off the conversation to a human agent
    pub async fn handoff_to_human(&self, _event: &MessageCreatedEvent) -> Result<BotAction, Box<dyn std::error::Error + Send + Sync>> {
        // In a real system, this would make an API call to change the conversation status/assignee.
        // Here we just return the BotAction.
        Ok(BotAction::HandoffToHuman)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    fn mock_event(content: &str, msg_type: &str) -> MessageCreatedEvent {
        MessageCreatedEvent {
            event: "message_created".to_string(),
            message: MessagePayload {
                id: "msg_1".to_string(),
                content: content.to_string(),
                message_type: msg_type.to_string(),
                conversation_id: "conv_1".to_string(),
                sender_id: None,
            },
            conversation: ConversationPayload {
                id: "conv_1".to_string(),
                status: "open".to_string(),
                inbox_id: "inbox_1".to_string(),
                account_id: "acc_1".to_string(),
                assignee_id: None,
                metadata: None,
            },
        }
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Support".to_string(),
                "Sales".to_string(),
                "Billing".to_string(),
                "Handoff".to_string(),
                "Gibberish".to_string(),
            ]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());

        assert_eq!(engine.classify_intent("Help me").await.unwrap(), Intent::Support);
        assert_eq!(engine.classify_intent("Buy").await.unwrap(), Intent::Sales);
        assert_eq!(engine.classify_intent("Invoice").await.unwrap(), Intent::Billing);
        assert_eq!(engine.classify_intent("Human").await.unwrap(), Intent::Handoff);
        assert_eq!(engine.classify_intent("What").await.unwrap(), Intent::Unknown);
    }

    #[tokio::test]
    async fn test_handle_message_created_outgoing_ignored() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());
        let event = mock_event("Hello", "outgoing");

        let action = engine.handle_message_created(&event).await.unwrap();
        assert_eq!(action, BotAction::NoAction);
    }

    #[tokio::test]
    async fn test_handle_message_created_support_draft() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Support".to_string(), // classification
                "This is a draft".to_string(), // drafting
            ]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());
        let event = mock_event("Help with login", "incoming");

        let action = engine.handle_message_created(&event).await.unwrap();
        match action {
            BotAction::DraftCopilot(msg) => assert_eq!(msg, "This is a draft"),
            _ => panic!("Expected DraftCopilot"),
        }
    }

    #[tokio::test]
    async fn test_handle_message_created_handoff() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Handoff".to_string(), // classification
            ]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());
        let event = mock_event("I want to speak to a manager", "incoming");

        let action = engine.handle_message_created(&event).await.unwrap();
        assert_eq!(action, BotAction::HandoffToHuman);
    }

    #[tokio::test]
    async fn test_handle_message_created_unknown() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Blah".to_string(), // classification -> Unknown
            ]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());
        let event = mock_event("asdasdasd", "incoming");

        let action = engine.handle_message_created(&event).await.unwrap();
        assert_eq!(action, BotAction::HandoffToHuman); // Fallback is handoff
    }

    #[tokio::test]
    async fn test_auto_respond() {
        let client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Auto response here".to_string(),
            ]),
        });

        let engine = ChatEngine::new(client, "test-model".to_string(), "sys".to_string());
        let event = mock_event("Question", "incoming");

        let action = engine.auto_respond(&event).await.unwrap();
        match action {
            BotAction::AutoResponse(msg) => assert_eq!(msg, "Auto response here"),
            _ => panic!("Expected AutoResponse"),
        }
    }
}
