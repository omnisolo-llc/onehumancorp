use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub email_address: Option<String>,
    pub greeting_enabled: bool,
    pub greeting_message: Option<String>,
    pub working_hours_enabled: bool,
    pub csat_survey_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub priority: i32,
    pub agent_last_seen_at: Option<DateTime<Utc>>,
    pub contact_last_seen_at: Option<DateTime<Utc>>,
    pub first_reply_created_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub inbox_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub content_type: String,
    pub status: String,
    pub is_private: bool,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
