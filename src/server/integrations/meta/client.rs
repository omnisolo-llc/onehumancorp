use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaMessage {
    pub mid: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboundMessage {
    pub platform: String, // "instagram", "whatsapp", "messenger"
    pub sender_id: String,
    pub recipient_id: String,
    pub text: String,
    pub timestamp: i64,
}

pub struct MetaClient {
    pub access_token: String,
    http_client: Client,
    base_url: String,
}

impl MetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            base_url: "https://graph.facebook.com/v19.0".to_string(),
        }
    }

    pub async fn send_message(&self, recipient_id: &str, text: &str, platform: &str, tenant_id: &str) -> Result<(), String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            &format!("meta_send_{}", platform),
            0.01
        ).await;

        let url = format!("{}/me/messages", self.base_url);
        let payload = serde_json::json!({
            "recipient": { "id": recipient_id },
            "message": { "text": text }
        });

        let res = self.http_client.post(&url)
            .query(&[("access_token", &self.access_token)])
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => Err(format!("Meta API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub fn normalize_webhook_payload(&self, payload: serde_json::Value) -> Vec<InboundMessage> {
        let mut messages = Vec::new();
        if let Some(entries) = payload.get("entry").and_then(|e| e.as_array()) {
            for entry in entries {
                if let Some(messaging) = entry.get("messaging").and_then(|m| m.as_array()) {
                    for msg_event in messaging {
                         if let (Some(sender), Some(message)) = (msg_event.get("sender"), msg_event.get("message")) {
                             if let (Some(sender_id), Some(text)) = (sender.get("id").and_then(|id| id.as_str()), message.get("text").and_then(|t| t.as_str())) {
                                 messages.push(InboundMessage {
                                     platform: "messenger".to_string(),
                                     sender_id: sender_id.to_string(),
                                     recipient_id: "me".to_string(),
                                     text: text.to_string(),
                                     timestamp: chrono::Utc::now().timestamp(),
                                 });
                             }
                         }
                    }
                }
            }
        }
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meta_client_normalization() {
        let client = MetaClient::new("test_token".to_string());
        let payload = serde_json::json!({
            "entry": [{
                "messaging": [{
                    "sender": {"id": "user123"},
                    "message": {"text": "hello"}
                }]
            }]
        });
        let msgs = client.normalize_webhook_payload(payload);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].sender_id, "user123");
        assert_eq!(msgs[0].text, "hello");
    }
}
