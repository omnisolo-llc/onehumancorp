use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub provider_type: String, // e.g., "whatsapp", "web_widget"
    pub config: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String, // e.g., "open", "resolved", "snoozed"
    pub assignee_id: Option<Uuid>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub channel_id: Option<Uuid>,
    pub content: String,
    pub message_type: String, // e.g., "incoming", "outgoing", "template"
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Channel Abstraction Trait
#[async_trait::async_trait]
pub trait ChannelAdapter: Send + Sync {
    /// Identifier for the provider (e.g. "whatsapp")
    fn provider_type(&self) -> &str;

    /// Receive an incoming message from the provider
    async fn receive_message(&self, payload: Value) -> Result<Message, Box<dyn std::error::Error + Send + Sync>>;

    /// Send an outgoing message via the provider
    async fn send_message(&self, message: &Message, channel: &Channel) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
