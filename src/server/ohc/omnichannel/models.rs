#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inbox {
    pub tenant_id: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelConnection {
    pub tenant_id: String,
    pub id: String,
    pub inbox_id: String,
    pub provider: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub tenant_id: String,
    pub id: String,
    pub name: String,
    pub identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conversation {
    pub tenant_id: String,
    pub id: String,
    pub channel_id: String,
    pub contact_id: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub tenant_id: String,
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: MessageType,
    pub receipt_status: ReceiptStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Inbound,
    Outbound,
    PrivateNote,
    AgentDraft,
    SystemEvent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}
