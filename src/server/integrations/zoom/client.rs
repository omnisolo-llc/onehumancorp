use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZoomMeetingRequest {
    pub topic: String,
    pub type_: u8,
    pub start_time: String,
    pub duration: u32,
    pub timezone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZoomMeetingResponse {
    pub id: u64,
    pub join_url: String,
}

pub struct ZoomClient {
    pub access_token: String,
    pub http_client: Client,
}

impl ZoomClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, user_id: &str, request: &ZoomMeetingRequest) -> Result<ZoomMeetingResponse, String> {
        let url = format!("https://api.zoom.us/v2/users/{}/meetings", user_id);
        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(request)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let meeting: ZoomMeetingResponse = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(meeting)
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
    fn test_zoom_client_creation() {
        let client = ZoomClient::new("test_access_token".to_string());
        assert_eq!(client.access_token, "test_access_token");
    }

    #[tokio::test]
    async fn test_create_meeting_compiles() {
        let client = ZoomClient::new("test_access_token".to_string());
        assert_eq!(client.access_token, "test_access_token");
    }
}
