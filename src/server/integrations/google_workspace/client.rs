use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Datelike, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(default)]
    pub parents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailMessage {
    pub id: String,
    #[serde(default)]
    pub snippet: String,
    #[serde(default)]
    pub payload: Option<GmailPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailPayload {
    #[serde(default)]
    pub headers: Vec<GmailHeader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailHeader {
    pub name: String,
    pub value: String,
}

pub struct GoogleWorkspaceClient {
    access_token: String,
    http_client: Client,
    base_url: String,
}

impl GoogleWorkspaceClient {
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

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Google Workspace access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    // ── Google Drive ──────────────────────────────────────────────

    pub async fn list_files(
        &self,
        folder_id: &str,
        page_size: u32,
    ) -> Result<Vec<DriveFile>, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/drive/v3/files", self.base_url.trim_end_matches('/'));

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[
                ("q", format!("'{}' in parents", folder_id).as_str()),
                ("pageSize", &page_size.to_string()),
                ("fields", "files(id,name,mimeType,parents)"),
            ])
            .send()
            .await
            .map_err(|e| format!("Network error listing Drive files: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Drive API error: {}", resp.status()));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Drive response: {}", e))?;

        let files = body["files"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(files)
    }

    pub async fn get_file(&self, file_id: &str) -> Result<DriveFile, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/drive/v3/files/{}",
            self.base_url.trim_end_matches('/'),
            file_id
        );

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("fields", "id,name,mimeType,parents")])
            .send()
            .await
            .map_err(|e| format!("Network error getting Drive file: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Drive API error: {}", resp.status()));
        }

        resp.json::<DriveFile>()
            .await
            .map_err(|e| format!("Failed to parse Drive file: {}", e))
    }

    pub async fn create_file(
        &self,
        name: &str,
        mime_type: &str,
        parent_id: &str,
        content: &[u8],
    ) -> Result<DriveFile, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/upload/drive/v3/files",
            self.base_url.trim_end_matches('/')
        );

        let metadata = serde_json::json!({
            "name": name,
            "mimeType": mime_type,
            "parents": [parent_id],
        });

        let boundary = format!("boundary_{}", uuid::Uuid::new_v4());
        let mut body = Vec::new();

        // metadata part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
        body.extend_from_slice(metadata.to_string().as_bytes());
        body.extend_from_slice(b"\r\n");

        // file content part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", mime_type).as_bytes());
        body.extend_from_slice(content);
        body.extend_from_slice(b"\r\n");

        // closing boundary
        body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .header(
                "Content-Type",
                format!("multipart/related; boundary={}", boundary),
            )
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Network error creating Drive file: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());
            return Err(format!("Drive API error {}: {}", status, text));
        }

        resp.json::<DriveFile>()
            .await
            .map_err(|e| format!("Failed to parse created Drive file: {}", e))
    }

    // ── Google Sheets ─────────────────────────────────────────────

    pub async fn read_range(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<Vec<Vec<String>>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/v4/spreadsheets/{}/values/{}",
            self.base_url
                .trim_end_matches('/')
                .replace("www.googleapis.com", "sheets.googleapis.com"),
            spreadsheet_id,
            range
        );

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Network error reading Sheets range: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Sheets API error: {}", resp.status()));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Sheets response: {}", e))?;

        let rows = body["values"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|row| {
                        row.as_array()
                            .map(|cells| {
                                cells
                                    .iter()
                                    .map(|c| c.as_str().unwrap_or("").to_string())
                                    .collect()
                            })
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(rows)
    }

    pub async fn write_range(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<String>],
    ) -> Result<(), String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/v4/spreadsheets/{}/values/{}",
            self.base_url
                .trim_end_matches('/')
                .replace("www.googleapis.com", "sheets.googleapis.com"),
            spreadsheet_id,
            range
        );

        let payload = serde_json::json!({
            "values": values,
        });

        let resp = self
            .http_client
            .put(&url)
            .bearer_auth(token)
            .query(&[("valueInputOption", "RAW")])
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error writing Sheets range: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Sheets API error: {}", resp.status()));
        }

        Ok(())
    }

    pub async fn create_spreadsheet(&self, title: &str) -> Result<String, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/v4/spreadsheets",
            self.base_url
                .trim_end_matches('/')
                .replace("www.googleapis.com", "sheets.googleapis.com"),
        );

        let payload = serde_json::json!({
            "properties": {
                "title": title,
            },
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating spreadsheet: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());
            return Err(format!("Sheets API error {}: {}", status, text));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse spreadsheet response: {}", e))?;

        body["spreadsheetId"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Response missing spreadsheetId".to_string())
    }

    // ── Gmail ─────────────────────────────────────────────────────

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/gmail/v1/users/me/messages/send",
            self.base_url.trim_end_matches('/')
        );

        let rfc2822_date = rfc2822_date_now();
        let raw_message = format!(
            "To: {}\r\nSubject: {}\r\nDate: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
            to, subject, rfc2822_date, body
        );

        let encoded = base64_encode(raw_message.as_bytes());

        let payload = serde_json::json!({
            "raw": encoded,
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error sending email: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());
            return Err(format!("Gmail API error {}: {}", status, text));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse send response: {}", e))?;

        body["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Response missing message id".to_string())
    }

    pub async fn list_messages(
        &self,
        query: &str,
        max_results: u32,
    ) -> Result<Vec<GmailMessage>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/gmail/v1/users/me/messages",
            self.base_url.trim_end_matches('/')
        );

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("q", query), ("maxResults", &max_results.to_string())])
            .send()
            .await
            .map_err(|e| format!("Network error listing messages: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Gmail API error: {}", resp.status()));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse message list: {}", e))?;

        let message_refs = body["messages"].as_array().cloned().unwrap_or_default();

        let mut messages = Vec::new();
        for msg_ref in message_refs {
            if let Some(id) = msg_ref["id"].as_str() {
                match self.get_message(id).await {
                    Ok(msg) => messages.push(msg),
                    Err(_) => continue,
                }
            }
        }

        Ok(messages)
    }

    pub async fn get_message(&self, message_id: &str) -> Result<GmailMessage, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/gmail/v1/users/me/messages/{}",
            self.base_url.trim_end_matches('/'),
            message_id
        );

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Network error getting message: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Gmail API error: {}", resp.status()));
        }

        resp.json::<GmailMessage>()
            .await
            .map_err(|e| format!("Failed to parse message: {}", e))
    }
}

