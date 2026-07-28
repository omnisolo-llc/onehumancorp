use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatInbox {
    pub id: i32,
    pub tenant_id: Uuid,
    pub name: String,
    pub enable_auto_assignment: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatContact {
    pub id: i32,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatConversation {
    pub id: i32,
    pub tenant_id: Uuid,
    pub inbox_id: i32,
    pub contact_id: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatMessage {
    pub id: i32,
    pub tenant_id: Uuid,
    pub conversation_id: i32,
    pub content: String,
    pub message_type: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatChannelWhatsapp {
    pub id: i32,
    pub tenant_id: Uuid,
    pub phone_number: String,
    pub provider: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatChannelWebWidget {
    pub id: i32,
    pub tenant_id: Uuid,
    pub website_url: String,
    pub website_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
