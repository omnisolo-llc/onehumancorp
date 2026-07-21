use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

const BASE_URL: &str = "https://graph.microsoft.com/v1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookEvent {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub body: Option<OutlookBody>,
    #[serde(default)]
    pub start: Option<OutlookDateTime>,
    #[serde(default)]
    pub end: Option<OutlookDateTime>,
    #[serde(default)]
    pub attendees: Vec<OutlookAttendee>,
    #[serde(default)]
    pub is_online_meeting: Option<bool>,
    #[serde(default)]
    pub online_meeting: Option<OutlookOnlineMeeting>,
    #[serde(default)]
    pub web_link: Option<String>,
    #[serde(default)]
    pub is_cancelled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookBody {
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookDateTime {
    #[serde(default)]
    pub date_time: String,
    #[serde(default)]
    pub time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookAttendee {
    #[serde(default)]
    pub email_address: OutlookEmailAddress,
    #[serde(default)]
    pub type_name: Option<String>,
    #[serde(rename = "type", default)]
    pub type_: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutlookEmailAddress {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookOnlineMeeting {
    #[serde(default)]
    pub join_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookCalendar {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub is_default_calendar: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusySlot {
    #[serde(default)]
    pub start: Option<OutlookDateTime>,
    #[serde(default)]
    pub end: Option<OutlookDateTime>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookScheduleResponse {
    #[serde(default)]
    pub value: Vec<OutlookSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookSchedule {
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub availability_view_items: Vec<FreeBusySlot>,
}

#[derive(Debug, Deserialize)]
struct OutlookListResponse<T> {
    value: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OutlookError {
    error: OutlookErrorDetail,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OutlookErrorDetail {
    message: String,
}

pub struct OutlookCalendarClient {
    access_token: String,
}

impl OutlookCalendarClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    #[cfg(test)]
    fn with_base_url_for_test(access_token: String, _base_url: String) -> Self {
        Self { access_token }
    }

    #[allow(dead_code)]
    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Outlook Calendar access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", BASE_URL, path)
    }

    pub async fn get_events(
        &self,
        calendar_id: Option<&str>,
        start: &str,
        end: &str,
    ) -> Result<Vec<OutlookEvent>, String> {
        let token = self.validated_access_token()?;
        let path = match calendar_id {
            Some(cid) => format!("/me/calendars/{}/events", cid),
            None => "/me/events".to_string(),
        };

        let url = format!(
            "{}?$filter=start/dateTime ge '{}' and end/dateTime le '{}'",
            self.url(&path),
            start,
            end
        );

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            let list: OutlookListResponse<OutlookEvent> = res
                .json()
                .await
                .map_err(|e| format!("response parse error: {}", e))?;
            Ok(list.value)
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn create_event(
        &self,
        subject: &str,
        body: &str,
        start: &str,
        end: &str,
        attendees: &[String],
        is_online_meeting: bool,
    ) -> Result<OutlookEvent, String> {
        let token = self.validated_access_token()?;
        let url = self.url("/me/events");

        let attendee_list: Vec<serde_json::Value> = attendees
            .iter()
            .map(|email| {
                serde_json::json!({
                    "emailAddress": {"address": email},
                    "type": "required"
                })
            })
            .collect();

        let payload = serde_json::json!({
            "subject": subject,
            "body": {"contentType": "HTML", "content": body},
            "start": {"dateTime": start, "timeZone": "UTC"},
            "end": {"dateTime": end, "timeZone": "UTC"},
            "attendees": attendee_list,
            "isOnlineMeeting": is_online_meeting,
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<OutlookEvent>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn update_event(
        &self,
        event_id: &str,
        subject: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<OutlookEvent, String> {
        let token = self.validated_access_token()?;
        let url = self.url(&format!("/me/events/{}", event_id));

        let mut payload = serde_json::json!({});
        if let Some(s) = subject {
            payload["subject"] = serde_json::json!(s);
        }
        if let Some(s) = start {
            payload["start"] = serde_json::json!({"dateTime": s, "timeZone": "UTC"});
        }
        if let Some(e) = end {
            payload["end"] = serde_json::json!({"dateTime": e, "timeZone": "UTC"});
        }

        let client = get_client();
        let res = client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<OutlookEvent>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn delete_event(&self, event_id: &str) -> Result<(), String> {
        let token = self.validated_access_token()?;
        let url = self.url(&format!("/me/events/{}", event_id));

        let client = get_client();
        let res = client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() || res.status().as_u16() == 204 {
            Ok(())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_free_busy(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Vec<FreeBusySlot>, String> {
        let token = self.validated_access_token()?;
        let url = self.url("/me/calendar/getSchedule");

        let payload = serde_json::json!({
            "schedules": ["/me"],
            "startTime": {"dateTime": start, "timeZone": "UTC"},
            "endTime": {"dateTime": end, "timeZone": "UTC"},
            "availabilityViewInterval": "60",
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            let schedule_res: OutlookScheduleResponse = res
                .json()
                .await
                .map_err(|e| format!("response parse error: {}", e))?;

            Ok(schedule_res
                .value
                .into_iter()
                .flat_map(|s| s.availability_view_items)
                .collect())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_calendars(&self) -> Result<Vec<OutlookCalendar>, String> {
        let token = self.validated_access_token()?;
        let url = self.url("/me/calendars");

        let client = get_client();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            let list: OutlookListResponse<OutlookCalendar> = res
                .json()
                .await
                .map_err(|e| format!("response parse error: {}", e))?;
            Ok(list.value)
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_outlook_server(response_body: &'static str) -> (String, oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0, "client closed connection before sending request");
                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none() {
                    if let Some(index) =
                        request.windows(4).position(|window| window == b"\r\n\r\n")
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
                }

                if let Some(body_start) = header_end {
                    if request.len() >= body_start + content_length {
                        break;
                    }
                }
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            request_tx.send(String::from_utf8(request).unwrap()).unwrap();
        });

        (base_url, request_rx)
    }

    async fn start_outlook_server_status(status: u16) -> (String, oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0);
                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none() {
                    if let Some(index) =
                        request.windows(4).position(|window| window == b"\r\n\r\n")
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
                }

                if let Some(body_start) = header_end {
                    if request.len() >= body_start + content_length {
                        break;
                    }
                }
            }

            let body_str = format!(r#"{{"error":{{"message":"test error","code":"test"}}}}"#);
            let response = format!(
                "HTTP/1.1 {} {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                status,
                match status { 204 => "No Content", _ => "OK" },
                body_str.len(),
                body_str
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            request_tx.send(String::from_utf8(request).unwrap()).unwrap();
        });

        (base_url, request_rx)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn get_events_sends_correct_request() {
        let response = r#"{
            "value": [{
                "id": "evt-123",
                "subject": "Team sync",
                "start": {"dateTime": "2026-01-15T10:00:00Z", "timeZone": "UTC"},
                "end": {"dateTime": "2026-01-15T11:00:00Z", "timeZone": "UTC"},
                "attendees": []
            }]
        }"#;
        let (base_url, request_rx) = start_outlook_server(response).await;

        let client = OutlookCalendarClient::with_base_url_for_test("ms_token".to_string(), base_url);
        let events = client
            .get_events(None, "2026-01-15T00:00:00Z", "2026-01-15T23:59:59Z")
            .await
            .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].subject, "Team sync");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /me/events?$filter="));
        assert!(request.contains("authorization: Bearer ms_token"));
    }

    #[tokio::test]
    async fn get_events_with_calendar_id() {
        let response = r#"{"value": []}"#;
        let (base_url, request_rx) = start_outlook_server(response).await;

        let client = OutlookCalendarClient::with_base_url_for_test("ms_token".to_string(), base_url);
        let _ = client
            .get_events(Some("cal-123"), "2026-01-15T00:00:00Z", "2026-01-15T23:59:59Z")
            .await
            .unwrap();

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /me/calendars/cal-123/events?$filter="));
    }

    #[tokio::test]
    async fn create_event_sends_correct_payload() {
        let response = r#"{
            "id": "evt-456",
            "subject": "Design review",
            "start": {"dateTime": "2026-01-15T14:00:00Z", "timeZone": "UTC"},
            "end": {"dateTime": "2026-01-15T15:00:00Z", "timeZone": "UTC"},
            "attendees": [{"emailAddress": {"address": "alice@example.com"}}],
            "isOnlineMeeting": true
        }"#;
        let (base_url, request_rx) = start_outlook_server(response).await;

        let client = OutlookCalendarClient::with_base_url_for_test("ms_token".to_string(), base_url);
        let event = client
            .create_event(
                "Design review",
                "<p>Please review</p>",
                "2026-01-15T14:00:00Z",
                "2026-01-15T15:00:00Z",
                &["alice@example.com".to_string()],
                true,
            )
            .await
            .unwrap();

        assert_eq!(event.subject, "Design review");
        assert!(event.is_online_meeting.unwrap());

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /me/events HTTP/1.1"));

        let body = request_body(&request);
        assert_eq!(body["subject"], "Design review");
        assert_eq!(body["isOnlineMeeting"], true);
        assert_eq!(body["attendees"][0]["emailAddress"]["address"], "alice@example.com");
    }

    #[tokio::test]
    async fn delete_event_returns_ok() {
        let (base_url, request_rx) = start_outlook_server_status(204).await;

        let client = OutlookCalendarClient::with_base_url_for_test("ms_token".to_string(), base_url);
        client.delete_event("evt-789").await.unwrap();

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("DELETE /me/events/evt-789 HTTP/1.1"));
    }

    #[tokio::test]
    async fn get_calendars_returns_list() {
        let response = r#"{
            "value": [
                {"id": "cal-1", "name": "Calendar", "isDefaultCalendar": true},
                {"id": "cal-2", "name": "Work"}
            ]
        }"#;
        let (base_url, _request_rx) = start_outlook_server(response).await;

        let client = OutlookCalendarClient::with_base_url_for_test("ms_token".to_string(), base_url);
        let calendars = client.get_calendars().await.unwrap();

        assert_eq!(calendars.len(), 2);
        assert_eq!(calendars[0].name, "Calendar");
        assert!(calendars[0].is_default_calendar.unwrap());
    }

    #[tokio::test]
    async fn blank_token_rejected_before_network() {
        let client = OutlookCalendarClient::new("   ".to_string());
        let err = client
            .get_events(None, "2026-01-15T00:00:00Z", "2026-01-15T23:59:59Z")
            .await
            .unwrap_err();
        assert_eq!(err, "Outlook Calendar access token is required");
    }
}
