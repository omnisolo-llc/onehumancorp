use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::channel::ChannelType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub organization_id: String,
    pub customer_id: Option<String>,
    pub channel_type: ChannelType,
    pub channel_identifier: String, // e.g., phone number, IG handle
    pub status: ConversationStatus,
    pub ai_handled: bool,
    pub human_takeover: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    Open,
    Resolved,
    RequiresAttention,
}
