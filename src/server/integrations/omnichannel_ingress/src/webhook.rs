use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaEntry {
    pub id: String,
    pub time: u64,
    pub messaging: Option<Vec<MetaMessaging>>,
    pub changes: Option<Vec<MetaChange>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaMessaging {
    pub sender: MetaId,
    pub recipient: MetaId,
    pub timestamp: u64,
    pub message: Option<MetaMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaChange {
    pub field: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaId {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetaMessage {
    pub mid: String,
    pub text: Option<String>,
}

pub fn parse_meta_payload(payload: &str) -> Result<MetaWebhookPayload, serde_json::Error> {
    serde_json::from_str(payload)
}
