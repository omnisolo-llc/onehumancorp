use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaMessage {
    pub id: String,
    pub text: String,
    pub from_id: String,
}

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(&self, recipient_id: &str, text: &str) -> Result<(), String>;
}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(&self, recipient_id: &str, text: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/me/messages?access_token={}", self.access_token);
        let res = self.http_client.post(&url)
            .json(&serde_json::json!({
                "recipient": { "id": recipient_id },
                "message": { "text": text }
            }))
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
