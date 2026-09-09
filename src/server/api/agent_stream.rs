use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::hub::Hub;
use ::server_omnisolo::orchestration::Message;
use axum::extract::{Extension, Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use chrono::Utc;
use futures_util::Stream;
use tokio::sync::broadcast;
use tokio_stream::wrappers::{BroadcastStream, errors::BroadcastStreamRecvError};

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(30);

pub fn router<S>(hub: Arc<Hub>) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new()
        .route(
            "/api/v1/agents/{id}/stream",
            axum::routing::get(stream_agent),
        )
        .with_state(hub)
}

pub async fn stream_agent(
    Path(agent_id): Path<String>,
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<server_common::Claims>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let org = claims
        .organization_id
        .as_deref()
        .filter(|org| !org.is_empty())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    if !hub
        .get_agent(&agent_id)
        .await
        .is_some_and(|agent| agent.organization_id == org)
    {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }
    let rx = hub.subscribe(agent_id.clone()).await;
    let expires_in = claims.exp.saturating_sub(Utc::now().timestamp()).max(0);
    if expires_in == 0 {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    let stream = futures_util::StreamExt::take_until(
        AgentEventStream::new(rx, agent_id),
        tokio::time::sleep(Duration::from_secs(expires_in as u64)),
    );
    let body = axum::body::Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/event-stream".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CACHE_CONTROL,
        "no-cache".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONNECTION,
        "keep-alive".parse().unwrap(),
    );

    Ok((headers, body))
}

fn message_to_sse_event(msg: &Message) -> String {
    let payload = serde_json::json!({
        "id": msg.id,
        "from": msg.from_agent,
        "to": msg.to_agent,
        "type": msg.r#type,
        "content": msg.content,
        "meeting_id": msg.meeting_id,
        "timestamp": msg.occurred_at_unix,
    });
    let event_type = if !msg.r#type.is_empty()
        && msg
            .r#type
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        msg.r#type.as_str()
    } else {
        "message"
    };
    format!("event: {}\ndata: {}\n\n", event_type, payload)
}

pub fn openai_chunk_from_message(msg: &Message, finish_reason: Option<&str>) -> serde_json::Value {
    serde_json::json!({
        "id": msg.id,
        "object": "chat.completion.chunk",
        "created": Utc::now().timestamp(),
        "model": "agent-stream",
        "choices": [{
            "index": 0,
            "delta": {
                "role": "assistant",
                "content": msg.content,
            },
            "finish_reason": finish_reason,
        }],
    })
}

pub fn openai_chunk_to_sse(chunk: &serde_json::Value) -> String {
    format!("data: {}\n\n", chunk)
}

struct AgentEventStream {
    rx: BroadcastStream<Message>,
    agent_id: String,
    keep_alive: tokio::time::Interval,
    closed: bool,
}

impl AgentEventStream {
    fn new(rx: broadcast::Receiver<Message>, agent_id: String) -> Self {
        Self {
            rx: BroadcastStream::new(rx),
            agent_id,
            keep_alive: tokio::time::interval(KEEP_ALIVE_INTERVAL),
            closed: false,
        }
    }
}

impl Stream for AgentEventStream {
    type Item = Result<String, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;

        if this.closed {
            return Poll::Ready(None);
        }
        match Pin::new(&mut this.rx).poll_next(cx) {
            Poll::Ready(Some(Ok(msg))) => {
                return Poll::Ready(Some(Ok(message_to_sse_event(&msg))));
            }
            Poll::Pending => {}
            Poll::Ready(Some(Err(BroadcastStreamRecvError::Lagged(count)))) => {
                return Poll::Ready(Some(Ok(format!(
                    "event: resync_required\ndata: {{\"missed\":{count}}}\n\n"
                ))));
            }
            Poll::Ready(None) => {
                this.closed = true;
                let done = format!(
                    "event: done\ndata: {}\n\n",
                    serde_json::json!({
                        "type": "stream_closed",
                        "agent_id": this.agent_id,
                    })
                );
                return Poll::Ready(Some(Ok(done)));
            }
        }

