use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleBusinessClientWrapper: Send + Sync {
    async fn sync_business_info(&self, location_id: &str, info: serde_json::Value) -> Result<(), String>;
    async fn post_review_reply(&self, location_id: &str, review_id: &str, reply: &str) -> Result<(), String>;
}

pub struct RealGoogleBusinessClient {
    access_token: String,
    http_client: Client,
}

impl RealGoogleBusinessClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl GoogleBusinessClientWrapper for RealGoogleBusinessClient {
    async fn sync_business_info(&self, location_id: &str, info: serde_json::Value) -> Result<(), String> {
        let url = format!("https://mybusiness.googleapis.com/v4/accounts/locations/{}", location_id);
        let res = self.http_client.patch(&url)
            .bearer_auth(&self.access_token)
            .json(&info)
            .send()
            .await;

        match res {
            Ok(r) if r.status().is_success() => Ok(()),
            Ok(r) => Err(format!("Google API error: {}", r.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn post_review_reply(&self, location_id: &str, review_id: &str, reply: &str) -> Result<(), String> {
        let url = format!("https://mybusiness.googleapis.com/v4/accounts/locations/{}/reviews/{}/reply", location_id, review_id);
        let payload = serde_json::json!({
            "comment": reply
        });

        let res = self.http_client.put(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(r) if r.status().is_success() => Ok(()),
            Ok(r) => Err(format!("Google API error: {}", r.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
