//! Real loopback wire contracts for the mounted OpenCode and OpenHands adapters.
use std::collections::BTreeSet;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use server_harness::middleware::harness::HarnessAdapterError;
use server_harness::middleware::opencode::{OpenCodeEventCorrelation, OpenCodeHttpAdapter};
use server_harness::middleware::openhands::{OpenHandsAdapterConfig, OpenHandsHttpAdapter};
use server_harness::middleware::types::{ModelApiDialect, ResolvedModelSelection};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::timeout;
use uuid::Uuid;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug)]
enum Kind {
    OpenCode,
    OpenHands,
}

const KINDS: [Kind; 2] = [Kind::OpenCode, Kind::OpenHands];

enum Adapter {
    OpenCode(Box<OpenCodeHttpAdapter>),
    OpenHands(Box<OpenHandsHttpAdapter>),
}

impl Adapter {
    async fn connect(kind: Kind, address: SocketAddr) -> Self {
        match kind {
            Kind::OpenCode => Self::OpenCode(Box::new(
                OpenCodeHttpAdapter::connect(address, REQUEST_TIMEOUT).unwrap(),
            )),
            Kind::OpenHands => Self::OpenHands(Box::new(
                OpenHandsHttpAdapter::connect(
                    address,
                    OpenHandsAdapterConfig::new(
                        "/workspace/project",
                        selection(),
                        "http://127.0.0.1/v1",
                    )
                    .with_timeouts(
                        REQUEST_TIMEOUT,
                        REQUEST_TIMEOUT,
                        Duration::from_millis(5),
                    ),
                )
                .await
                .unwrap(),
            )),
        }
    }

    async fn delete(&self) -> Result<(), String> {
        match self {
            Self::OpenCode(adapter) => adapter
                .delete_session("ses_wire")
                .await
                .map_err(|e| e.to_string()),
            Self::OpenHands(adapter) => adapter
                .delete_conversation("wire")
                .await
                .map_err(|e| e.to_string()),
        }
    }

    async fn post(&self) -> Result<(), String> {
        match self {
            Self::OpenCode(adapter) => adapter.abort("ses_wire").await.map_err(|e| e.to_string()),
            Self::OpenHands(adapter) => adapter.cancel("wire").await.map_err(|e| e.to_string()),
        }
    }
}

fn selection() -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "wire-test".to_owned(),
        reasoning_effort: None,
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: None,
        capabilities: BTreeSet::new(),
        binding_revision: "wire".to_owned(),
        binding_digest: "wire".to_owned(),
        metadata: Default::default(),
    }
}

#[derive(Clone)]
struct Reply {
    bytes: Arc<Vec<u8>>,
    hold_open: bool,
    split_bytes: bool,
    after_abort: Option<Arc<Vec<u8>>>,
}

impl Reply {
    fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: Arc::new(bytes.into()),
            hold_open: false,
            split_bytes: false,
            after_abort: None,
        }
    }

    fn held(mut self) -> Self {
        self.hold_open = true;
        self
    }
}

struct WireServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<(String, String)>>>,
    closed: Arc<AtomicUsize>,
    task: JoinHandle<()>,
}

impl WireServer {
    async fn start(reply: Reply) -> Self {
        Self::start_with_event(reply, None).await
    }

