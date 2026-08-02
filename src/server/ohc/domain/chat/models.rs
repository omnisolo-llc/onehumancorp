use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub assignee_id: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
