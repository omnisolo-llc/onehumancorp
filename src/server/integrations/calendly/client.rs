use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendlyEventType {
    pub uri: String,
    pub name: String,
    pub scheduling_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendlyWebhookPayload {
    pub event: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendlyInvitee {
    pub email: String,
    pub name: String,
    pub uri: String,
}

pub struct CalendlyClient {
    pub api_key: String,
    pub http_client: Client,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_event_types(&self, user_uri: &str) -> Result<Vec<CalendlyEventType>, String> {
        let url = format!("https://api.calendly.com/event_types?user={}", user_uri);
        let res = self.http_client.get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    let collection = body["collection"].as_array();
                    if let Some(items) = collection {
                        let mut event_types = Vec::new();
                        for item in items {
                            if let Ok(event_type) = serde_json::from_value(item.clone()) {
                                event_types.push(event_type);
                            }
                        }
                        Ok(event_types)
                    } else {
                        Ok(Vec::new())
                    }
                } else {
                    Err(format!("Calendly API error: {}", resp.status()))
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
    fn test_calendly_client_creation() {
        let client = CalendlyClient::new("test_api_key".to_string());
        assert_eq!(client.api_key, "test_api_key");
    }

    #[test]
    fn test_calendly_webhook_payload_parsing() {
        let json_payload = r#"{
            "event": "invitee.created",
            "payload": {
                "email": "test@example.com",
                "name": "John Doe",
                "uri": "https://api.calendly.com/invitees/123"
            }
        }"#;

        let event: CalendlyWebhookPayload = serde_json::from_str(json_payload).unwrap();
        assert_eq!(event.event, "invitee.created");
        let invitee: CalendlyInvitee = serde_json::from_value(event.payload).unwrap();
        assert_eq!(invitee.email, "test@example.com");
        assert_eq!(invitee.name, "John Doe");
    }
}
