use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleBusinessProfileClientWrapper: Send + Sync {
    async fn send_message(&self, conversation_id: &str, text: &str) -> Result<String, String>;
    async fn reply_to_review(&self, account_id: &str, location_id: &str, review_id: &str, reply: &str) -> Result<String, String>;
}

pub struct RealGoogleBusinessProfileClient {
    access_token: String,
    http_client: Client,
}

impl RealGoogleBusinessProfileClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl GoogleBusinessProfileClientWrapper for RealGoogleBusinessProfileClient {
    async fn send_message(&self, conversation_id: &str, text: &str) -> Result<String, String> {
        let url = format!("https://businessmessages.googleapis.com/v1/conversations/{}/messages", conversation_id);

        let payload = serde_json::json!({
            "messageId": uuid::Uuid::new_v4().to_string(),
            "representative": {
                "representativeType": "BOT"
            },
            "text": text
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("message_id".to_string()) // In a real app we'd return actual message id
                } else {
                    Err(format!("Google Business Profile API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn reply_to_review(&self, account_id: &str, location_id: &str, review_id: &str, reply: &str) -> Result<String, String> {
        let url = format!("https://mybusiness.googleapis.com/v4/accounts/{}/locations/{}/reviews/{}/reply", account_id, location_id, review_id);

        let payload = serde_json::json!({
            "comment": reply
        });

        let res = self.http_client.put(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("reply_id".to_string()) // Returning mock reply id
                } else {
                    Err(format!("Google Business Profile API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
