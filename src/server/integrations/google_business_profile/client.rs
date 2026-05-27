use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleBusinessProfileClientWrapper: Send + Sync {
    async fn update_hours(&self, location_id: &str, hours: &serde_json::Value) -> Result<String, String>;
    async fn fetch_reviews(&self, location_id: &str) -> Result<String, String>;
    async fn reply_to_review(&self, location_id: &str, review_id: &str, reply: &str) -> Result<String, String>;
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
    async fn update_hours(&self, location_id: &str, hours: &serde_json::Value) -> Result<String, String> {
        let url = format!("https://mybusinessbusinessinformation.googleapis.com/v1/locations/{}", location_id);

        let payload = serde_json::json!({
            "regularHours": hours
        });

        let res = self.http_client.patch(&url)
            .query(&[("updateMask", "regularHours")])
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "google_business_profile_update_hours",
                        0.01
                    ).await;
                    Ok("{}".to_string())
                } else {
                    Err(format!("Google Business Profile API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn fetch_reviews(&self, location_id: &str) -> Result<String, String> {
        let url = format!("https://mybusiness.googleapis.com/v4/accounts/account_id/locations/{}/reviews", location_id);

        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "google_business_profile_fetch_reviews",
                        0.01
                    ).await;
                    Ok(resp.text().await.unwrap_or_else(|_| "{}".to_string()))
                } else {
                    Err(format!("Google Business Profile API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn reply_to_review(&self, location_id: &str, review_id: &str, reply: &str) -> Result<String, String> {
        let url = format!("https://mybusiness.googleapis.com/v4/accounts/account_id/locations/{}/reviews/{}/reply", location_id, review_id);

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
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "google_business_profile_reply_review",
                        0.01
                    ).await;
                    Ok("{}".to_string())
                } else {
                    Err(format!("Google Business Profile API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
