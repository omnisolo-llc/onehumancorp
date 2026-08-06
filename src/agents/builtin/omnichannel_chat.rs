use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use ohc_builtin_agent_core::types::{ChatRequest, Message as CoreMessage, Role, ToolError};

/// Omnichannel Chat Integration - Chatwoot Retirement
/// Implements native AI auto-responder, copilot response drafting, intent classification, and human agent handoff features in Rust.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Incoming,
    Outgoing,
    Activity,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub identifier: Option<String>,
    pub custom_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub account_id: i64,
    pub inbox_id: i64,
    pub status: ConversationStatus,
    pub assignee_id: Option<i64>,
    pub contact_id: i64,
    pub custom_attributes: HashMap<String, serde_json::Value>,
    pub waiting_since: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub account_id: i64,
    pub conversation_id: i64,
    pub message_type: MessageType,
    pub content: Option<String>,
    pub sender_type: Option<String>,
    pub sender_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub message: Option<Message>,
    pub conversation: Option<Conversation>,
    pub contact: Option<Contact>,
}

/// A simplified adapter trait to allow sending messages back out to the real channel.
#[async_trait::async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn send_message(&self, conversation_id: i64, content: &str) -> Result<(), ToolError>;
    async fn update_conversation_status(&self, conversation_id: i64, status: ConversationStatus, assignee_id: Option<i64>) -> Result<(), ToolError>;
    async fn send_internal_draft(&self, conversation_id: i64, draft_content: &str) -> Result<(), ToolError>;
}

/// Core Chat Engine trait mirroring Chatwoot bot/webhook protocols.
#[async_trait::async_trait]
pub trait ChatEngine: Send + Sync {
    async fn handle_webhook(&self, payload: WebhookPayload) -> Result<(), ToolError>;
    async fn generate_auto_response(&self, message: &Message, conversation: &Conversation) -> Result<Option<String>, ToolError>;
    async fn draft_copilot_response(&self, message: &Message, conversation: &Conversation) -> Result<Option<String>, ToolError>;
    async fn classify_intent(&self, message: &Message) -> Result<String, ToolError>;
    async fn handoff_to_human(&self, conversation_id: i64, reason: &str) -> Result<(), ToolError>;
}

/// A default native implementation for OHC that replaces Chatwoot bots.
pub struct NativeRustChatEngine {
    llm_client: Arc<dyn crate::llm::LlmClient>,
    channel_adapter: Arc<dyn ChannelAdapter>,
}

impl NativeRustChatEngine {
    pub fn new(llm_client: Arc<dyn crate::llm::LlmClient>, channel_adapter: Arc<dyn ChannelAdapter>) -> Self {
        Self { llm_client, channel_adapter }
    }
}

#[async_trait::async_trait]
impl ChatEngine for NativeRustChatEngine {
    async fn handle_webhook(&self, payload: WebhookPayload) -> Result<(), ToolError> {
        if payload.event == "message_created" {
            if let Some(msg) = payload.message {
                if msg.message_type == MessageType::Incoming {
                    // Safe unwrapping logic: only proceed if the conversation exists.
                    if let Some(ref conv) = payload.conversation {
                        // 1. Classify intent
                        let intent = self.classify_intent(&msg).await?;

                        // 2. Draft copilot response (always useful for the human agent)
                        if let Some(draft) = self.draft_copilot_response(&msg, conv).await? {
                            // Send draft as an internal private note/activity
                            self.channel_adapter.send_internal_draft(msg.conversation_id, &draft).await?;
                        }

                        // 3. Decide if auto-responder should fire or handoff
                        if intent == "escalate" || intent == "human_requested" {
                            self.handoff_to_human(msg.conversation_id, "User requested human agent").await?;
                        } else if let Some(auto_resp) = self.generate_auto_response(&msg, conv).await? {
                            // Send the auto response back to the user
                            self.channel_adapter.send_message(msg.conversation_id, &auto_resp).await?;
                        }
                    } else {
                        return Err(ToolError::Unexpected("Conversation missing from payload".to_string()));
                    }
                }
            }
        }
        Ok(())
    }

