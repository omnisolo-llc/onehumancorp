use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Channel {
    pub id: Uuid,
    pub provider_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Conversation {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Message {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
