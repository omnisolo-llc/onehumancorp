use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Intent {
    Query,
    ActionRequest,
    Negotiation,
    Response,
    Broadcast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeammateMessage {
    pub message_id: String,
    pub tenant_id: String,
    pub sender_agent_id: String,
    pub target_department: Option<String>,
    pub target_agent_id: Option<String>,
    pub intent: Intent,
    pub payload: Value,
    pub context_id: Option<String>,
}
