# OmniSolo Harness Middleware and Portable Session Design

**Status:** Approved architecture; implementation has not started.

**Supersedes:** `2026-08-21-codex-app-server-compatibility-design.md`.

## Objective

Build a harness-neutral OmniSolo control plane that can run one durable session
and its tasks through different agent harnesses without making any external
harness the product's persistence contract.

The initial adapter set targets Codex, OpenCode, DeepSeek Harness, Pi, Kimi,
OpenHands, AgentBoardTT OpenHarness, and the existing OmniSolo harness. ACP is a
supported adapter protocol, not the canonical data model. A2A remains available
for remote agent delegation, and MCP remains the standard tool/resource layer.

Users may choose a harness for a session or task. OmniSolo supports both:

1. Exact resume in the same harness when its native session state remains valid.
2. Cross-harness handoff into a newly created native session using portable,
   observable state.

Harness workers and the gateway must deploy and scale independently in Docker
and Kubernetes. The existing OmniSolo harness remains a first-class adapter so
it can continue evolving without privileged access to the control plane.

## Source Baseline

The design was reconciled on 2026-08-22 against these pinned sources:

| System | Revision or contract |
| --- | --- |
| OpenAI Codex | `4f39251a010a8bd7d692d25fb33832ff06f1635a` |
| OpenCode | `e00890c67261a435cee6409366a68999a93393fd` |
| OpenHands SDK/Agent Server | `ddac55697c5d15cf8a34495b5ed6d46c86db092a` |
| Pi | `c49906ec77788625aacbdc53ebca6fbe65bd20f5` |
| Kimi Code | `d4e0ad4b2d04d676b6d139ee320ea162289d3f4b` |
| Kimi CLI | `cbc15c076d17f70fec9f89c90c0502e68657f505` |
| DeepSeek Harness | `b150a551b8d465e31e418e1b2eaf5e79bbb7d28e` |
| AgentBoardTT OpenHarness | `85c54682a209ca7c3fc8b1ab2e820b6724dc3028` |
| ACP | stable protocol v1; v2 alpha reviewed only for forward planning |
| A2A | wire v1.0 |
| MCP | stable revision 2026-07-28 |

Generated schemas and pinned source are authoritative where documentation and
implementation differ. Adapter compatibility tests must pin exact versions.

## Architectural Decisions

1. **OmniSolo owns durable truth.** A normalized append-only event log is the
   source of truth. Session, task, message, status, usage, and search tables are
   rebuildable projections.
2. **Native state is retained, not flattened.** Each adapter may store
   versioned raw records and native cursors beside the portable log. This makes
   exact same-harness resume and lossless export possible.
3. **Task, turn, and attempt are distinct.** A task is the durable objective, a
   turn is one user-to-agent interaction cycle, and an attempt is one concrete
   worker execution or retry.
4. **Statuses remain orthogonal.** Session status, task state, turn stop reason,
   attempt state, tool state, interaction state, and process state are never
   collapsed into one enum.
5. **Capabilities are discovered.** Harness, model, tool, media, approval,
   workspace, and resume capabilities are versioned snapshots, not assumptions
   derived from a harness name.
6. **Security authority is never transferred.** Approval history is portable;
   approval grants, credentials, live sandbox state, and process handles are not.
7. **Observable state transfers.** User-visible reasoning summaries may enter
   the portable capsule. Raw chain-of-thought, encrypted reasoning, and hidden
   provider state remain native-only.
8. **Unknown required events fail closed.** Every event declares whether it is
   required for replay. An adapter must refuse exact reconstruction when it
   encounters an unknown required event.
9. **Patch semantics are explicit.** Patchable values distinguish absent, null,
   and concrete value. JSON object merging is not used as an implicit patch
   protocol.
10. **Branding is OmniSolo.** New public types, packages, services, configuration,
    and documentation use OmniSolo naming. Historical storage identifiers are
    changed only through explicit migrations.

## System Architecture

```text
Clients
   |
OmniSolo API / Gateway
   |-- authentication and tenant isolation
   |-- session and task commands
   |-- routing and scheduling
   |-- approvals and user interactions
   |-- canonical event ingestion
   |
Harness Control Plane
   |-- harness and model-runtime registry
   |-- capability negotiation
   |-- worker leases and attempt routing
   |-- exact-resume and handoff compiler
   |
   +-- OmniSolo harness workers
   +-- Codex workers
   +-- OpenCode workers
   +-- Kimi/Pi/OpenHands/other workers
   |
Durable State
   |-- PostgreSQL canonical log and projections
   |-- object storage for artifacts, snapshots, and native records
   |-- secret manager references
   +-- optional queue/cache for delivery and leases
```

