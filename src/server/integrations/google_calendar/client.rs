use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleCalendarClientWrapper: Send + Sync {
    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String>;
}

pub struct RealGoogleCalendarClient {
    pub access_token: String,
    http_client: Client,
}

impl RealGoogleCalendarClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token, http_client: Client::new() }
    }
}

#[async_trait]
impl GoogleCalendarClientWrapper for RealGoogleCalendarClient {
    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "summary": summary,
                "start": {
                    "dateTime": start_time
                },
                "end": {
                    "dateTime": end_time
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "google_calendar_create_event",
                        0.05
                    ).await;
                    let body = resp.text().await.unwrap_or_default();
                    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
                    let id = v.get("id").and_then(|id| id.as_str()).unwrap_or("").to_string();
                    Ok(id)
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
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
        let client = RealGoogleCalendarClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_create_event_error_handling() {
        let client = RealGoogleCalendarClient::new("token".to_string());
        let _ = client.create_event("Test", "2023-01-01T10:00:00Z", "2023-01-01T11:00:00Z").await;
    }
}
