use super::client::MetaClient;
use serde_json::Value;

pub struct MetaWebhookHandler {
    client: MetaClient,
}

impl MetaWebhookHandler {
    pub fn new(access_token: String) -> Self {
        Self {
            client: MetaClient::new(access_token),
        }
    }

    pub async fn handle_request(&self, body: Value, tenant_id: &str) -> Result<(), String> {
        let messages = self.client.normalize_webhook_payload(body);
        for msg in messages {
            tracing::info!("Tenant {}: Normalized Inbound Message: {:?}", tenant_id, msg);
            // In a real implementation, this would emit an event to the department worker
        }
        Ok(())
    }

    pub fn verify_token(&self, hub_mode: &str, hub_challenge: &str, hub_verify_token: &str, expected_token: &str) -> Result<String, String> {
        if hub_mode == "subscribe" && hub_verify_token == expected_token {
            Ok(hub_challenge.to_string())
        } else {
            Err("Verification failed".to_string())
        }
    }
}