The gateway and control plane do not execute model loops. A worker owns one
attempt lease and embeds or launches its harness adapter. Workers may be
stateless when the native harness supports external persistence, or stateful
behind a stable service when exact resume requires local native state.

## Identity Hierarchy

```text
Tenant
  Project / Workspace
    Session
      SessionBinding[]
      Task[]
        Turn[]
        Attempt[] (optionally linked to a Turn)
      Event[]
      Artifact[]
      WorkspaceSnapshot[]
      NativeRecord[]
```

- A `Session` is the user-visible durable conversation and shared context.
- A `SessionBinding` links a session or scoped task to one native harness session.
- A `Task` represents an objective and may survive harness changes.
- A `Turn` belongs to exactly one task, starts with admitted input, and ends
  with a terminal stop reason. A turn can span multiple attempts during retry,
  worker recovery, or harness handoff.
- An `Attempt` belongs to exactly one task and optionally one turn. Interactive
  attempts reference a turn; task-level attempts cover detached commands,
  monitors, and other background work. It binds work to one worker, harness,
  model runtime, and effective policy. Retries always receive new attempt IDs.
- Background tasks and subagents are tasks with typed parent relationships, not
  unstructured tool output.

Worker commands always require task and attempt IDs. Turn ID is required only
for turn-bound work. A handoff terminates or fences the source attempt while the
turn remains non-terminal; the target attempt may continue that turn.

## Actors and Principals

`ActorDescriptor` gives messages, decisions, commands, and events a stable
author independent from a harness-native role string. It records actor ID,
tenant, kind (`user`, `agent`, `subagent`, `service`, `harness`, or `system`),
display identity, authenticated-principal reference, delegation parent, agent
profile/version, and native aliases. Authentication claims and credentials are
not embedded.

An `ActorBinding` records which actor a harness-native author or subagent ID
represented during a binding generation. Importers must not merge actors based
only on display name.

Cross-harness capsules contain a stripped `HistoricalActor` projection with only
capsule-local actor ID, non-authoritative historical role, display label and
delegation lineage. Authenticated-principal references, native aliases, service
identity and authorization are excluded. Imported transcript content always
enters a delimited historical-data channel. It cannot become system, developer,
tool, or policy authority based on its actor role or content. Only allowlisted
instruction layers in the sanitized `RuntimeConfigSnapshot`, accepted by target
policy, may populate privileged prompt channels.

## Lifecycle State Families

Canonical states are separate, extensible families with timestamps and reason
codes. Unknown native states are preserved in extensions and map to `unknown`,
not a guessed canonical value.

| Record | Canonical states |
| --- | --- |
| Session | open, handing_off, archived, deleting, deleted, error, unknown |
| Task | queued, running, handing_off, waiting_input, paused, completed, failed, cancelled, lost, unknown |
| Turn | admitted, queued, running, waiting_input, completed, interrupted, failed, cancelled, unknown |
| Attempt | pending, leased, starting, running, quiescing, checkpointing, succeeded, failed, cancelled, lost, fenced, unknown |
| Tool call | pending, running, completed, failed, cancelled, unknown |
| Interaction | pending, accepted, declined, cancelled, expired, superseded, error, unknown |
| Process | pending, running, exited, failed, killed, timed_out, lost, unknown |

Terminal status, stop reason, transport error, and retry decision remain
separate fields. For example, `Turn.completed` may still have a model stop reason
of `max_tokens`; a retry does not rewrite the preceding attempt's terminal state.

Every mutable lifecycle projection carries `state_version` and is updated with
compare-and-swap against the event being applied. Legal transitions are defined
per family in generated schema fixtures. Core invariants are:

- attempt, turn, tool, interaction, process, binding and lease terminal states
  never reopen; a retry or recovery creates a new attempt or operation;
- a turn remains non-terminal across failed/recovered attempts and becomes
  terminal only through one explicit terminal event;
- a task derives waiting/running projections from its current attempts and
  interactions, but reaches terminal state only through one explicit event
  after required children settle;
- session `archived` may return to `open`; `handing_off` returns to `open` or
  advances to error/archived according to its operation outcome;
- stale and out-of-order transitions fail unless they are exact idempotent
  duplicates; native status updates never bypass canonical transition checks.

Bindings transition `creating -> inactive -> active -> quiescing -> fenced ->
closed`, with `error` reachable from non-terminal states. Leases transition
`offered -> active -> released | expired | revoked`; a lease generation never
reopens or decreases.

## Portable Session Manifest

`SessionManifest` contains:

