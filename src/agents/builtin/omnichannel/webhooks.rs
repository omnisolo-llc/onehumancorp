use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event: String,
    pub account_id: String,
    // Add other relevant fields for webhook payload
}

pub struct WebhookDispatcher {
    pub endpoints: Vec<String>,
}

impl WebhookDispatcher {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
        }
    }

    pub async fn dispatch(&self, _event: WebhookEvent) -> Result<(), String> {
        // Implement HTTP POST to registered endpoints
        Ok(())
    }
}
