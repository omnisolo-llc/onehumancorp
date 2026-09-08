# Harness-Neutral JSON-RPC Runtime and Codex App-Server v2 Codec Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a reusable bidirectional JSON-RPC process runtime and a native Codex app-server v2 codec that routes real sessions, turns, streaming events, and server-request exchanges through the existing OmniSolo harness worker.

**Architecture:** Keep the existing first-party OmniSolo adapter and legacy custom-envelope process adapter. Add a protocol-neutral JSONL runtime plus a codec interface; select `CodexAppServerV2Codec` when `HarnessProtocolKind::CodexAppServer` is configured. Extend the adapter/worker stream boundary so native turn events can be delivered while `WorkerExchange` resolves pending Codex requests concurrently.

**Tech Stack:** Rust 2024, Tokio, `serde_json`, `async-trait`, `tokio-stream`, tonic gRPC, Codex app-server v2 JSONL/JSON-RPC, Cargo integration tests, and an opt-in live Codex E2E.

---

## File Map

- Create `src/server/harness/middleware/json_rpc.rs` for JSONL framing, JSON-RPC IDs/messages, child-process ownership, request correlation, notification routing, server-request response routing, timeouts, and process failure.
- Create `src/server/harness/middleware/protocol.rs` for the protocol codec trait, native operation descriptions, streaming execution item types, and the protocol-neutral adapter runtime boundary.
- Create `src/server/harness/middleware/codex_app_server.rs` for Codex app-server v2 request construction, response decoding, notification mapping, safe capsule item encoding, server-request response encoding, and Codex-specific native state.
- Modify `src/server/harness/middleware/mod.rs` to expose the new modules.
- Modify `src/server/harness/middleware/harness.rs` to preserve the legacy process adapter, select the protocol runtime for native protocols, expose streaming attempt operations, and retain existing adapter compatibility.
- Modify `src/server/harness/middleware/grpc.rs` to consume adapter streams without holding the adapter mutex and to route native server-request exchanges concurrently.
- Modify `src/server/harness/tests/external_adapters.rs`, `src/server/harness/tests/worker_grpc.rs`, and inline harness tests to mark legacy fake processes as `custom` where they use the OmniSolo envelope.
- Create `src/server/harness/tests/json_rpc_runtime.rs` for deterministic transport/runtime tests.
- Create `src/server/harness/tests/codex_app_server.rs` for codec and fake app-server integration tests.
- Modify `src/server/harness/Cargo.toml` and `src/server/harness/BUILD.bazel` for module and test dependencies/targets.
- Create `src/server/harness_worker/tests/live_codex_app_server_e2e.rs` for the ignored, real API worker E2E.
- Modify `src/server/harness_worker/Cargo.toml` only if the live test requires a missing test dependency.
- Modify deployment documentation/configuration only after the worker E2E is passing; retain existing Docker/Kubernetes environment names.

## Task 1: Establish the Failing Runtime Contract

**Files:**
- Create: `src/server/harness/middleware/json_rpc.rs`
- Create: `src/server/harness/middleware/protocol.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Create: `src/server/harness/tests/json_rpc_runtime.rs`

- [x] **Step 1: Write the failing JSON-RPC framing and correlation tests.**

Add tests that express the runtime API before its implementation exists:

```rust
#[tokio::test]
async fn runtime_correlates_out_of_order_responses_and_routes_notifications() {
    let script = fake_json_rpc_server_script();
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(script))
        .await
        .unwrap();
    let mut notifications = runtime.subscribe_notifications();

    let first = runtime.request("first", serde_json::json!({"value": 1}));
    let second = runtime.request("second", serde_json::json!({"value": 2}));
    let (first, second) = tokio::join!(first, second);

    assert_eq!(first.unwrap(), serde_json::json!({"result": "first"}));
    assert_eq!(second.unwrap(), serde_json::json!({"result": "second"}));
    assert_eq!(notifications.recv().await.unwrap().method, "progress");
}

#[tokio::test]
async fn runtime_exposes_server_requests_and_accepts_a_matching_response() {
    let runtime = JsonRpcProcessRuntime::spawn(
        JsonRpcProcessConfig::shell(fake_server_request_script()),
    )
    .await
    .unwrap();
    let mut requests = runtime.subscribe_server_requests();

    runtime.notify("start", serde_json::Value::Null).await.unwrap();
    let request = requests.recv().await.unwrap();
    assert_eq!(request.method, "item/commandExecution/requestApproval");
    runtime
        .respond_server_request(request.id, serde_json::json!({"decision": "accept"}))
        .await
        .unwrap();
}