| Field group | Required data |
| --- | --- |
| Schema | capsule version, minimum reader version, producer, created time |
| Identity | tenant, project, workspace, session IDs and external aliases |
| Display | title, labels, tags, archive state |
| Lifecycle | created/updated/last-active times and normalized session state |
| Lineage | parent, fork source, handoff source, root session, operation IDs |
| Pointers | active task, turn, event watermark, workspace snapshot, binding |
| Policy | retention class, data classification, export restrictions |
| Integrity | manifest digest, event range digests, referenced object digests |

Session status is a projection. It must not claim that a native process is still
running after its worker lease expires.

## Portable Session Capsule

`SessionCapsule` is the normative cross-harness transfer object. It is not a
database dump and is generated at one durable event boundary. Its manifest
contains:

- capsule ID, schema version, minimum reader version, compiler version and time;
- tenant/session identity, source binding, target harness intent and handoff ID;
- inclusive durable event range and branch/lineage selection;
- session, task, turn, actor and message projection references at that range;
- effective runtime configuration, model intent and capability requirements;
- workspace snapshot and artifact manifests with content digests;
- portable record batches, their ordering and integrity digests;
- redaction policy ID/version and transformations applied;
- structured compatibility and loss report;
- signature/encryption metadata and retention policy.

The transfer schema uses a versioned allowlist. Permitted record families are
session/task/turn projections, stripped historical actors, settled messages and content,
settled tool history, interactions and outcomes, plans/todos/goals, completed
process history, runtime configuration after secret removal, usage/errors,
workspace snapshots, artifacts, portable context checkpoints and compaction
lineage.

Adapter-native extensions, `NativeRecord` payloads or references, credentials,
live grants, and runtime handles are excluded from cross-harness capsules by
default. A future extension can become portable only through a reviewed capsule
schema revision and explicit redaction rules. Unknown fields and unknown record
kinds fail capsule generation rather than passing through.

`PortableContextCheckpoint` contains only a summary, selected visible messages,
source durable-event ranges, retained ancestor boundary, compaction provenance,
usage and integrity digest. It contains no binding, native cursor, native-record
reference, worker, attempt, lease, pending runtime object or reusable authority.
`ResumeCheckpoint` is native recovery state and is never included in a
cross-harness capsule.

`LossReport` contains one entry per dropped, transformed, summarized, rendered,
or unsupported source field. Each entry records the canonical source path,
reason, severity, target representation, whether user acknowledgement is
required, and any capability that would avoid the loss. Capsule generation is
blocked for loss classified as unsafe or required-for-replay.

## Harness Descriptor and Binding

`HarnessDescriptor` identifies an adapter independently from model providers:

- stable harness and adapter IDs;
- display name, implementation version, protocol kind and protocol version;
- build/source revision and schema revision;
- supported lifecycle operations;
- supported content and media types;
- tool, terminal, workspace, approval, question, planning, subagent, compaction,
  branching, streaming, replay, and exact-resume capabilities;
- persistence expectations and native state locality;
- required worker image and platform constraints;
- native extension namespace and migration support.

`SessionBinding` contains:

- OmniSolo session ID and harness descriptor ID;
- binding scope (`session` or `task`) and owning session/task ID;
- workspace mutation scope ID and access mode (`read_only` or `read_write`);
- native session/thread/conversation ID;
- binding state, active generation and creation/last-use times;
- capability snapshot ID and effective adapter configuration digest;
- last imported native cursor and last exported OmniSolo sequence;
- native checkpoint or resume-token reference;
- worker-pool and state-locality hints, never an immortal pod identity;
- exact-resume eligibility and invalidation reason;
- native record set and integrity digest.

A handoff creates a new binding. It never mutates one harness's native ID into
another harness's ID.

The database enforces at most one active read-write binding generation for the
same binding owner and workspace mutation scope. Session-scoped bindings own the
default shared conversation/workspace. A task-scoped binding receives the
portable session projection as historical context but may write only task-owned
events and an isolated workspace scope. Multiple read-only task bindings may be
active. A task that needs the shared writable workspace must use the active
session binding or trigger a session-scoped handoff.

## Canonical Event Log

Every durable mutation is represented by `EventEnvelope`:

```text
event_id
tenant_id, session_id
task_id?, turn_id?, source_attempt_id?
ingest_attempt_id?, actor_id?, worker_id?, harness_id?
binding_id?, binding_generation?
lease_id?, lease_generation?, fencing_token?
durable_sequence?
delivery_stream_id?, delivery_sequence?
aggregate_id, aggregate_sequence?
branch_id?, parent_event_ids[]
event_type, payload_schema, payload_version
occurred_at, ingested_at
correlation_id?, causation_id?, idempotency_key?
durability: durable | transient
replay_requirement: required | ignorable
visibility and data-classification labels
native provenance and source event identifiers
payload
extensions
```

