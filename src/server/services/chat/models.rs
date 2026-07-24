use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeChatInbox {
    pub id: String,
    pub tenant_id: String,
    pub channel_type: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeChatContact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeChatConversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub content: String,
    pub is_ai_draft: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}