    async fn generate_auto_response(&self, message: &Message, _conversation: &Conversation) -> Result<Option<String>, ToolError> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI customer service agent. Respond to the customer politely.".to_string(),
            messages: vec![CoreMessage {
                role: Role::User,
                content: message.content.clone().unwrap_or_default(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.7,
        };

        let response = self.llm_client.chat(req).await.map_err(|e| ToolError::Unexpected(e.to_string()))?;
        Ok(Some(response.message.content))
    }

    async fn draft_copilot_response(&self, message: &Message, _conversation: &Conversation) -> Result<Option<String>, ToolError> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI copilot assisting a human agent. Draft a suggested response for the human agent to review.".to_string(),
            messages: vec![CoreMessage {
                role: Role::User,
                content: message.content.clone().unwrap_or_default(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.7,
        };

        let response = self.llm_client.chat(req).await.map_err(|e| ToolError::Unexpected(e.to_string()))?;
        Ok(Some(response.message.content))
    }

    async fn classify_intent(&self, message: &Message) -> Result<String, ToolError> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an intent classifier. Output ONLY the intent string: 'human_requested', 'escalate', or 'general_inquiry'.".to_string(),
            messages: vec![CoreMessage {
                role: Role::User,
                content: message.content.clone().unwrap_or_default(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.1,
        };

        let response = self.llm_client.chat(req).await.map_err(|e| ToolError::Unexpected(e.to_string()))?;
        Ok(response.message.content.trim().to_string())
    }

    async fn handoff_to_human(&self, conversation_id: i64, reason: &str) -> Result<(), ToolError> {
        // We set the status to Open (it might already be Open, but we make sure)
        // and add a private note about the handoff reason.
        // In a real system, assignee_id might be set to a specific human agent.
        self.channel_adapter.update_conversation_status(conversation_id, ConversationStatus::Open, None).await?;
        self.channel_adapter.send_internal_draft(conversation_id, &format!("Handoff initiated: {}", reason)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tokio::sync::Mutex;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockLlmClient {
        response_sequence: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut seq = self.response_sequence.lock().await;
            let content = if !seq.is_empty() { seq.remove(0) } else { "Mocked LLM Response".to_string() };

            Ok(ChatResponse {
                message: CoreMessage {
                    role: Role::Assistant,
                    content,
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

    struct MockChannelAdapter {
        messages_sent: Mutex<Vec<(i64, String)>>,
        drafts_sent: Mutex<Vec<(i64, String)>>,
        status_updates: Mutex<Vec<(i64, ConversationStatus)>>,
    }

    impl MockChannelAdapter {
        fn new() -> Self {
            Self {
                messages_sent: Mutex::new(vec![]),
                drafts_sent: Mutex::new(vec![]),
                status_updates: Mutex::new(vec![]),
            }
        }
    }

    #[async_trait::async_trait]
    impl ChannelAdapter for MockChannelAdapter {
        async fn send_message(&self, conversation_id: i64, content: &str) -> Result<(), ToolError> {
            self.messages_sent.lock().await.push((conversation_id, content.to_string()));
            Ok(())
        }

        async fn update_conversation_status(&self, conversation_id: i64, status: ConversationStatus, _assignee_id: Option<i64>) -> Result<(), ToolError> {
            self.status_updates.lock().await.push((conversation_id, status));
            Ok(())
        }

        async fn send_internal_draft(&self, conversation_id: i64, draft_content: &str) -> Result<(), ToolError> {
            self.drafts_sent.lock().await.push((conversation_id, draft_content.to_string()));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_native_chat_engine_intent_classification() {
        let llm = Arc::new(MockLlmClient {
            response_sequence: Mutex::new(vec!["human_requested".to_string(), "general_inquiry".to_string()]),
        });
        let adapter = Arc::new(MockChannelAdapter::new());
        let engine = NativeRustChatEngine::new(llm, adapter);

        let mut msg = Message {
            id: 1,
            account_id: 1,
            conversation_id: 1,
            message_type: MessageType::Incoming,
            content: Some("I need to speak to a human".to_string()),
            sender_type: Some("Contact".to_string()),
            sender_id: Some(1),
            created_at: Utc::now(),
        };

        let intent = engine.classify_intent(&msg).await.unwrap();
        assert_eq!(intent, "human_requested");

        msg.content = Some("What are your hours?".to_string());
        let intent2 = engine.classify_intent(&msg).await.unwrap();
        assert_eq!(intent2, "general_inquiry");
    }

    #[tokio::test]
    async fn test_native_chat_engine_webhook_routing() {
        let llm = Arc::new(MockLlmClient {
            response_sequence: Mutex::new(vec!["general_inquiry".to_string(), "Draft...".to_string(), "Auto resp...".to_string()]),
        });
        let adapter = Arc::new(MockChannelAdapter::new());
        let engine = NativeRustChatEngine::new(llm, adapter.clone());

        let msg = Message {
            id: 1,
            account_id: 1,
            conversation_id: 1,
            message_type: MessageType::Incoming,
            content: Some("Hello".to_string()),
            sender_type: Some("Contact".to_string()),
            sender_id: Some(1),
            created_at: Utc::now(),
        };

        let conv = Conversation {
            id: 1,
            account_id: 1,
            inbox_id: 1,
            status: ConversationStatus::Open,
            assignee_id: None,
            contact_id: 1,
            custom_attributes: HashMap::new(),
            waiting_since: None,
        };

        let payload = WebhookPayload {
            event: "message_created".to_string(),
            message: Some(msg),
            conversation: Some(conv),
            contact: None,
        };

        let result = engine.handle_webhook(payload).await;
        assert!(result.is_ok());

        let messages = adapter.messages_sent.lock().await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].1, "Auto resp...");

        let drafts = adapter.drafts_sent.lock().await;
        assert_eq!(drafts.len(), 1);
        assert_eq!(drafts[0].1, "Draft...");
    }

    #[tokio::test]
    async fn test_native_chat_engine_webhook_missing_conversation() {
        let llm = Arc::new(MockLlmClient {
            response_sequence: Mutex::new(vec![]),
        });
        let adapter = Arc::new(MockChannelAdapter::new());
        let engine = NativeRustChatEngine::new(llm, adapter);

        let msg = Message {
            id: 1,
            account_id: 1,
            conversation_id: 1,
            message_type: MessageType::Incoming,
            content: Some("Hello".to_string()),
            sender_type: Some("Contact".to_string()),
            sender_id: Some(1),
            created_at: Utc::now(),
        };

        let payload = WebhookPayload {
            event: "message_created".to_string(),
            message: Some(msg),
            conversation: None,
            contact: None,
        };

        let result = engine.handle_webhook(payload).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_chat_engine_escalation() {
        let llm = Arc::new(MockLlmClient {
            response_sequence: Mutex::new(vec!["human_requested".to_string(), "Draft...".to_string()]),
        });
        let adapter = Arc::new(MockChannelAdapter::new());
        let engine = NativeRustChatEngine::new(llm, adapter.clone());

        let msg = Message {
            id: 2,
            account_id: 1,
            conversation_id: 1,
            message_type: MessageType::Incoming,
            content: Some("I want to escalate to an agent".to_string()),
            sender_type: Some("Contact".to_string()),
            sender_id: Some(1),
            created_at: Utc::now(),
        };

        let conv = Conversation {
            id: 1,
            account_id: 1,
            inbox_id: 1,
            status: ConversationStatus::Open,
            assignee_id: None,
            contact_id: 1,
            custom_attributes: HashMap::new(),
            waiting_since: None,
        };

        let payload = WebhookPayload {
            event: "message_created".to_string(),
            message: Some(msg),
            conversation: Some(conv),
            contact: None,
        };

        let result = engine.handle_webhook(payload).await;
        assert!(result.is_ok());

        let status_updates = adapter.status_updates.lock().await;
        assert_eq!(status_updates.len(), 1);
        assert_eq!(status_updates[0].1, ConversationStatus::Open);

        let drafts = adapter.drafts_sent.lock().await;
        assert_eq!(drafts.len(), 2);
        assert_eq!(drafts[0].1, "Draft...");
        assert!(drafts[1].1.contains("Handoff initiated"));
    }
}
