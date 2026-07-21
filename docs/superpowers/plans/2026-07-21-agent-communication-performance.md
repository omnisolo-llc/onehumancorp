# Agent Communication & Performance Improvement Plan

> **For agentic workers:** Use superpowers:subagent-driven-development to implement this plan task-by-task.

**Goal:** Transform OHC's fragmented agent communication into a unified, high-performance real-time system inspired by WorkBuddy's multi-agent orchestration — enabling small business owners to seamlessly orchestrate AI agents via WebSocket/SSE with sub-100ms latency, automatic reconnection, and zero message loss.

**Architecture:** Replace the current 3 fragmented WebSocket protocols + 4 polling loops with a unified SSE+WebSocket hybrid transport layer. Centralize Redis pub/sub through a shared connection pool. Replace std::sync::RwLock with tokio::sync::RwLock in the Hub. Add OpenAI-compatible SSE streaming for LLM responses. Implement message batching and backpressure propagation.

**Tech Stack:** Rust (Axum, Tokio, Redis, NATS), TypeScript/React (Next.js), Protobuf, SSE (text/event-stream), WebSocket

---

## Phase 1: Critical Performance Fixes (Week 1)

### Task 1: Replace std::sync::RwLock with tokio::sync::RwLock in Hub

**Files:**
- Modify: `src/server/hub.rs:24-47, 266-359, 420-432`

**Interfaces:**
- Consumes: Hub struct with 6 RwLock fields
- Produces: Async-safe Hub with no blocking runtime threads

- [ ] **Step 1: Write failing test for concurrent publish without blocking**

```rust
// In src/server/hub.rs #[cfg(test)] mod tests:
#[tokio::test]
async fn test_concurrent_publish_does_not_block_runtime() {
    let hub = create_test_hub();
    let handles: Vec<_> = (0..100).map(|i| {
        let hub = hub.clone();
        tokio::spawn(async move {
            let msg = Message {
                id: format!("msg-{}", i),
                from_agent: "test".to_string(),
                to_agent: "agent-1".to_string(),
                r#type: "test".to_string(),
                content: format!("payload-{}", i),
                meeting_id: String::new(),
                occurred_at_unix: chrono::Utc::now().timestamp(),
            };
            hub.publish(msg)
        })
    }).collect();
    for h in handles { h.await.unwrap().unwrap(); }
}
```

- [ ] **Step 2: Run test — should deadlock or timeout with std::sync::RwLock**

```bash
cargo test --lib -p ohc-mono -- hub::tests::test_concurrent_publish_does_not_block_runtime
```

- [ ] **Step 3: Replace all 6 RwLock fields with tokio::sync::RwLock**

```rust
// Change imports
use tokio::sync::RwLock;  // instead of std::sync::RwLock

// Change field declarations (lines 27-39)
agents: RwLock<HashMap<String, Agent>>,
meetings: RwLock<HashMap<String, MeetingRoom>>,
inbox: RwLock<HashMap<String, Vec<Message>>>,
subs: RwLock<HashMap<String, broadcast::Sender<Message>>>,
mesh_events: RwLock<HashMap<String, broadcast::Sender<MeshEvent>>>,
teammate_events: RwLock<HashMap<String, broadcast::Sender<TeammateMeshEvent>>>,
recent_events: RwLock<Vec<HubEvent>>,
auto_cor_track: RwLock<std::collections::HashSet<String>>,
agent_cache: RwLock<Option<Arc<Vec<Agent>>>>,
meetings_cache: RwLock<Option<Arc<Vec<MeetingRoom>>>>,
```

- [ ] **Step 4: Convert all `.write().unwrap()` to `.write().await` and `.read().unwrap()` to `.read().await`**

Key locations: `publish()` (line 267-269), `get_inbox()` (line 420-423), `subscribe()` (line 427-432), `register_agent()`, `remove_agent()`, `create_meeting()`, `get_agent()`, `list_agents()`, cache invalidation methods.

