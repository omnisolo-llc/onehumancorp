use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    WebWidget,
    WhatsApp,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel_type: ChannelType,
    pub config: serde_json::Value,
}
