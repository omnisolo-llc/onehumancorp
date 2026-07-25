use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OmnichannelInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OmnichannelChannel {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
    pub channel_type: String,
    pub config: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OmnichannelConversation {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OmnichannelMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub tenant_id: Uuid,
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
