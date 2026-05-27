use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleCalendarClientWrapper: Send + Sync {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String>;
    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String>;
}

pub struct RealGoogleCalendarClient {
    access_token: String,
    http_client: Client,
}

impl RealGoogleCalendarClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl GoogleCalendarClientWrapper for RealGoogleCalendarClient {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let url = "https://www.googleapis.com/calendar/v3/freeBusy";

        let payload = serde_json::json!({
            "timeMin": time_min,
            "timeMax": time_max,
            "items": [{"id": "primary"}]
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
                        "google_calendar_get_free_busy",
                        0.01
                    ).await;
                    Ok("{}".to_string()) // In a real app we'd return parsed free/busy data
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events?conferenceDataVersion=1";

        let payload = serde_json::json!({
            "summary": summary,
            "start": { "dateTime": start_time },
            "end": { "dateTime": end_time },
            "conferenceData": {
                "createRequest": {
                    "requestId": uuid::Uuid::new_v4().to_string(),
                    "conferenceSolutionKey": {
                        "type": "hangoutsMeet"
                    }
                }
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
                        "google_calendar_create_event",
                        0.01
                    ).await;
                    Ok("event_id".to_string()) // Returning mock event id
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
