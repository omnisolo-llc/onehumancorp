use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait NylasClientWrapper: Send + Sync {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String>;
    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String>;
}

pub struct RealNylasClient {
    pub access_token: String,
    http_client: Client,
}

impl RealNylasClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl NylasClientWrapper for RealNylasClient {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let url = "https://api.nylas.com/v3/calendars/free-busy";

        let payload = serde_json::json!({
            "start_time": time_min,
            "end_time": time_max,
            "emails": ["primary"]
        });

        let res = self.http_client.post(url)
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
                        "nylas_get_free_busy",
                        0.01
                    ).await;
                    Ok("{}".to_string())
                } else {
                    Err(format!("Nylas API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        let url = "https://api.nylas.com/v3/grants/me/events";

        let payload = serde_json::json!({
            "title": summary,
            "when": {
                "start_time": start_time,
                "end_time": end_time,
            }
        });

        let res = self.http_client.post(url)
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
                        "nylas_create_event",
                        0.01
                    ).await;
                    Ok("event_id".to_string())
                } else {
                    Err(format!("Nylas API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