- [ ] **Step 5: Make `publish()` async and fix call sites**

```rust
pub async fn publish(self: std::sync::Arc<Self>, msg: Message) -> Result<(), String> {
    let mut inbox = self.inbox.write().await;
    let mut meetings = self.meetings.write().await;
    let subs = self.subs.read().await;
    // ... rest of logic
}
```

- [ ] **Step 6: Run test — should pass without blocking**

```bash
cargo test --lib -p ohc-mono -- hub::tests::test_concurrent_publish_does_not_block_runtime
```

- [ ] **Step 7: Run all hub tests to verify no regressions**

```bash
cargo test --lib -p ohc-mono -- hub::tests
```

- [ ] **Step 8: Commit**

```bash
git add src/server/hub.rs
git commit -m "perf: replace std::sync::RwLock with tokio::sync::RwLock in Hub"
```

---

### Task 2: Unify Redis Client Singletons and Add Connection Pooling

**Files:**
- Create: `src/server/redis_pool.rs`
- Modify: `src/server/api/sync.rs:47-50`
- Modify: `src/server/api/agent_feed.rs:45-52`
- Modify: `src/server/hub.rs:58-60`

**Interfaces:**
- Consumes: REDIS_URL env var
- Produces: `get_redis_pool() -> Arc<redis::aio::ConnectionPool>`

- [ ] **Step 1: Write failing test for shared Redis pool**

```rust
// In src/server/redis_pool.rs:
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shared_pool_returns_same_instance() {
        let pool1 = get_redis_pool();
        let pool2 = get_redis_pool();
        // Same Arc pointer = same instance
        assert!(std::sync::Arc::ptr_eq(&pool1, &pool2));
    }
}
```

- [ ] **Step 2: Create unified Redis pool module**

```rust
// src/server/redis_pool.rs
use std::sync::Arc;
use std::sync::OnceLock;

static REDIS_POOL: OnceLock<Arc<redis::aio::ConnectionPool>> = OnceLock::new();

pub fn get_redis_pool() -> Arc<redis::aio::ConnectionPool> {
    REDIS_POOL.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let client = redis::Client::open(redis_url)
            .expect("Failed to create Redis client");
        let pool = redis::aio::ConnectionPool::new(client);
        Arc::new(pool)
    }).clone()
}
```

- [ ] **Step 3: Refactor sync.rs to use shared pool**

Replace `crate::get_redis_client()` with `crate::redis_pool::get_redis_pool()` and use `pool.get().await` instead of `client.get_async_pubsub().await`.

- [ ] **Step 4: Refactor agent_feed.rs to use shared pool**

Replace `SHARED_REDIS_CLIENT` OnceLock with `crate::redis_pool::get_redis_pool()`.

- [ ] **Step 5: Refactor hub.rs to use shared pool**

Replace `redis_client: Option<redis::Client>` with pool access.

- [ ] **Step 6: Remove duplicate Redis client initialization code**

- [ ] **Step 7: Run tests**

```bash
cargo test --lib -p ohc-mono -- redis_pool::tests
cargo test --lib -p ohc-mono -- api::sync::tests
cargo test --lib -p ohc-mono -- api::agent_feed::tests
```

- [ ] **Step 8: Commit**

```bash
git add src/server/redis_pool.rs src/server/api/sync.rs src/server/api/agent_feed.rs src/server/hub.rs
git commit -m "perf: unify Redis clients into shared connection pool"
```

---

### Task 3: Add SSE Streaming Endpoint for Agent LLM Responses

**Files:**
- Create: `src/server/api/agent_stream.rs`
- Modify: `src/server/lib.rs` (add route)

**Interfaces:**
- Consumes: Agent task results, LLM token streams
- Produces: `GET /api/v1/agents/:id/stream` returning `text/event-stream`

