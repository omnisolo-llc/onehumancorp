use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub feature_type: String,
    pub action: Option<String>,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}
