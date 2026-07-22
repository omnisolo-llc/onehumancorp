<<<<<<< HEAD
use reqwest::Client;
use serde_json::Value;

pub struct CalendlyClient {
    access_token: String,
    http_client: Client,
    base_url: String,
}

impl CalendlyClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            base_url: "https://api.calendly.com".to_string(),
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

    fn api_url(&self, path: &str) -> String {
        format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            path
        )
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Calendly access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        let token = self.validated_access_token()?;

        let user_url = self.api_url("/users/me");
        let user_resp = self
            .http_client
            .get(&user_url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Network error fetching user: {}", e))?;

        if !user_resp.status().is_success() {
            return Err(format!("Calendly API error fetching user: {}", user_resp.status()));
        }

        let user_json: Value = user_resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse user response: {}", e))?;

        let user_uri = user_json["resource"]["uri"]
            .as_str()
            .ok_or("Missing user URI in response")?;

        let events_url = self.api_url("/event_types");
        let events_resp = self
            .http_client
            .get(&events_url)
            .bearer_auth(token)
            .query(&[("user", user_uri)])
            .send()
            .await
            .map_err(|e| format!("Network error fetching event types: {}", e))?;

        if !events_resp.status().is_success() {
            return Err(format!(
                "Calendly API error fetching event types: {}",
                events_resp.status()
            ));
        }

        let events_json: Value = events_resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse event types response: {}", e))?;

        let event_types = events_json["collection"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|et| et["name"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Ok(event_types)
    }

    pub async fn create_event(
        &self,
        event_type_uri: &str,
        start_time: &str,
        invitee_email: &str,
    ) -> Result<String, String> {
        let token = self.validated_access_token()?;

        let url = self.api_url("/scheduled_events");

        let payload = serde_json::json!({
            "event_type": event_type_uri,
            "start_time": start_time,
            "status": "active",
            "invitees": [{
                "email": invitee_email
            }]
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating event: {}", e))?;

        if resp.status().is_success() {
            let json: Value = resp
                .json()
                .await
                .map_err(|e| format!("Failed to parse create event response: {}", e))?;

            let event_uri = json["resource"]["uri"]
                .as_str()
                .or_else(|| json["uri"].as_str())
                .unwrap_or("")
                .to_string();

            Ok(event_uri)
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(format!("Calendly API error creating event: {} - {}", status, body))
        }
    }

    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let token = self.validated_access_token()?;

        let url = self.api_url("/scheduled_events");

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[
                ("status", "active"),
                ("start_time", time_min),
                ("end_time", time_max),
            ])
            .send()
            .await
            .map_err(|e| format!("Network error fetching free/busy: {}", e))?;

        if resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            Ok(text)
        } else {
            Err(format!("Calendly API error fetching free/busy: {}", resp.status()))
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

    async fn start_calendly_server(
        response_body: &'static str,
    ) -> (String, oneshot::Receiver<String>) {
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

    async fn start_calendly_multi_server(
        responses: Vec<&'static str>,
    ) -> (String, oneshot::Receiver<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let mut all_requests = Vec::new();

            for response_body in responses {
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

                all_requests.push(String::from_utf8(request).unwrap());

                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    response_body.len(),
                    response_body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }

            request_tx.send(all_requests).unwrap();
        });

        (base_url, request_rx)
    }

    fn request_path(request: &str) -> &str {
        request.split_whitespace().nth(1).unwrap_or("")
    }

    fn request_method(request: &str) -> &str {
        request.split_whitespace().next().unwrap_or("")
    }

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap_or(serde_json::Value::Null)
    }

    #[tokio::test]
    async fn fetch_event_types_returns_names() {
        let user_response = r#"{
            "resource": {
                "uri": "https://api.calendly.com/users/AAAA1111"
            }
        }"#;

        let events_response = r#"{
            "collection": [
                {"name": "30 Min Meeting", "uri": "https://api.calendly.com/event_types/1"},
                {"name": "1 Hour Consultation", "uri": "https://api.calendly.com/event_types/2"}
            ]
        }"#;

        let (base_url, requests_rx) = start_calendly_multi_server(vec![user_response, events_response]).await;
        let client = CalendlyClient::with_base_url_for_test("test-token".to_string(), base_url);

        let event_types = client.fetch_event_types().await.unwrap();

        assert_eq!(event_types.len(), 2);
        assert_eq!(event_types[0], "30 Min Meeting");
        assert_eq!(event_types[1], "1 Hour Consultation");

        let requests = requests_rx.await.unwrap();
        assert_eq!(requests.len(), 2);

        let user_request = &requests[0];
        assert!(request_method(user_request) == "GET");
        assert!(request_path(user_request).contains("/users/me"));
        assert!(
            user_request.contains("authorization: Bearer test-token")
                || user_request.contains("Authorization: Bearer test-token")
        );

        let events_request = &requests[1];
        assert!(request_method(events_request) == "GET");
        assert!(request_path(events_request).contains("/event_types"));
        assert!(
            events_request.contains("authorization: Bearer test-token")
                || events_request.contains("Authorization: Bearer test-token")
        );
    }

    #[tokio::test]
    async fn create_event_returns_uri() {
        let response = r#"{
            "resource": {
                "uri": "https://api.calendly.com/scheduled_events/EEEE2222",
                "status": "active"
            }
        }"#;

        let (base_url, request_rx) = start_calendly_server(response).await;
        let client = CalendlyClient::with_base_url_for_test("test-token".to_string(), base_url);

        let event_uri = client
            .create_event(
                "https://api.calendly.com/event_types/1",
                "2026-07-21T10:00:00Z",
                "invitee@example.com",
            )
            .await
            .unwrap();

        assert_eq!(event_uri, "https://api.calendly.com/scheduled_events/EEEE2222");

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "POST");
        assert!(request_path(&request).contains("/scheduled_events"));

        let body = request_body(&request);
        assert_eq!(body["event_type"], "https://api.calendly.com/event_types/1");
        assert_eq!(body["start_time"], "2026-07-21T10:00:00Z");
        assert_eq!(body["invitees"][0]["email"], "invitee@example.com");
    }

    #[tokio::test]
    async fn get_free_busy_returns_events() {
        let response = r#"{
            "collection": [
                {
                    "uri": "https://api.calendly.com/scheduled_events/3333",
                    "start_time": "2026-07-21T14:00:00Z",
                    "end_time": "2026-07-21T15:00:00Z",
                    "status": "active"
                }
            ]
        }"#;

        let (base_url, request_rx) = start_calendly_server(response).await;
        let client = CalendlyClient::with_base_url_for_test("test-token".to_string(), base_url);

        let result = client
            .get_free_busy("2026-07-21T00:00:00Z", "2026-07-22T00:00:00Z")
            .await
            .unwrap();

        let json: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json["collection"].as_array().unwrap().len(), 1);
        assert_eq!(
            json["collection"][0]["uri"],
            "https://api.calendly.com/scheduled_events/3333"
        );

        let request = request_rx.await.unwrap();
        assert!(request_method(&request) == "GET");
        assert!(request_path(&request).contains("/scheduled_events"));
        assert!(request.contains("start_time=2026-07-21T00%3A00%3A00Z"));
        assert!(request.contains("end_time=2026-07-22T00%3A00%3A00Z"));
    }

    #[tokio::test]
    async fn access_token_validation_rejects_blank() {
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

        let client = CalendlyClient::with_base_url_for_test("   ".to_string(), base_url);
        let error = client.fetch_event_types().await.unwrap_err();

        assert_eq!(error, "Calendly access token is required");
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;
        assert!(!*request_seen.lock().await);
    }

    #[tokio::test]
    async fn api_error_returns_status() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

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

            let response = "HTTP/1.1 401 Unauthorized\r\ncontent-type: application/json\r\ncontent-length: 26\r\nconnection: close\r\n\r\n{\"error\":\"Invalid token\"}";
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client = CalendlyClient::with_base_url_for_test("invalid-token".to_string(), base_url);
        let error = client.fetch_event_types().await.unwrap_err();

        assert!(error.contains("401"));
=======
pub struct CalendlyClient {
    pub api_key: String,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        CalendlyClient { api_key }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        // Mock implementation to return a vector of event types
        Ok(vec!["30-min Consultation".to_string()])
>>>>>>> 97cc191c1 (perf: tokio RwLock, Redis pool, SSE streaming, unified WS, backpressure, React hooks)
    }
}
