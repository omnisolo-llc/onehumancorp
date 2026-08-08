use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: String,
    pub tenant_id: String,
    pub primary_email: Option<String>,
    pub instagram_handle: Option<String>,
    pub whatsapp_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub channel: String,
    pub content: String,
    pub direction: String, // "inbound" or "outbound"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftReply {
    pub message_id: String,
    pub draft_content: String,
    pub status: String, // "pending_approval", "approved", "sent"
}
