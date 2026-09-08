# Harness-Neutral JSON-RPC Runtime and Codex App-Server v2 Codec

**Status:** Approved design

**Goal:** Replace the current OmniSolo-envelope-only process path for Codex with a native, bidirectional Codex app-server v2 translation while establishing a reusable JSON-RPC runtime for future harness codecs.

## Scope

This change covers the protocol boundary used by OmniSolo session, task, turn,
attempt, worker, and exchange operations. It does not reimplement the entire
Codex product API as an OmniSolo public API. Methods that do not have a
corresponding `HarnessAdapter` operation remain explicit unsupported-capability
errors instead of being silently approximated.

The implementation targets the Codex app-server v2 schema exposed by the
configured Codex binary. Protocol version negotiation is explicit. A v1,
unknown, or incompatible server fails during initialization with a typed
protocol error.

The first-party OmniSolo harness remains a separate adapter. Existing custom
JSON-lines process harnesses remain supported through the legacy OmniSolo
process envelope. The new runtime and codec are selected only for
`HarnessProtocolKind::CodexAppServer` and other protocols that opt into the
new codec boundary.

## Architecture

```text
HarnessAdapter
    |
ProtocolProcessAdapter
    |-- JsonRpcProcessRuntime
    |     |-- child process and stdin writer
    |     |-- stdout reader and JSONL framing
    |     |-- request/response correlation
    |     |-- notification and server-request routing
    |     |-- pending request resolution and cancellation
    |
    +-- HarnessProtocolCodec
          |
          +-- CodexAppServerV2Codec
```

### JSON-RPC process runtime

The runtime owns one long-lived child process for a worker adapter. It starts
the configured executable and arguments, keeps stderr out of the protocol
stream, and exposes typed operations for:

- sending a client request and awaiting its response;
- sending a notification;
- subscribing to native notifications for an active thread/turn;
- observing native server-initiated requests;
- answering a server request by its JSON-RPC request ID;
- cancelling a pending request or terminating the process.

The reader task is the only component that reads stdout. It classifies each
JSON object as a response, notification, or server request and routes it by
ID. Responses can arrive interleaved with notifications and unrelated server
requests. A request timeout never causes a second request to reuse the same
ID. Process exit completes all pending requests with a process-exited error and
closes event subscriptions.

The runtime is protocol-neutral. JSON-RPC framing, request IDs, error objects,
timeouts, and transport failures are not duplicated in individual harness
codecs.

### Codec boundary

The codec owns only native method names, parameter/response construction,
notification classification, server-request response encoding, and conversion
to the existing OmniSolo harness types. It receives a `HarnessSessionRequest`
or attempt operation and returns a native operation description, event mapping,
or typed unsupported-operation error.

The codec must preserve the native method, request ID, raw params/result/error,
thread ID, turn ID, and native event cursor in the existing extension/native
payload fields. It must not use a native ID as an OmniSolo durable sequence or
lease fence.

## Codex v2 mapping

The native executable is configured through the existing worker environment:

```text
OMNISOLO_HARNESS_EXECUTABLE=codex
OMNISOLO_HARNESS_ARGS_JSON=["app-server","--stdio"]
OMNISOLO_HARNESS_PROTOCOL=codex_app_server
```

The process inherits authentication from the worker deployment. Credentials
are never placed in JSON-RPC payloads, event payloads, logs, capsules, or
temporary files.

### Initialization

On first use, the runtime sends native v2 `initialize` with an OmniSolo client
identity and the capabilities required for streaming, server requests, and
experimental fields used by the codec. It waits for a successful response and
sends the `initialized` notification. Initialization is performed once per
child process and is retried only after the process has been replaced.

The selected protocol/schema revision and native capability response are
retained in the harness descriptor/native record. Missing required capabilities
produce an actionable adapter error before a session is admitted.

### Session operations

| OmniSolo operation | Native request | Result |
| --- | --- | --- |
| create | `thread/start` | Native thread ID and initial cursor |
| import/handoff | `thread/start`, then `thread/inject_items` | New thread plus imported historical context |
| resume | `thread/resume` | Existing native thread and loaded cursor |
| fork | `thread/fork` | New native thread ID |
| snapshot/reconcile | `thread/read` | Current native thread/cursor metadata |
| quiesce/cancel | Active `turn/interrupt` when a turn exists | Native thread remains resumable |
| close | `thread/archive` | Archived native thread |
| delete | `thread/delete` | Deleted native thread |

Session operations carry the existing tenant/session/task metadata through
codec-owned native metadata where the Codex schema permits it. The native
thread ID is returned as `NativeSession.native_session_id`; the latest native
turn/event cursor is returned as `native_cursor`.

Capsule import uses `thread/inject_items` with raw Responses API items that are
explicitly marked as historical OmniSolo context. Imported content is placed
in the model-visible history channel and is never sent as
`developerInstructions`, `baseInstructions`, permissions, approval policy, or
other privileged configuration. Capsule integrity, tenant, session, and target
harness checks remain mandatory before injection.

### Attempt operations

| OmniSolo operation | Native request | Behavior |
| --- | --- | --- |
| start/execute/resume | `turn/start` | Stream native notifications until turn completion |
| steer | `turn/steer` | Require the active native turn ID as `expectedTurnId` |
| cancel/quiesce | `turn/interrupt` | Interrupt the active native turn and close its event stream |
| reconcile | `thread/read` plus active-turn state | Rebuild the latest observable execution state |
| checkpoint | Native cursor record | Return an opaque native checkpoint reference without claiming portable exact resume |

