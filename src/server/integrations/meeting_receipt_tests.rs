// Included by the Zoom and Daily client tests to exercise each real HTTP path.
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn call_with_response(response: Vec<u8>) -> (Result<String, String>, Vec<Vec<u8>>) {
    let mut client = TestedClient::new("local-fixture-token".into());
    client.http_client = TestedClient::http_client_builder(std::time::Duration::from_secs(30))
        .no_proxy()
        .build()
        .unwrap();
    call_with_client_response(client, response, false).await
}

async fn call_with_client_response(
    client: TestedClient,
    response: Vec<u8>,
    hold_connection_open: bool,
) -> (Result<String, String>, Vec<Vec<u8>>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let captured = requests.clone();
    let server = tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 1024];
            loop {
                let count = stream.read(&mut buffer).await.unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..count]);
                assert!(request.len() < 16_384);
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if request.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            captured.lock().unwrap().push(request);
            let _ = stream.write_all(&response).await;
            if hold_connection_open {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    });
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        client.create_meeting_at("Fixture Topic", &format!("http://{address}/create")),
    )
    .await
    .expect("local fixture request must complete");
    server.abort();
    let recorded = requests.lock().unwrap().clone();
    assert_eq!(
        recorded.len(),
        1,
        "a mutating creation must not be replayed"
    );
    (result, recorded)
}

fn http_response(status: &str, body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()
    ).into_bytes();
    response.extend_from_slice(body);
    response
}

fn receipt(value: serde_json::Value) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({RECEIPT_FIELD: value})).unwrap()
}

async fn assert_unknown(body: &[u8]) {
    let (result, _) = call_with_response(http_response("201 Created", body)).await;
    let error = result.expect_err("invalid receipt must not invent a meeting URL");
    assert!(error.contains("outcome unknown"), "{error}");
    assert!(error.contains("reconcile"), "{error}");
    assert!(!error.contains("local-fixture-token"));
}

#[tokio::test]
async fn accepts_actual_url_and_additive_provider_metadata() {
    let url = "https://tenant.example.test/room%20one?invite=fixture";
    let body = serde_json::to_vec(&serde_json::json!({
        RECEIPT_FIELD: url, "id": 123, "future_metadata": {"enabled": true}
    }))
    .unwrap();
    let (result, requests) = call_with_response(http_response("201 Created", &body)).await;
    assert_eq!(result.unwrap(), url);
    let request = String::from_utf8_lossy(&requests[0]);
    assert!(request.starts_with("POST /create HTTP/1.1\r\n"));
    assert!(request.contains("authorization: Bearer local-fixture-token\r\n"));
    let payload: serde_json::Value =
        serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert!(payload.get("topic").is_some() || payload.get("name").is_some());
}

#[tokio::test]
async fn rejects_empty_and_malformed_json_receipts() {
    for body in [b"".as_slice(), b"not json", b"{", b"null", b"[]"] {
        assert_unknown(body).await;
    }
}

#[tokio::test]
async fn rejects_missing_and_wrong_type_receipts() {
    assert_unknown(b"{}").await;
    for value in [
        serde_json::Value::Null,
        serde_json::json!(42),
        serde_json::json!(true),
    ] {
        assert_unknown(&receipt(value)).await;
    }
}

#[tokio::test]
async fn rejects_empty_relative_unsafe_and_credential_urls() {
    for value in [
        "",
        "   ",
        "/relative",
        "javascript:alert(1)",
        "http://example.test/room",
        "https://user:secret@example.test/room",
        " https://example.test/room",
        "https://exa\nmple.test/room",
    ] {
        assert_unknown(&receipt(serde_json::json!(value))).await;
    }
}

#[tokio::test]
async fn rejects_duplicate_receipt_fields() {
    let body = format!(
        "{{\"{RECEIPT_FIELD}\":\"https://a.example.test/room\",\"{RECEIPT_FIELD}\":\"https://b.example.test/room\"}}"
    );
    assert_unknown(body.as_bytes()).await;
}

#[tokio::test]
async fn truncated_success_body_is_unknown_without_replay() {
    let (result, _) = call_with_response(
        b"HTTP/1.1 201 Created\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort".to_vec(),
    )
    .await;
    assert!(result.unwrap_err().contains("outcome unknown"));
}

#[tokio::test]
async fn disconnect_after_request_is_unknown_without_replay() {
    let (result, _) = call_with_response(Vec::new()).await;
    assert!(result.unwrap_err().contains("outcome unknown"));
}

#[tokio::test]
async fn provider_errors_never_return_meeting_urls_or_response_secrets() {
    for status in [
        "401 Unauthorized",
        "403 Forbidden",
        "429 Too Many Requests",
        "503 Unavailable",
    ] {
        let (result, _) =
            call_with_response(http_response(status, b"secret-provider-response")).await;
        let error = result.unwrap_err();
        assert!(!error.contains("secret-provider-response"));
    }
}

#[tokio::test]
async fn rejects_oversized_receipt_body() {
    let mut body = receipt(serde_json::json!("https://tenant.example.test/room"));
    body.resize(65_537, b' ');
    assert_unknown(&body).await;
}

#[tokio::test]
async fn rejects_oversized_chunked_receipt_body() {
    let mut response =
        b"HTTP/1.1 201 Created\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n".to_vec();
    let mut body = receipt(serde_json::json!("https://tenant.example.test/room"));
    body.resize(65_537, b' ');
    response.extend_from_slice(format!("{:X}\r\n", body.len()).as_bytes());
    response.extend_from_slice(&body);
    response.extend_from_slice(b"\r\n0\r\n\r\n");
    let (result, _) = call_with_response(response).await;
    assert!(result.unwrap_err().contains("outcome unknown"));
}

#[tokio::test]
async fn accepts_exactly_64_kib_receipt_body() {
    let url = "https://tenant.example.test/room";
    let mut body = receipt(serde_json::json!(url));
    body.resize(65_536, b' ');
    let (result, _) = call_with_response(http_response("201 Created", &body)).await;
    assert_eq!(result.unwrap(), url);
}

#[tokio::test]
async fn redirects_do_not_replay_post_or_turn_into_success() {
    let response = b"HTTP/1.1 307 Temporary Redirect\r\nLocation: /unexpected\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec();
    let (result, _) = call_with_response(response).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn stalled_receipt_obeys_client_deadline_and_is_unknown() {
    let mut client = TestedClient::new("local-fixture-token".into());
    // Use the production builder with a short deadline to exercise a stalled
    // body deterministically without spending the normal 30-second deadline.
    client.http_client = TestedClient::http_client_builder(std::time::Duration::from_millis(50))
        .no_proxy()
        .build()
        .unwrap();
    let response =
        b"HTTP/1.1 201 Created\r\nContent-Length: 100\r\nConnection: close\r\n\r\n".to_vec();
    let started = std::time::Instant::now();
    let (result, _) = call_with_client_response(client, response, true).await;
    assert!(result.unwrap_err().contains("outcome unknown"));
    assert!(started.elapsed() < std::time::Duration::from_millis(750));
}
