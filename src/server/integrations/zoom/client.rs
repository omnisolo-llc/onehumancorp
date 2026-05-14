use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZoomMeeting {
    pub id: i64,
    pub join_url: String,
    pub topic: String,
}

#[async_trait]
pub trait ZoomClientWrapper: Send + Sync {
    async fn create_meeting(&self, topic: &str) -> Result<ZoomMeeting, String>;
}

pub struct RealZoomClient {
    access_token: String,
    http_client: Client,
}

impl RealZoomClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ZoomClientWrapper for RealZoomClient {
    async fn create_meeting(&self, topic: &str) -> Result<ZoomMeeting, String> {
        let url = "https://api.zoom.us/v2/users/me/meetings";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "topic": topic,
                "type": 1, // Instant meeting
                "settings": {
                    "host_video": true,
                    "participant_video": true,
                    "join_before_host": true,
                    "mute_upon_entry": true
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let created: ZoomMeeting = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(created)
                } else {
                    Err(format!("Zoom API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