`turn/start` input is built from the prompt as a text `UserInput` item. The
adapter may include safe context fragments and runtime workspace roots from
the request. Model, service tier, reasoning effort, sandbox, approval, and
workspace overrides are taken from the effective runtime snapshot when
present; they are never inferred from an untrusted capsule string.

## Event and interaction translation

The codec maps native notifications into `HarnessEvent` values. Every mapped
event retains the original native notification in a `native` object and carries
the latest native cursor when available.

| Native notification family | OmniSolo event |
| --- | --- |
| `turn/started` | `turn.started` |
| `item/agentMessage/delta` | transient `assistant.text_chunk` |
| completed agent message | durable `assistant.final` / final text |
| reasoning summary/text deltas | `assistant.reasoning` with visibility metadata |
| plan updates | `plan.updated` |
| command execution output/terminal interaction | `tool.output` / `tool.interaction` |
| file change output/patch updates | `file.change` |
| MCP progress/tool completion | `mcp.progress` / `tool.call_settled` |
| `turn/diff/updated` | `turn.diff.updated` |
| `thread/tokenUsage/updated` | durable `usage.recorded` |
| `turn/completed` | durable `turn.completed` |
| warning/error/model reroute | corresponding durable warning/error event |

Unknown native notifications are preserved as ignorable native events. An
unknown notification that the v2 schema marks required for replay fails the
execution rather than being discarded. Final text and usage are extracted
from authoritative completion records; deltas are never treated as the final
record by themselves.

Codex server-initiated requests are stored by native JSON-RPC request ID and
emitted as an `interaction.required` event containing the request method,
request ID, turn/session identity, and sanitized native params. The worker
exchange path accepts a response payload containing that native request ID and
encodes the corresponding native JSON-RPC response. A decline, cancellation,
disconnect, timeout, or turn failure resolves the pending native request with
the appropriate native error response.

The adapter must release the worker adapter mutex while an active turn waits
for native events. This permits `WorkerExchange` to resolve an approval or
question concurrently with `AttemptCommand` streaming events. Event delivery
remains fenced by the existing lease generation and fencing token.

## Failure and lifecycle rules

- Malformed JSON, invalid JSON-RPC envelopes, mismatched IDs, unsupported
  methods, native error objects, timeouts, and process exits have distinct
  `HarnessAdapterError` variants or stable error messages.
- A failed initialization cannot be reused. The child is terminated and the
  next operation starts a fresh initialization attempt.
- A failed turn does not delete the native thread. Reconcile or resume may
  recover it when Codex reports the thread as loadable.
- A stale OmniSolo lease cannot answer a native server request or append an
  event. Pending requests are rejected when their attempt is fenced.
- Native cursors, thread IDs, request IDs, and raw native records are opaque
  provenance. Durable OmniSolo event sequences remain allocated by the worker
  service.
- Close and delete are idempotent at the OmniSolo envelope layer. Native
  not-found responses are accepted only when the requested operation is an
  already-completed idempotent duplicate.

## Testing and acceptance

### Unit tests

Tests will cover the JSON-RPC runtime and codec without a real model:

- JSONL framing, response correlation, concurrent request IDs, notification
  routing, server-request routing, timeout, malformed JSON, process exit, and
  cancellation;
- exact v2 JSON shapes for initialization, every mapped thread/turn method,
  injection, interruption, and native server-request responses;
- all supported native notification mappings, final-text selection, usage,
  cursor updates, raw-event preservation, and required/ignorable unknown events;
- capsule validation and safe historical-item encoding;
- approval, question, dynamic-tool, MCP elicitation, disconnect, and fencing
  resolution;
- explicit unsupported-operation and native-error behavior.

### Integration tests

The worker/gRPC tests will run a deterministic fake v2 app-server process. They
will verify initialization, session lifecycle, streamed events, concurrent
approval exchange, event fencing, duplicate commands, replay, resume, fork,
interrupt, process failure, and cleanup. Existing OmniSolo and legacy custom
process tests must remain green.

### Real API E2E

An opt-in integration test will launch the actual configured `codex
app-server --stdio` binary using the host's protected authentication. It will:

1. perform the native v2 handshake;
2. create a native thread;
3. start a turn with a deterministic marker prompt;
4. verify streamed assistant events and the authoritative completed response;
5. resume the same native thread and verify a second turn;
6. interrupt or close the native session and verify worker cleanup;
7. assert that no credential value appears in captured protocol, event, or
   stderr output.

The live test is explicitly opt-in so normal CI never depends on an external
model account. The verification run for this change will enable it and report
the exact command, pass/fail count, and native protocol result without
printing secrets.

## Deployment compatibility

The runtime uses only the configured executable, arguments, environment, and
working directory. It requires no host-specific socket or filesystem state.
Workers can therefore run independently in Docker or Kubernetes with the
Codex binary and authentication mounted into the worker image/pod. The gateway
and worker remain separate scaling units. Native session affinity is represented
by the binding/native session metadata, not by a permanent pod identity.

## Acceptance criteria

The implementation is complete when:

1. A worker configured with `codex app-server --stdio` performs a successful
   native v2 handshake without an OmniSolo envelope being sent to Codex.
2. Session and attempt operations listed above produce the exact native v2
   requests and return valid OmniSolo native/session/event structures.
3. Native streaming and server-request interactions work concurrently through
   the existing worker gRPC APIs.
4. The fake app-server integration suite and all existing harness tests pass.
5. The opt-in real API E2E passes through the actual worker path with a real
   model response.
6. No credentials are persisted in source, tests, logs, events, capsules, or
   artifacts.