- [ ] **Step 1: Write failing test for SSE endpoint**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, routing::get, Router};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_agent_stream_returns_event_stream() {
        let app = Router::new()
            .route("/agents/{id}/stream", get(stream_agent));

        let req = Request::builder()
            .uri("/agents/test-123/stream")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "text/event-stream"
        );
    }
}
```

- [ ] **Step 2: Implement SSE streaming handler**

```rust
// src/server/api/agent_stream.rs
use axum::extract::{Path, State};
use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

pub async fn stream_agent(
    Path(agent_id): Path<String>,
    State(hub): State<Arc<crate::hub::Hub>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = hub.subscribe_agent_events(&agent_id);
    let stream = BroadcastStream::new(rx).filter_map(|result| {
        async move {
            match result {
                Ok(event) => {
                    let data = serde_json::to_string(&event).ok()?;
                    Some(Ok(Event::default().data(data)))
                }
                Err(_) => None,
            }
        }
    });
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(30))
            .text("ping"),
    )
}
```

- [ ] **Step 3: Add OpenAI-compatible streaming format**

```rust
// For LLM token streaming, emit OpenAI-compatible chunks:
fn format_llm_chunk(token: &str, done: bool) -> Event {
    let chunk = serde_json::json!({
        "id": format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        "object": "chat.completion.chunk",
        "choices": [{
            "delta": { "content": token },
            "finish_reason": if done { Some("stop") } else { None },
            "index": 0,
        }]
    });
    Event::default().data(chunk.to_string())
}
```

- [ ] **Step 4: Register route in server**

- [ ] **Step 5: Run tests**

```bash
cargo test --lib -p ohc-mono -- api::agent_stream::tests
```

- [ ] **Step 6: Commit**

```bash
git add src/server/api/agent_stream.rs src/server/lib.rs
git commit -m "feat: add SSE streaming endpoint for agent LLM responses"
```

---

### Task 4: Add Backpressure and Message Batching to WebSocket Handlers

**Files:**
- Modify: `src/server/api/sync.rs:40-84`
- Modify: `src/server/api/agent_feed.rs:124-160`

**Interfaces:**
- Consumes: Redis pub/sub messages
- Produces: Batched WS frames with backpressure

- [ ] **Step 1: Write test for message batching**

```rust
#[tokio::test]
async fn test_ws_messages_are_batched_within_interval() {
    // Publish 10 messages rapidly
    // WS client should receive batched frames, not 10 individual frames
    // Verify batch size <= 50ms window
}
```

- [ ] **Step 2: Implement message batching with configurable interval**

```rust
// Add to sync.rs handle_sync_socket:
const BATCH_INTERVAL: Duration = Duration::from_millis(50);
const MAX_BATCH_SIZE: usize = 20;

let mut batch = Vec::new();
let mut batch_interval = tokio::time::interval(BATCH_INTERVAL);
batch_interval.tick().await; // consume first tick

