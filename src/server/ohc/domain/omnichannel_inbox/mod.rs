use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChannelConnection {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub provider_type: String,
    pub capabilities: sqlx::types::Json<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub mod repository;
pub mod service;
pub mod api;
pub mod api_tests;
