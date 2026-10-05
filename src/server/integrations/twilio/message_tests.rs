use super::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const ACCOUNT: &str = "AC11111111111111111111111111111111";
const SID: &str = "SM22222222222222222222222222222222";

enum Reply {
    Http(u16, String),
    Disconnect,
    Stall,
    StallBody,
    Redirect,
    Chunked(String),
}

struct Provider {
    client: RealTwilioClient,
    requests: Arc<Mutex<Vec<String>>>,
    task: tokio::task::JoinHandle<()>,
}

impl Drop for Provider {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Provider {
    async fn start(reply: Reply) -> Self {
        let request_timeout = if matches!(&reply, Reply::Stall | Reply::StallBody) {
            Duration::from_millis(200)
        } else {
            Duration::from_secs(2)
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = requests.clone();
        let task = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0_u8; 1024];
                let header_end = loop {
                    let size = stream.read(&mut buffer).await.unwrap();
                    assert!(size > 0, "client closed before sending request headers");
                    request.extend_from_slice(&buffer[..size]);
                    assert!(request.len() < 16384);
                    if let Some(index) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                        break index + 4;
                    }
                };
                let headers = std::str::from_utf8(&request[..header_end]).unwrap();
                let length = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|value| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap();
                while request.len() < header_end + length {
                    let size = stream.read(&mut buffer).await.unwrap();
                    assert!(size > 0, "client closed before sending request body");
                    request.extend_from_slice(&buffer[..size]);
                }
                recorded
                    .lock()
                    .unwrap()
                    .push(String::from_utf8(request).unwrap());
                match &reply {
                    Reply::Disconnect => continue,
                    Reply::Stall => {
                        let _stream = stream;
                        std::future::pending::<()>().await;
                    }
                    Reply::StallBody => {
                        stream
                            .write_all(b"HTTP/1.1 201 Created\r\nContent-Length: 100\r\n\r\n{")
                            .await
                            .unwrap();
                        let _stream = stream;
                        std::future::pending::<()>().await;
                    }
                    Reply::Redirect => {
                        stream.write_all(b"HTTP/1.1 307 Temporary Redirect\r\nLocation: /replayed\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await.unwrap();
                    }
                    Reply::Chunked(body) => {
                        let wire = format!(
                            "HTTP/1.1 201 Created\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{body}\r\n0\r\n\r\n",
                            body.len()
                        );
                        let _ = stream.write_all(wire.as_bytes()).await;
                    }
                    Reply::Http(status, body) => {
                        let wire = format!(
                            "HTTP/1.1 {status} fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                            body.len()
                        );
                        // Oversized-response tests may close the socket before the full write.
                        let _ = stream.write_all(wire.as_bytes()).await;
                    }
                }
            }
        });
        Self {
            client: RealTwilioClient {
                account_sid: ACCOUNT.into(),
                auth_token: "synthetic-loopback-only".into(),
                http_client: RealTwilioClient::http_client_builder()
                    .no_proxy()
                    .timeout(request_timeout)
                    .build()
                    .unwrap(),
                provisioning_api: format!("http://{address}"),
            },
            requests,
            task,
        }
    }

    async fn send(&self, whatsapp: bool) -> Result<MessageReceipt, MessageSendError> {
        let result = tokio::time::timeout(Duration::from_secs(4), async {
            if whatsapp {
                self.client
                    .send_whatsapp("+14155550123", "+14155550456", "café 😀=a&b")
                    .await
            } else {
                self.client
                    .send_sms("+14155550123", "+14155550456", "café 😀=a&b")
                    .await
            }
        })
        .await
        .expect("message attempt must have a bounded deadline");
        assert!(!self.task.is_finished(), "recording HTTP provider failed");
        let requests = self.requests.lock().unwrap();
        assert_eq!(
            requests.len(),
            1,
            "a mutating message request must never be replayed automatically"
        );
        let request = &requests[0];
        assert!(request.starts_with(&format!(
            "POST /2010-04-01/Accounts/{ACCOUNT}/Messages.json "
        )));
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: basic ")
        );
        let prefix = if whatsapp { "whatsapp%3A" } else { "" };
        assert!(request.ends_with(&format!("To={prefix}%2B14155550123&From={prefix}%2B14155550456&Body=caf%C3%A9+%F0%9F%98%80%3Da%26b")));
        result
    }
}

