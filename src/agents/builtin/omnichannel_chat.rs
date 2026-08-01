use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// External Chatwoot dependencies are 100% RETIRED. The builtin AI agent microservice connects directly
/// via high-performance Rust IPC/gRPC to OHC's native Rust Chat Engine.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Sender {
    Contact {
        id: i32,
        name: String,
        email: Option<String>,
        phone_number: Option<String>,
    },
    User {
        id: i32,
        name: String,
        email: String,
    },
    AgentBot {
        id: i32,
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i32,
    pub display_id: i32,
    pub account: Account,
    pub additional_attributes: Option<serde_json::Value>,
    pub channel: String,
    pub custom_attributes: Option<serde_json::Value>,
    pub inbox_id: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMessage {
    pub id: i32,
    pub account: Account,
    pub additional_attributes: Option<serde_json::Value>,
    pub content_attributes: Option<serde_json::Value>,
    pub content_type: String,
    pub content: String,
    pub conversation: Conversation,
    pub created_at: String,
    pub inbox: Inbox,
    pub message_type: String, // "incoming", "outgoing", "template", etc.
    pub private: bool,
    pub sender: Option<Sender>,
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntentClassification {
    Greeting,
    SupportInquiry,
    SalesInquiry,
    HumanHandoffRequested,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutoResponderAction {
    Reply(String),
    Handoff(String),
    NoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HandoffRequest {
    pub conversation_id: i32,
    pub reason: String,
}

pub struct OmnichannelChatEngine;

impl OmnichannelChatEngine {
    pub fn new() -> Self {
        Self
    }

    /// Process an incoming message and classify its intent.
    pub fn process_incoming_message(
        &self,
        message: &WebhookMessage,
    ) -> IntentClassification {
        let content_lower = message.content.to_lowercase();

        if content_lower.contains("human") || content_lower.contains("agent") || content_lower.contains("help me") {
            return IntentClassification::HumanHandoffRequested;
        }

        if content_lower.contains("buy") || content_lower.contains("price") || content_lower.contains("cost") {
            return IntentClassification::SalesInquiry;
        }

        if content_lower.contains("broken") || content_lower.contains("issue") || content_lower.contains("bug") || content_lower.contains("not working") {
            return IntentClassification::SupportInquiry;
        }

        if content_lower.starts_with("hi") || content_lower.starts_with("hello") || content_lower.starts_with("hey") {
            return IntentClassification::Greeting;
        }

        IntentClassification::Unknown
    }

    /// Automatically responds to the user if the intent can be handled without human intervention.
    pub fn draft_copilot_response(
        &self,
        message: &WebhookMessage,
        intent: &IntentClassification,
    ) -> AutoResponderAction {
        match intent {
            IntentClassification::Greeting => {
                AutoResponderAction::Reply("Hello! How can we help you today?".to_string())
            }
            IntentClassification::SalesInquiry => {
                AutoResponderAction::Reply("Thanks for reaching out! You can view our pricing at https://example.com/pricing. Can I help you with anything else?".to_string())
            }
            IntentClassification::HumanHandoffRequested => {
                self.trigger_human_handoff(message.conversation.id, "User explicitly requested human assistance.")
            }
            IntentClassification::SupportInquiry => {
                AutoResponderAction::Reply("I'm sorry to hear you're experiencing issues. Could you please provide more details? If it's urgent, I can also transfer you to a human agent.".to_string())
            }
            IntentClassification::Unknown => {
                AutoResponderAction::NoAction
            }
        }
    }

    /// Create a handoff request to alert human agents.
    pub fn trigger_human_handoff(&self, conversation_id: i32, reason: &str) -> AutoResponderAction {
        AutoResponderAction::Handoff(format!("Handoff requested for conversation {}: {}", conversation_id, reason))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_message(content: &str) -> WebhookMessage {
        WebhookMessage {
            id: 1,
            account: Account { id: 1, name: "Test Account".to_string() },
            additional_attributes: None,
            content_attributes: None,
            content_type: "text".to_string(),
            content: content.to_string(),
            conversation: Conversation {
                id: 101,
                display_id: 101,
                account: Account { id: 1, name: "Test Account".to_string() },
                additional_attributes: None,
                channel: "web_widget".to_string(),
                custom_attributes: None,
                inbox_id: 1,
                status: "open".to_string(),
            },
            created_at: "2023-01-01T00:00:00Z".to_string(),
            inbox: Inbox { id: 1, name: "Test Inbox".to_string() },
            message_type: "incoming".to_string(),
            private: false,
            sender: Some(Sender::Contact {
                id: 1,
                name: "John Doe".to_string(),
                email: Some("john@example.com".to_string()),
                phone_number: None,
            }),
            source_id: None,
        }
    }

    #[test]
    fn test_intent_classification() {
        let engine = OmnichannelChatEngine::new();

        let msg = create_mock_message("Hello there!");
        assert_eq!(engine.process_incoming_message(&msg), IntentClassification::Greeting);

        let msg = create_mock_message("I want to speak to a human please.");
        assert_eq!(engine.process_incoming_message(&msg), IntentClassification::HumanHandoffRequested);

        let msg = create_mock_message("What is the price of this item?");
        assert_eq!(engine.process_incoming_message(&msg), IntentClassification::SalesInquiry);

        let msg = create_mock_message("The app is broken and not working.");
        assert_eq!(engine.process_incoming_message(&msg), IntentClassification::SupportInquiry);

        let msg = create_mock_message("Random nonsense here.");
        assert_eq!(engine.process_incoming_message(&msg), IntentClassification::Unknown);
    }

    #[test]
    fn test_auto_responder() {
        let engine = OmnichannelChatEngine::new();

        let msg = create_mock_message("Hello there!");
        let intent = engine.process_incoming_message(&msg);
        let action = engine.draft_copilot_response(&msg, &intent);

        match action {
            AutoResponderAction::Reply(reply_text) => {
                assert!(reply_text.contains("Hello!"));
            },
            _ => panic!("Expected Reply action"),
        }

        let msg = create_mock_message("Human please");
        let intent = engine.process_incoming_message(&msg);
        let action = engine.draft_copilot_response(&msg, &intent);

        match action {
            AutoResponderAction::Handoff(reason) => {
                assert!(reason.contains("101"));
                assert!(reason.contains("User explicitly requested human assistance"));
            },
            _ => panic!("Expected Handoff action"),
        }
    }
}
