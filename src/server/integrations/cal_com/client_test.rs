use super::*;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn fixture(response: Vec<u8>, booking_link: bool) -> (Result<String, String>, Vec<Vec<u8>>) {
    fixture_action(
        response,
        if booking_link {
            Action::BookingLink
        } else {
            Action::CreateEvent
        },
    )
    .await
}

enum Action {
    CreateEvent,
    BookingLink,
    Availability,
}

async fn fixture_action(
    response: Vec<u8>,
    action: Action,
) -> (Result<String, String>, Vec<Vec<u8>>) {
    fixture_with_deadline(response, action, Duration::from_secs(30), false).await
}

async fn fixture_with_deadline(
    response: Vec<u8>,
    action: Action,
    deadline: Duration,
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
            let mut buffer = [0; 1024];
            loop {
                let count = stream.read(&mut buffer).await.unwrap();
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&buffer[..count]);
                assert!(request.len() < 16384);
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (key, value) = line.split_once(':')?;
                            key.eq_ignore_ascii_case("content-length")
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
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    });
    let mut client = CalComClient::new("sensitive-fixture-token".into());
    client.api_base_url = format!("http://{address}");
    client.http_client = CalComClient::http_client_builder(deadline)
        .no_proxy()
        .build()
        .unwrap();
    let result = match action {
        Action::BookingLink => client.get_booking_link("consultation").await,
        Action::Availability => {
            client
                .get_free_busy("2026-10-05T10:00:00Z", "2026-10-05T10:30:00Z")
                .await
        }
        Action::CreateEvent => {
            client
                .create_event("Fixture", "2026-10-05T10:00:00Z", "2026-10-05T10:30:00Z")
                .await
        }
    };
    server.abort();
    let captured = requests.lock().unwrap().clone();
    (result, captured)
}