#[tokio::test]
async fn runtime_reports_malformed_frames_timeout_and_process_exit() {
    let malformed = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        "printf '%s\\n' '{not-json}'",
    ))
    .await
    .unwrap();
    assert!(matches!(malformed.next_message().await, Err(JsonRpcError::Malformed(_))));

    let timeout_runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        "read line; sleep 2",
    ))
    .await
    .unwrap();
    assert!(matches!(
        timeout_runtime.request_with_timeout("slow", serde_json::Value::Null, Duration::from_millis(20)).await,
        Err(JsonRpcError::Timeout)
    ));
}
```

The test helper scripts must answer only JSON-RPC messages, send a response
with a different request ID first, and emit one notification between responses.
They must not use `jq`, network access, or credentials.

Define the test-local helpers explicitly in this test file:

```rust
fn fake_json_rpc_server_script() -> &'static str {
    r#"
while IFS= read -r line; do
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^"]*\)".*/\1/p')
  case "$method" in
    first) printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"result":"second"}}' ;;
    second) printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"result":"first"}}' ;;
    *) printf '%s\n' '{"jsonrpc":"2.0","method":"progress","params":{"step":1}}' ;;
  esac
done
"#
}

fn fake_server_request_script() -> &'static str {
    r#"
while IFS= read -r line; do
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^"]*\)".*/\1/p')
  if [ "$method" = "start" ]; then
    printf '%s\n' '{"jsonrpc":"2.0","id":9001,"method":"item/commandExecution/requestApproval","params":{"command":"echo test"}}'
    read response
    printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"accepted":true}}'
  fi
done
"#
}
```

- [x] **Step 2: Run the focused tests and verify they fail for the missing runtime contract.**

Run:

```bash
cargo test -p server_harness --test json_rpc_runtime -- --nocapture
```

Expected result: compilation fails because `JsonRpcProcessRuntime`,
`JsonRpcProcessConfig`, `JsonRpcError`, and the notification/server-request
types are not yet implemented. Fix only test syntax or dependency omissions if
the failure is unrelated to the missing implementation.

- [x] **Step 3: Define the protocol-neutral public types and module exports.**

Add these stable boundaries:

```rust
pub type JsonRpcResult = Result<serde_json::Value, JsonRpcError>;

pub struct JsonRpcProcessConfig {
    pub executable: String,
    pub args: Vec<String>,
    pub environment: std::collections::BTreeMap<String, String>,
    pub request_timeout: std::time::Duration,
}

pub struct JsonRpcId {
    pub raw: serde_json::Value,
    pub key: String,
}

pub enum JsonRpcError {
    Spawn(String),
    Io(String),
    Malformed(String),
    InvalidMessage(String),
    Timeout,
    ProcessExited,
    UnknownRequest(JsonRpcId),
    RequestFailed(JsonRpcErrorObject),
    Cancelled,
}

pub struct JsonRpcRequestSpec {
    pub method: String,
    pub params: serde_json::Value,
}