loop {
    tokio::select! {
        msg = stream.next() => {
            if let Some(Ok(payload)) = msg {
                if let Ok(data) = payload.get_payload::<String>() {
                    batch.push(data);
                    if batch.len() >= MAX_BATCH_SIZE {
                        let batched = serde_json::json!({
                            "type": "batch",
                            "items": batch.drain(..).collect::<Vec<_>>(),
                        });
                        if sender.send(WsMessage::Text(batched.to_string().into())).await.is_err() {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
        _ = batch_interval.tick() => {
            if !batch.is_empty() {
                let batched = serde_json::json!({
                    "type": "batch",
                    "items": batch.drain(..).collect::<Vec<_>>(),
                });
                if sender.send(WsMessage::Text(batched.to_string().into())).await.is_err() {
                    break;
                }
            }
        }
    }
}
```

- [ ] **Step 3: Add backpressure with bounded channel**

```rust
// Replace direct Redis->WS forwarding with bounded mpsc channel
let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(256);

// Redis subscriber pushes to tx (non-blocking)
// WS sender reads from rx with batch logic
// If tx is full, drop oldest messages (oldest = least relevant)
```

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**

```bash
git add src/server/api/sync.rs src/server/api/agent_feed.rs
git commit -m "perf: add message batching and backpressure to WebSocket handlers"
```

---

## Phase 2: Unified Transport & UI Improvements (Week 2)

### Task 5: Unified WebSocket Protocol with Multiplexed Channels

**Files:**
- Create: `src/server/api/unified_ws.rs`
- Modify: `src/server/lib.rs` (add unified WS route)

**Interfaces:**
- Consumes: All existing WS topics (sync, feed, mesh)
- Produces: Single WS connection with channel multiplexing

- [ ] **Step 1: Design multiplexed message format**

```json
{
  "channel": "agent_feed|sync|mesh",
  "topic": "inventory:tenant-123",
  "data": { ... },
  "seq": 12345,
  "ts": 1679900000000
}
```

- [ ] **Step 2: Implement unified WS handler**

```rust
// Single WS endpoint that multiplexes channels:
// - ?channels=sync,feed,mesh
// - Client subscribes/unsubscribes via WS messages:
//   {"action": "subscribe", "channel": "sync", "topic": "inventory:tenant-123"}
//   {"action": "unsubscribe", "channel": "sync", "topic": "inventory:tenant-123"}
```

- [ ] **Step 3: Add sequence numbers for gap detection**

```rust
// Each message gets a monotonically increasing seq number per channel
// Client can detect gaps and request replay:
//   {"action": "replay", "channel": "sync", "from_seq": 12340}
```

- [ ] **Step 4: Add server-side message replay from Redis stream**

```rust
// Use Redis XREAD with stream IDs for replay:
// XRANGE channel:{topic} {from_seq} +
```

- [ ] **Step 5: Run tests**

- [ ] **Step 6: Commit**

```bash
git add src/server/api/unified_ws.rs
git commit -m "feat: unified WebSocket with multiplexed channels and gap detection"
```

---

### Task 6: Add SSE Fallback for Next.js Web UI

**Files:**
- Create: `src/ui/next/src/hooks/useAgentSSE.ts`
- Modify: `src/ui/next/src/app/agents/page.tsx:159-182`
- Modify: `src/ui/next/src/app/feed/page.tsx:45-97`

**Interfaces:**
- Consumes: `/api/v1/agents/:id/stream` SSE endpoint
- Produces: React hook returning streaming agent events

- [ ] **Step 1: Create useAgentSSE hook**

```typescript
// src/ui/next/src/hooks/useAgentSSE.ts
import { useEffect, useState, useCallback, useRef } from 'react';

interface SSEOptions {
  agentId: string;
  onMessage: (data: any) => void;
  onError?: (error: Event) => void;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
}

export function useAgentSSE({
  agentId,
  onMessage,
  onError,
  reconnectInterval = 3000,
  maxReconnectAttempts = 10,
}: SSEOptions) {
  const [connected, setConnected] = useState(false);
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const eventSourceRef = useRef<EventSource | null>(null);

  const connect = useCallback(() => {
    if (eventSourceRef.current) {
      eventSourceRef.current.close();
    }

    const es = new EventSource(`/api/v1/agents/${agentId}/stream`);
    eventSourceRef.current = es;

    es.onopen = () => {
      setConnected(true);
      setReconnectAttempts(0);
    };

    es.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        onMessage(data);
      } catch (e) {
        console.error('SSE parse error:', e);
      }
    };

    es.onerror = (error) => {
      setConnected(false);
      es.close();
      onError?.(error);

      if (reconnectAttempts < maxReconnectAttempts) {
        setTimeout(() => {
          setReconnectAttempts((prev) => prev + 1);
          connect();
        }, reconnectInterval * Math.min(reconnectAttempts + 1, 5));
      }
    };
  }, [agentId, onMessage, reconnectAttempts, reconnectInterval, maxReconnectAttempts]);

  useEffect(() => {
    connect();
    return () => eventSourceRef.current?.close();
  }, [connect]);

  return { connected, reconnectAttempts };
}
```

- [ ] **Step 2: Create useAgentWebSocket hook with reconnection**

```typescript
// src/ui/next/src/hooks/useAgentWebSocket.ts
export function useAgentWebSocket({
  url,
  onMessage,
  reconnectInterval = 3000,
}: {
  url: string;
  onMessage: (data: any) => void;
  reconnectInterval?: number;
}) {
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimer = useRef<NodeJS.Timeout>();

  const connect = useCallback(() => {
    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => setConnected(true);
    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === 'batch') {
          data.items.forEach((item: string) => onMessage(JSON.parse(item)));
        } else {
          onMessage(data);
        }
      } catch (e) {
        console.error('WS parse error:', e);
      }
    };
    ws.onerror = () => ws.close();
    ws.onclose = () => {
      setConnected(false);
      reconnectTimer.current = setTimeout(connect, reconnectInterval);
    };
  }, [url, onMessage, reconnectInterval]);

  useEffect(() => {
    connect();
    return () => {
      wsRef.current?.close();
      clearTimeout(reconnectTimer.current);
    };
  }, [connect]);

  return { connected };
}
```

- [ ] **Step 3: Update agents/page.tsx to use new hooks**

Replace lines 159-182 with `useAgentWebSocket` hook that has automatic reconnection and handles batched messages.

- [ ] **Step 4: Update feed/page.tsx to use new hooks**

- [ ] **Step 5: Run tests**

```bash
cd src/ui/next && npx vitest run
```

- [ ] **Step 6: Commit**

```bash
git add src/ui/next/src/hooks/useAgentSSE.ts src/ui/next/src/hooks/useAgentWebSocket.ts src/ui/next/src/app/agents/page.tsx src/ui/next/src/app/feed/page.tsx
git commit -m "feat: add SSE/WS hooks with reconnection and batch handling for Next.js UI"
```

---

### Task 7: Replace Polling Loops with Event-Driven Notification

**Files:**
- Modify: `src/server/msgbus.rs:334` (IpcBus polling)
- Modify: `src/agents/builtin/mesh/transport.rs:343` (PgTransport polling)
- Modify: `src/agents/builtin/mesh/transport.rs:606` (SqliteTransport polling)

**Interfaces:**
- Consumes: Current 10ms/50ms polling implementations
- Produces: Event-driven notification via PostgreSQL LISTEN/NOTIFY or filesystem watches

- [ ] **Step 1: Write test for LISTEN/NOTIFY based transport**

```rust
#[tokio::test]
async fn test_pg_notify_replaces_polling() {
    // Verify messages arrive via NOTIFY without polling
}
```

- [ ] **Step 2: Implement PostgreSQL LISTEN/NOTIFY for PgTransport**

```rust
// Replace polling loop with:
// 1. LISTEN on mesh_messages_notify channel
// 2. On NOTIFY, read new messages with SKIP LOCKED
// 3. Fall back to polling only if LISTEN fails
```

- [ ] **Step 3: Increase IpcBus polling interval to 100ms (10x reduction)**

```rust
// Change from 10ms to 100ms sleep
tokio::time::sleep(Duration::from_millis(100)).await;
```

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**

```bash
git add src/server/msgbus.rs src/agents/builtin/mesh/transport.rs
git commit -m "perf: replace polling loops with PostgreSQL LISTEN/NOTIFY"
```

---

### Task 8: Add Message Compression for WebSocket Frames

**Files:**
- Modify: `src/server/api/sync.rs`
- Modify: `src/server/api/agent_feed.rs`
- Modify: `src/ui/next/src/hooks/useAgentWebSocket.ts`

**Interfaces:**
- Consumes: JSON WebSocket messages
- Produces: gzip-compressed frames when > 1KB

- [ ] **Step 1: Write test for compression**

```rust
#[tokio::test]
async fn test_large_ws_messages_are_compressed() {
    // Send message > 1KB, verify it arrives compressed
}
```

- [ ] **Step 2: Add server-side compression**

```rust
// Use flate2 for compression on messages > 1KB
use flate2::write::GzEncoder;
use flate2::Compression;