`durable_sequence` is allocated transactionally only for durable records and is
the canonical replay, integrity, and handoff cursor. `delivery_sequence` orders
connection-scoped transient and durable emissions and may contain gaps after
reconnect. Adapter stream offsets are recorded as native provenance and cannot
replace either cursor. Final authoritative records are always durable.

Every worker-originated mutation, including events, checkpoints, native records,
artifacts, and workspace updates, carries lease generation and fencing token.
Storage compares them transactionally with the current attempt lease. A stale or
partitioned worker is rejected even when its payload has a new event ID. Native
provenance and idempotency keys independently deduplicate redelivery from the
valid lease holder.

`source_attempt_id` is semantic provenance and may identify a predecessor whose
native events are being reconciled after recovery. `ingest_attempt_id` and its
lease fields identify the currently authorized writer. Storage validates only
ingest authority while preserving source attribution. An adapter must not relabel
old native work as produced by the recovery attempt.

An event marked required-for-replay must be durable. Projection rows are not
events and consume no event sequence; they retain the durable sequence through
which they were built. Object uploads are staged under the attempt lease and
become visible only when fenced metadata publication succeeds, so stale uploads
can be garbage-collected but never enter a capsule or workspace snapshot.

Events are immutable. Corrections append replacement or invalidation events.
Compaction changes a model-facing projection; it does not delete source events.

Each event parent must exist in the same session at a lower durable sequence.
The default branch is linear with one parent; fork events create a new branch
from an existing event, and explicit merge events may reference multiple branch
heads. Branch-head updates use expected-head compare-and-swap. Cycles,
cross-session parents and missing parents are rejected transactionally. A capsule
contains the full ancestor closure for selected events, or a portable context
checkpoint that explicitly replaces the omitted prefix.

## Messages and Content

`Message` contains stable ID, role, author, origin, phase, parent/correlation
links, task/turn ownership, timestamps, status, content-part order, visibility,
redaction state, and native provenance.

`ContentPart` is a tagged union supporting:

- text and user-visible reasoning summary;
- image, audio, video, and generic file;
- resource link and embedded resource;
- structured JSON with schema reference;
- tool call and tool result references;
- artifact, workspace path, symbol, range, and external task references;
- citation and annotation data;
- adapter-native content in a namespaced extension.

Large or binary data is stored once in content-addressed object storage. Content
parts hold digest, size, media type, logical name, and authorized locator.
Absolute host paths and expiring remote URLs are not canonical identities.

Streaming chunks record stream ID and chunk sequence when replay is required.
The settled content part remains authoritative because several harnesses do not
guarantee concatenated deltas equal the final item.

## Tools and Effects

The tool model contains four records:

1. `ToolDefinitionSnapshot`: qualified name, description, source, input/output
   schemas, annotations, version and digest.
2. `ToolCall`: stable ID, definition snapshot, raw input, parsed input, status,
   timing, parent task/turn/attempt, retry and native provenance.
3. `ToolProgress`: ordered typed progress, terminal chunks, locations, percent,
   artifact references, and adapter-native display data.
4. `ToolResult`: typed content, structured output, error, timing, artifacts,
   observed effects and provider execution metadata.

Tool source is explicit: built-in, MCP, skill, plugin, client dynamic tool,
harness-native tool, or remote agent. An observed filesystem or network effect
is historical evidence, not reusable permission.

## Interactions and Policy

`Interaction` unifies approval, question, elicitation, authentication challenge,
and additional-input requests without reducing them to a Boolean.

It records request ID, subject, action, description, risk, schema/options,
requested scope, expiry, status, response, response actor, timestamps, policy
evaluation, and native provenance. Approval outcomes include accepted, declined,
cancelled, expired, superseded, and error.

Portable history records what was requested and decided. Session grants,
remembered rules, credentials, and policy amendments are re-evaluated at the
target harness and environment. Handoff invalidates all live authorization.

## Plans, Todos, Goals, and Tasks

- `Plan` is a versioned proposed execution structure with ordered steps.
- `Todo` is an actionable tracked item and may be represented by snapshots or
  entity updates depending on source fidelity.
- `Goal` holds objective, completion criterion, lifecycle, token/cost/turn/time
  budgets and accumulated usage.
- `Task` owns execution lifecycle, dependencies, parent/subagent relationships,
  accepted inputs, outputs, artifacts and terminal result.

The handoff compiler may render these records into target-specific prompt text
when the target lacks native support, while retaining their canonical types.

## Processes, Terminals, and Background Work