pub struct JsonRpcErrorObject {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub struct JsonRpcNotification {
    pub method: String,
    pub params: serde_json::Value,
}

pub struct JsonRpcServerRequest {
    pub id: JsonRpcId,
    pub method: String,
    pub params: serde_json::Value,
}

pub struct JsonRpcProcessRuntime {
    inner: std::sync::Arc<RuntimeState>,
}

struct RuntimeState {
    writer: tokio::sync::Mutex<tokio::process::ChildStdin>,
    pending: tokio::sync::Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<JsonRpcResult>>>,
    notifications: tokio::sync::broadcast::Sender<JsonRpcNotification>,
    server_requests: tokio::sync::broadcast::Sender<JsonRpcServerRequest>,
}

impl JsonRpcProcessRuntime {
    pub fn shell(script: impl Into<String>) -> JsonRpcProcessConfig;
    pub async fn spawn(config: JsonRpcProcessConfig) -> Result<Self, JsonRpcError>;
    pub async fn request(&self, method: &str, params: serde_json::Value) -> JsonRpcResult;
    pub async fn request_with_timeout(
        &self,
        method: &str,
        params: serde_json::Value,
        timeout: std::time::Duration,
    ) -> JsonRpcResult;
    pub async fn notify(&self, method: &str, params: serde_json::Value) -> Result<(), JsonRpcError>;
    pub async fn next_message(&self) -> Result<JsonRpcInbound, JsonRpcError>;
    pub fn subscribe_notifications(&self) -> tokio::sync::broadcast::Receiver<JsonRpcNotification>;
    pub fn subscribe_server_requests(&self) -> tokio::sync::broadcast::Receiver<JsonRpcServerRequest>;
    pub async fn respond_server_request(
        &self,
        id: JsonRpcId,
        result: serde_json::Value,
    ) -> Result<(), JsonRpcError>;
    pub async fn respond_server_error(
        &self,
        id: JsonRpcId,
        error: JsonRpcErrorObject,
    ) -> Result<(), JsonRpcError>;
    pub async fn shutdown(&self) -> Result<(), JsonRpcError>;
}

pub enum JsonRpcInbound {
    Response { id: JsonRpcId, result: JsonRpcResult },
    Notification(JsonRpcNotification),
    ServerRequest(JsonRpcServerRequest),
}
```

`JsonRpcId` must preserve the original JSON ID and compare by a canonical
string representation so numeric and string IDs cannot collide. The inbound
message enum must distinguish responses, notifications, and server requests;
an object with an `id` and `method` is a server request, while an object with an
`id` and `result`/`error` is a response.

- [x] **Step 4: Run the focused tests and verify the framing/runtime behavior passes.**

Run:

```bash
cargo test -p server_harness --test json_rpc_runtime -- --nocapture
```

Expected result: all runtime tests pass with zero failures. Confirm the child
process is terminated by `shutdown` and no reader task remains after process
exit.

- [ ] **Step 5: Commit only newly introduced runtime files if the worktree permits selective staging.**

Use:

```bash
git add src/server/harness/middleware/json_rpc.rs \
  src/server/harness/middleware/protocol.rs \
  src/server/harness/middleware/mod.rs \
  src/server/harness/tests/json_rpc_runtime.rs
git commit -m "feat: add harness json-rpc process runtime"
```

If an existing dirty file contains unrelated earlier work, leave that file
unstaged and record the task boundary in the final change summary.

## Task 2: Add the Codec Boundary and Codex v2 Request Shapes

**Files:**
- Modify: `src/server/harness/middleware/protocol.rs`
- Create: `src/server/harness/middleware/codex_app_server.rs`
- Create: `src/server/harness/tests/codex_app_server.rs`
- Modify: `src/server/harness/middleware/mod.rs`

- [x] **Step 1: Write failing codec shape tests.**

Add tests for exact v2 JSON shapes and native result extraction:

```rust
#[test]
fn codex_initialize_uses_v2_client_identity_and_capabilities() {
    let request = CodexAppServerV2Codec::new().initialize_request();
    assert_eq!(request.method, "initialize");
    assert_eq!(request.params["clientInfo"]["name"], "omnisolo");
    assert!(request.params.get("capabilities").is_some());
}

fn session_request() -> HarnessSessionRequest {
    HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "objective")
}

fn attempt_request() -> HarnessSessionRequest {
    HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "prompt")
}

fn capsule() -> SessionCapsule {
    let session_id = Uuid::new_v4();
    OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant", "portable objective")
            .with_session_id(session_id)
            .with_task_id(Uuid::new_v4()),
    )
    .unwrap()
    .export_capsule("codex", Uuid::new_v4())
    .unwrap()
}

#[test]
fn codex_session_operations_use_native_thread_methods() {
    let codec = CodexAppServerV2Codec::new();
    let request = codec.session_request(SessionOperation::Create, &session_request());
    assert_eq!(request.method, "thread/start");
    assert_eq!(request.params["ephemeral"], false);

    let request = codec.resume_request("thread-1", &session_request());
    assert_eq!(request.method, "thread/resume");
    assert_eq!(request.params["threadId"], "thread-1");
}

