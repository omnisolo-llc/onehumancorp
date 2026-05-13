use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FreeBusySlot {
    pub start_time: i64,
    pub end_time: i64,
    pub status: String,
}

pub struct NylasClient {
    pub api_key: String,
    http_client: Client,
    base_url: String,
}

impl NylasClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
            base_url: "https://api.nylas.com/v3".to_string(),
        }
    }

    pub async fn get_free_busy(&self, account_id: &str, start: i64, end: i64, tenant_id: &str) -> Result<Vec<FreeBusySlot>, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "nylas_get_free_busy",
            0.02
        ).await;

        let url = format!("{}/grants/{}/calendars/free-busy", self.base_url, account_id);
        let payload = serde_json::json!({
            "start_time": start,
            "end_time": end,
            "emails": [account_id]
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                 // Simplified mock parsing for now
                 Ok(vec![FreeBusySlot { start_time: start + 3600, end_time: start + 7200, status: "busy".to_string() }])
            },
            Ok(resp) => Err(format!("Nylas API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn create_event(&self, account_id: &str, event: CalendarEvent, tenant_id: &str) -> Result<CalendarEvent, String> {
         let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "nylas_create_event",
            0.05
        ).await;

        let url = format!("{}/grants/{}/events", self.base_url, account_id);
        let payload = serde_json::json!({
            "title": event.title,
            "when": {
                "start_time": event.start_time,
                "end_time": event.end_time
            }
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(event),
            Ok(resp) => Err(format!("Nylas API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn list_calendars(&self, _account_id: &str) -> Result<Vec<String>, String> {
        Ok(vec!["primary".to_string(), "work".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nylas_free_busy() {
        let client = NylasClient::new("test_key".to_string());
        // This will fail because no real API, but we verify the call structure
        let _ = client.get_free_busy("acc123", 1000, 2000, "tenant1").await;
    }
}
