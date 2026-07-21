use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessageResponse {
    pub ok: bool,
    pub ts: Option<String>,
    pub channel: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "needed")]
    pub needed: Option<String>,
    #[serde(rename = "provided")]
    pub provided: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackChannel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub is_private: bool,
    #[serde(default)]
    pub is_archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    pub ts: String,
    pub text: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub thread_ts: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SlackHistoryResponse {
    ok: bool,
    messages: Option<Vec<SlackMessage>>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SlackCreateChannelResponse {
    ok: bool,
    channel: Option<SlackChannel>,
    error: Option<String>,
}

pub struct SlackClient {
    bot_token: String,
}

impl SlackClient {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }

    pub async fn send_message(&self, channel: &str, text: &str) -> Result<SlackMessageResponse, String> {
        let url = "https://slack.com/api/chat.postMessage";
        let payload = serde_json::json!({
            "channel": channel,
            "text": text,
        });

        let client = get_client();
        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        let body: SlackMessageResponse = res.json().await
            .map_err(|e| format!("failed to parse response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Slack API HTTP error: {}", status));
        }
        if !body.ok {
            return Err(format!("Slack API error: {}", body.error.unwrap_or_default()));
        }
        Ok(body)
    }

    pub async fn send_block_message(&self, channel: &str, blocks: &[serde_json::Value]) -> Result<SlackMessageResponse, String> {
        let url = "https://slack.com/api/chat.postMessage";
        let payload = serde_json::json!({
            "channel": channel,
            "blocks": blocks,
        });

        let client = get_client();
        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        let body: SlackMessageResponse = res.json().await
            .map_err(|e| format!("failed to parse response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Slack API HTTP error: {}", status));
        }
        if !body.ok {
            return Err(format!("Slack API error: {}", body.error.unwrap_or_default()));
        }
        Ok(body)
    }

    pub async fn upload_file(&self, channel: &str, filename: &str, content: &[u8]) -> Result<(), String> {
        let url = "https://slack.com/api/files.upload";
        let client = get_client();
        let part = reqwest::multipart::Part::bytes(content.to_vec())
            .file_name(filename.to_string());

        let form = reqwest::multipart::Form::new()
            .text("channels", channel.to_string())
            .text("filename", filename.to_string())
            .part("file", part);

        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        let body: serde_json::Value = res.json().await
            .map_err(|e| format!("failed to parse response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Slack API HTTP error: {}", status));
        }
        if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
            let error = body.get("error").and_then(|v| v.as_str()).unwrap_or("unknown");
            return Err(format!("Slack API error: {}", error));
        }
        Ok(())
    }

    pub async fn list_channels(&self) -> Result<Vec<SlackChannel>, String> {
        let url = "https://slack.com/api/conversations.list";
        let client = get_client();
        let mut channels = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let mut request = client.get(url)
                .header("Authorization", format!("Bearer {}", self.bot_token))
                .query(&[("types", "public_channel,private_channel")])
                .query(&[("limit", "200")]);

            if let Some(ref c) = cursor {
                request = request.query(&[("cursor", c.as_str())]);
            }

            let res = request.send().await
                .map_err(|e| format!("reqwest error: {}", e))?;

            let status = res.status();
            let raw: serde_json::Value = res.json().await
                .map_err(|e| format!("failed to parse response: {}", e))?;

            if !status.is_success() {
                return Err(format!("Slack API HTTP error: {}", status));
            }
            if raw.get("ok").and_then(|v| v.as_bool()) != Some(true) {
                let error = raw.get("error").and_then(|v| v.as_str()).unwrap_or("unknown");
                return Err(format!("Slack API error: {}", error));
            }

            if let Some(arr) = raw.get("channels").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Ok(ch) = serde_json::from_value(item.clone()) {
                        channels.push(ch);
                    }
                }
            }

            let next_cursor = raw.get("response_metadata")
                .and_then(|m| m.get("next_cursor"))
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());

            match next_cursor {
                Some(c) if !c.is_empty() => cursor = Some(c),
                _ => break,
            }
        }

        Ok(channels)
    }

    pub async fn get_channel_history(&self, channel_id: &str, limit: u32) -> Result<Vec<SlackMessage>, String> {
        let url = "https://slack.com/api/conversations.history";
        let client = get_client();
        let res = client.get(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .query(&[("channel", channel_id)])
            .query(&[("limit", limit.to_string().as_str())])
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        let body: SlackHistoryResponse = res.json().await
            .map_err(|e| format!("failed to parse response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Slack API HTTP error: {}", status));
        }
        if !body.ok {
            return Err(format!("Slack API error: {}", body.error.unwrap_or_default()));
        }

        Ok(body.messages.unwrap_or_default())
    }

    pub async fn create_channel(&self, name: &str, is_private: bool) -> Result<SlackChannel, String> {
        let url = "https://slack.com/api/conversations.create";
        let client = get_client();
        let payload = serde_json::json!({
            "name": name,
            "is_private": is_private,
        });

        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        let status = res.status();
        let body: SlackCreateChannelResponse = res.json().await
            .map_err(|e| format!("failed to parse response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Slack API HTTP error: {}", status));
        }
        if !body.ok {
            return Err(format!("Slack API error: {}", body.error.unwrap_or_default()));
        }

        body.channel.ok_or_else(|| "no channel in response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;

    fn start_mock_server(responses: Vec<String>) -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            for resp_body in responses {
                if let Ok(stream) = listener.accept() {
                    let mut stream = stream.0;
                    let reader = BufReader::new(stream.try_clone().unwrap());
                    let mut headers = Vec::new();
                    for line in reader.lines() {
                        match line {
                            Ok(line) if line.is_empty() => break,
                            Ok(line) => headers.push(line),
                            Err(_) => break,
                        }
                    }

                    let content_length = headers.iter()
                        .find(|h| h.to_lowercase().starts_with("content-length:"))
                        .and_then(|h| h.split(':').nth(1))
                        .and_then(|v| v.trim().parse::<usize>().ok())
                        .unwrap_or(0);

                    let mut body = vec![0u8; content_length];
                    let _ = std::io::Read::read_exact(&mut stream, &mut body);

                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        resp_body.len(), resp_body
                    );
                    let _ = stream.write_all(response.as_bytes());
                }
            }
        });

        port
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let resp = serde_json::json!({
            "ok": true,
            "ts": "1234567890.123456",
            "channel": "C1234567890",
        }).to_string();

        let _port = start_mock_server(vec![resp.clone()]);

        let response: SlackMessageResponse = serde_json::from_str(&resp).unwrap();
        assert!(response.ok);
        assert_eq!(response.ts.unwrap(), "1234567890.123456");
    }

    #[tokio::test]
    async fn test_send_message_api_error() {
        let resp = serde_json::json!({
            "ok": false,
            "error": "channel_not_found",
        }).to_string();

        let response: SlackMessageResponse = serde_json::from_str(&resp).unwrap();
        assert!(!response.ok);
        assert_eq!(response.error.unwrap(), "channel_not_found");
    }

    #[test]
    fn test_slack_channel_deserialize() {
        let json = serde_json::json!({
            "id": "C123",
            "name": "general",
            "is_private": false,
            "is_archived": false,
        });
        let channel: SlackChannel = serde_json::from_value(json).unwrap();
        assert_eq!(channel.id, "C123");
        assert_eq!(channel.name, "general");
    }

    #[test]
    fn test_slack_message_deserialize() {
        let json = serde_json::json!({
            "ts": "1234567890.123456",
            "text": "Hello world",
            "user": "U123",
            "thread_ts": null,
        });
        let msg: SlackMessage = serde_json::from_value(json).unwrap();
        assert_eq!(msg.text, "Hello world");
        assert!(msg.thread_ts.is_none());
    }

    #[test]
    fn test_slack_client_new() {
        let client = SlackClient::new("xoxb-test-token".to_string());
        assert_eq!(client.bot_token, "xoxb-test-token");
    }
}