#[test]
fn codex_turn_input_is_a_text_user_input_and_steer_requires_expected_turn() {
    let codec = CodexAppServerV2Codec::new();
    let start = codec.turn_request("thread-1", "turn prompt", &attempt_request());
    assert_eq!(start.method, "turn/start");
    assert_eq!(start.params["threadId"], "thread-1");
    assert_eq!(start.params["input"][0]["type"], "text");
    assert_eq!(start.params["input"][0]["text"], "turn prompt");

    let steer = codec.steer_request("thread-1", "turn-1", "new prompt");
    assert_eq!(steer.method, "turn/steer");
    assert_eq!(steer.params["expectedTurnId"], "turn-1");
}

#[test]
fn codex_thread_result_becomes_native_session_and_capsule_items_are_historical() {
    let codec = CodexAppServerV2Codec::new();
    let native = codec.parse_thread_result(serde_json::json!({
        "thread": {"id": "thread-1"}
    })).unwrap();
    assert_eq!(native.native_session_id, "thread-1");

    let items = codec.capsule_items(&capsule()).unwrap();
    assert!(items.iter().all(|item| item["role"] == "user"));
    assert!(items.iter().all(|item| item["metadata"]["source"] == "omnisolo.historical"));
}
```

The tests must also assert `thread/fork`, `thread/archive`, `thread/delete`,
`thread/read`, `turn/interrupt`, `thread/inject_items`, safe runtime option
projection, and rejection of missing native IDs.

- [x] **Step 2: Run the codec tests and verify they fail before implementation.**

Run:

```bash
cargo test -p server_harness --test codex_app_server -- --nocapture
```

Expected result: compilation fails because the codec trait, request types, and
Codex v2 codec methods do not exist.

- [x] **Step 3: Define the codec API and streaming item types.**

Add the protocol boundary without coupling it to Codex names:

```rust
pub enum SessionOperation {
    Create,
    Import(SessionCapsule),
    Resume { native_session_id: String },
    Fork { native_session_id: Option<String> },
    Snapshot,
    Quiesce,
    Cancel,
    Close,
    Delete,
}

pub enum AttemptOperation {
    Start,
    Execute,
    Resume,
    Steer,
    Cancel,
    Quiesce,
    Reconcile,
}

pub enum HarnessExecutionItem {
    Event(HarnessEvent),
    Completed { final_text: Option<String>, usage: Option<serde_json::Value> },
}

pub type HarnessExecutionStream = std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<HarnessExecutionItem, HarnessAdapterError>> + Send>>;

pub struct NativeTurnState {
    pub thread_id: String,
    pub active_turn_id: Option<String>,
    pub native_cursor: Option<String>,
    pub pending_import: Option<Vec<serde_json::Value>>,
}

pub struct ProtocolEvent {
    pub event: HarnessEvent,
    pub native_cursor: Option<String>,
    pub final_text: Option<String>,
    pub usage: Option<serde_json::Value>,
    pub terminal: bool,
}

pub enum ServerResponse {
    Result(serde_json::Value),
    Error(JsonRpcErrorObject),
}

pub trait HarnessProtocolCodec: Send + Sync {
    fn initialize_request(&self) -> JsonRpcRequestSpec;
    fn session_request(&self, operation: SessionOperation, request: &HarnessSessionRequest) -> Result<JsonRpcRequestSpec, HarnessAdapterError>;
    fn attempt_request(&self, operation: AttemptOperation, request: &HarnessSessionRequest, prompt: &str, native_session_id: Option<&str>) -> Result<JsonRpcRequestSpec, HarnessAdapterError>;
    fn decode_session_result(&self, operation: SessionOperation, result: serde_json::Value) -> Result<NativeSession, HarnessAdapterError>;
    fn decode_notification(&self, notification: &JsonRpcNotification, state: &mut NativeTurnState) -> Result<ProtocolEvent, HarnessAdapterError>;
    fn server_request_event(&self, request: &JsonRpcServerRequest) -> Result<HarnessEvent, HarnessAdapterError>;
    fn encode_server_response(&self, method: &str, response: &serde_json::Value) -> Result<ServerResponse, HarnessAdapterError>;
}

