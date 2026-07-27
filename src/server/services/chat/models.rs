use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
<<<<<<< HEAD
=======
    pub working_hours_enabled: Option<bool>,
    pub out_of_office_message: Option<String>,
    pub greeting_enabled: Option<bool>,
    pub greeting_message: Option<String>,
>>>>>>> 262e0889a (Research Report: Implement Custom Rust Omnichannel Chat System (#35333))
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
<<<<<<< HEAD
=======
    pub bot_assignee_id: Option<Uuid>,
>>>>>>> 262e0889a (Research Report: Implement Custom Rust Omnichannel Chat System (#35333))
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
