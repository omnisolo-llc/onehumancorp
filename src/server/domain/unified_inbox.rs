use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConversation {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel_provider: String,
    pub channel_identifier: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub intent_metadata: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedActionCard {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub message_id: Option<String>,
    pub action_type: String,
    pub proposed_content: Option<String>,
    pub context_used: Option<serde_json::Value>,
    pub status: String,
    pub created_at: String,
    pub resolved_at: Option<String>,
}
