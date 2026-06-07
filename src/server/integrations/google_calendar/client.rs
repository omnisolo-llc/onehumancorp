use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[async_trait]
pub trait GoogleCalendarClientWrapper: Send + Sync {
    async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String>;
    async fn create_event(
        &self,
        summary: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<String, String>;
}

pub struct RealGoogleCalendarClient {
    access_token: String,
    http_client: Client,
    base_url: String,
}

impl RealGoogleCalendarClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            base_url: "https://www.googleapis.com".to_string(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(access_token: String, base_url: String) -> Self {
        Self {
            access_token,
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

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Google Calendar access token is required".to_string())
        } else {
            Ok(token)
        }
    }
}

fn created_event_reference(json: &Value) -> Result<String, String> {
    if let Some(hangout_link) = json["hangoutLink"].as_str() {
        if !hangout_link.trim().is_empty() {
            return Ok(hangout_link.to_string());
        }
    }

    if let Some(entry_points) = json["conferenceData"]["entryPoints"].as_array() {
        if let Some(video_uri) = entry_points.iter().find_map(|entry_point| {
            let is_video = entry_point["entryPointType"].as_str() == Some("video");
            entry_point["uri"]
                .as_str()
                .filter(|uri| is_video && !uri.trim().is_empty())
        }) {
            return Ok(video_uri.to_string());
        }
    }

    if let Some(event_id) = json["id"].as_str() {
        if !event_id.trim().is_empty() {
            return Ok(event_id.to_string());
        }
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
        let token = self.validated_access_token()?;

        let payload = serde_json::json!({
            "timeMin": time_min,
            "timeMax": time_max,
            "items": [{"id": "primary"}]
        });

        let res = self
            .http_client
            .post(url)
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("{}".to_string()) // In a real app we'd return parsed free/busy data
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
        let token = self.validated_access_token()?;

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

        let res = self
            .http_client
            .post(url)
            .query(&[("conferenceDataVersion", "1")])
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await;

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
        assert!(
            request.starts_with(
                "POST /calendar/v3/calendars/primary/events?conferenceDataVersion=1 HTTP/1.1"
            )
        );
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
}
