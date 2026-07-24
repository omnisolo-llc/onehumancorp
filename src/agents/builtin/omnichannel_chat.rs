use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chatwoot Retirement & Custom Rust Omnichannel Chat System Standard
/// Native AI auto-responder, copilot response drafting, intent classification, and human agent handoff features in Rust.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Intent {
    Support,
    Sales,
    Billing,
    HandoffRequest,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub role: String, // "customer", "agent", "bot"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationState {
    pub conversation_id: String,
    pub messages: Vec<ChatMessage>,
    pub requires_handoff: bool,
    pub assigned_agent: Option<String>,
}

pub struct NativeOmnichannelEngine {
    conversations: HashMap<String, ConversationState>,
}

impl NativeOmnichannelEngine {
    pub fn new() -> Self {
        Self {
            conversations: HashMap::new(),
        }
    }

    pub fn classify_intent(message: &str) -> Intent {
        let msg = message.to_lowercase();
        if msg.contains("human") || msg.contains("agent") || msg.contains("representative") {
            Intent::HandoffRequest
        } else if msg.contains("buy") || msg.contains("price") || msg.contains("quote") {
            Intent::Sales
        } else if msg.contains("invoice") || msg.contains("charge") || msg.contains("refund") {
            Intent::Billing
        } else if msg.contains("help") || msg.contains("broken") || msg.contains("issue") {
            Intent::Support
        } else {
            Intent::Unknown
        }
    }

    pub fn draft_copilot_response(_conversation: &ConversationState, intent: &Intent) -> String {
        match intent {
            Intent::Sales => "Copilot Draft: Here is our pricing page: [Link]. Would you like a custom quote?".to_string(),
            Intent::Billing => "Copilot Draft: I can help you with your invoice. Could you provide the invoice number?".to_string(),
            Intent::Support => "Copilot Draft: I'm sorry to hear you're having an issue. Let me troubleshoot this for you.".to_string(),
            Intent::HandoffRequest => "Copilot Draft: I am transferring you to a human agent right now.".to_string(),
            Intent::Unknown => "Copilot Draft: I'm an AI assistant. How can I help you today?".to_string(),
        }
    }

    pub fn auto_respond(&mut self, conversation_id: &str, customer_message: &str) -> String {
        let state = self.conversations.entry(conversation_id.to_string()).or_insert_with(|| ConversationState {
            conversation_id: conversation_id.to_string(),
            messages: Vec::new(),
            requires_handoff: false,
            assigned_agent: None,
        });

        state.messages.push(ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: customer_message.to_string(),
            role: "customer".to_string(),
        });

        if state.requires_handoff {
            return "A human agent will be with you shortly.".to_string();
        }

        let intent = Self::classify_intent(customer_message);

        if intent == Intent::HandoffRequest {
            state.requires_handoff = true;
            let response = "I understand you want to speak with a human. I am handing you off to our support team now.".to_string();
            state.messages.push(ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                content: response.clone(),
                role: "bot".to_string(),
            });
            return response;
        }

        let response = Self::draft_copilot_response(state, &intent);
        // Clean up copilot draft prefix for actual auto-responder
        let auto_reply = response.replace("Copilot Draft: ", "");

        state.messages.push(ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: auto_reply.clone(),
            role: "bot".to_string(),
        });

        auto_reply
    }

    pub fn execute_human_handoff(&mut self, conversation_id: &str, agent_id: &str) -> Result<(), String> {
        if let Some(state) = self.conversations.get_mut(conversation_id) {
            state.requires_handoff = false;
            state.assigned_agent = Some(agent_id.to_string());
            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_classification() {
        assert_eq!(NativeOmnichannelEngine::classify_intent("I need to talk to a human"), Intent::HandoffRequest);
        assert_eq!(NativeOmnichannelEngine::classify_intent("What is the price of this item?"), Intent::Sales);
        assert_eq!(NativeOmnichannelEngine::classify_intent("Please refund my last charge"), Intent::Billing);
        assert_eq!(NativeOmnichannelEngine::classify_intent("The system is broken"), Intent::Support);
        assert_eq!(NativeOmnichannelEngine::classify_intent("Hello there"), Intent::Unknown);
    }

    #[test]
    fn test_auto_responder_and_handoff() {
        let mut engine = NativeOmnichannelEngine::new();
        let conv_id = "conv_123";

        let reply1 = engine.auto_respond(conv_id, "What is the price?");
        assert!(reply1.contains("pricing page"));

        let reply2 = engine.auto_respond(conv_id, "I need to talk to a human");
        assert!(reply2.contains("handing you off"));

        let state = engine.conversations.get(conv_id).unwrap();
        assert!(state.requires_handoff);
        assert_eq!(state.messages.len(), 4);

        let reply3 = engine.auto_respond(conv_id, "Hello?");
        assert_eq!(reply3, "A human agent will be with you shortly.");

        assert!(engine.execute_human_handoff(conv_id, "agent_x").is_ok());
        let state = engine.conversations.get(conv_id).unwrap();
        assert!(!state.requires_handoff);
        assert_eq!(state.assigned_agent, Some("agent_x".to_string()));
    }
}