// ── Helpers ──────────────────────────────────────────────────────

fn rfc2822_date_now() -> String {
    rfc2822_date_at(std::time::SystemTime::now())
}

fn rfc2822_date_at(now: std::time::SystemTime) -> String {
    let secs = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Preserve the epoch fallback for clocks before 1970, and avoid panicking
    // if the host clock is outside Chrono's representable range.
    let date = i64::try_from(secs)
        .ok()
        .and_then(|secs| DateTime::<Utc>::from_timestamp(secs, 0))
        .unwrap_or(DateTime::<Utc>::UNIX_EPOCH);

    // to_rfc2822() does not zero-pad the day. Format the numeric year separately
    // to preserve the existing unsigned spelling even for years beyond 9999.
    format!(
        "{} {} {}",
        date.format("%a, %d %b"),
        date.year(),
        date.format("%H:%M:%S %z")
    )
}

fn base64_encode(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_server(response_body: &'static str) -> (String, oneshot::Receiver<String>) {
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

    #[test]
    fn rfc2822_date_preserves_utc_calendar_wire_vectors() {
        use std::time::{Duration, UNIX_EPOCH};

        let vectors = [
            (0, "Thu, 01 Jan 1970 00:00:00 +0000"),
            (1, "Thu, 01 Jan 1970 00:00:01 +0000"),
            (86_399, "Thu, 01 Jan 1970 23:59:59 +0000"),
            (86_400, "Fri, 02 Jan 1970 00:00:00 +0000"),
            (951_782_400, "Tue, 29 Feb 2000 00:00:00 +0000"),
            (951_868_800, "Wed, 01 Mar 2000 00:00:00 +0000"),
            (1_709_164_800, "Thu, 29 Feb 2024 00:00:00 +0000"),
            (1_709_251_200, "Fri, 01 Mar 2024 00:00:00 +0000"),
            (1_791_158_400, "Mon, 05 Oct 2026 00:00:00 +0000"),
            (4_107_542_399, "Sun, 28 Feb 2100 23:59:59 +0000"),
            (4_107_542_400, "Mon, 01 Mar 2100 00:00:00 +0000"),
            (13_574_563_200, "Tue, 29 Feb 2400 00:00:00 +0000"),
            (13_574_649_600, "Wed, 01 Mar 2400 00:00:00 +0000"),
            (253_402_300_799, "Fri, 31 Dec 9999 23:59:59 +0000"),
            (253_402_300_800, "Sat, 01 Jan 10000 00:00:00 +0000"),
        ];

        for (seconds, expected) in vectors {
            let now = UNIX_EPOCH + Duration::from_secs(seconds);
            assert_eq!(rfc2822_date_at(now), expected, "timestamp: {seconds}");
        }
    }

    #[test]
    fn rfc2822_date_keeps_epoch_fallback_and_truncates_subseconds() {
        use std::time::{Duration, UNIX_EPOCH};

        assert_eq!(
            rfc2822_date_at(UNIX_EPOCH - Duration::from_nanos(1)),
            "Thu, 01 Jan 1970 00:00:00 +0000"
        );
        assert_eq!(
            rfc2822_date_at(UNIX_EPOCH - Duration::from_secs(86_400)),
            "Thu, 01 Jan 1970 00:00:00 +0000"
        );
        assert_eq!(
            rfc2822_date_at(UNIX_EPOCH + Duration::new(86_399, 999_999_999)),
            "Thu, 01 Jan 1970 23:59:59 +0000"
        );
    }

    #[test]
    fn rfc2822_date_falls_back_for_representable_out_of_range_clock() {
        use std::time::{Duration, UNIX_EPOCH};

        let seconds = DateTime::<Utc>::MAX_UTC.timestamp() as u64 + 1;
        let Some(now) = UNIX_EPOCH.checked_add(Duration::from_secs(seconds)) else {
            eprintln!("platform SystemTime cannot represent a clock beyond Chrono's range");
            return;
        };
        assert_eq!(rfc2822_date_at(now), "Thu, 01 Jan 1970 00:00:00 +0000");
    }

    #[test]
    fn base64_encode_preserves_url_safe_unpadded_wire_vectors() {
        // RFC 4648 ASCII vectors plus independently encoded binary/UTF-8 cases.
        // The binary tails distinguish URL-safe and padded engines.
        let vectors: &[(&[u8], &str)] = &[
            (b"", ""),
            (b"f", "Zg"),
            (b"fo", "Zm8"),
            (b"foo", "Zm9v"),
            (b"foob", "Zm9vYg"),
            (b"fooba", "Zm9vYmE"),
            (b"foobar", "Zm9vYmFy"),
            (b"\0", "AA"),
            (b"\0\0", "AAA"),
            (b"\xfb", "-w"),
            (b"\xfb\xff", "-_8"),
            (b"\xfb\xff\xff", "-___"),
            (b"=", "PQ"),
            ("Café ☕".as_bytes(), "Q2Fmw6kg4piV"),
            ("こんにちは世界".as_bytes(), "44GT44KT44Gr44Gh44Gv5LiW55WM"),
            ("🙂 café\r\n正文".as_bytes(), "8J-ZgiBjYWbDqQ0K5q2j5paH"),
        ];

        for (input, expected) in vectors {
            assert_eq!(base64_encode(input), *expected, "input bytes: {input:?}");
        }
    }

    #[tokio::test]
    async fn list_files_returns_parsed_drive_files() {
        let response = r#"{
            "files": [
                {"id": "f1", "name": "doc.txt", "mimeType": "text/plain", "parents": ["root"]},
                {"id": "f2", "name": "image.png", "mimeType": "image/png", "parents": ["root"]}
            ]
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            GoogleWorkspaceClient::with_base_url_for_test("valid-token".to_string(), base_url);

        let files = client.list_files("root", 10).await.unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].id, "f1");
        assert_eq!(files[0].name, "doc.txt");
        assert_eq!(files[1].name, "image.png");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /drive/v3/files?"));
        assert!(request.contains("q=%27root%27+in+parents"));
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );
    }

    #[tokio::test]
    async fn get_file_returns_single_drive_file() {
        let response =
            r#"{"id": "abc", "name": "report.pdf", "mimeType": "application/pdf", "parents": []}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            GoogleWorkspaceClient::with_base_url_for_test("my-token".to_string(), base_url);

        let file = client.get_file("abc").await.unwrap();
        assert_eq!(file.id, "abc");
        assert_eq!(file.name, "report.pdf");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /drive/v3/files/abc?"));
    }

    #[tokio::test]
    async fn read_range_returns_sheet_values() {
        let response = r#"{"values": [["A1", "B1"], ["A2", "B2"]]}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            GoogleWorkspaceClient::with_base_url_for_test("sheet-token".to_string(), base_url);

        let values = client.read_range("spreadsheet123", "A1:B2").await.unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], vec!["A1", "B1"]);
        assert_eq!(values[1], vec!["A2", "B2"]);

        let request = request_rx.await.unwrap();
        assert!(request.contains("/v4/spreadsheets/spreadsheet123/values/A1:B2"));
    }

    #[tokio::test]
    async fn create_spreadsheet_returns_id() {
        let response =
            r#"{"spreadsheetId": "sheet-abc-123", "properties": {"title": "New Sheet"}}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client = GoogleWorkspaceClient::with_base_url_for_test("token".to_string(), base_url);

        let id = client.create_spreadsheet("New Sheet").await.unwrap();
        assert_eq!(id, "sheet-abc-123");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /v4/spreadsheets"));
        let body = request_body(&request);
        assert_eq!(body["properties"]["title"], "New Sheet");
    }

    #[tokio::test]
    async fn list_messages_returns_parsed_messages() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            // Request 1: message list
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).position(|w| w == b"\r\n\r\n").is_some() {
                    break;
                }
            }
            let list_response = r#"{"messages": [{"id": "msg1"}, {"id": "msg2"}]}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                list_response.len(),
                list_response
            );
            stream.write_all(resp.as_bytes()).await.unwrap();

            // Request 2: get message msg1
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).position(|w| w == b"\r\n\r\n").is_some() {
                    break;
                }
            }
            let msg1_response = r#"{"id": "msg1", "snippet": "Hello world"}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                msg1_response.len(),
                msg1_response
            );
            stream.write_all(resp.as_bytes()).await.unwrap();

            // Request 3: get message msg2
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).position(|w| w == b"\r\n\r\n").is_some() {
                    break;
                }
            }
            let msg2_response = r#"{"id": "msg2", "snippet": "Test email"}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                msg2_response.len(),
                msg2_response
            );
            stream.write_all(resp.as_bytes()).await.unwrap();
        });

        let client =
            GoogleWorkspaceClient::with_base_url_for_test("gmail-token".to_string(), base_url);

        let messages = client.list_messages("is:unread", 5).await.unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].id, "msg1");
        assert_eq!(messages[0].snippet, "Hello world");
        assert_eq!(messages[1].id, "msg2");
        assert_eq!(messages[1].snippet, "Test email");
    }

    #[tokio::test]
    async fn send_email_posts_to_gmail_api() {
        let response = r#"{"id": "sent-123", "labelIds": ["SENT"]}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            GoogleWorkspaceClient::with_base_url_for_test("gmail-token".to_string(), base_url);

        let msg_id = client
            .send_email("user@example.com", "Test", "Body text")
            .await
            .unwrap();
        assert_eq!(msg_id, "sent-123");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /gmail/v1/users/me/messages/send"));
        let body = request_body(&request);
        assert!(body["raw"].is_string());
    }

    #[tokio::test]
    async fn send_email_preserves_raw_mime_bytes() {
        for (subject, message_body) in [
            ("Test", "Body text"),
            ("Café ☕", "こんにちは世界\r\n🙂 café\0正文"),
            ("", ""),
        ] {
            let (base_url, request_rx) = start_server(r#"{"id":"sent-123"}"#).await;
            let client =
                GoogleWorkspaceClient::with_base_url_for_test("gmail-token".to_string(), base_url);

            assert_eq!(
                client
                    .send_email("user@example.com", subject, message_body)
                    .await
                    .unwrap(),
                "sent-123"
            );

            let request = request_rx.await.unwrap();
            assert!(request.starts_with("POST /gmail/v1/users/me/messages/send"));
            let payload = request_body(&request);
            assert_eq!(payload.as_object().unwrap().len(), 1);
            let raw = payload["raw"].as_str().unwrap();
            assert!(!raw.contains(['=', '+', '/']));
            let decoded = URL_SAFE_NO_PAD.decode(raw).unwrap();
            let message = std::str::from_utf8(&decoded).unwrap();
            let date = message
                .split("\r\n")
                .find_map(|line| line.strip_prefix("Date: "))
                .unwrap();
            assert!(!date.is_empty());
            let expected = format!(
                "To: user@example.com\r\nSubject: {subject}\r\nDate: {date}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{message_body}"
            );
            assert_eq!(decoded, expected.as_bytes());
        }
    }

    #[tokio::test]
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client = GoogleWorkspaceClient::with_base_url_for_test("   ".to_string(), base_url);
        let error = client.list_files("root", 10).await.unwrap_err();
        assert_eq!(error, "Google Workspace access token is required");
    }

    #[tokio::test]
    async fn reports_api_error_status() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let response =
                "HTTP/1.1 403 Forbidden\r\ncontent-length: 0\r\nconnection: close\r\n\r\n";
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client = GoogleWorkspaceClient::with_base_url_for_test("token".to_string(), base_url);
        let error = client.get_file("x").await.unwrap_err();
        assert!(error.contains("403"));
    }
}
