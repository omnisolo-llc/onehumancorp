use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use reqwest::Client;

#[derive(serde::Deserialize)]
struct ZoomMeetingResponse {
    join_url: String,
}

#[async_trait::async_trait]
pub trait ZoomClientWrapper: Send + Sync {
    async fn create_meeting(&self, topic: &str, start_time: &str, duration: i32) -> Result<String, String>;
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

#[async_trait::async_trait]
impl ZoomClientWrapper for RealZoomClient {
    async fn create_meeting(&self, topic: &str, start_time: &str, duration: i32) -> Result<String, String> {
        let url = "https://api.zoom.us/v2/users/me/meetings";
        let payload = serde_json::json!({
            "topic": topic,
            "type": 2,
            "start_time": start_time,
            "duration": duration,
            "timezone": "UTC"
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "zoom_create_meeting",
                        0.01
                    ).await;

                    if let Ok(data) = resp.json::<ZoomMeetingResponse>().await {
                        Ok(data.join_url)
                    } else {
                        Err("Failed to parse join URL".to_string())
                    }
                } else {
                    Err(format!("Zoom API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct ZoomProvider {
    client: Arc<dyn ZoomClientWrapper>,
    metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealZoomClient::new(access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom Integration".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ZoomClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom Integration".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_meeting(&self, topic: &str, start_time: &str, duration: i32) -> Result<String, String> {
        self.client.create_meeting(topic, start_time, duration).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockClient { calls: Arc<AtomicUsize> }
    #[async_trait::async_trait]
    impl ZoomClientWrapper for MockClient {
        async fn create_meeting(&self, _1: &str, _2: &str, _3: i32) -> Result<String, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok("url".to_string())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let calls = Arc::new(AtomicUsize::new(0));
        let p = ZoomProvider::with_client(Arc::new(MockClient{ calls: calls.clone() }));
        let res = p.create_meeting("1", "2", 3).await.unwrap();
        assert_eq!(res, "url");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