Portable completed process history includes command/argv, logical cwd, sanitized
environment keys, ordered stdout/stderr chunks, timing, exit code, signal,
timeout/OOM state, artifacts and associated tool call.

PIDs, PTYs, process handles, sockets, in-memory buffers, containers, terminal
dimensions, and stdin ownership are attempt-local runtime state. On worker loss,
their projections become `lost` unless the native harness proves reconnection.

Detached commands, background work, and subagents are durable tasks with worker
leases. Their logs and outputs can continue after a foreground turn finishes.

## Workspace and Artifacts

`WorkspaceDescriptor` identifies logical roots, mount policy, OS/architecture,
shell, toolchain hints, environment allowlist, repository identity and data
locality. It does not embed credentials or assume a host path is portable.

`WorkspaceSnapshot` may include:

- content-addressed tree or archive;
- Git repository identity, base commit, branch and remote;
- working-tree patch, index patch and untracked-file manifest;
- submodule state and file metadata;
- snapshot creator, reason, completeness and integrity;
- parent snapshot and event boundary.

Conversation rollback, compaction, or revert does not imply filesystem rollback.
Workspace restoration is a separate explicit operation and policy decision.

`Artifact` records digest, size, media type, logical name, kind, provenance,
producer event, retention and authorized storage reference.

## Effective Runtime Configuration

Every attempt references an immutable `RuntimeConfigSnapshot` containing:

- system, developer and user instruction layers with provenance;
- agent/personality/collaboration settings;
- requested model intent and response schema;
- reasoning, sampling, context and output limits;
- active tools and their exact definition snapshots;
- MCP, skill, plugin and hook descriptors;
- abstract sandbox, filesystem, network and approval policy;
- compaction, retry, budget and telemetry settings;
- environment and workspace snapshot references.

The effective snapshot is recorded after defaults and policy have been applied.
This prevents resume behavior from silently changing when global configuration
or adapter defaults change.

## Model Runtime and Open-Source Hosting Extension Points

Harness selection and model execution are separate decisions. A harness may use
a managed API today and a self-hosted open-source runtime later without changing
the session schema or adapter contract.

`ModelDescriptor` reserves:

- stable model ID, family, revision and optional weight/tokenizer digests;
- modalities, tool/reasoning/structured-output capabilities;
- context/output limits and quantization compatibility;
- license, provenance and policy labels;
- pricing or internal resource-accounting metadata.

`ModelRuntimeDescriptor` reserves:

- runtime kind: managed API, OpenAI-compatible endpoint, local process,
  Kubernetes service, remote cluster, or custom;
- serving engine: vLLM, TGI, SGLang, Ollama, llama.cpp, or custom;
- API dialect/version and capability discovery endpoint;
- endpoint and credential references, never secret values;
- supported model revisions and adapter/LoRA sets;
- resource profile: accelerator family/count, memory, quantization, tensor and
  pipeline parallelism;
- placement, region, tenancy, data-locality and network policy;
- batching, context-cache and speculative-decoding capabilities;
- readiness, health, draining and autoscaling metadata;
- minimum/maximum replicas and scale-to-zero support.

`ModelBinding` is attempt-specific and records the selected descriptor/runtime,
realized capabilities, serving revision, routing reason and usage-accounting
source. Portable sessions preserve requested model intent and historical binding,
but a handoff may bind to another compatible runtime.

No serving engine is required for the first implementation. These fields and
interfaces are placeholders with forward-compatible unknown variants. Initial
managed and OpenAI-compatible providers implement the same boundary that future
self-hosted inference workers will implement.

### Model runtime protocol placeholder

Harness adapters receive a `ModelBinding` and a stable OmniSolo inference
endpoint, not a serving-engine-specific address. Managed providers and local
OpenAI-compatible endpoints can initially be routed directly behind that
boundary. Future inference workers implement a separate, versioned runtime
protocol supporting:

- register, heartbeat, capability snapshot, model load/unload and drain;
- capacity advertisement and fenced capacity leases;
- inference admission with request idempotency and model-binding revision;
- text/content streaming, tool-call blocks and authoritative final response;
- cancellation, deadlines, backpressure and overload responses;
- usage, cache, latency, cost/resource and finish-reason reconciliation;
- health, readiness, warm/cold state and autoscaling pressure;
- failover classification distinguishing safe pre-admission retry from
  uncertain post-admission execution.

The inference gateway maps the stable endpoint to this protocol. Harness
adapters therefore do not change when vLLM, TGI, SGLang, Ollama, llama.cpp, or a
custom cluster is introduced. Capability snapshots prevent a runtime from being
selected for unsupported media, tools, context size, or structured output.

