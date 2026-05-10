use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManychatMessage {
    pub subscriber_id: String,
    pub message_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManychatWebhookEvent {
    pub subscriber_id: String,
    pub type_: String,
    pub data: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManychatOauthCallback {
    pub code: String,
    pub state: Option<String>,
}

pub struct ManychatClient {
    pub api_key: String,
    pub http_client: Client,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_message(&self, subscriber_id: &str, text: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent";
        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "subscriber_id": subscriber_id,
                "data": {
                    "version": "v2",
                    "content": {
                        "messages": [
                            {
                                "type": "text",
                                "text": text
                            }
                        ]
                    }
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manychat_client_creation() {
        let client = ManychatClient::new("test_api_key".to_string());
        assert_eq!(client.api_key, "test_api_key");
    }

    #[test]
    fn test_manychat_webhook_event_parsing() {
        let json_payload = r#"{
            "subscriber_id": "12345",
            "type_": "message",
            "data": { "text": "Hello Manychat" }
        }"#;

        let event: ManychatWebhookEvent = serde_json::from_str(json_payload).unwrap();
        assert_eq!(event.subscriber_id, "12345");
        assert_eq!(event.type_, "message");
        assert_eq!(event.data["text"], "Hello Manychat");
    }
}
