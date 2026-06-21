use async_trait::async_trait;
use reqwest::Client;


#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealMetaClient {
    access_token: String,
    phone_number_id: Option<String>,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            phone_number_id: None,
            http_client: Client::new(),
        }
    }

    pub fn with_phone_number_id(access_token: String, phone_number_id: String) -> Self {
        Self {
            access_token,
            phone_number_id: Some(phone_number_id),
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        let from_id = self.phone_number_id.as_deref().unwrap_or("default");
        let url = match platform {
            "whatsapp" => format!("https://graph.facebook.com/v19.0/{}/messages", from_id),
            _ => format!("https://graph.facebook.com/v19.0/{}/messages", from_id), // Simplified URL mapping
        };

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "text",
            "text": {
                "body": body
            }
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Meta API error: {}", resp.status()))
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
    fn test_real_client_creation() {
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    // Because send_message issues a real network request using reqwest,
    // we omit a full unit test calling it here to prevent external dependencies and network flakes in the test suite.
    // Provider tests cover the mock path.
}