        if this.keep_alive.poll_tick(cx).is_ready() {
            let ping = ": ping\n\n".to_owned();
            return Poll::Ready(Some(Ok(ping)));
        }

        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn pending_stream_wakes_on_message_without_waiting_for_heartbeat() {
        let (tx, rx) = broadcast::channel(4);
        let mut stream = AgentEventStream::new(rx, "agent".into());
        assert!(stream.next().await.unwrap().unwrap().contains("ping"));
        tokio::spawn(async move {
            tokio::task::yield_now().await;
            tx.send(Message {
                id: "wake".into(),
                r#type: "chat".into(),
                ..Default::default()
            })
            .unwrap();
        });
        let started = std::time::Instant::now();
        let event = tokio::time::timeout(Duration::from_millis(200), stream.next())
            .await
            .expect("broadcast must wake the pending SSE stream")
            .unwrap()
            .unwrap();
        assert!(event.contains("wake"));
        assert!(
            started.elapsed() < Duration::from_millis(100),
            "delivery waited for a timer instead of a broadcast wakeup"
        );
    }

    #[tokio::test]
    async fn closed_stream_finishes_after_one_done_event() {
        let (tx, rx) = broadcast::channel(4);
        let mut stream = AgentEventStream::new(rx, "agent".into());
        drop(tx);
        assert!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .contains("stream_closed")
        );
        assert!(stream.next().await.is_none());
    }

    #[test]
    fn message_type_cannot_inject_an_sse_frame() {
        let event = message_to_sse_event(&Message {
            r#type: "chat\ndata: injected\n\nevent: forged".into(),
            ..Default::default()
        });
        assert!(!event.contains("\ndata: injected"));
    }

    fn test_claims() -> server_common::Claims {
        serde_json::from_value(serde_json::json!({"sub": "user", "iat": 0, "exp": 4000000000_i64, "organization_id": "test-org"})).unwrap()
    }

    fn make_hub() -> Arc<Hub> {
        let pool = crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://localhost/test")
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        Arc::new(Hub::new(tx, pool))
    }

    #[tokio::test]
    async fn stream_rejects_agent_from_another_organization() {
        use tower::ServiceExt;
        let hub = make_hub();
        hub.register_agent(server_omnisolo::orchestration::Agent {
            id: "private-agent".into(),
            organization_id: "private-org".into(),
            ..Default::default()
        })
        .await;
        let claims: server_common::Claims = serde_json::from_value(serde_json::json!({
            "sub": "user", "iat": 0, "exp": 4000000000_i64, "organization_id": "other-org"
        }))
        .unwrap();
        let app: axum::Router = router(hub).layer(axum::Extension(claims));
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/api/v1/agents/private-agent/stream")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn router_uses_axum_v08_capture_syntax() {
        let _: axum::Router<()> = router(make_hub());
    }

    #[tokio::test]
    async fn router_builds_with_axum_named_path_captures() {
        let _router = router::<()>(make_hub());
    }

    #[tokio::test]
    async fn test_stream_agent_returns_sse_content_type() {
        let hub = make_hub();
        hub.register_agent(server_omnisolo::orchestration::Agent {
            id: "test-agent".into(),
            organization_id: "test-org".into(),
            ..Default::default()
        })
        .await;
        let response = stream_agent(
            Path("test-agent".to_string()),
            State(hub),
            Extension(test_claims()),
        )
        .await
        .unwrap();

        let response = response.into_response();
        let ct = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(ct, "text/event-stream");
    }

    #[tokio::test]
    async fn test_stream_agent_sets_cache_control() {
        let hub = make_hub();
        hub.register_agent(server_omnisolo::orchestration::Agent {
            id: "test-agent".into(),
            organization_id: "test-org".into(),
            ..Default::default()
        })
        .await;
        let response = stream_agent(
            Path("test-agent".to_string()),
            State(hub),
            Extension(test_claims()),
        )
        .await
        .unwrap();

        let response = response.into_response();
        let cc = response
            .headers()
            .get("cache-control")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(cc, "no-cache");
    }

    #[tokio::test]
    async fn test_stream_agent_sets_connection() {
        let hub = make_hub();
        hub.register_agent(server_omnisolo::orchestration::Agent {
            id: "test-agent".into(),
            organization_id: "test-org".into(),
            ..Default::default()
        })
        .await;
        let response = stream_agent(
            Path("test-agent".to_string()),
            State(hub),
            Extension(test_claims()),
        )
        .await
        .unwrap();

        let response = response.into_response();
        let conn = response
            .headers()
            .get("connection")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(conn, "keep-alive");
    }

    #[test]
    fn test_message_to_sse_event_format() {
        let msg = Message {
            id: "msg-1".to_string(),
            from_agent: "agent-a".to_string(),
            to_agent: "agent-b".to_string(),
            r#type: "chat".to_string(),
            content: "hello world".to_string(),
            occurred_at_unix: 1234567890,
            meeting_id: "mtg-1".to_string(),
        };

        let event = message_to_sse_event(&msg);
        assert!(event.starts_with("event: chat\n"));
        assert!(event.contains("data: "));
        assert!(event.ends_with("\n\n"));
        assert!(event.contains("msg-1"));
        assert!(event.contains("hello world"));
    }

    #[test]
    fn test_openai_chunk_from_message_format() {
        let msg = Message {
            id: "msg-1".to_string(),
            from_agent: "agent-a".to_string(),
            to_agent: "agent-b".to_string(),
            r#type: "chat".to_string(),
            content: "token text".to_string(),
            occurred_at_unix: 1234567890,
            meeting_id: "".to_string(),
        };

        let chunk = openai_chunk_from_message(&msg, None);
        assert_eq!(chunk["id"], "msg-1");
        assert_eq!(chunk["object"], "chat.completion.chunk");
        assert_eq!(chunk["model"], "agent-stream");
        assert_eq!(chunk["choices"][0]["delta"]["content"], "token text");
        assert!(chunk["choices"][0]["finish_reason"].is_null());
    }

    #[test]
    fn test_openai_chunk_with_finish_reason() {
        let msg = Message {
            id: "msg-2".to_string(),
            from_agent: "agent-a".to_string(),
            to_agent: "agent-b".to_string(),
            r#type: "chat".to_string(),
            content: "done".to_string(),
            occurred_at_unix: 1234567890,
            meeting_id: "".to_string(),
        };

        let chunk = openai_chunk_from_message(&msg, Some("stop"));
        assert_eq!(chunk["choices"][0]["finish_reason"], "stop");
    }

    #[test]
    fn test_openai_chunk_to_sse_format() {
        let chunk = serde_json::json!({
            "id": "msg-1",
            "object": "chat.completion.chunk",
            "choices": [{"delta": {"content": "hi"}}],
        });
        let sse = openai_chunk_to_sse(&chunk);
        assert!(sse.starts_with("data: "));
        assert!(sse.ends_with("\n\n"));
        assert!(sse.contains("chat.completion.chunk"));
    }

    #[tokio::test]
    async fn test_agent_event_stream_receives_messages() {
        let hub = make_hub();
        let mut rx = hub.subscribe("stream-test".to_string()).await;

        let msg = Message {
            id: "msg-stream-1".to_string(),
            from_agent: "sender".to_string(),
            to_agent: "stream-test".to_string(),
            r#type: "chat".to_string(),
            content: "streamed content".to_string(),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        hub.clone().publish(msg.clone()).await.unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.id, "msg-stream-1");
        assert_eq!(received.content, "streamed content");
    }
}