fn maybe_compress(data: &[u8]) -> Vec<u8> {
    if data.len() > 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    } else {
        data.to_vec()
    }
}
```

- [ ] **Step 3: Add client-side decompression**

```typescript
// In useAgentWebSocket.ts, decompress if needed:
async function decompressIfNeeded(data: ArrayBuffer): Promise<string> {
  if (data.byteLength > 1024) {
    const ds = new DecompressionStream('gzip');
    const writer = ds.writable.getWriter();
    writer.write(new Uint8Array(data));
    writer.close();
    const reader = ds.readable.getReader();
    const chunks: Uint8Array[] = [];
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks.push(value);
    }
    const total = chunks.reduce((a, c) => a + c.length, 0);
    const result = new Uint8Array(total);
    let offset = 0;
    for (const chunk of chunks) {
      result.set(chunk, offset);
      offset += chunk.length;
    }
    return new TextDecoder().decode(result);
  }
  return new TextDecoder().decode(data);
}
```

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**

```bash
git add src/server/api/sync.rs src/server/api/agent_feed.rs src/ui/next/src/hooks/useAgentWebSocket.ts
git commit -m "perf: add gzip compression for large WebSocket frames"
```

---

## Phase 3: Advanced Features (Week 3)

### Task 9: Multi-Agent Parallel Execution with Work Results Streaming

**Files:**
- Create: `src/server/api/agent_orchestrate.rs`
- Modify: `src/server/hub.rs`

**Interfaces:**
- Consumes: User task prompt, expert catalog
- Produces: Parallel agent execution with real-time progress streaming

- [ ] **Step 1: Design orchestration protocol**

```json
// Request:
POST /api/v1/agents/orchestrate
{
  "task": "Analyze last month's sales and create a report",
  "experts": ["data-analyst", "report-writer", "chart-generator"],
  "stream": true
}

