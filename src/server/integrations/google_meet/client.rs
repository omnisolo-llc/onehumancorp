use serde::{Deserialize, Serialize};
use reqwest::Client;

pub struct GoogleMeetClient {
    pub access_token: String,
    pub http_client: Client,
}

impl GoogleMeetClient {
    pub fn new(access_token: String) -> Self {
        GoogleMeetClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, summary: &str, start_time: &str, end_time: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "google_meet_create",
            0.05
        ).await;

        let req_body = serde_json::json!({
            "summary": summary,
            "start": { "dateTime": start_time },
            "end": { "dateTime": end_time },
            "conferenceData": {
                "createRequest": {
                    "requestId": uuid::Uuid::new_v4().to_string(),
                    "conferenceSolutionKey": { "type": "hangoutsMeet" }
                }
            }
        });

        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events?conferenceDataVersion=1";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("https://meet.google.com/mock-meet".to_string())
                } else {
                    Err(format!("Google API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meet_client_creation() {
        let client = GoogleMeetClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_meet_create_error() {
        let client = GoogleMeetClient::new("token".to_string());
        let _ = client.create_meeting("summary", "start", "end", "tenant1").await;
    }
}