    async fn start_with_event(reply: Reply, event: Option<Reply>) -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = requests.clone();
        let closed = Arc::new(AtomicUsize::new(0));
        let observed_closed = closed.clone();
        let task = tokio::spawn(async move {
            let mut connections = JoinSet::new();
            loop {
                tokio::select! {
                    accepted = listener.accept() => {
                        let (mut socket, _) = accepted.unwrap();
                        let reply = reply.clone();
                        let event = event.clone();
                        let recorded = recorded.clone();
                        let closed = observed_closed.clone();
                        connections.spawn(async move {
                            let (method, path) = read_request(&mut socket).await;
                            recorded.lock().unwrap().push((method, path.clone()));
                            let reply = match (path.as_str(), event) {
                                ("/ready", _) => json_reply(json!({"status":"ready"})),
                                ("/server_info", _) => json_reply(json!({"version":"1.43.1"})),
                                ("/event", Some(event)) => event,
                                ("/session/ses_wire/prompt_async", _) => Reply::new(b"HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n".to_vec()),
                                _ => reply,
                            };
                            if reply.split_bytes {
                                for byte in reply.bytes.iter() {
                                    if socket.write_all(&[*byte]).await.is_err() { return; }
                                    tokio::task::yield_now().await;
                                }
                            } else if socket.write_all(&reply.bytes).await.is_err() {
                                return;
                            }
                            if let Some(bytes) = &reply.after_abort {
                                while !recorded.lock().unwrap().iter().any(|(_, path)| path.ends_with("/abort")) {
                                    tokio::task::yield_now().await;
                                }
                                if socket.write_all(bytes).await.is_err() { return; }
                            }
                            if reply.hold_open && matches!(socket.read(&mut [0; 1]).await, Ok(0)) {
                                closed.fetch_add(1, Ordering::SeqCst);
                            }
                        });
                    }
                    _ = connections.join_next(), if !connections.is_empty() => {}
                }
            }
        });
        Self {
            address,
            requests,
            closed,
            task,
        }
    }

    fn count(&self, method: &str) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(actual, _)| actual == method)
            .count()
    }

    fn abort_count(&self) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, path)| path.ends_with("/abort"))
            .count()
    }
}

impl Drop for WireServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn read_request(socket: &mut TcpStream) -> (String, String) {
    let mut bytes = Vec::new();
    let mut buffer = [0; 1024];
    loop {
        if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
            let head = std::str::from_utf8(&bytes[..end]).unwrap();
            let length = head
                .lines()
                .filter_map(|line| line.split_once(':'))
                .find_map(|(name, value)| {
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);
            if bytes.len() >= end + 4 + length {
                let mut words = head.split_whitespace();
                return (
                    words.next().unwrap().to_owned(),
                    words.next().unwrap().to_owned(),
                );
            }
        }
        let count = socket.read(&mut buffer).await.unwrap();
        assert!(count > 0, "request ended before its body");
        bytes.extend_from_slice(&buffer[..count]);
        assert!(
            bytes.len() < 64 * 1024,
            "fixture request exceeded its bound"
        );
    }
}

fn json_reply(value: Value) -> Reply {
    let body = serde_json::to_vec(&value).unwrap();
    let mut bytes = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len()).into_bytes();
    bytes.extend(body);
    Reply::new(bytes)
}

#[tokio::test]
async fn content_length_finishes_without_waiting_for_connection_close() {
    for kind in KINDS {
        let server = WireServer::start(json_reply(json!(true)).held()).await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_ok(),
            "{kind:?} waited for EOF after the complete body"
        );
    }
}

#[tokio::test]
async fn chunked_json_accepts_extensions_and_trailers() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2;wire=yes\r\ntr\r\n2\r\nue\r\n0\r\nX-Wire: done\r\n\r\n".to_vec())).await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_ok(),
            "{kind:?} did not decode chunked JSON"
        );
    }
}

#[tokio::test]
async fn content_encoding_does_not_enable_automatic_decompression() {
    // Gzip encoding of `true`; the adapters never requested compressed content.
    let gzip_true = [
        31, 139, 8, 0, 0, 0, 0, 0, 2, 3, 43, 41, 42, 77, 5, 0, 141, 76, 252, 253, 4, 0, 0, 0,
    ];
    for kind in KINDS {
        let mut response = format!(
            "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n",
            gzip_true.len()
        )
        .into_bytes();
        response.extend_from_slice(&gzip_true);
        let server = WireServer::start(Reply::new(response)).await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} automatically decompressed an unsolicited body"
        );
    }
}

#[tokio::test]
async fn truncated_content_length_is_never_success() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(
            b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\n\r\ntrue".to_vec(),
        ))
        .await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} accepted a truncated body"
        );
    }
}