Inference lifecycle is durable: `queued -> capacity_leased -> admitted ->
streaming -> completed | failed | cancelled | uncertain`. Admission atomically
binds the request ID, model binding, runtime worker, capacity-lease generation
and request digest. Runtime emissions are accepted only from that fenced
capacity lease. Authoritative final response and usage are committed before the
lease is released. On gateway restart, pre-admission work may be safely routed
again; post-admission work without a terminal record remains `uncertain` unless
the runtime proves idempotent replay. A stale capacity holder cannot publish
stream or final records after reassignment.

## Native Records

`NativeRecord` contains harness ID, adapter version, native schema/version,
record kind, native identity/cursor/ordinal, capture time, payload digest,
integrity metadata, encryption/classification and object-storage reference.

Examples include Codex rollout lines, Pi JSONL entries, OpenCode durable events,
Kimi protocol records, OpenHands typed events, DeepSeek epochs, and ACP unknown
extension payloads.

Native records are not interpreted as portable authority. They support exact
resume, diagnostics, migration, lossless re-export, and future adapter upgrades.

## Transfer Semantics

### Exact same-harness resume

`ResumeCheckpoint` is committed only at an adapter-declared safe point and
contains one consistent tuple:

```text
checkpoint_id
binding_id, binding_generation
task_id, turn_id?, source_attempt_id
created_by_attempt_id, created_by_lease_generation
native_cursor and native-record-set digest
canonical durable_sequence
runtime-config digest and model binding
workspace-snapshot digest and effect watermark
command-inbox watermark and command-outbox watermark
pending interaction/input references
created_at and integrity digest
```

Prompt admission and control commands first enter a durable, idempotent command
inbox. Worker acknowledgements, terminal outcomes, and externally visible effect
records enter a durable outbox before the command is considered complete. A
checkpoint cannot advance beyond an admitted command without recording its
outcome or explicitly marking it unresolved.

No general harness can guarantee exactly-once external effects. After a crash,
an unresolved non-idempotent command or tool effect is marked `uncertain` and is
reconciled or sent for policy/user resolution; it is never automatically rerun.
The phrase exact resume therefore means exact reconstruction of recorded harness
state at a safe point, not exactly-once execution of arbitrary side effects.

Exact resume is allowed only when:

- the binding and native checkpoint are intact;
- harness, adapter and native schema versions are compatible;
- required capabilities remain available;
- workspace and model-runtime preconditions are satisfied;
- current credentials and policies authorize reconstruction;
- the checkpoint tuple and command watermarks pass integrity validation;
- no unknown required native event is encountered.

The worker resumes native state, then reconciles native events after the stored
cursor into the OmniSolo event log. Recovery runs as a new attempt with a new
lease. Reconciled records retain `source_attempt_id` from the checkpoint/native
record while the new attempt and lease provide `ingest_attempt_id` authority.

### Cross-harness handoff

`HandoffOperation` is a durable, idempotent state machine:

```text
requested -> fencing -> quiescing -> snapshotting -> compiling
          -> awaiting_loss_ack -> target_creating -> activating -> completed
          -> failed | cancelled
```

Session handoff is the default because tasks commonly share a workspace and
conversation. Task-scoped handoff is allowed only when the task has an isolated
workspace and no shared session writers.

`HandoffOperation` records scope kind/owner, workspace mutation scope, source and
target binding generations, fencing epoch, phase version, cut tuple, capsule and
loss-report digests, acknowledgement interaction, target creation key and final
result. Session-scoped operations project the session as `handing_off`;
task-scoped operations project only the owning task as `handing_off`.

The handoff coordinator:

1. Atomically projects the selected session or task owner as `handing_off`,
   increments its scope fencing epoch, stops new assignment in that scope, and
   durably queues or rejects affected user input.
2. Fences every source lease allowed to mutate the selected session/workspace,
   requests quiescence, and waits for acknowledgement or lease expiry.
3. Resolves, cancels, or durably carries pending interactions and input according
   to policy. No live authorization crosses the boundary.
4. After all writers are fenced, creates the workspace snapshot and atomically
   records the durable event watermark, workspace revision, source binding
   generation and checkpoint tuple as the handoff cut.
5. Builds a portable capsule from the allowlisted canonical records and objects.
6. Discovers target harness and model-runtime capabilities.
7. Produces the loss report. Unsafe degradation fails the operation. Loss that
   requires acknowledgement enters `awaiting_loss_ack` and creates one durable
   interaction keyed by handoff ID, report version/digest, actor and expiry. A
   changed report invalidates the previous acknowledgement.
8. Re-evaluates policy and requests fresh credentials through references.
9. Creates a new native target session and `SessionBinding` without activating it.
10. Imports typed state where supported and renders a deterministic context
   package for unsupported semantic records.
