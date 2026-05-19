use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ZoomClientWrapper: Send + Sync {
    async fn create_meeting(&self, topic: &str, start_time: &str) -> Result<String, String>;
}

pub struct RealZoomClient {
    pub access_token: String,
    http_client: Client,
}

impl RealZoomClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token, http_client: Client::new() }
    }
}

#[async_trait]
impl ZoomClientWrapper for RealZoomClient {
    async fn create_meeting(&self, topic: &str, start_time: &str) -> Result<String, String> {
        let url = "https://api.zoom.us/v2/users/me/meetings";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "topic": topic,
                "type": 2, // Scheduled meeting
                "start_time": start_time
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "zoom_create_meeting",
                        0.05
                    ).await;
                    let body = resp.text().await.unwrap_or_default();
                    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
                    let join_url = v.get("join_url").and_then(|url| url.as_str()).unwrap_or("").to_string();
                    Ok(join_url)
                } else {
                    Err(format!("Zoom API error: {}", resp.status()))
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
    fn test_client_creation() {
        let client = RealZoomClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_create_meeting_error_handling() {
        let client = RealZoomClient::new("token".to_string());
        let _ = client.create_meeting("Topic", "2023-01-01T10:00:00Z").await;
    }
}