#[tokio::test]
async fn malformed_or_conflicting_content_lengths_are_rejected() {
    for kind in KINDS {
        for headers in [
            "Content-Length: 0\r\nContent-Length: 4",
            "Content-Length: invalid\r\nContent-Length: 4",
        ] {
            let server = WireServer::start(Reply::new(
                format!("HTTP/1.1 200 OK\r\n{headers}\r\n\r\ntrue").into_bytes(),
            ))
            .await;
            let adapter = Adapter::connect(kind, server.address).await;
            assert!(
                adapter.delete().await.is_err(),
                "{kind:?} accepted {headers:?}"
            );
        }
    }
}

#[tokio::test]
async fn enormous_chunk_length_returns_an_error_without_panicking() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(
            format!(
                "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n",
                usize::MAX
            )
            .into_bytes(),
        ))
        .await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} accepted a missing enormous chunk"
        );
    }
}

#[tokio::test]
async fn transfer_encoding_and_content_length_cannot_disagree() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Length: 4\r\n\r\n4\r\ntrue\r\n0\r\n\r\n".to_vec())).await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} accepted contradictory framing"
        );
    }
}

#[tokio::test]
async fn invalid_status_and_unsupported_transfer_codings_are_rejected() {
    for kind in KINDS {
        for response in [
            "HTTP/9.9 200 OK\r\nContent-Length: 4\r\n\r\ntrue",
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: identity\r\n\r\ntrue",
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked, chunked\r\n\r\n4\r\ntrue\r\n0\r\n\r\n",
        ] {
            let server = WireServer::start(Reply::new(response.as_bytes().to_vec())).await;
            let adapter = Adapter::connect(kind, server.address).await;
            assert!(
                adapter.delete().await.is_err(),
                "{kind:?} accepted {response:?}"
            );
        }
    }
}

#[tokio::test]
async fn response_headers_have_an_independent_bound() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: 4\r\nX-Large: {}\r\n\r\ntrue",
                "a".repeat(64 * 1024)
            )
            .into_bytes(),
        ))
        .await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} accepted oversized headers"
        );
    }
}

#[tokio::test]
async fn unterminated_large_header_fails_before_the_request_deadline() {
    for kind in KINDS {
        let server = WireServer::start(
            Reply::new(
                format!("HTTP/1.1 200 OK\r\nX-Large: {}", "a".repeat(1024 * 1024)).into_bytes(),
            )
            .held(),
        )
        .await;
        let adapter = Adapter::connect(kind, server.address).await;
        let result = timeout(REQUEST_TIMEOUT / 2, adapter.delete()).await;
        assert!(
            matches!(result, Ok(Err(_))),
            "{kind:?} did not bound an unterminated header: {result:?}"
        );
    }
}

#[tokio::test]
async fn decoded_json_body_accepts_exact_limit_and_rejects_one_more() {
    for (kind, limit) in [
        (Kind::OpenCode, 8 * 1024 * 1024),
        (Kind::OpenHands, 16 * 1024 * 1024),
    ] {
        for (extra, chunked) in [(0, false), (1, false), (0, true), (1, true)] {
            let mut bytes = format!(
                "HTTP/1.1 200 OK\r\n{}true",
                if chunked {
                    format!("Transfer-Encoding: chunked\r\n\r\n{:x}\r\n", limit + extra)
                } else {
                    format!("Content-Length: {}\r\n\r\n", limit + extra)
                }
            )
            .into_bytes();
            bytes.resize(bytes.len() + limit + extra - 4, b' ');
            if chunked {
                bytes.extend_from_slice(b"\r\n0\r\n\r\n");
            }
            let server = WireServer::start(Reply::new(bytes)).await;
            let adapter = Adapter::connect(kind, server.address).await;
            assert_eq!(
                adapter.delete().await.is_ok(),
                extra == 0,
                "{kind:?} decoded body limit + {extra}; chunked={chunked}"
            );
        }
    }
}

#[tokio::test]
async fn streamed_json_body_is_bounded_without_content_length() {
    for (kind, limit) in [
        (Kind::OpenCode, 8 * 1024 * 1024),
        (Kind::OpenHands, 16 * 1024 * 1024),
    ] {
        let mut bytes = b"HTTP/1.1 200 OK\r\n\r\ntrue".to_vec();
        bytes.resize(bytes.len() + limit - 3, b' ');
        let server = WireServer::start(Reply::new(bytes)).await;
        let adapter = Adapter::connect(kind, server.address).await;
        assert!(
            adapter.delete().await.is_err(),
            "{kind:?} accepted a streamed oversized body"
        );
    }
}