pub struct ProtocolProcessAdapter {
    runtime: JsonRpcProcessRuntime,
    codec: Box<dyn HarnessProtocolCodec>,
    state: NativeTurnState,
}
```

`ProtocolEvent` must carry the mapped event, cursor, final text/usage updates,
and an explicit terminal flag. The codec trait remains synchronous and pure;
the JSON-RPC runtime owns all asynchronous I/O.

- [x] **Step 4: Implement Codex v2 request/response construction.**

Implement `CodexAppServerV2Codec` with these exact mappings:

```text
initialize              -> initialize
Create                  -> thread/start
Import                  -> thread/start + thread/inject_items
Resume                  -> thread/resume
Fork                    -> thread/fork
Snapshot/Reconcile      -> thread/read
Close                   -> thread/archive
Delete                  -> thread/delete
Start/Execute/Resume    -> turn/start
Steer                   -> turn/steer
Cancel/Quiesce          -> turn/interrupt
```

Use `threadId`, `turnId`, `expectedTurnId`, `input`, `clientUserMessageId`,
`cwd`, `model`, `modelProvider`, `approvalPolicy`, `sandbox`, and
`runtimeWorkspaceRoots` exactly as named by the generated v2 schema. Decode
the response thread ID from `result.thread.id`, reject an empty ID, and use
the native turn ID plus notification sequence as the opaque cursor.

For capsule import, encode only allowlisted settled messages, tool outcomes,
plans, and visible summaries as raw Responses API items. Add
`metadata.source = "omnisolo.historical"` and never encode system/developer
instructions, credentials, approvals, live handles, or native cursors.

- [x] **Step 5: Implement notification and server-request mapping.**

Map Codex v2 notifications by method prefix and exact payload fields:

```text
item/agentMessage/delta       -> transient assistant.text_chunk
item/agentMessage/completed   -> durable assistant.final
item/reasoning/*              -> assistant.reasoning
item/plan/delta               -> plan.updated
item/commandExecution/*      -> tool.output/tool.interaction
item/fileChange/*             -> file.change
item/mcpToolCall/progress     -> mcp.progress
turn/started                  -> turn.started
turn/diff/updated             -> turn.diff.updated
thread/tokenUsage/updated     -> usage.recorded
turn/completed                -> turn.completed and Completed
warning/error/model/*         -> durable warning/error events
```

Preserve the full native notification under `payload.native`, retain its
thread/turn/request IDs, and classify unknown methods as ignorable unless the
codec marks them required for replay. Map server requests to
`interaction.required` with sanitized params and a native request ID. Encode
approval/question/tool/MCP responses as the native result or error object.

- [x] **Step 6: Run the codec tests and verify all exact shape/mapping tests pass.**

Run:

```bash
cargo test -p server_harness --test codex_app_server -- --nocapture
```

Expected result: codec unit tests pass with zero failures, including request
shape, response parsing, notification mapping, capsule safety, and explicit
unsupported/error cases.

## Task 3: Integrate Protocol Adapters Without Breaking Legacy Harnesses

**Files:**
- Modify: `src/server/harness/middleware/harness.rs`
- Modify: `src/server/harness/middleware/protocol.rs`
- Modify: `src/server/harness/middleware/grpc.rs`
- Modify: `src/server/harness/tests/external_adapters.rs`
- Modify: `src/server/harness/middleware/mod.rs`

- [x] **Step 1: Add a failing adapter-selection test.**

Add a test proving that a `CodexAppServer` spec sends native `initialize`
instead of an OmniSolo envelope, while a `Custom` spec preserves the existing
envelope:

```rust
#[tokio::test]
async fn process_adapter_selects_native_codec_only_for_codex_app_server() {
    let native = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", "read line"], "codex")
            .with_protocol(HarnessProtocolKind::CodexAppServer),
    );
    assert_eq!(native.protocol_kind(), HarnessProtocolKind::CodexAppServer);

    let custom = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("/bin/sh", ["-c", "read line"], "future")
            .with_protocol(HarnessProtocolKind::Custom),
    );
    assert_eq!(custom.protocol_kind(), HarnessProtocolKind::Custom);
}
```

Update all existing fake process specs that answer `request_id`/`ok` envelopes
to explicitly use `HarnessProtocolKind::Custom`; their behavior is not a
native Codex protocol test.

- [x] **Step 2: Run the focused adapter tests and verify the selection test fails.**

Run:

```bash
cargo test -p server_harness process_adapter_selects_native_codec_only_for_codex_app_server -- --nocapture
```

Expected result: the test fails because the current process adapter has no
native codec selection or `protocol_kind` accessor.

- [x] **Step 3: Add the protocol adapter implementation.**

Refactor `ProcessHarnessAdapter` to contain either the existing legacy channel
or a protocol runtime plus boxed codec. Keep the public constructor and
`HarnessAdapter` methods stable. Add:

```rust
impl ProcessHarnessAdapter {
    pub fn protocol_kind(&self) -> HarnessProtocolKind;
    async fn ensure_protocol_runtime(&mut self) -> Result<&ProtocolProcessAdapter, HarnessAdapterError>;
}
```

`HarnessProtocolKind::CodexAppServer` constructs `CodexAppServerV2Codec` and
`JsonRpcProcessRuntime`; `Custom` and all protocol kinds without a registered
codec use the legacy OmniSolo envelope implementation. The descriptor must
advertise the actual codec protocol version and capabilities, not the legacy
process envelope.

- [x] **Step 4: Add streaming attempt operations with a compatibility default.**

Extend `HarnessAdapter` with one operation that returns an owned stream:

```rust
async fn attempt_stream(
    &mut self,
    operation: AttemptOperation,
    request: HarnessSessionRequest,
    attempt_id: &str,
    prompt: &str,
    native_session_id: Option<&str>,
) -> Result<HarnessExecutionStream, HarnessAdapterError>;
```

The default implementation calls the existing vector-returning methods and
emits `Event` items followed by one `Completed` item. The native protocol
adapter overrides it by subscribing before sending the native turn request,
then returns an owned receiver stream so the adapter mutex is not held while
notifications arrive.

- [x] **Step 5: Run legacy adapter and worker tests.**

Run:

```bash
cargo test -p server_harness --test external_adapters -- --nocapture
cargo test -p server_harness --test worker_grpc -- --nocapture
```

Expected result: existing custom-process and first-party OmniSolo tests pass,
and the new selection/stream compatibility tests pass.

## Task 4: Make Worker Streaming and Native Exchanges Concurrent

**Files:**
- Modify: `src/server/harness/middleware/grpc.rs`
- Modify: `src/server/harness/tests/worker_grpc.rs`
- Modify: `src/server/harness/middleware/harness.rs`

- [ ] **Step 1: Write the failing concurrent approval integration test.**

Add a fake Codex server that emits a command approval request and blocks until
it receives a JSON-RPC response. The test must call `attempt_command`, read an
`interaction.required` event, then call `interaction` with:

```json
{
  "native_request_id": 9001,
  "result": {"decision": "accept"}
}
```

After the exchange returns, the attempt stream must yield the assistant delta,
completion, and no deadlock/timeout. Assert that the exchange response is
accepted and that the native request ID was not treated as an OmniSolo event
sequence.

- [ ] **Step 2: Run the new integration test and verify it fails by timing out.**

Run:

```bash
cargo test -p server_harness --test worker_grpc grpc_worker_resolves_native_codex_server_request -- --nocapture
```

Expected result: the test fails or times out because the existing service holds
the adapter mutex while awaiting the entire execution.

- [ ] **Step 3: Consume adapter streams outside the mutex.**

In `attempt_command`, lock the adapter only long enough to call
`attempt_stream`, store the returned owned stream, and drop the guard before
iterating. Move final-text and usage synthesis into a stream item handler so
events are assigned durable/delivery sequences as they arrive. Preserve the
existing replay history and duplicate-command behavior.

When a stream item is an error, emit a fenced error response and release the
attempt runtime state exactly as the current adapter-error path does. When a
native completion lacks a final text event, append the existing
`assistant.final` event from `Completed.final_text`.

- [ ] **Step 4: Route native exchanges through the shared runtime.**

In `ProtocolProcessAdapter::exchange`, recognize the codec-owned response kind,
validate the payload contains `native_request_id` and exactly one `result` or
`error`, and call `respond_server_request`/`respond_server_error`. Return the
native acknowledgement payload. Keep legacy exchange forwarding unchanged for
custom processes.

- [ ] **Step 5: Run the worker integration suite.**

Run:

```bash
cargo test -p server_harness --test worker_grpc -- --nocapture
cargo test -p omnisolo_harness_worker --test worker_e2e -- --nocapture
```

Expected result: the concurrent approval test, existing worker gRPC tests, and
both worker binary E2E tests pass with zero failures.

## Task 5: Complete Session State, Capsule Import, and Failure Semantics

**Files:**
- Modify: `src/server/harness/middleware/codex_app_server.rs`
- Modify: `src/server/harness/middleware/protocol.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Modify: `src/server/harness/tests/codex_app_server.rs`

- [ ] **Step 1: Add failing lifecycle and failure tests.**

Cover this sequence against the fake app-server:

```text
create -> import -> execute -> steer -> reconcile -> resume -> fork -> cancel -> close -> delete
```

Assert each exact native method, returned thread/turn cursor, explicit
not-found/error behavior, process-exit behavior, and that a failed turn leaves
the native thread resumable. Add a capsule fixture containing a tool result,
assistant message, plan, and interaction outcome; assert no authority-bearing
fields or credentials enter `thread/inject_items`.

- [ ] **Step 2: Run the lifecycle tests and verify missing mappings fail.**

Run:

```bash
cargo test -p server_harness --test codex_app_server codex_lifecycle -- --nocapture
```

Expected result: the new test fails for unimplemented lifecycle operations or
incorrect native response handling.

- [ ] **Step 3: Implement native state and lifecycle mappings.**

Store per-adapter native state:

```rust
struct NativeTurnState {
    thread_id: String,
    active_turn_id: Option<String>,
    native_cursor: Option<String>,
    pending_import: Option<Vec<serde_json::Value>>,
}
```

Use `thread/read` for reconcile, `thread/resume` for an existing ID,
`thread/fork` for a new binding, `turn/interrupt` for active cancellation,
`thread/archive` for close, and `thread/delete` for delete. Treat native
not-found as an error unless the OmniSolo envelope is an exact idempotent
duplicate. Make process replacement invalidate active native session state so
resume must explicitly reload the thread.

- [ ] **Step 4: Implement safe capsule injection and cursor propagation.**

Validate capsule integrity and target identity before encoding. Convert only
allowlisted portable records to raw Responses API items, add the historical
metadata marker, call `thread/inject_items`, and retain the resulting thread
ID/cursor. Include native provenance in every mapped event without changing
durable sequence allocation.

- [ ] **Step 5: Run focused lifecycle and full server harness tests.**

Run:

```bash
cargo test -p server_harness --test codex_app_server -- --nocapture
cargo test -p server_harness --lib -- --nocapture
```

Expected result: all Codex codec/lifecycle tests and all existing server
harness unit tests pass.

## Task 6: Add the Deterministic Fake App-Server Worker E2E

**Files:**
- Modify: `src/server/harness/tests/worker_grpc.rs`
- Modify: `src/server/harness_worker/tests/worker_e2e.rs` if shared helpers are needed
- Modify: `src/server/harness/Cargo.toml` only if a test-only stream helper is missing

- [ ] **Step 1: Add a failing full worker lifecycle test.**

Configure `HarnessWorkerGrpcService` with a fake v2 app-server process and
exercise through gRPC:

1. health;
2. native session create;
3. attempt execute and streamed assistant events;
4. native approval request plus concurrent interaction response;
5. resume with the returned native thread ID;
6. fork and verify a new native thread ID;
7. interrupt/cancel and close/delete;
8. duplicate command and replay delivery.

Assert the fake process saw `initialize`, `initialized`, native thread/turn
methods, and no OmniSolo envelope fields such as `protocol_version` or
`request_id` paired with `ok`.

- [ ] **Step 2: Run the worker lifecycle test and verify it fails before the complete integration.**

Run:

```bash
cargo test -p server_harness --test worker_grpc grpc_worker_routes_native_codex_v2_lifecycle -- --nocapture
```

Expected result: failure identifies the first missing native integration
behavior, not a fake-server syntax error.

- [ ] **Step 3: Implement the fake server assertions and worker test helpers.**

Keep the fake server deterministic and local. Record each inbound JSON line in
memory or an explicit temporary file owned by the test. Use hardcoded fake
thread/turn IDs and an approval request ID. Never invoke a model or read host
authentication in this test.

- [ ] **Step 4: Run the complete deterministic worker suite.**

Run:

```bash
cargo test -p server_harness --test worker_grpc -- --nocapture
cargo test -p omnisolo_harness_worker --test worker_e2e -- --nocapture
```

Expected result: all deterministic native, legacy external, and first-party
OmniSolo worker tests pass.

## Task 7: Add the Real Codex App-Server v2 E2E

**Files:**
- Create: `src/server/harness_worker/tests/live_codex_app_server_e2e.rs`
- Modify: `src/server/harness_worker/Cargo.toml` only if required for the test

- [ ] **Step 1: Add an ignored live test that requires explicit opt-in.**

The test must be marked ignored with a reason such as
`requires live Codex authentication` and launch the worker binary with:

```text
OMNISOLO_HARNESS_EXECUTABLE=codex
OMNISOLO_HARNESS_ARGS_JSON=["app-server","--stdio"]
OMNISOLO_HARNESS_PROTOCOL=codex_app_server
OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS=180
```

Use a temporary workspace directory and a deterministic prompt requiring the
marker `OMNISOLO_NATIVE_APP_SERVER_E2E_OK`. Do not set or print an API key;
the child inherits the host Codex CLI authentication. Capture protocol and
stderr only in memory or a test-owned temporary file and assert no configured
credential value is present without ever printing that value.

- [ ] **Step 2: Run the ignored test without the implementation and verify the native path is not yet passing.**

Run:

```bash
OMNISOLO_LIVE_CODEX_E2E=1 cargo test -p omnisolo_harness_worker \
  --test live_codex_app_server_e2e -- --ignored --nocapture
```

Expected result before the codec integration is complete: the test fails at
native initialization or response translation. Record the exact failure
without exposing protocol credentials.

- [ ] **Step 3: Implement the live worker assertions.**

Verify in order:

1. worker `/readyz` and gRPC health;
2. `initialize`/`initialized` native handshake;
3. `thread/start` and a non-empty native session ID;
4. `turn/start` and at least one streamed assistant delta;
5. authoritative completion containing the marker;
6. `thread/resume` using the native ID and a second marker turn;
7. `thread/archive` or `thread/delete` and worker process shutdown;
8. no OmniSolo legacy envelope was sent to the Codex child.

- [ ] **Step 4: Run the live E2E after implementation.**

Run the exact command from Step 2. Expected result: one ignored test is
selected and passes, with zero failures and a real model response. Also run
the deterministic worker tests immediately afterward to ensure the live child
did not affect shared state.

## Task 8: Final Compatibility, Deployment, and Verification

**Files:**
- Modify: `src/server/harness/BUILD.bazel`
- Modify: `src/server/harness/Cargo.toml`
- Modify: `src/server/harness_worker/Cargo.toml` if needed
- Modify: `docs/omnisolo-harness-compatibility-inventory.md`
- Modify: deployment documentation/configuration only when a concrete change is required

- [ ] **Step 1: Add all new Rust modules/tests to Cargo and Bazel metadata.**

Ensure the new modules are listed in `rust_library.srcs`, the integration test
target includes its test source, and dependencies used by the runtime are
declared in both Cargo and Bazel. Run the repository’s existing metadata/build
checks rather than relying only on Cargo.

- [ ] **Step 2: Document the native Codex worker configuration.**

Document the executable, args JSON, protocol name, timeout, required Codex
binary, and authentication reference/mount behavior. Do not document a raw
API key or add one to any fixture. Keep Docker/Kubernetes worker scaling and
session affinity guidance aligned with the existing deployment manifests.

- [ ] **Step 3: Run formatting, compile, unit, integration, and deployment checks.**

Run:

```bash
cargo fmt --all -- --check
cargo test -p server_harness --lib -- --nocapture
cargo test -p server_harness --tests -- --nocapture
cargo test -p omnisolo_harness_worker --tests -- --nocapture
bash deploy/tests/harness_worker_deployment_contract_test.sh
git diff --check
```

Expected result: every command exits `0`; test output reports zero failures.

- [ ] **Step 4: Re-run the live API E2E as the final protocol verification.**

Run:

```bash
OMNISOLO_LIVE_CODEX_E2E=1 cargo test -p omnisolo_harness_worker \
  --test live_codex_app_server_e2e -- --ignored --nocapture
```

Read the complete output, record pass/fail counts, verify no worker or Codex
child remains, and check the final VCS diff for credential material before
claiming completion.

- [ ] **Step 5: Request a final code review before integration.**

Review the complete diff against
`docs/superpowers/specs/2026-08-23-harness-json-rpc-codec-design.md`, focusing
on protocol fidelity, event ordering, server-request concurrency, lease
fencing, capsule authority boundaries, and live-test secret handling. Resolve
all critical/important findings before declaring the feature complete.