// SSE Response:
event: orchestration_start
data: {"plan_id": "plan-123", "steps": [...]}

event: agent_start
data: {"agent_id": "data-analyst", "step": 1}

event: agent_progress
data: {"agent_id": "data-analyst", "progress": 0.5, "status": "analyzing data"}

event: agent_complete
data: {"agent_id": "data-analyst", "result": {...}}

event: orchestration_complete
data: {"plan_id": "plan-123", "results": [...]}
```

- [ ] **Step 2: Implement orchestration engine**

```rust
pub async fn orchestrate_agents(
    task: String,
    experts: Vec<String>,
    hub: Arc<Hub>,
) -> impl Stream<Item = OrchestrationEvent> {
    // 1. Decompose task into sub-tasks
    // 2. Assign sub-tasks to experts
    // 3. Execute in parallel via tokio::spawn
    // 4. Stream progress events via broadcast channel
    // 5. Collect results and synthesize
}
```

- [ ] **Step 3: Add agent-to-agent delegation via mesh**

```rust
// Agents can delegate subtasks to other agents:
// Agent A -> mesh: "delegate task to Agent B"
// Agent B receives via mesh transport
// Agent B executes and returns result via mesh
```

- [ ] **Step 4: Run tests**

- [ ] **Step 5: Commit**

```bash
git add src/server/api/agent_orchestrate.rs src/server/hub.rs
git commit -m "feat: multi-agent parallel orchestration with real-time streaming"
```

---

### Task 10: Agent Performance Metrics Dashboard

**Files:**
- Create: `src/server/api/agent_metrics.rs`
- Modify: `src/ui/next/src/app/agents/page.tsx`

**Interfaces:**
- Consumes: Agent execution telemetry
- Produces: Real-time metrics via SSE

- [ ] **Step 1: Define metrics schema**

```rust
#[derive(Serialize)]
pub struct AgentMetrics {
    pub agent_id: String,
    pub messages_processed: u64,
    pub avg_response_time_ms: f64,
    pub active_connections: u32,
    pub memory_usage_bytes: u64,
    pub last_active_at: DateTime<Utc>,
    pub error_rate: f64,
    pub cost_accumulated: f64,
}
```

- [ ] **Step 2: Implement metrics collection endpoint**

- [ ] **Step 3: Add real-time metrics streaming via SSE**

- [ ] **Step 4: Add metrics visualization in agents page**

- [ ] **Step 5: Run tests**

- [ ] **Step 6: Commit**

```bash
git add src/server/api/agent_metrics.rs src/ui/next/src/app/agents/page.tsx
git commit -m "feat: agent performance metrics with real-time SSE streaming"
```

---

## Global Constraints

- All Rust code must pass `cargo clippy` with no warnings
- All TypeScript tests must pass with `npm test`
- WebSocket messages must be backwards-compatible (support both batched and non-batched)
- SSE endpoints must include proper CORS headers for Tauri desktop app
- All new code must include `#[cfg(test)]` modules with happy-path and error-path tests
- Redis operations must have timeout and retry logic
- Hub lock acquisition must follow consistent ordering to prevent deadlocks