11. In one control-plane transaction, activates the target binding, releases
    queued input, updates canonical pointers, and records source/target linkage,
    compiler version and import report.

Recovery resumes the state machine from its last committed phase. Target
creation uses the handoff ID as an idempotency key. A failed operation either
reactivates an intact source binding before target activation or remains paused
for operator resolution; it never permits both source and target writers.

Active work is not migrated. Source attempts are completed, interrupted, lost,
or fenced before the cut. A new target attempt may continue a still-open turn.

## Non-Transferable State

The portable capsule excludes:

- API keys, access/refresh tokens, cookies and secret values;
- live approvals, remembered grants and sandbox enforcement state;
- raw chain-of-thought and provider-encrypted reasoning;
- PIDs, PTYs, process/container handles, sockets and subscriptions;
- callbacks, closures, promises, iterators, locks, timers and retry tasks;
- loaded plugins, MCP clients, model clients and executable tool handlers;
- telemetry spans, caches and in-memory projections;
- unsnapshotted filesystem, Git index, worktree or side effects;
- absolute paths or external URLs without bundled authorized content.

Secret references may transfer only as identifiers that the destination must
resolve and authorize independently.

## Worker Protocol

The private worker protocol is versioned bidirectional gRPC/protobuf. It must
support:

- register/heartbeat/drain and capability snapshots;
- create/import/resume/close/delete native sessions;
- start/steer/cancel/wait attempts;
- ordered event streaming with acknowledgement and replay cursor;
- approval, question, elicitation and credential-reference exchanges;
- workspace/artifact upload and download by digest;
- checkpoint/export of native records;
- health/readiness and resource pressure reporting.

The protocol uses three envelopes rather than impossible placeholder IDs:

1. `WorkerControlEnvelope` carries worker/pool/adapter identity, protocol and
   capability versions, correlation and idempotency IDs. It is used for
   registration, heartbeat, readiness and drain and has no session identity.
2. `SessionOperationEnvelope` carries tenant/session, operation ID and generation,
   binding scope/owner/generation, workspace scope, harness/worker identity,
   correlation and idempotency IDs, plus an operation fencing token. It covers
   native session create/import/resume/close/delete before an attempt exists.
3. `AttemptCommandEnvelope` carries tenant/session/task/attempt, optional turn,
   binding scope/generation, harness/worker, capability version, lease
   ID/generation/fencing token, correlation and idempotency IDs.

Session-operation fencing prevents duplicate or stale handoff workers from
creating or activating competing native sessions. Attempt fencing governs all
attempt-originated mutations. Worker delivery is at least once; canonical
mutation is idempotent.

ACP and native harness protocols terminate inside adapter workers. They are not
exposed as the internal control-plane contract.

## Kubernetes and Docker Deployment

- Gateway/control-plane pods are stateless aside from durable stores.
- Each harness has an independently scalable deployment or worker pool.
- Model inference workers are a separate optional pool from harness workers.
- Attempts use renewable leases with fencing tokens; expired workers cannot
  continue writing canonical events.
- Stateful native resume uses a persistent volume, external native store, or
  stable harness service selected by the adapter.
- Workspaces use object storage, persistent volumes, remote workspaces, or
  disposable materialization from snapshots.
- Pod identity is never stored as durable session identity.
- Readiness distinguishes accepting new attempts from serving existing leases.
- Draining stops new assignments and checkpoints resumable native state.
- Autoscaling signals include queued attempts, active leases, memory pressure,
  model-runtime capacity and harness-specific concurrency.

## Persistence Layout

The initial relational model should include:

```text
harness_descriptors          harness_capability_snapshots
model_descriptors            model_runtime_descriptors
model_runtime_capabilities   model_bindings
model_runtime_workers        model_capacity_leases
inference_requests           inference_admissions
inference_stream_records     inference_final_responses
actors                       actor_bindings
sessions                     session_bindings
tasks                        turns
attempts                     worker_leases
operation_fences             handoff_operations
command_inbox                command_outbox
events                       event_native_provenance
event_branches               event_parents
messages                     content_parts
tool_definition_snapshots    tool_calls
tool_progress                tool_results
interactions                 interaction_responses
plans                        plan_steps
todos                        goals
processes                    process_chunks
artifacts                    workspace_descriptors
workspace_snapshots          runtime_config_snapshots
usage_records                error_records
compactions                  portable_context_checkpoints
resume_checkpoints           native_record_sets
native_records               session_capsules
capsule_record_batches       handoff_loss_reports
```

