use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Option<String>,
    pub summary: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchResponse {
    pub id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEventsResponse {
    pub events: Vec<CalendarEvent>,
    #[serde(rename = "nextSyncToken")]
    pub next_sync_token: Option<String>,
}

#[async_trait]
pub trait GoogleCalendarClientWrapper: Send + Sync {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String>;
    async fn create_event(
        &self,
        summary: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<String, String>;
    async fn cancel_event(&self, event_id: &str) -> Result<(), String>;
    async fn list_events_sync(
        &self,
        sync_token: Option<&str>,
    ) -> Result<SyncEventsResponse, String>;
}

use std::sync::Arc;

#[async_trait::async_trait]
pub trait TokenStorage: Send + Sync {
    async fn refresh_if_needed(
        &self,
        current_refresh_token: &str,
        force: bool,
    ) -> Result<Option<OAuthTokenResponse>, String>;
    async fn mark_disconnected(&self) -> Result<(), String>;
}

pub struct RealGoogleCalendarClient {
    pub access_token: tokio::sync::RwLock<String>,
    pub refresh_token: Option<String>,
    pub expires_at: tokio::sync::RwLock<Option<chrono::DateTime<chrono::Utc>>>,
    pub token_storage: Option<Arc<dyn TokenStorage>>,
    pub http_client: Client,
    pub base_url: String,
}

impl RealGoogleCalendarClient {
    pub fn new(
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
        token_storage: Option<Arc<dyn TokenStorage>>,
    ) -> Self {
        Self {
            access_token: tokio::sync::RwLock::new(access_token),
            refresh_token,
            expires_at: tokio::sync::RwLock::new(expires_at),
            token_storage,
            http_client: Client::new(),
            base_url: std::env::var("GOOGLE_CALENDAR_API_BASE")
                .unwrap_or_else(|_| "https://www.googleapis.com".to_string()),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_base_url_for_test(access_token: String, base_url: String) -> Self {
        Self {
            access_token: tokio::sync::RwLock::new(access_token),
            refresh_token: None,
            expires_at: tokio::sync::RwLock::new(None),
            token_storage: None,
            http_client: Client::new(),
            base_url,
        }
    }

    fn calendar_api_url(&self, path: &str) -> String {
        format!(
            "{}/calendar/v3/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    pub async fn get_valid_token(&self, force_refresh: bool) -> Result<String, String> {
        let mut needs_refresh = force_refresh;
        if !needs_refresh {
            let exp = *self.expires_at.read().await;
            if let Some(exp_time) = exp {
                if chrono::Utc::now() + chrono::Duration::seconds(60) >= exp_time {
                    needs_refresh = true;
                }
            }
        }

        if needs_refresh {
            if let (Some(storage), Some(refresh)) = (&self.token_storage, &self.refresh_token) {
                if let Some(new_token) = storage.refresh_if_needed(refresh, force_refresh).await? {
                    *self.access_token.write().await = new_token.access_token.clone();
                    if let Some(secs) = new_token.expires_in {
                        *self.expires_at.write().await =
                            Some(chrono::Utc::now() + chrono::Duration::seconds(secs as i64));
                    }
                    return Ok(new_token.access_token);
                }
            }
        }

        let token = self.access_token.read().await.clone();
        let trimmed = token.trim();
        if trimmed.is_empty() {
            Err("Google Calendar access token is required".to_string())
        } else {
            Ok(trimmed.to_string())
        }
    }

    fn parse_free_busy_response(json: &Value) -> Result<String, String> {
        let mut events: Vec<serde_json::Value> = Vec::new();
        if let Some(calendars) = json["calendars"].as_object() {
            if let Some(primary) = calendars.get("primary") {
                if let Some(busy) = primary["busy"].as_array() {
                    for b in busy {
                        events.push(serde_json::json!({
                            "start": b["start"],
                            "end": b["end"]
                        }));
                    }
                }
            }
        }
        Ok(serde_json::json!({ "events": events }).to_string())
    }
}

fn created_event_reference(json: &Value) -> Result<String, String> {
    if let Some(link) = json["hangoutLink"].as_str() {
        return Ok(link.to_string());
    }
    if let Some(points) = json["conferenceData"]["entryPoints"].as_array() {
        for point in points {
            if point["entryPointType"] == "video" {
                if let Some(uri) = point["uri"].as_str() {
                    return Ok(uri.to_string());
                }
            }
        }
    }
    if let Some(id) = json["id"].as_str() {
        return Ok(id.to_string());
    }
    Err(
        "Google Calendar create_event response did not include an event id or Meet link"
            .to_string(),
    )
}

#[async_trait]
impl GoogleCalendarClientWrapper for RealGoogleCalendarClient {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let url = self.calendar_api_url("freeBusy");
        let mut token = self.get_valid_token(false).await?;

        let payload = serde_json::json!({
            "timeMin": time_min,
            "timeMax": time_max,
            "items": [{"id": "primary"}]
        });

        let mut res = self
            .http_client
            .post(&url)
            .bearer_auth(&token)
            .json(&payload)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .post(&url)
                    .bearer_auth(&token)
                    .json(&payload)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("Google Calendar API response parse error: {}", e))?;
                    Self::parse_free_busy_response(&json)
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn create_event(
        &self,
        summary: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<String, String> {
        let url = self.calendar_api_url("calendars/primary/events");
        let mut token = self.get_valid_token(false).await?;

        let payload = serde_json::json!({
            "summary": summary,
            "start": { "dateTime": start_time },
            "end": { "dateTime": end_time },
            "conferenceData": {
                "createRequest": {
                    "requestId": format!("ohc-google-meet-{}", uuid::Uuid::new_v4()),
                    "conferenceSolutionKey": {
                        "type": "hangoutsMeet"
                    }
                }
            }
        });

        let mut res = self
            .http_client
            .post(&url)
            .query(&[("conferenceDataVersion", "1")])
            .bearer_auth(&token)
            .json(&payload)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .post(&url)
                    .query(&[("conferenceDataVersion", "1")])
                    .bearer_auth(&token)
                    .json(&payload)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("Google Calendar API response parse error: {}", e))?;
                    created_event_reference(&json)
                } else {
                    Err(format!("Google Calendar API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn cancel_event(&self, event_id: &str) -> Result<(), String> {
        let url = self.calendar_api_url(&format!("calendars/primary/events/{}", event_id));
        let mut token = self.get_valid_token(false).await?;

        let mut res = self
            .http_client
            .delete(&url)
            .bearer_auth(&token)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .delete(&url)
                    .bearer_auth(&token)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().is_success()
                    || resp.status().as_u16() == 410
                    || resp.status().as_u16() == 404
                {
                    Ok(())
                } else {
                    Err(format!(
                        "Google Calendar API error on cancel: {}",
                        resp.status()
                    ))
                }
            }
            Err(e) => Err(format!("Network error cancelling event: {}", e)),
        }
    }

    async fn list_events_sync(
        &self,
        sync_token: Option<&str>,
    ) -> Result<SyncEventsResponse, String> {
        let url = self.calendar_api_url("calendars/primary/events");
        let mut token = self.get_valid_token(false).await?;

        let mut query = vec![("singleEvents".to_string(), "false".to_string())];
        if let Some(t) = sync_token {
            query.push(("syncToken".to_string(), t.to_string()));
        }

        let mut res = self
            .http_client
            .get(&url)
            .bearer_auth(&token)
            .query(&query)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .get(&url)
                    .bearer_auth(&token)
                    .query(&query)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().as_u16() == 410 {
                    return Err("410 Gone".to_string());
                }
                if resp.status().is_success() {
                    let json = resp
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("Parse error: {}", e))?;

                    let next_sync_token = json["nextSyncToken"].as_str().map(|s| s.to_string());

                    let events = json["items"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|item| {
                            let start = item["start"]["dateTime"]
                                .as_str()
                                .or_else(|| item["start"]["date"].as_str())
                                .map(|s| s.to_string());
                            let end = item["end"]["dateTime"]
                                .as_str()
                                .or_else(|| item["end"]["date"].as_str())
                                .map(|s| s.to_string());
                            CalendarEvent {
                                id: item["id"].as_str().map(|s| s.to_string()),
                                summary: item["summary"].as_str().map(|s| s.to_string()),
                                start,
                                end,
                                status: item["status"].as_str().map(|s| s.to_string()),
                            }
                        })
                        .collect();

                    Ok(SyncEventsResponse {
                        events,
                        next_sync_token,
                    })
                } else {
                    Err(format!("Sync API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

impl RealGoogleCalendarClient {
    pub async fn exchange_code(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        code: &str,
    ) -> Result<OAuthTokenResponse, String> {
        let client = Client::new();
        let res = client
            .post(
                std::env::var("GOOGLE_OAUTH_TOKEN_URL")
                    .unwrap_or_else(|_| "https://oauth2.googleapis.com/token".to_string()),
            )
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("redirect_uri", redirect_uri),
                ("grant_type", "authorization_code"),
                ("code", code),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json::<OAuthTokenResponse>()
                        .await
                        .map_err(|e| format!("Parse error: {}", e))
                } else {
                    Err(format!("OAuth exchange error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn refresh_token(
        client_id: &str,
        client_secret: &str,
        refresh_token: &str,
    ) -> Result<OAuthTokenResponse, String> {
        let client = Client::new();
        let res = client
            .post(
                std::env::var("GOOGLE_OAUTH_TOKEN_URL")
                    .unwrap_or_else(|_| "https://oauth2.googleapis.com/token".to_string()),
            )
            .form(&[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json::<OAuthTokenResponse>()
                        .await
                        .map_err(|e| format!("Parse error: {}", e))
                } else {
                    Err(format!("OAuth refresh error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn setup_watch(
        &self,
        channel_id: &str,
        webhook_address: &str,
    ) -> Result<WatchResponse, String> {
        let url = self.calendar_api_url("calendars/primary/events/watch");
        let mut token = self.get_valid_token(false).await?;

        let payload = serde_json::json!({
            "id": channel_id,
            "type": "web_hook",
            "address": webhook_address
        });

        let mut res = self
            .http_client
            .post(&url)
            .bearer_auth(&token)
            .json(&payload)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .post(&url)
                    .bearer_auth(&token)
                    .json(&payload)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.json::<WatchResponse>()
                        .await
                        .map_err(|e| format!("Parse error: {}", e))
                } else {
                    Err(format!("Watch API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn list_events(
        &self,
        time_min: &str,
        time_max: &str,
        max_results: u32,
    ) -> Result<Vec<CalendarEvent>, String> {
        let url = self.calendar_api_url("calendars/primary/events");
        let mut token = self.get_valid_token(false).await?;

        let query = [
            ("timeMin", time_min.to_string()),
            ("timeMax", time_max.to_string()),
            ("singleEvents", "true".to_string()),
            ("orderBy", "startTime".to_string()),
            ("maxResults", max_results.to_string()),
        ];

        let mut res = self
            .http_client
            .get(&url)
            .bearer_auth(&token)
            .query(&query)
            .send()
            .await;

        if let Ok(resp) = &res {
            if resp.status().as_u16() == 401 {
                token = self.get_valid_token(true).await?;
                res = self
                    .http_client
                    .get(&url)
                    .bearer_auth(&token)
                    .query(&query)
                    .send()
                    .await;
            }
        }

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp
                        .json::<Value>()
                        .await
                        .map_err(|e| format!("Google Calendar response parse error: {}", e))?;

                    let events = json["items"]
                        .as_array()
                        .ok_or_else(|| "Missing items array in response".to_string())?
                        .iter()
                        .map(|item| {
                            let start = item["start"]["dateTime"]
                                .as_str()
                                .or_else(|| item["start"]["date"].as_str())
                                .map(|s| s.to_string());
                            let end = item["end"]["dateTime"]
                                .as_str()
                                .or_else(|| item["end"]["date"].as_str())
                                .map(|s| s.to_string());
                            CalendarEvent {
                                id: item["id"].as_str().map(|s| s.to_string()),
                                summary: item["summary"].as_str().map(|s| s.to_string()),
                                start,
                                end,
                                status: item["status"].as_str().map(|s| s.to_string()),
                            }
                        })
                        .collect();
                    Ok(events)
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
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::{Mutex, oneshot};

    async fn start_google_calendar_server(
        response_body: &'static str,
    ) -> (String, oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 1024];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0, "client closed connection before sending request");
                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none()
                    && let Some(index) = request.windows(4).position(|window| window == b"\r\n\r\n")
                {
                    header_end = Some(index + 4);
                    let headers = String::from_utf8_lossy(&request[..index]);
                    content_length = headers
                        .lines()
                        .find_map(|line| {
                            line.strip_prefix("content-length: ")
                                .or_else(|| line.strip_prefix("Content-Length: "))
                        })
                        .and_then(|value| value.trim().parse::<usize>().ok())
                        .unwrap_or(0);
                }

                if let Some(body_start) = header_end
                    && request.len() >= body_start + content_length
                {
                    break;
                }
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            request_tx
                .send(String::from_utf8(request).unwrap())
                .unwrap();
        });

        (base_url, request_rx)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn create_event_requests_meet_conference_and_returns_response_link() {
        let response = r#"{
            "id": "calendar-event-123",
            "hangoutLink": "https://meet.google.com/aaa-bbbb-ccc",
            "conferenceData": {
                "entryPoints": [
                    {
                        "entryPointType": "video",
                        "uri": "https://meet.google.com/ddd-eeee-fff"
                    }
                ]
            }
        }"#;
        let (base_url, request_rx) = start_google_calendar_server(response).await;
        let client =
            RealGoogleCalendarClient::with_base_url_for_test("valid-token".to_string(), base_url);

        let created_link = client
            .create_event(
                "Intro call",
                "2026-06-06T09:00:00-07:00",
                "2026-06-06T09:30:00-07:00",
            )
            .await
            .unwrap();

        assert_eq!(created_link, "https://meet.google.com/aaa-bbbb-ccc");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with(
            "POST /calendar/v3/calendars/primary/events?conferenceDataVersion=1 HTTP/1.1"
        ));
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );

        let body = request_body(&request);
        assert_eq!(body["summary"], "Intro call");
        assert_eq!(body["start"]["dateTime"], "2026-06-06T09:00:00-07:00");
        assert_eq!(body["end"]["dateTime"], "2026-06-06T09:30:00-07:00");
        assert_eq!(
            body["conferenceData"]["createRequest"]["conferenceSolutionKey"]["type"],
            "hangoutsMeet"
        );
        assert!(
            body["conferenceData"]["createRequest"]["requestId"]
                .as_str()
                .unwrap()
                .starts_with("ohc-google-meet-")
        );
    }

    #[tokio::test]
    async fn create_event_rejects_blank_access_token_before_network_request() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let request_seen = Arc::new(Mutex::new(false));
        let request_seen_by_server = request_seen.clone();

        tokio::spawn(async move {
            let accepted =
                tokio::time::timeout(std::time::Duration::from_millis(100), listener.accept())
                    .await;
            if accepted.is_ok() {
                *request_seen_by_server.lock().await = true;
            }
        });

        let client = RealGoogleCalendarClient::with_base_url_for_test("   ".to_string(), base_url);
        let error = client
            .create_event(
                "Intro call",
                "2026-06-06T09:00:00-07:00",
                "2026-06-06T09:30:00-07:00",
            )
            .await
            .unwrap_err();

        assert_eq!(error, "Google Calendar access token is required");
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        assert!(!*request_seen.lock().await);
    }

    #[tokio::test]
    async fn cancel_event_sends_delete_request() {
        let response = "";
        let (base_url, request_rx) = start_google_calendar_server(response).await;
        let client =
            RealGoogleCalendarClient::with_base_url_for_test("valid-token".to_string(), base_url);

        client.cancel_event("event-123").await.unwrap();

        let request = request_rx.await.unwrap();
        assert!(
            request.starts_with("DELETE /calendar/v3/calendars/primary/events/event-123 HTTP/1.1")
        );
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );
    }

    #[tokio::test]
    async fn get_free_busy_parses_busy_slots() {
        let response = r#"{
            "calendars": {
                "primary": {
                    "busy": [
                        { "start": "2026-07-21T09:00:00Z", "end": "2026-07-21T10:00:00Z" },
                        { "start": "2026-07-21T14:00:00Z", "end": "2026-07-21T15:00:00Z" }
                    ]
                }
            }
        }"#;
        let (base_url, _) = start_google_calendar_server(response).await;
        let client =
            RealGoogleCalendarClient::with_base_url_for_test("valid-token".to_string(), base_url);

        let result = client
            .get_free_busy("2026-07-21T00:00:00Z", "2026-07-22T00:00:00Z")
            .await
            .unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        let events = parsed["events"].as_array().unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0]["start"], "2026-07-21T09:00:00Z");
        assert_eq!(events[0]["end"], "2026-07-21T10:00:00Z");
    }
}

#[cfg(test)]
mod retry_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockTokenStorage {
        refresh_called: Arc<Mutex<usize>>,
        fail_refresh: bool,
    }

    #[async_trait::async_trait]
    impl TokenStorage for MockTokenStorage {
        async fn refresh_if_needed(
            &self,
            _current_refresh_token: &str,
            _force: bool,
        ) -> Result<Option<OAuthTokenResponse>, String> {
            let mut called = self.refresh_called.lock().await;
            *called += 1;

            if self.fail_refresh {
                return Err("invalid_grant".to_string());
            }

            Ok(Some(OAuthTokenResponse {
                access_token: "new-token".to_string(),
                refresh_token: None,
                expires_in: Some(3600),
            }))
        }

        async fn mark_disconnected(&self) -> Result<(), String> {
            Ok(())
        }
    }

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    async fn start_mock_server_401_then_200() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 1024];
            let read = stream.read(&mut buffer).await.unwrap();
            request.extend_from_slice(&buffer[..read]);

            let response =
                "HTTP/1.1 401 Unauthorized\r\ncontent-length: 0\r\nconnection: close\r\n\r\n";
            stream.write_all(response.as_bytes()).await.unwrap();

            // second request
            let (mut stream2, _) = listener.accept().await.unwrap();
            let mut request2 = Vec::new();
            let read2 = stream2.read(&mut buffer).await.unwrap();
            request2.extend_from_slice(&buffer[..read2]);

            let req_str = String::from_utf8_lossy(&request2);
            assert!(req_str.contains("Bearer new-token"));

            let resp_body = r#"{"items": []}"#;
            let response2 = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                resp_body.len(),
                resp_body
            );
            stream2.write_all(response2.as_bytes()).await.unwrap();
        });

        base_url
    }

    #[tokio::test]
    async fn test_401_triggers_refresh_and_retry_once_successfully() {
        let base_url = start_mock_server_401_then_200().await;
        let refresh_called = Arc::new(Mutex::new(0));
        let storage = Arc::new(MockTokenStorage {
            refresh_called: refresh_called.clone(),
            fail_refresh: false,
        });

        let client = RealGoogleCalendarClient {
            access_token: tokio::sync::RwLock::new("old-token".to_string()),
            refresh_token: Some("refresh".to_string()),
            expires_at: tokio::sync::RwLock::new(Some(
                chrono::Utc::now() + chrono::Duration::hours(1),
            )),
            token_storage: Some(storage),
            http_client: reqwest::Client::new(),
            base_url,
        };

        let result = client
            .list_events("2026-01-01T00:00:00Z", "2026-01-02T00:00:00Z", 10)
            .await
            .unwrap();
        assert_eq!(result.len(), 0);

        assert_eq!(*refresh_called.lock().await, 1);
        assert_eq!(*client.access_token.read().await, "new-token");
    }

    async fn start_mock_server_always_401() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buffer = [0_u8; 1024];
                let _ = stream.read(&mut buffer).await.unwrap();

                let response =
                    "HTTP/1.1 401 Unauthorized\r\ncontent-length: 0\r\nconnection: close\r\n\r\n";
                let _ = stream.write_all(response.as_bytes()).await;
            }
        });

        base_url
    }

    #[tokio::test]
    async fn test_401_refresh_failure_propagates_error() {
        let base_url = start_mock_server_always_401().await;
        let refresh_called = Arc::new(Mutex::new(0));
        let storage = Arc::new(MockTokenStorage {
            refresh_called: refresh_called.clone(),
            fail_refresh: true,
        });

        let client = RealGoogleCalendarClient {
            access_token: tokio::sync::RwLock::new("old-token".to_string()),
            refresh_token: Some("refresh".to_string()),
            expires_at: tokio::sync::RwLock::new(Some(
                chrono::Utc::now() + chrono::Duration::hours(1),
            )),
            token_storage: Some(storage),
            http_client: reqwest::Client::new(),
            base_url,
        };

        let result = client
            .list_events("2026-01-01T00:00:00Z", "2026-01-02T00:00:00Z", 10)
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, "invalid_grant");

        assert_eq!(*refresh_called.lock().await, 1);
        assert_eq!(*client.access_token.read().await, "old-token");
    }

    #[tokio::test]
    async fn test_expired_token_refreshes_before_request() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 1024];
            let read = stream.read(&mut buffer).await.unwrap();
            request.extend_from_slice(&buffer[..read]);

            let req_str = String::from_utf8_lossy(&request);
            assert!(req_str.contains("Bearer new-token")); // Sent new token directly!

            let resp_body = r#"{"items": []}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                resp_body.len(),
                resp_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let refresh_called = Arc::new(Mutex::new(0));
        let storage = Arc::new(MockTokenStorage {
            refresh_called: refresh_called.clone(),
            fail_refresh: false,
        });

        let client = RealGoogleCalendarClient {
            access_token: tokio::sync::RwLock::new("old-token".to_string()),
            refresh_token: Some("refresh".to_string()),
            // Set expires_at in the past!
            expires_at: tokio::sync::RwLock::new(Some(
                chrono::Utc::now() - chrono::Duration::hours(1),
            )),
            token_storage: Some(storage),
            http_client: reqwest::Client::new(),
            base_url,
        };

        let result = client
            .list_events("2026-01-01T00:00:00Z", "2026-01-02T00:00:00Z", 10)
            .await
            .unwrap();
        assert_eq!(result.len(), 0);

        assert_eq!(*refresh_called.lock().await, 1);
        assert_eq!(*client.access_token.read().await, "new-token");
    }
}
