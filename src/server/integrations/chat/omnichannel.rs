use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnichannelMessage {
    pub id: String,
    pub channel: String, // "instagram", "whatsapp", "email"
    pub content: String,
    pub sender: String,
}

pub fn handle_incoming_message(msg: OmnichannelMessage) {
    tracing::info!("Received message from channel: {}", msg.channel);
}
