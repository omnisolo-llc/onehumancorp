use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCustomerGraph {
    pub tenant_id: String,
    pub customer_id: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub instagram_handle: Option<String>,
    pub whatsapp_number: Option<String>,
    pub past_orders: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingMessage {
    pub tenant_id: String,
    pub source_channel: String, // "instagram", "whatsapp", "email", "sms"
    pub sender_id: String,
    pub message_content: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequiredDraft {
    pub tenant_id: String,
    pub draft_id: String,
    pub customer_id: String,
    pub original_message: IncomingMessage,
    pub drafted_reply: String,
    pub status: String, // "pending_approval", "approved", "rejected"
}
