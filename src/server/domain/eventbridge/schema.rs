use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhcEvent {
    pub event_id: Uuid,
    pub tenant_id: Uuid,
    pub source: String,
    pub event_type: String,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentFeedItemStatus {
    PendingApproval,
    Approved,
    Discarded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFeedItem {
    pub item_id: Uuid,
    pub tenant_id: Uuid,
    pub agent_type: String,
    pub status: AgentFeedItemStatus,
    pub action_payload: Value,
    pub created_at: DateTime<Utc>,
}