Payloads that vary by event type may remain versioned JSON/JSONB initially, but
identity, ordering, tenancy, lifecycle, correlation, integrity and query-critical
fields are typed columns. Object bytes live outside the relational database.

## Migration from Current OmniSolo State

1. Introduce the canonical schema and worker protocol alongside existing APIs.
2. Wrap the current OmniSolo harness as the first adapter using the same worker
   contract as external harnesses.
3. Convert current `AgentEvent` emissions into canonical events while retaining
   the legacy stream as a projection.
4. Import `agent_session_data`, checkpoints and hibernation files as legacy
   native records plus best-effort canonical messages/configuration.
5. Replace opaque `StateHandoff` payloads with versioned capsule references and
   import reports while retaining the old field during a bounded migration.
6. Add external adapters one at a time behind conformance tests.
7. Move reads to canonical projections, then retire duplicated session stores.

Before changing a public or internal legacy route, generate a checked-in
compatibility inventory from current protobuf services, HTTP routes, event
variants and persisted payload versions. Each entry maps its request, response,
stream lifecycle, error behavior and deprecation policy to a canonical command
or projection. Golden fixtures and restart tests preserve that inventory during
the migration; the superseded design document is not the compatibility source.

Legacy imports must label unavailable fields instead of inventing IDs,
timestamps, tool outcomes, or resume guarantees.

## Adapter Conformance

Every adapter is tested against a shared suite covering:

- capability discovery and version negotiation;
- create, exact resume, close, delete and fork where supported;
- prompt admission, streaming, cancellation, steering and retry;
- message/content round trips including binary artifacts;
- tool call/progress/result correlation;
- approvals, questions and disconnect resolution;
- sequence ordering, duplicate delivery and cursor replay;
- workspace snapshots, commands, diffs and artifacts;
- compaction, branching and subagent lineage;
- worker crash, lease expiry, drain and restart;
- stale worker writes after lease expiry and reassignment, covering events,
  checkpoints, native records, artifacts and workspace mutations;
- checkpoint crash windows around command admission, effect recording and
  outbox acknowledgement, including uncertain non-idempotent effects;
- concurrent tool effects, background tasks, child-agent output, pending
  interactions and worker partitions during every handoff state;
- unknown ignorable and unknown required events;
- native export/import and loss-report generation;
- durable replay and integrity ranges across discarded transient deltas;
- capsule redaction canaries in instructions, extensions, tool output,
  artifacts and native records, proving allowlist enforcement;
- model-runtime registration, capacity leasing, routing, streaming,
  cancellation, failover classification and usage reconciliation, including a
  stale capacity holder and gateway restart before/after uncertain admission;
- event DAG rejection for cycles, cross-session parents and missing ancestors,
  branch-head compare-and-swap, and capsule ancestor closure;
- lifecycle transition fixtures covering duplicate, stale and out-of-order
  terminal events for every state family;
- spoofed imported system/service actors and historical prompt-channel content,
  proving they cannot acquire target instruction or authorization authority;
- tenant isolation, redaction and secret exclusion.

Golden fixtures are pinned to harness and protocol revisions. Tests never depend
on live credentials. Optional end-to-end suites exercise real harness binaries
and model endpoints in isolated containers.

## Acceptance Criteria

1. OmniSolo can create a session, run tasks and resume through its native harness
   using the new worker contract without a behavior regression.
2. At least two external harness adapters can run independently scaled workers.
3. Same-harness restart resumes from native state without duplicating canonical
   events, losing admitted commands, or automatically repeating unresolved
   non-idempotent effects.
4. Cross-harness handoff preserves the portable transcript, completed tool
   history, interactions, plans, artifacts, effective config and workspace state,
   and produces an explicit loss report.
5. Credentials, live grants, raw chain-of-thought and process handles never enter
   a portable capsule.
6. Gateway, harness workers and future model-runtime workers have independent
   Docker images, health checks and Kubernetes scaling policies.
7. A future self-hosted open-source model runtime can be registered without a
   session-schema migration or harness-adapter rewrite, and the placeholder
   protocol covers fenced capacity, routing, streaming, cancellation, failover
   and usage reconciliation.
8. Existing public APIs remain available through compatibility-inventory
   projections and golden lifecycle fixtures during migration.
9. The old Codex-parity design is not implemented as a competing runtime.
10. A partitioned stale worker cannot commit any mutation after lease
    reassignment, and handoff never permits simultaneous source/target writers.

## Deferred Decisions

- The first external harness adapter after the OmniSolo adapter.
- The initial object-store implementation for standalone mode.
- Whether native records receive application-level encryption in addition to
  storage-layer encryption.
- Which self-hosted inference engine is supported first; the architecture does
  not currently select one.
