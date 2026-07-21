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
        let url = format!(
            "{}/drive/v3/files",
            self.base_url.trim_end_matches('/')
        );

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
        body.extend_from_slice(
            b"Content-Type: application/json; charset=UTF-8\r\n\r\n",
        );
        body.extend_from_slice(metadata.to_string().as_bytes());
        body.extend_from_slice(b"\r\n");

        // file content part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            format!("Content-Type: {}\r\n\r\n", mime_type).as_bytes(),
        );
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

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<String, String> {
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
            .query(&[
                ("q", query),
                ("maxResults", &max_results.to_string()),
            ])
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

        let message_refs = body["messages"]
            .as_array()
            .cloned()
            .unwrap_or_default();

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
    use std::time::{SystemTime, UNIX_EPOCH};

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let days = secs / 86400;
    let mut y = 1970u32;
    let mut remaining = days as u32;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let leap = is_leap(y);
    let month_days: [u32; 12] = [
        31,
        if leap { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let day_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    let mut m = 0u32;
    let mut d = remaining as u32;
    while m < 12 && d >= month_days[m as usize] {
        d -= month_days[m as usize];
        m += 1;
    }
    let day_of_week = ((days + 4) % 7) as usize; // Jan 1 1970 was a Thursday

    let time_secs = secs % 86400;
    let hh = time_secs / 3600;
    let mm = (time_secs % 3600) / 60;
    let ss = time_secs % 60;

    format!(
        "{}, {:02} {} {} {:02}:{:02}:{:02} +0000",
        day_names[day_of_week],
        d + 1,
        month_names[m as usize],
        y,
        hh,
        mm,
        ss
    )
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut result = Vec::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize]);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize]);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize]);
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize]);
        }
    }
    String::from_utf8(result).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_server(
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

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap()
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
        let response = r#"{"id": "abc", "name": "report.pdf", "mimeType": "application/pdf", "parents": []}"#;
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
        let response = r#"{"spreadsheetId": "sheet-abc-123", "properties": {"title": "New Sheet"}}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            GoogleWorkspaceClient::with_base_url_for_test("token".to_string(), base_url);

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
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client =
            GoogleWorkspaceClient::with_base_url_for_test("   ".to_string(), base_url);
        let error = client.list_files("root", 10).await.unwrap_err();
        assert_eq!(error, "Google Workspace access token is required");
    }

    #[tokio::test]
    async fn reports_api_error_status() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let response = "HTTP/1.1 403 Forbidden\r\ncontent-length: 0\r\nconnection: close\r\n\r\n";
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client =
            GoogleWorkspaceClient::with_base_url_for_test("token".to_string(), base_url);
        let error = client.get_file("x").await.unwrap_err();
        assert!(error.contains("403"));
    }
}