## Performance Targets

| Metric | Current | Target |
|--------|---------|--------|
| WS message latency (p99) | ~200ms | <50ms |
| Max concurrent WS clients | ~500 | 5,000+ |
| LLM token streaming latency | N/A (no SSE) | <100ms |
| Memory per WS connection | ~50KB | <10KB |
| Redis pubsub connections | 1 per client | Shared pool |
| UI re-render frequency | Every message | Batched (50ms) |
| Reconnection recovery | Manual | Automatic (3s) |
| Message loss on overload | Permanent | Zero (with replay) |

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    UI Layer                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ Next.js  │  │  Tauri   │  │  Mobile  │              │
│  │ (SSE/WS) │  │  (WS)    │  │ (SSE)   │              │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘              │
│       │              │              │                    │
│  ┌────┴──────────────┴──────────────┴────┐              │
│  │         Unified WebSocket Gateway     │              │
│  │    (multiplexed channels, batching,   │              │
│  │     backpressure, compression)        │              │
│  └────────────────┬──────────────────────┘              │
├───────────────────┼─────────────────────────────────────┤
│              Transport Layer                             │
│  ┌────────┬───────┴──────┬──────────┐                   │
│  │ Redis  │ PostgreSQL   │  NATS    │                   │
│  │ PubSub │ LISTEN/NOTIFY│ JetStream│                   │
│  └────────┴──────────────┴──────────┘                   │
├─────────────────────────────────────────────────────────┤
│              Agent Orchestration                         │
│  ┌──────────────────────────────────────┐               │
│  │  Hub (tokio::sync::RwLock)          │               │
│  │  ├── Agent Registry                  │               │
│  │  ├── Message Bus (unified)           │               │
│  │  ├── Meeting Rooms                   │               │
│  │  └── Mesh Transport                  │               │
│  └──────────────────────────────────────┘               │
│  ┌──────────────────────────────────────┐               │
│  │  Agent Harness (parallel execution)  │               │
│  │  ├── LLM Streaming (SSE chunks)      │               │
│  │  ├── Tool Execution (sandboxed)      │               │
│  │  └── Agent-to-Agent (mesh)           │               │
│  └──────────────────────────────────────┘               │
└─────────────────────────────────────────────────────────┘
```

## WorkBuddy-Inspired Features for Small Business

| WorkBuddy Feature | OHC Equivalent | Implementation |
|---|---|---|
| Multi-expert parallel execution | Agent orchestration | Task 9 |
| Real-time progress streaming | SSE endpoints | Task 3 |
| WeChat/WeCom integration | Existing WhatsApp/Teams | Already exists |
| Local file access | Sandboxed agent execution | Already exists |
| Skills packages | MCP protocol + tools | Already exists |
| Mobile remote control | SSE for mobile clients | Task 6 |
| Task decomposition | Orchestration engine | Task 9 |
| Document generation | Agent tool results streaming | Task 3 |