fn response(body: &[u8]) -> Vec<u8> {
    let mut bytes = format!("HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).into_bytes();
    bytes.extend_from_slice(body);
    bytes
}

#[tokio::test]
async fn preserves_existing_nonempty_string_id_contract_without_claiming_v1_schema_parity() {
    let (result, requests) = fixture(
        response(br#"{"booking":{"id":"existing-opaque-id"},"extra":true}"#),
        false,
    )
    .await;
    assert_eq!(result.unwrap(), "existing-opaque-id");
    assert_eq!(requests.len(), 1);
    let request = String::from_utf8_lossy(&requests[0]);
    assert!(request.starts_with("POST /bookings?apiKey=sensitive-fixture-token HTTP/1.1\r\n"));
    let payload: serde_json::Value =
        serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(
        payload,
        serde_json::json!({
            "title": "Fixture",
            "start": "2026-10-05T10:00:00Z",
            "end": "2026-10-05T10:30:00Z"
        })
    );
}

#[tokio::test]
async fn malformed_or_missing_receipts_never_invent_booking_ids() {
    for body in [
        "",
        "not-json",
        "{}",
        "null",
        r#"{"booking":{}}"#,
        r#"{"booking":{"id":null}}"#,
    ] {
        let (result, requests) = fixture(response(body.as_bytes()), false).await;
        let error = result.expect_err("missing receipt must not invent a booking ID");
        assert!(error.contains("outcome unknown"), "{error}");
        assert_eq!(requests.len(), 1);
    }
}

#[tokio::test]
async fn empty_duplicate_and_unverified_numeric_ids_reject() {
    for body in [
        r#"{"booking":{"id":""}}"#,
        r#"{"booking":{"id":"   "}}"#,
        r#"{"booking":{"id":42}}"#,
        r#"{"booking":{"id":"one","id":"two"}}"#,
    ] {
        let (result, requests) = fixture(response(body.as_bytes()), false).await;
        assert!(result.is_err(), "accepted unsupported receipt {body}");
        assert_eq!(requests.len(), 1);
    }
}

#[tokio::test]
async fn disconnect_does_not_expose_api_key_or_replay() {
    let (result, requests) = fixture(Vec::new(), false).await;
    let error = result.unwrap_err();
    assert!(error.contains("outcome unknown"), "{error}");
    assert!(!error.contains("sensitive-fixture-token"));
    assert!(!error.contains("apiKey="));
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn booking_link_requires_verified_mapping_before_network_io() {
    let (result, requests) = fixture(response(br#"{"event_types":[]}"#), true).await;
    let error = result.expect_err("an arbitrary 2xx must not produce an invented tenant URL");
    assert!(error.contains("verified"), "{error}");
    assert!(
        requests.is_empty(),
        "missing lookup mapping must fail before sending a request"
    );
}

#[tokio::test]
async fn truncated_success_is_unknown_without_replay() {
    let (result, requests) = fixture(
        b"HTTP/1.1 201 Created\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort".to_vec(),
        false,
    )
    .await;
    assert!(result.unwrap_err().contains("outcome unknown"));
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn non_success_and_redirects_never_replay_or_claim_booking() {
    for status in [
        "401 Unauthorized",
        "408 Request Timeout",
        "503 Unavailable",
        "307 Temporary Redirect",
    ] {
        let raw = format!(
            "HTTP/1.1 {status}\r\nLocation: /unexpected\r\nContent-Length: 13\r\nConnection: close\r\n\r\nprovider-data"
        );
        let (result, requests) = fixture(raw.into_bytes(), false).await;
        assert!(!result.unwrap_err().contains("provider-data"));
        assert_eq!(requests.len(), 1);
    }
}

#[tokio::test]
async fn receipt_size_bound_accepts_exact_limit_and_rejects_larger_bodies() {
    for size in [65_536, 65_537] {
        let mut body = br#"{"booking":{"id":"existing-id"}}"#.to_vec();
        body.resize(size, b' ');
        let (result, requests) = fixture(response(&body), false).await;
        assert_eq!(requests.len(), 1);
        if size == 65_536 {
            assert_eq!(result.unwrap(), "existing-id");
        } else {
            assert!(result.unwrap_err().contains("outcome unknown"));
        }
    }
}

#[tokio::test]
async fn chunked_receipt_size_is_bounded_without_content_length() {
    let mut body = br#"{"booking":{"id":"existing-id"}}"#.to_vec();
    body.resize(65_537, b' ');
    let mut raw = format!(
        "HTTP/1.1 201 Created\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:X}\r\n",
        body.len()
    )
    .into_bytes();
    raw.extend_from_slice(&body);
    raw.extend_from_slice(b"\r\n0\r\n\r\n");
    let (result, requests) = fixture(raw, false).await;
    assert!(result.unwrap_err().contains("outcome unknown"));
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn availability_transport_errors_do_not_expose_query_credentials() {
    let (result, requests) = fixture_action(Vec::new(), Action::Availability).await;
    let error = result.unwrap_err();
    assert!(!error.contains("sensitive-fixture-token"));
    assert!(!error.contains("apiKey="));
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn truncated_availability_is_an_error_instead_of_fabricated_empty_body() {
    let (result, requests) = fixture_action(
        b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort".to_vec(),
        Action::Availability,
    )
    .await;
    assert!(result.is_err());
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn stalled_receipt_obeys_client_deadline_without_replay_or_token_leak() {
    let started = std::time::Instant::now();
    let (result, requests) = fixture_with_deadline(
        b"HTTP/1.1 201 Created\r\nContent-Length: 100\r\nConnection: close\r\n\r\n".to_vec(),
        Action::CreateEvent,
        Duration::from_millis(50),
        true,
    )
    .await;
    let error = result.unwrap_err();
    assert!(error.contains("outcome unknown"));
    assert!(!error.contains("sensitive-fixture-token"));
    assert!(started.elapsed() < Duration::from_millis(750));
    assert_eq!(requests.len(), 1);
}
