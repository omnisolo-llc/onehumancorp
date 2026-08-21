# Codex Harness Parity and App Server Compatibility Design

**Status:** Expanded design pending review; implementation has not started.

## Goal

Bring the existing OHC agent to feature parity with the current open-source Codex
harness and expose that behavior through a compatible App Server control plane.
The result must be usable by App Server clients without requiring an OHC-specific
client implementation, while preserving the existing OHC model providers, domain
tools, memory systems, subagent implementations, sandbox, gRPC service, and
legacy RPC methods.

This is an implementation of the harness capabilities in the existing agent, not
a migration to the Codex runtime and not a thin unary RPC wrapper around the
current loop.

## Source Baseline

The feature inventory is based on the official Codex repository at revision
`2aaefa32b0762491d1340675a6082fad26bbb57f` reviewed on 2026-08-21, together with
the official [Codex App Server documentation](https://learn.chatgpt.com/docs/app-server),
the [Codex as a platform article](https://developers.openai.com/blog/codex-as-a-platform),
and the [Codex SDK documentation](https://learn.chatgpt.com/docs/codex-sdk).

The source baseline includes both the stable and experimental App Server API.
Experimental methods and fields are part of this parity target. The connection
must negotiate them through the same initialization capability used by Codex.
The protocol inventory is regenerated from the pinned source during
implementation so method names, field names, and notification names cannot drift
silently.

## Design Principles

1. **One execution system.** The current `Agent` loop remains the execution
   authority. App Server turns, legacy RPC calls, gRPC requests, subagents,
   workflows, and direct callers all use the same tool registry, policies,
   providers, memory, and event stream.
2. **Protocol fidelity.** Client requests, responses, notifications, server
   requests, approval decisions, lifecycle ordering, error codes, experimental
   gating, and pagination are implemented from the official schema rather than
   approximated with ad hoc JSON.
3. **Blended ownership.** Harness behavior is added to the existing `agent.rs`,
   `service.rs`, `json_rpc_server.rs`, `checkpointer.rs`, tool modules, sandbox,
   MCP, plugin, memory, config, and telemetry boundaries. A new standalone
   runner layer is not introduced, and no new implementation is placed in a
   file named for a Codex runner.
4. **OHC policy remains authoritative.** App Server sandbox, permissions,
   approval, model, and account fields translate into existing OHC policy and
   provider abstractions. They do not create a second security or model-routing
   implementation.
5. **Truthful capabilities.** Every method is present in the protocol surface.
   A backend-dependent feature either executes through an OHC provider or
   returns a typed, actionable capability/configuration error. It is never
   silently advertised as working while doing something else.
6. **Durable public history is separate from recovery state.** Thread and item
   history is a client-facing contract. Checkpoints, rollout state, scratchpads,
   and provider-specific recovery data remain implementation details linked to a
   thread but are not substituted for public history.

## Complete Feature Inventory

The following inventory is the acceptance scope. The method groups are exact
wire methods from the reviewed App Server protocol; the surrounding harness
behavior is required to make those methods meaningful.

### Connection, transport, and protocol

- JSON-RPC request, response, notification, and server-initiated request
  handling over newline-delimited JSON on stdin/stdout.
- The existing HTTP JSON endpoint remains available. The same dispatcher is
  reused by JSONL and WebSocket transports; WebSocket support includes the
  control-socket upgrade path used by App Server clients.
- One-time `initialize` handshake followed by `initialized`, client identity,
  experimental API negotiation, attestation capability negotiation, MCP
  extension negotiation, and per-connection notification opt-out.
- Omitted wire JSON-RPC version compatibility, explicit `jsonrpc: "2.0"`
  compatibility for existing callers, parse errors, invalid requests, invalid
  parameters, method-not-found errors, and server overload errors.
- Server-to-client requests with independently correlated IDs. Responses to
  approval, user-input, dynamic-tool, MCP elicitation, auth-token refresh,
  attestation, and current-time requests must be routable while other turns are
  running.
- Bounded inbound and outbound queues, slow-client handling, connection cleanup,
  cancellation-aware task draining, per-connection RPC gates, and serialized
  access to shared resources. Requests are keyed by thread, process, filesystem
  watch, fuzzy-search session, MCP OAuth server, or global resource as needed.

### Threads, turns, items, and history

The public model is a durable thread containing turns, each containing ordered
items. Items include user messages, hook prompts, agent messages, plans,
reasoning, command execution, file changes, MCP calls, dynamic tools, and
collaborative-agent activity. Each item supports started, delta/progress where
applicable, and completed lifecycle states.

The implementation includes thread creation, resumption, branching, loading,
archiving, deletion, unarchiving, closing, subscription management, metadata,
names, goals, settings, memory mode, sections, rollback/revert, compaction,
history injection, summaries, usage, status, token-usage replay, pagination,
redaction, search, and recovery after process restart.

The exact thread and turn methods are:

```text
thread/start                    thread/resume
thread/fork                     thread/archive
thread/delete                   thread/unsubscribe
thread/increment_elicitation    thread/decrement_elicitation
thread/name/set                 thread/goal/set
thread/goal/get                 thread/goal/clear
thread/queue/add                thread/queue/list
thread/queue/update             thread/queue/delete
thread/queue/reorder            thread/queue/start
thread/metadata/update          thread/section/move
thread/settings/update          thread/memoryMode/set
memory/reset                    thread/unarchive
thread/compact/start            thread/shellCommand
thread/approveGuardianDeniedAction
thread/backgroundTerminals/clean
thread/backgroundTerminals/list
thread/backgroundTerminals/terminate
thread/rollback                 thread/revert
thread/list                     thread/loaded/list
thread/read                     thread/turns/list
thread/items/list               thread/inject_items
thread/search                   thread/searchOccurrences
threadSection/list              threadSection/create
threadSection/update            threadSection/delete
project/list                    project/read
project/create                  project/import
project/update                  project/move
project/delete
turn/start                      turn/steer
turn/interrupt                  review/start
getConversationSummary
```

Thread list/read APIs support the official filters, sort directions, cursors,
archived state, current working directory, model provider, source kind, search
term, section, ancestry, parent, and loaded-state projections. Resume and fork
support full history and metadata-only pagination modes. Public history is
written atomically and is safe under concurrent readers and one serialized
writer per mutable thread.

Turn execution supports immediate acknowledgement, user input items, model and
reasoning overrides, working-directory and permission overrides, collaboration
modes, skill/app/plugin invocation, steering, interruption, queued follow-up
turns, background terminals, automatic context compaction, plan updates, diffs,
token accounting, cost accounting, moderation metadata, retries, provider
routing, and completed/interrupted/failed status.

### Event and approval surface

The server emits all current lifecycle and progress notifications, including
thread status, thread metadata/name/goal/queue changes, thread token usage,
turn start/completion/diff/plan, hook start/completion, item start/completion,
agent-message deltas, plan deltas, reasoning deltas, command output and terminal
interaction, file-change output and patch updates, MCP progress, server-request
resolution, warnings, errors, deprecations, config warnings, model rerouting and
verification, safety buffering, fuzzy search, filesystem changes, account/app
updates, remote-control status, migration progress/completion, and Windows
sandbox events.

The exact notification groups include:

```text
error                           warning
thread/started                  thread/status/changed
thread/archived                 thread/deleted
thread/unarchived               thread/closed
thread/reverted                 thread/name/updated
thread/goal/updated             thread/goal/cleared
thread/queue/changed            thread/settings/updated
thread/tokenUsage/updated       thread/compacted
thread/environment/connected   thread/environment/disconnected
thread/project/updated          turn/started
turn/completed                  turn/diff/updated
turn/plan/updated               turn/moderationMetadata
hook/started                    hook/completed
item/started                    item/completed
item/agentMessage/delta         item/plan/delta
item/reasoning/summaryTextDelta item/reasoning/summaryPartAdded
item/reasoning/textDelta        item/commandExecution/outputDelta
item/commandExecution/terminalInteraction
item/fileChange/outputDelta     item/fileChange/patchUpdated
item/mcpToolCall/progress       item/autoApprovalReview/started
item/autoApprovalReview/completed
autoApprovalReview/strictReviewRequired
command/exec/outputDelta        process/outputDelta
process/exited                  serverRequest/resolved
rawResponseItem/completed       rawResponse/completed
mcpServer/startupStatus/updated mcpServer/oauthLogin/completed
mcpServer/event/stream/notification
account/updated                 account/rateLimits/updated
app/list/updated                skills/changed
project/changed                 remoteControl/status/changed
externalAgentConfig/import/progress
externalAgentConfig/import/completed
fs/changed                      fuzzyFileSearch/sessionUpdated
fuzzyFileSearch/sessionCompleted
model/rerouted                  model/verification
model/safetyBuffering/updated   guardianWarning
configWarning                   deprecationNotice
windows/worldWritableWarning    windowsSandbox/setupCompleted
thread/realtime/started         thread/realtime/itemAdded
thread/realtime/transcript/delta
thread/realtime/transcript/done
thread/realtime/outputAudio/delta
thread/realtime/sdp              thread/realtime/error
thread/realtime/closed
```

The server-initiated request surface includes:

```text
item/commandExecution/requestApproval
item/fileChange/requestApproval
item/tool/requestUserInput
item/permissions/requestApproval
item/tool/call
mcpServer/elicitation/request
account/chatgptAuthTokens/refresh
attestation/generate
currentTime/read
applyPatchApproval                (legacy)
execCommandApproval               (legacy)
```

Approvals cover command execution, file changes, network access, additional
filesystem permissions, MCP tools, guardian review, user input, and dynamic
tools. Decisions support one-call, session, policy amendment, decline, cancel,
and typed error outcomes. Pending requests are scoped to connection, thread,
turn, and item and are resolved on accept, decline, cancellation, interruption,
disconnect, or turn failure.

### Execution, files, processes, and search

The harness execution layer includes the unified command executor, shell
snapshots, PTY support, output head/tail buffering, process handles, stdin,
resize, terminate, kill, environment selection, command canonicalization,
parsed-command policy, executable identity, network policy, approval requests,
background terminal lifecycle, and platform-specific sandboxing.

Client methods:

```text
command/exec                   command/exec/write
command/exec/terminate         command/exec/resize
process/spawn                  process/writeStdin
process/kill                   process/resizePty
fs/readFile                    fs/writeFile
fs/createDirectory             fs/getMetadata
fs/readDirectory               fs/remove
fs/copy                        fs/watch
fs/unwatch                     fuzzyFileSearch
fuzzyFileSearch/sessionStart   fuzzyFileSearch/sessionUpdate
fuzzyFileSearch/sessionStop    gitDiffToRemote
thread/shellCommand
```

Filesystem calls are constrained by the same workspace roots and permission
profiles used by agent tools. Watches, process IDs, and command sessions cannot
cross connections or threads. Fuzzy search supports cancellation and live
session notifications.

### Tools, MCP, apps, skills, plugins, and hooks

The existing tool registry is extended with the Codex-style tool lifecycle,
tool namespaces, deferred loading, dynamic client tools, MCP tool exposure,
MCP resources, MCP OAuth, MCP event streams, MCP progress, elicitation, app
connectors, app widgets/resources, skill discovery and extra roots, plugin
installation and sharing, marketplace configuration, and lifecycle hooks.

Client methods:

```text
mcpServer/oauth/login           config/mcpServer/reload
mcpServerStatus/list            mcpServer/resource/read
mcpServer/event/stream/start    mcpServer/event/stream/stop
mcpServer/tool/call             app/list
app/read                        app/installed
skills/list                     skills/extraRoots/set
skills/config/write             hooks/list
plugin/list                     plugin/search
plugin/installed                plugin/read
plugin/skill/read               plugin/install
plugin/uninstall                plugin/share/save
plugin/share/updateTargets      plugin/share/list
plugin/share/checkout           plugin/share/delete
marketplace/add                 marketplace/remove
marketplace/upgrade
```

Client-declared dynamic tools are exposed in `thread/start`, invoked through
`item/tool/call`, and represented as public items. MCP and app operations use
the existing OHC MCP clients and marketplace abstractions; no second tool
transport is introduced.

### Configuration, models, accounts, and environments

Configuration behavior includes layered config, managed requirements, runtime
reload, atomic single and batch edits, readonly managed keys, permission
profiles, approval policy, sandbox policy, network policy, web search mode,
personality, model defaults, reasoning effort, service tier, hooks, plugin and
skill roots, feature flags, diagnostics, and config warnings.

Client methods:

```text
config/read                     config/value/write
config/batchWrite               configRequirements/read
experimentalFeature/list        experimentalFeature/enablement/set
permissionProfile/list          model/list
modelProvider/capabilities/read collaborationMode/list
environment/add                 environment/info
environment/status              windowsSandbox/setupStart
windowsSandbox/readiness        server/diagnostics
initialize
```

The account surface includes API-key and configured-provider status, login and
logout lifecycle, ChatGPT-compatible token refresh through an injectable auth
provider, Bedrock discovery/setup, rate limits and reset credit, token usage,
workspace messages, credit nudges, feedback upload, and account notifications.
The OHC provider boundary supplies local/API-compatible implementations and
returns a typed provider-unavailable response for hosted-only operations when
no corresponding service is configured.

Client methods:

```text
account/login/start              account/login/cancel
account/logout                   account/read
account/rateLimits/read          account/rateLimitResetCredit/consume
account/usage/read               account/workspaceMessages/read
account/sendAddCreditsNudgeEmail account/bedrock/discover
account/bedrock/setup            feedback/upload
getAuthStatus
```

### Collaboration, review, realtime, remote control, and migration

The harness includes review requests, collaboration mode presets, multi-agent
spawning and status, parent/child thread relationships, direct-input rules,
remote-control pairing and client revocation, realtime thread transport,
external-agent artifact detection/import/history, and Windows setup lifecycle.

Realtime covers start, stop, text, audio, speech, voice listing, session IDs,
WebRTC/WebSocket transport negotiation, transcript deltas, output audio deltas,
items, SDP, errors, and close events.

Client methods:

```text
review/start                    collaborationMode/list
remoteControl/enable            remoteControl/disable
remoteControl/status/read       remoteControl/pairing/start
remoteControl/pairing/status    remoteControl/client/list
remoteControl/client/revoke     thread/realtime/start
thread/realtime/appendAudio     thread/realtime/appendText
thread/realtime/appendSpeech    thread/realtime/stop
thread/realtime/listVoices      externalAgentConfig/detect
externalAgentConfig/import      externalAgentConfig/import/recordHistory
externalAgentConfig/import/readHistories
```

The experimental `mock/experimentalMethod` endpoint is retained as a protocol
conformance probe and is never enabled as an agent capability in production.

Remote control and realtime are implemented behind OHC service/provider
interfaces so local deployments can use local transports and hosted deployments
can supply their existing coordination service. The protocol and lifecycle do
not depend on OpenAI-hosted infrastructure.

### Legacy compatibility

The v1 compatibility surface remains supported alongside v2:

```text
getConversationSummary           getAuthStatus
gitDiffToRemote                  fuzzyFileSearch
fuzzyFileSearch/sessionStart     fuzzyFileSearch/sessionUpdate
fuzzyFileSearch/sessionStop      applyPatchApproval
execCommandApproval
```

Existing OHC methods, including agent-protocol task methods, marketplace
methods, workflow methods, scalable-agent methods, expert-team methods, Ralph
loop methods, and current HTTP/gRPC entry points, retain their current response
shapes. They use the shared runtime and state but are not forced into the App
Server schema.

## Blended Architecture

### Agent runtime and events

`src/agents/builtin/agent.rs` remains the core loop. It will gain the structured
runtime events and controls needed by App Server items:

- turn and item IDs, parent/child item relationships, and ordered event
  sequencing;
- text, reasoning, plan, command, file-change, MCP, dynamic-tool, hook,
  subagent, diff, usage, cost, warning, and error events;
- cancellation tokens, interruption, steering, queued input, and background
  process handles;
- checkpoint and compaction hooks that preserve public history and runtime
  recovery state independently;
- approval suspension points that await the shared approval broker;
- model routing, retry, token-budget, moderation, safety buffering, and
  provider verification events;
- skill, plugin, app, MCP, review, and collaboration hooks using existing OHC
  registries.

The event enum is the single translation boundary. Existing callback consumers
continue to receive the existing event variants; new structured variants are
additive and are mapped to legacy events where necessary.

### Service and state

`src/agents/builtin/service.rs` owns process-level initialization and shared
state. Add the following state services at its existing ownership boundary:

- a durable thread/history store with an in-memory test implementation;
- a live thread registry with connection subscriptions and loaded-thread list;
- a running-turn registry with cancellation, steering, queued input, and
  process handles;
- an approval broker and server-request router;
- configuration/model/account/provider registries;
- MCP/app/plugin/skill/hook watchers and refresh workers;
- telemetry, diagnostics, cost, and backpressure gauges.

Public history is stored in the configured OHC state database or state directory
with atomic commits and explicit schema versioning. `checkpointer.rs` continues
to store execution recovery and workspace checkpoints and gains thread links,
history cursors, and redaction metadata rather than becoming a second thread
manager.

### Protocol and transport

`src/agents/builtin/json_rpc_server.rs` becomes the shared connection boundary
for HTTP, JSONL, and WebSocket. Its dispatcher owns protocol decoding,
initialization, capability gating, request serialization, response correlation,
notification fan-out, server requests, overload handling, and connection
cleanup. Transport-specific readers and writers only frame messages and never
execute agent work directly.

The existing HTTP endpoint remains unary-compatible for current callers. A
streaming HTTP/WebSocket session and stdio JSONL session share the same
connection state and dispatcher. A bounded `tokio::mpsc` queue is used for every
outbound connection. A full queue disconnects a slow streaming client or returns
the documented retryable overload error before work is started.

### Existing subsystems used as authorities

| Harness concern | Existing OHC authority | Integration work |
| --- | --- | --- |
| Agent loop, plans, subagents | `agent.rs`, `scalable_multi_agent.rs`, `expert_team.rs` | Add structured lifecycle, cancellation, steering, and child-thread projection |
| Models and providers | `provider.rs`, `llm/`, `local_provider.rs`, `auth.rs` | Add model catalog, capabilities, rerouting, verification, and account adapters |
| Tools and file changes | `tools/`, `tool_executor_engine.rs` | Add item lifecycle, dynamic namespaces, output streaming, and approval suspension |
| Shell and process execution | `tools/bash.rs`, `tools/runner.rs`, `server/harness/executor.rs` | Add PTY/process handles, command APIs, output deltas, and background terminals |
| Sandbox and permissions | `sandbox/`, `server/harness/sandbox/`, `tools_gating.rs` | Translate App Server policy into existing evaluator and network proxy |
| MCP and apps | `mcp/`, `server/harness/mcp/`, `tools/mcp_dynamic.rs` | Add resources, OAuth, event streams, elicitation, app catalog, and progress |
| Skills, plugins, marketplace, hooks | `progressive_skills.rs`, `plugins.rs`, `tools/marketplace.rs`, `guardrails/` | Add watchers, install/config surfaces, hook lifecycle, and capability gating |
| Memory and context | `memory/`, `memory_store.rs`, `compaction.rs`, `prompt_construction.rs` | Add memory mode, reset, context projection, compaction events, and token replay |
| Persistence and recovery | `checkpointer.rs`, `json_store.rs`, `sqlite_memory.rs` | Add public thread/turn/item state, pagination, search, rollback, and migrations |
| Service and compatibility | `service.rs`, `agent_protocol.rs`, existing RPC handlers | Preserve all existing methods and route them through shared runtime state |
| Telemetry and operations | `observability.rs`, `server/harness/telemetry/` | Add request gauges, cost worker, diagnostics, warnings, and overload metrics |

## Error, Security, and Capability Rules

- Validate all paths against the workspace and configured permission roots before
  spawning a process or touching a file.
- Reuse the current sandbox manager, network proxy, tenant checks, high-risk
  tool gating, guardrails, and human-in-the-loop policy. App Server approval
  decisions cannot bypass them.
- Scope every mutable operation to its connection, tenant, thread, turn, and
  process/watch/session identifier where applicable.
- Reject duplicate initialization, requests before initialization, malformed
  experimental fields without opt-in, obsolete fields, stale approval IDs, and
  responses to unknown server-request IDs.
- Redact provider credentials, auth tokens, secrets, and sensitive tool output
  from public history, diagnostics, and telemetry.
- On client disconnect, cancel only work owned by that connection unless the
  thread has an explicitly configured durable owner. Resolve pending approvals
  and release process/watch resources deterministically.
- Advertise experimental methods only after opt-in and filter experimental
  notifications per connection. Stable methods remain available without opt-in.
- Use typed JSON-RPC errors for unsupported provider configuration, policy
  denial, stale state, overload, and process lifecycle failures.

## Implementation Phases

This is one parity program with independently testable phases. No feature group
is intentionally excluded; the phases provide dependency order and reviewable
changes.

1. **Protocol contract and transports.** Add the complete request/response/
   notification/server-request model, JSONL framing, HTTP compatibility,
   WebSocket framing, initialization, experimental gating, connection state,
   bounded queues, request IDs, errors, and conformance fixtures.
2. **Thread and history services.** Add durable thread/turn/item records,
   public projections, cursors, pagination, thread lifecycle, sections,
   metadata, goals, queue, search, summaries, rollback, fork, archive, and
   recovery. Add restart and concurrent-reader/writer tests.
3. **Turn integration.** Extend `AgentEvent`, runtime controls, cancellation,
   steering, queued input, compaction, usage, cost, retries, plans, diffs,
   child agents, and all item/turn notifications. Verify event ordering with a
   deterministic fake provider.
4. **Approval and execution parity.** Integrate command/file/network/
   permission/guardian/user-input/MCP/dynamic approvals, unified command and
   process sessions, PTY behavior, background terminals, filesystem APIs,
   watches, fuzzy search, and sandbox policy translation.
5. **Tool ecosystem parity.** Integrate MCP resources, OAuth, progress,
   elicitations and event streams; app connectors; dynamic tools; skills and
   watchers; plugins, marketplace, sharing, and hooks.
6. **Configuration and provider parity.** Integrate layered config, managed
   requirements, config writes/reload, feature enablement, permissions, model
   catalog/capabilities, account/auth flows, Bedrock adapters, rate/usage
   surfaces, attestation, current time, feedback, and diagnostics.
7. **Collaboration, realtime, and operations.** Integrate review, collaboration
   modes, multi-agent thread relationships, realtime audio/text/speech and
   transport events, remote-control pairing, external-agent migration, Windows
   lifecycle, telemetry, cost workers, and graceful shutdown.
8. **Compatibility and rollout.** Run the complete protocol matrix, legacy RPC
   suite, gRPC suite, provider tests, security tests, restart tests, slow-client
   tests, and end-to-end subprocess tests. Enable the streaming control plane by
   default only after all required capability groups pass.

Every phase must leave the existing service buildable and must add a capability
matrix entry describing the implementation, provider dependencies, and test
coverage for each method and notification.

## Testing and Conformance

- Generate or import protocol fixtures from the pinned official schema and
  assert that every client method, server request, notification, and legacy
  method has a dispatcher entry.
- Test JSONL framing, omitted and explicit JSON-RPC version, malformed input,
  duplicate initialization, experimental opt-in, notification opt-out,
  overload, slow-client disconnect, request serialization, and cleanup.
- Test the full thread state machine, pagination boundaries, fork/rollback,
  public-history redaction, atomic persistence, restart recovery, and stale
  writer handling.
- Test every turn/item event mapping, delta aggregation, ordering, interruption,
  steering, queued input, compaction, token usage, cost, failure, and retry.
- Test every approval class, decision variant, timeout/cancel/disconnect cleanup,
  connection/thread scoping, policy denial, and guardian escalation.
- Test command, process, filesystem, watch, search, sandbox, MCP, skills,
  plugin, app, hook, config, auth, model, review, realtime, remote-control,
  migration, and Windows-capability paths with fakes where external services
  are required.
- Preserve and run all existing unit, integration, Bazel, gRPC, HTTP, provider,
  memory, tool, sandbox, and workflow tests.
- Add one subprocess integration test for initialize -> thread/start ->
  turn/start -> streamed item events -> server approval request -> client
  response -> turn/completed, then restart and resume the same thread.

## Rollout and Drift Management

The implementation records the reviewed source revision and protocol inventory in
the capability matrix. A later Codex source update is handled by re-running the
inventory, adding schema fixtures first, then implementing changed behavior
behind an explicit compatibility version. Stable and experimental surfaces are
reported separately, but both are maintained by this project.

The legacy unary paths remain enabled throughout development. A compatibility
mode flag can route only selected clients to the streaming dispatcher while the
existing entry points continue to use the same agent and state services.

## Risks and Mitigations

- **Large surface area:** implement by dependency-ordered phases with a machine-
  checked capability matrix and no undocumented partial behavior.
- **Protocol drift:** pin the source revision, retain generated fixtures, and
  fail CI when a method or notification lacks a mapping.
- **State divergence:** make the runtime event boundary the only writer of
  public items and checkpoint links, with transactional persistence.
- **Approval deadlocks:** use cancellation-aware broker entries and test every
  terminal path, including disconnect.
- **Security regression:** translate into existing policy authorities and add
  cross-tenant, path traversal, stale-ID, and provider-secret tests.
- **External-service differences:** isolate hosted functionality behind OHC
  provider traits and return explicit configuration errors when a deployment
  lacks the required service.
- **Operational pressure:** bound queues, serialize shared mutations, expose
  diagnostics, and make shutdown drain active work predictably.

## Review Gate

This expanded design supersedes the earlier stable-core-only proposal. After
written approval of this document, create the detailed implementation plan with
exact files, tests, and phase checkpoints. Production code changes begin only
after that plan is reviewed.
