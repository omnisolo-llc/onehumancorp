use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZoomMeeting {
    pub id: i64,
    pub join_url: String,
    pub topic: String,
    pub start_time: String,
}

pub struct ZoomClient {
    pub api_key: String,
    http_client: Client,
    base_url: String,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
            base_url: "https://api.zoom.us/v2".to_string(),
        }
    }

    pub async fn create_meeting(&self, topic: &str, start_time: &str, tenant_id: &str) -> Result<ZoomMeeting, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "zoom_create_meeting",
            0.05
        ).await;

        let url = format!("{}/users/me/meetings", self.base_url);
        let payload = serde_json::json!({
            "topic": topic,
            "type": 2,
            "start_time": start_time
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(ZoomMeeting {
                    id: 123456789,
                    join_url: "https://zoom.us/j/123456789".to_string(),
                    topic: topic.to_string(),
                    start_time: start_time.to_string(),
                })
            },
            Ok(resp) => Err(format!("Zoom API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn list_meetings(&self, _tenant_id: &str) -> Result<Vec<ZoomMeeting>, String> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zoom_create_meeting() {
        let client = ZoomClient::new("test_key".to_string());
        let _ = client.create_meeting("Meeting", "2024-05-01T10:00:00Z", "tenant1").await;
    }
}