fn receipt() -> String {
    serde_json::json!({"sid": SID, "account_sid": ACCOUNT, "status": "queued"}).to_string()
}

#[tokio::test]
async fn sms_and_whatsapp_return_the_provider_message_sid() {
    for whatsapp in [false, true] {
        let provider = Provider::start(Reply::Http(201, receipt())).await;
        let result = provider.send(whatsapp).await.unwrap();
        assert_eq!(result.sid, SID);
    }
}

#[tokio::test]
async fn provider_rejections_are_terminal_and_never_replayed() {
    for status in [400, 401, 403, 429] {
        let provider = Provider::start(Reply::Http(status, "{}".into())).await;
        assert_eq!(
            provider.send(false).await,
            Err(MessageSendError::Rejected { status })
        );
    }
}

#[tokio::test]
async fn server_errors_are_unknown_and_never_replayed() {
    for whatsapp in [false, true] {
        let provider = Provider::start(Reply::Http(503, "{}".into())).await;
        assert_unknown(provider.send(whatsapp).await);
    }
}

#[tokio::test]
async fn disconnect_after_request_body_requires_reconciliation_without_replay() {
    for whatsapp in [false, true] {
        let provider = Provider::start(Reply::Disconnect).await;
        assert_unknown(provider.send(whatsapp).await);
    }
}

#[tokio::test]
async fn stalled_headers_and_body_have_bounded_unknown_outcomes() {
    for reply in [Reply::Stall, Reply::StallBody] {
        let provider = Provider::start(reply).await;
        let start = std::time::Instant::now();
        let result = provider.send(false).await;
        assert_unknown(result);
        assert!(start.elapsed() < Duration::from_secs(2));
    }
}

#[tokio::test]
async fn malformed_success_receipts_are_unknown() {
    let mut bodies = vec!["".to_string(), "{".into(), "{}".into()];
    for (key, value) in [
        ("sid", serde_json::json!("")),
        ("sid", serde_json::json!(123)),
        (
            "sid",
            serde_json::json!("PN22222222222222222222222222222222"),
        ),
        (
            "sid",
            serde_json::json!("SMzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"),
        ),
        (
            "account_sid",
            serde_json::json!("AC33333333333333333333333333333333"),
        ),
        ("account_sid", serde_json::Value::Null),
        ("error_code", serde_json::json!(21610)),
        ("error_message", serde_json::json!("failed")),
        ("status", serde_json::json!("failed")),
        ("success", serde_json::json!(false)),
        ("error", serde_json::json!({"code":21610})),
    ] {
        let mut response: serde_json::Value = serde_json::from_str(&receipt()).unwrap();
        response[key] = value;
        bodies.push(response.to_string());
    }
    for body in bodies {
        let provider = Provider::start(Reply::Http(201, body)).await;
        assert_unknown(provider.send(false).await);
    }
}

#[tokio::test]
async fn redirects_cannot_replay_the_post() {
    let provider = Provider::start(Reply::Redirect).await;
    assert_unknown(provider.send(false).await);
}

#[tokio::test]
async fn advertised_and_chunked_receipts_enforce_the_body_cap() {
    let mut body: serde_json::Value = serde_json::from_str(&receipt()).unwrap();
    body["padding"] = "x".repeat(65_536).into();
    for reply in [
        Reply::Http(201, body.to_string()),
        Reply::Chunked(body.to_string()),
    ] {
        let provider = Provider::start(reply).await;
        assert_eq!(
            provider.send(false).await,
            Err(MessageSendError::UnknownOutcome {
                reason: "receipt exceeded size limit"
            })
        );
    }
}

#[tokio::test]
async fn mms_message_sid_is_a_valid_receipt_too() {
    let provider = Provider::start(Reply::Http(201, receipt().replace("SM222", "MM222"))).await;
    let result = provider.send(false).await.unwrap();
    assert_eq!(result.sid, "MM22222222222222222222222222222222");
}

fn assert_unknown(result: Result<MessageReceipt, MessageSendError>) {
    let error = result.unwrap_err();
    assert!(
        matches!(error, MessageSendError::UnknownOutcome { .. }),
        "{error:?}"
    );
    assert!(error.to_string().contains("reconcile before retrying"));
}
