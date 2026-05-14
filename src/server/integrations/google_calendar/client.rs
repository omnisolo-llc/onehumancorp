use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendarEvent {
    pub id: String,
    pub summary: String,
    pub start_time: String,
    pub end_time: String,
}

#[async_trait]
pub trait GoogleCalendarClientWrapper: Send + Sync {
    async fn create_event(&self, calendar_id: &str, event: CalendarEvent) -> Result<CalendarEvent, String>;
    async fn list_events(&self, calendar_id: &str) -> Result<Vec<CalendarEvent>, String>;
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
    async fn create_event(&self, calendar_id: &str, event: CalendarEvent) -> Result<CalendarEvent, String> {
        let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events", calendar_id);
        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "summary": event.summary,
                "start": { "dateTime": event.start_time },
                "end": { "dateTime": event.end_time }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let created: CalendarEvent = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(created)
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn list_events(&self, calendar_id: &str) -> Result<Vec<CalendarEvent>, String> {
        let url = format!("https://www.googleapis.com/calendar/v3/calendars/{}/events", calendar_id);
        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                    let items = body["items"].as_array().ok_or("Invalid response format")?;
                    let mut events = Vec::new();
                    for item in items {
                        events.push(CalendarEvent {
                            id: item["id"].as_str().unwrap_or_default().to_string(),
                            summary: item["summary"].as_str().unwrap_or_default().to_string(),
                            start_time: item["start"]["dateTime"].as_str().unwrap_or_default().to_string(),
                            end_time: item["end"]["dateTime"].as_str().unwrap_or_default().to_string(),
                        });
                    }
                    Ok(events)
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