#[tokio::test]
async fn redirect_never_replays_a_post_or_contacts_the_location() {
    for kind in KINDS {
        let destination = WireServer::start(json_reply(json!(true))).await;
        let reply = Reply::new(format!("HTTP/1.1 307 Temporary Redirect\r\nLocation: http://{}/stolen\r\nContent-Length: 4\r\n\r\ntrue", destination.address).into_bytes());
        let source = WireServer::start(reply).await;
        let adapter = Adapter::connect(kind, source.address).await;
        assert!(
            adapter.post().await.is_err(),
            "{kind:?} accepted a redirect"
        );
        assert_eq!(source.count("POST"), 1);
        assert!(destination.requests.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn ambiguous_post_disconnect_and_timeout_are_never_replayed() {
    for kind in KINDS {
        for reply in [Reply::new(Vec::new()), Reply::new(Vec::new()).held()] {
            let server = WireServer::start(reply).await;
            let adapter = Adapter::connect(kind, server.address).await;
            assert!(
                adapter.post().await.is_err(),
                "{kind:?} accepted an unknown POST outcome"
            );
            assert_eq!(
                server.count("POST"),
                1,
                "{kind:?} replayed an unknown POST outcome"
            );
        }
    }
}

#[tokio::test]
async fn cancelling_an_inflight_post_closes_the_connection_without_replaying() {
    for kind in KINDS {
        let server = WireServer::start(Reply::new(Vec::new()).held()).await;
        let adapter = Adapter::connect(kind, server.address).await;
        let request = tokio::spawn(async move { adapter.post().await });
        timeout(REQUEST_TIMEOUT, async {
            while server.count("POST") == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        request.abort();
        assert!(request.await.unwrap_err().is_cancelled());
        timeout(REQUEST_TIMEOUT, async {
            while server.closed.load(Ordering::SeqCst) == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("cancelled request retained its socket");
        assert_eq!(server.count("POST"), 1);
    }
}

fn correlation() -> OpenCodeEventCorrelation {
    OpenCodeEventCorrelation {
        session_id: Uuid::nil(),
        task_id: None,
        turn_id: None,
        attempt_id: "wire".to_owned(),
        native_session_id: "ses_wire".to_owned(),
        admitted_user_message_id: None,
    }
}

#[tokio::test]
async fn chunked_sse_decodes_split_utf8_and_cancel_sends_one_abort() {
    let event = "data: {\"id\":\"é\",\"type\":\"server.connected\",\"properties\":{}}\r\n\r\n";
    let mut bytes =
        b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n"
            .to_vec();
    for byte in event.bytes() {
        bytes.extend_from_slice(&[b'1', b'\r', b'\n', byte, b'\r', b'\n']);
    }
    let mut event_reply = Reply::new(bytes).held();
    event_reply.split_bytes = true;
    let server = WireServer::start_with_event(json_reply(json!(true)), Some(event_reply)).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    let event = stream.next_event().await.unwrap();
    assert_eq!(event.event.event_type, "harness.connected");
    stream.cancel().await.unwrap();
    stream.cancel().await.unwrap();
    drop(stream);
    let requests = server.requests.lock().unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|(_, path)| path.ends_with("/abort"))
            .count(),
        1
    );
}

#[tokio::test]
async fn chunked_sse_retains_decoded_line_and_event_limits() {
    for (data, expected) in [
        (
            format!("data: {}\n\n", "a".repeat(64 * 1024)),
            "line exceeded",
        ),
        (
            format!(
                "{}\n",
                format!("data: {}\n", "a".repeat(60 * 1024)).repeat(5)
            ),
            "event exceeded",
        ),
    ] {
        let event = Reply::new(format!("HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{data}\r\n0\r\n\r\n", data.len()).into_bytes());
        let server = WireServer::start_with_event(json_reply(json!(true)), Some(event)).await;
        let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
        let mut stream = adapter
            .prompt_async(correlation(), "wire", &selection())
            .await
            .unwrap();
        let error = stream.next_event().await.unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        stream.cancel().await.unwrap();
    }
}

fn held_sse() -> Reply {
    Reply::new(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n\r\n".to_vec()).held()
}

#[tokio::test]
async fn unknown_stream_cancel_is_not_replayed_on_drop_and_holds_admission() {
    let server = WireServer::start_with_event(Reply::new(Vec::new()), Some(held_sse())).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    assert!(stream.cancel().await.is_err());
    drop(stream);
    let replay = timeout(Duration::from_millis(100), async {
        while server.abort_count() == 1 {
            tokio::task::yield_now().await;
        }
    })
    .await;
    assert!(
        replay.is_err(),
        "dropping the failed cancellation replayed /abort"
    );
    assert!(matches!(
        adapter
            .prompt_async(correlation(), "must reconcile first", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    let error = adapter
        .shutdown()
        .await
        .expect_err("external shutdown concealed an unknown cancellation");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert_eq!(
        server.abort_count(),
        1,
        "shutdown replayed the unknown cancellation"
    );
}

#[tokio::test]
async fn unknown_stream_cancel_cannot_be_retried_or_report_success() {
    let server = WireServer::start_with_event(Reply::new(Vec::new()), Some(held_sse())).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    assert!(stream.cancel().await.is_err());
    let error = stream
        .cancel()
        .await
        .expect_err("unknown cancellation became success");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert_eq!(
        server.abort_count(),
        1,
        "explicit cancel replayed an unknown outcome"
    );
}

#[tokio::test]
async fn unknown_shutdown_cancellation_is_not_replayed_by_the_owned_stream() {
    let server = WireServer::start_with_event(Reply::new(Vec::new()), Some(held_sse())).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    assert!(adapter.shutdown().await.is_err());
    let error = stream
        .cancel()
        .await
        .expect_err("unknown shutdown became success");
    assert!(error.to_string().contains("unknown"), "{error}");
    drop(stream);
    let replay = timeout(Duration::from_millis(100), async {
        while server.abort_count() == 1 {
            tokio::task::yield_now().await;
        }
    })
    .await;
    assert!(
        replay.is_err(),
        "stream cleanup replayed shutdown's unknown /abort"
    );
    assert_eq!(server.abort_count(), 1);
}

#[tokio::test]
async fn dropping_a_pending_cancel_preserves_unknown_state_and_admission() {
    let server =
        WireServer::start_with_event(Reply::new(Vec::new()).held(), Some(held_sse())).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    {
        let cancellation = stream.cancel();
        tokio::pin!(cancellation);
        tokio::select! {
            result = &mut cancellation => panic!("cancellation unexpectedly finished: {result:?}"),
            _ = async { while server.abort_count() == 0 { tokio::task::yield_now().await; } } => {}
        }
    }
    drop(stream);
    assert!(matches!(
        adapter
            .prompt_async(correlation(), "must reconcile first", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    let error = adapter
        .shutdown()
        .await
        .expect_err("external shutdown concealed an unknown cancellation");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert_eq!(server.abort_count(), 1);
}

#[tokio::test]
async fn matching_terminal_event_does_not_release_an_unknown_cancellation() {
    let mut event = held_sse();
    event.after_abort = Some(Arc::new(format!("data: {}\n\n", json!({"id":"cancelled-current-turn","type":"session.error","properties":{"sessionID":"ses_wire","error":{"name":"MessageAbortedError","data":{"message":"stopped"}}}})).into_bytes()));
    let server = WireServer::start_with_event(Reply::new(Vec::new()), Some(event)).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    assert!(stream.cancel().await.is_err());
    assert!(matches!(
        adapter
            .prompt_async(correlation(), "must reconcile first", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    let terminal = stream.next_event().await.unwrap();
    assert!(terminal.terminal);
    assert_eq!(terminal.event.event_type, "turn.cancelled");
    let error = stream
        .cancel()
        .await
        .expect_err("terminal event concealed an unknown cancellation");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert!(matches!(
        adapter
            .prompt_async(correlation(), "abort may still arrive", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    drop(stream);
    let error = adapter
        .shutdown()
        .await
        .expect_err("external shutdown concealed an unknown cancellation");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert_eq!(server.abort_count(), 1);
}

#[tokio::test]
async fn terminal_event_cannot_release_admission_while_abort_is_in_flight() {
    let mut event = held_sse();
    event.after_abort = Some(Arc::new(format!("data: {}\n\n", json!({"id":"terminal-during-abort","type":"session.error","properties":{"sessionID":"ses_wire","error":{"name":"MessageAbortedError","data":{"message":"stopped"}}}})).into_bytes()));
    let server = WireServer::start_with_event(Reply::new(Vec::new()).held(), Some(event)).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address, REQUEST_TIMEOUT).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "wire", &selection())
        .await
        .unwrap();
    {
        let cancellation = adapter.abort("ses_wire");
        tokio::pin!(cancellation);
        let terminal = tokio::select! {
            result = &mut cancellation => panic!("abort unexpectedly finished: {result:?}"),
            result = stream.next_event() => result.unwrap(),
        };
        assert!(terminal.terminal);
        assert!(matches!(
            adapter
                .prompt_async(correlation(), "abort is still pending", &selection())
                .await,
            Err(HarnessAdapterError::InvalidRequest(_))
        ));
    }
    // Dropping the request cannot prove the session-scoped abort has completed.
    assert!(matches!(
        adapter
            .prompt_async(correlation(), "abort outcome is unknown", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    drop(stream);
    let error = adapter
        .shutdown()
        .await
        .expect_err("external shutdown concealed an unknown cancellation");
    assert!(error.to_string().contains("unknown"), "{error}");
    assert_eq!(server.abort_count(), 1);
}

#[tokio::test]
async fn normalized_conversation_paths_are_rejected_before_mutation() {
    let server = WireServer::start(json_reply(json!(true))).await;
    let Adapter::OpenHands(adapter) = Adapter::connect(Kind::OpenHands, server.address).await
    else {
        unreachable!()
    };
    for id in [".", ".."] {
        assert!(
            adapter.delete_conversation(id).await.is_err(),
            "normalized conversation ID {id} was accepted"
        );
    }
    assert_eq!(server.count("DELETE"), 0, "malformed IDs sent a mutation");
}

#[tokio::test]
async fn encoded_conversation_ids_preserve_the_exact_target_path() {
    let server = WireServer::start(json_reply(json!(true))).await;
    let Adapter::OpenHands(adapter) = Adapter::connect(Kind::OpenHands, server.address).await
    else {
        unreachable!()
    };
    for (id, encoded) in [
        ("wire", "wire"),
        ("wire/part", "wire%2Fpart"),
        ("wire?value", "wire%3Fvalue"),
        ("%2e%2e", "%252e%252e"),
        ("..tail", "..tail"),
        ("é", "%C3%A9"),
    ] {
        adapter.delete_conversation(id).await.unwrap();
        assert_eq!(
            server.requests.lock().unwrap().last().unwrap().1,
            format!("/api/conversations/{encoded}")
        );
    }
}

#[test]
fn ambient_proxy_configuration_is_ignored() {
    const CHILD: &str = "OHC_HTTP_WIRE_PROXY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            for kind in KINDS {
                let server = WireServer::start(json_reply(json!(true))).await;
                let adapter = Adapter::connect(kind, server.address).await;
                assert!(
                    adapter.delete().await.is_ok(),
                    "{kind:?} used an ambient proxy"
                );
            }
        });
        return;
    }
    let proxy = std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    proxy.set_nonblocking(true).unwrap();
    let proxy_url = format!("http://{}", proxy.local_addr().unwrap());
    let mut child = std::process::Command::new(std::env::current_exe().unwrap());
    child
        .args([
            "--exact",
            "ambient_proxy_configuration_is_ignored",
            "--nocapture",
        ])
        .env(CHILD, "1");
    for key in [
        "HTTP_PROXY",
        "http_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
    ] {
        child.env(key, &proxy_url);
    }
    child.env("NO_PROXY", "").env("no_proxy", "");
    let output = child.output().unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(matches!(proxy.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock));
}
