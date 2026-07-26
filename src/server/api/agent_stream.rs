use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use chrono::Utc;
use futures_util::Stream;
use tokio::sync::broadcast;
use crate::hub::Hub;
use ::server_ohc::orchestration::Message;

const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(30);

pub fn router<S>(hub: Arc<Hub>) -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    axum::Router::new()
        .route(
            "/{id}/stream",
            axum::routing::get(stream_agent),
        )
        .with_state(hub)
}

pub async fn stream_agent(
    Path(agent_id): Path<String>,
    State(hub): State<Arc<Hub>>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let rx = hub.subscribe(agent_id.clone()).await;
    let stream = AgentEventStream::new(rx, agent_id);
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
    format!(
        "event: {}\ndata: {}\n\n",
        msg.r#type,
        payload
    )
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
    rx: broadcast::Receiver<Message>,
    agent_id: String,
    keep_alive: tokio::time::Interval,
}

impl AgentEventStream {
    fn new(rx: broadcast::Receiver<Message>, agent_id: String) -> Self {
        Self {
            rx,
            agent_id,
            keep_alive: tokio::time::interval(KEEP_ALIVE_INTERVAL),
        }
    }
}

impl Stream for AgentEventStream {
    type Item = Result<String, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;

        match this.rx.try_recv() {
            Ok(msg) => {
                return Poll::Ready(Some(Ok(message_to_sse_event(&msg))));
            }
            Err(broadcast::error::TryRecvError::Empty) => {}
            Err(broadcast::error::TryRecvError::Lagged(_)) => {}
            Err(broadcast::error::TryRecvError::Closed) => {
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
            let ping = format!(
                ": ping\n\n"
            );
            return Poll::Ready(Some(Ok(ping)));
        }

        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    fn make_hub() -> Arc<Hub> {
        let pool = crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://localhost/test")
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        Arc::new(Hub::new(tx, pool))
    }

    #[tokio::test]
    async fn test_stream_agent_returns_sse_content_type() {
        let hub = make_hub();
        let response = stream_agent(Path("test-agent".to_string()), State(hub))
            .await
            .unwrap();

        let response = response.into_response();
        let ct = response.headers().get("content-type").unwrap().to_str().unwrap();
        assert_eq!(ct, "text/event-stream");
    }

    #[tokio::test]
    async fn test_stream_agent_sets_cache_control() {
        let hub = make_hub();
        let response = stream_agent(Path("test-agent".to_string()), State(hub))
            .await
            .unwrap();

        let response = response.into_response();
        let cc = response.headers().get("cache-control").unwrap().to_str().unwrap();
        assert_eq!(cc, "no-cache");
    }

    #[tokio::test]
    async fn test_stream_agent_sets_connection() {
        let hub = make_hub();
        let response = stream_agent(Path("test-agent".to_string()), State(hub))
            .await
            .unwrap();

        let response = response.into_response();
        let conn = response.headers().get("connection").unwrap().to_str().unwrap();
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
