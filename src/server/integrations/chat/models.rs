use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatIntent {
    Support,
    Sales,
    Billing,
    General,
    Escalation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_type: String, // "customer", "agent", "bot"
    pub content: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationState {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub status: String, // "open", "resolved", "snoozed", "bot"
    pub assigned_agent_id: Option<String>,
    pub intent: Option<ChatIntent>,
}
