use super::models::{ChatIntent, ChatMessage, ConversationState};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ChatEngine {
    // In-memory state as cache, backed by Postgres
    conversations: Arc<RwLock<HashMap<String, ConversationState>>>,
    messages: Arc<RwLock<Vec<ChatMessage>>>,
    _db: Option<PgPool>, // underscore to bypass dead_code warning for now, as DB logic isn't fully wired yet
}

impl Default for ChatEngine {
    fn default() -> Self {
        Self::new(None)
    }
}

impl ChatEngine {
    pub fn new(db: Option<PgPool>) -> Self {
        Self {
            conversations: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(Vec::new())),
            _db: db,
        }
    }

    pub fn handle_incoming_message(&self, msg: ChatMessage) -> Result<Option<ChatMessage>, String> {
        let mut convs = self.conversations.write().map_err(|_| "Lock error")?;

        let conv = convs.entry(msg.conversation_id.clone()).or_insert(ConversationState {
            id: msg.conversation_id.clone(),
            tenant_id: msg.tenant_id.clone(),
            customer_id: msg.sender_id.clone(),
            status: "bot".to_string(),
            assigned_agent_id: None,
            intent: None,
        });

        // Add message
        self.messages.write().map_err(|_| "Lock error")?.push(msg.clone());

        // 1. Intent Classification
        if conv.intent.is_none() {
            conv.intent = Some(self.classify_intent(&msg.content));
        }

        // 2. Human Agent Handoff
        if conv.intent == Some(ChatIntent::Escalation) || msg.content.to_lowercase().contains("human") {
            conv.status = "open".to_string();
            // In a real system, we'd trigger an event to OHC Agent Triage here
            return Ok(Some(ChatMessage {
                id: format!("auto-{}", msg.created_at + 1),
                tenant_id: msg.tenant_id.clone(),
                conversation_id: msg.conversation_id.clone(),
                sender_id: "system".to_string(),
                sender_type: "bot".to_string(),
                content: "Transferring you to a human agent...".to_string(),
                created_at: msg.created_at + 1,
            }));
        }

        // 3. AI Auto-Responder
        if conv.status == "bot" {
            let auto_reply = self.generate_auto_response(&msg.content);
            let reply_msg = ChatMessage {
                id: format!("auto-{}", msg.created_at + 1),
                tenant_id: msg.tenant_id.clone(),
                conversation_id: msg.conversation_id.clone(),
                sender_id: "bot".to_string(),
                sender_type: "bot".to_string(),
                content: auto_reply,
                created_at: msg.created_at + 1,
            };
            self.messages.write().map_err(|_| "Lock error")?.push(reply_msg.clone());
            return Ok(Some(reply_msg));
        }

        Ok(None)
    }

    // 4. Copilot Response Drafting
    pub fn draft_copilot_response(&self, conversation_id: &str) -> Result<String, String> {
        let msgs = self.messages.read().map_err(|_| "Lock error")?;

        let last_customer_msg = msgs.iter()
            .filter(|m| m.conversation_id == conversation_id && m.sender_type == "customer")
            .last();

        if let Some(msg) = last_customer_msg {
            Ok(format!("Drafting response for: '{}' - Our team is looking into this right now.", msg.content))
        } else {
            Ok("No conversation history to draft from.".to_string())
        }
    }

    fn classify_intent(&self, content: &str) -> ChatIntent {
        let lower = content.to_lowercase();
        if lower.contains("buy") || lower.contains("price") {
            ChatIntent::Sales
        } else if lower.contains("bill") || lower.contains("invoice") {
            ChatIntent::Billing
        } else if lower.contains("angry") || lower.contains("manager") {
            ChatIntent::Escalation
        } else {
            ChatIntent::Support
        }
    }

    fn generate_auto_response(&self, content: &str) -> String {
        let lower = content.to_lowercase();
        // check word boundary to avoid "this" triggering "hi"
        let words: Vec<&str> = lower.split_whitespace().collect();
        if words.contains(&"hello") || words.contains(&"hi") {
            "Hello! How can I assist you today?".to_string()
        } else if lower.contains("hours") {
            "We are open 9 AM to 5 PM, Monday through Friday.".to_string()
        } else {
            "Thank you for your message. I'm an AI assistant. How can I help?".to_string()
        }
    }
}
