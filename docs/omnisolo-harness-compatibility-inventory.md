# OmniSolo Harness Compatibility Inventory

This inventory is the migration contract for the harness middleware. Existing
routes and payloads remain available while their durable meaning is projected
into the canonical session/task/event model.

| Legacy surface | Compatibility behavior | Canonical projection | Removal policy |
| --- | --- | --- | --- |
| `AgentEvent` stream | Existing event variants and ordering remain unchanged for legacy consumers. | `EventEnvelope` with separate durable and delivery sequences; `ToolCall` and settled messages are typed projections. | Keep until all consumers read canonical events. |
| `StateHandoff` protobuf | Decode and publish the existing opaque payload unchanged. | Store as a legacy native record and require an explicit importer before it can affect canonical state. | Deprecate after capsule import coverage is complete. |
| `InteropProtocol::handoff` | Existing topic, lock, retry, and idempotency behavior remains available. | Additive `SessionOperationEnvelope` capsule path is preferred for new workers. | Keep as a legacy projection. |
| `InteropProtocol::resume_mission` | Delegates to the existing handoff behavior. | New workers use fenced `resume` session operation with a capsule or native checkpoint. | Keep until legacy clients migrate. |
| Opaque checkpoint or hibernation bytes | Preserve original bytes and source metadata. | Import as `NativeRecord` plus best-effort portable context; never treat bytes as authority. | No destructive conversion. |
| Existing API and gRPC routes | No route is removed by this middleware change. | Commands are associated with tenant, session, task, turn, attempt, binding, and lease identifiers when available. | Route removal requires a separate versioned migration. |
| Worker delivery | At-least-once delivery remains valid. | Durable mutation is idempotent by event/command key and fenced by lease generation and token. | Protocol v1 remains readable. |

## Event Mapping

| `AgentEvent` variant | Canonical event type | Portable form |
| --- | --- | --- |
| `RunStarted` | `run.started` | Durable event and attempt history |
| `TextChunk` | `assistant.text_chunk` | Transient delivery unless settled |
| `ToolCall` | `tool.call_settled` | Tool definition/call/result records |
| `TaskComplete` | `task.completed` | Settled assistant message and task result |
| `TaskError` | `task.failed` | Error record and task result |
| `UserInterventionRequired` | `interaction.required` | Interaction request; live grant is not portable |
| `CheckpointSaved` | `context.checkpoint_saved` | Native checkpoint reference only for same-harness resume |
| `Handoff` | `task.handoff_requested` | Handoff operation and capsule |
| `RewindOccurred` | `context.rewound` | Historical context record |
| `GuardrailTripped` | `guardrail.tripped` | Error/policy history |
| `CostUpdate` | `usage.cost_updated` | Usage record |

## Additive Wire Rules

- New worker fields are additive protobuf fields; unknown fields are preserved
  by protobuf decoders and opaque extension maps.
- Session operations do not require a task or attempt. Attempt commands always
  require task, attempt, lease, generation, and fencing data.
- Legacy opaque handoff payloads never populate system/developer instructions,
  credentials, approvals, native cursors, or process authority.
- A compatibility change must add a fixture and a restart/idempotency test to
  the relevant package before changing the inventory.

## Native Harness Worker Matrix

Each session owns its native adapter process and resolved model selection. A
worker can serve multiple sessions, but one session's model, cancellation, or
shutdown cannot mutate another session's harness state.

| Harness | Native transport | Pinned worker | Advertised native controls |
| --- | --- | --- | --- |
| OmniSolo | in-process | `omnisolo/harness-worker:0.1.0` | prompt, stream, cancel, steer, branch, exact resume, capsule import, native export |
| Codex | app-server v2 JSON-RPC | `omnisolo/harness-worker-codex:0.149.0` | prompt, stream, cancel, steer, approvals, questions, workspace, branch, exact resume, capsule import, native export |
| OpenCode | native HTTP/SSE | `omnisolo/harness-worker-opencode:1.18.15` | prompt, stream, cancel, workspace, exact resume, capsule import |
| DeepSeek | native JSON-RPC | `omnisolo/harness-worker-deepseek:0.1.1-rc.2` | prompt, stream, workspace, capsule import |
| Pi | native RPC JSONL | `omnisolo/harness-worker-pi:0.73.1` | prompt, stream, cancel, steer, workspace, capsule import |
| Kimi | ACP v1 | `omnisolo/harness-worker-kimi:1.49.0` | prompt, stream, cancel, approvals, questions, workspace, exact resume, capsule import |
| OpenHands | Agent Server HTTP | `omnisolo/harness-worker-openhands:1.43.1` | prompt, stream, cancel, approvals, per-session workspace, exact resume, capsule import |
| OpenHarness | SDK sidecar JSONL | `omnisolo/harness-worker-openharness:0.6.0` | prompt, stream, cancel, steer, workspace, exact resume, capsule import |

## OpenAI-Compatible Shim Matrix

These harnesses use the same loopback provider facade and a fixed, fail-closed
shim protocol. Their pinned upstream package or release is present in the
worker image, while provider credentials remain owned by the worker facade.

| Harness | Pinned upstream | Pinned worker | Shim entrypoint | Advertised controls |
| --- | --- | --- | --- | --- |
| Aider | `aider-chat==0.86.0` | `omnisolo/harness-worker-aider:0.86.0` | `omnisolo-openai-shim` | prompt, stream, cancel, steer, workspace, capsule import |
| Goose | `aaif-goose/goose:v1.33.1` | `omnisolo/harness-worker-goose:1.33.1` | `omnisolo-openai-shim` | prompt, stream, cancel, steer, workspace, capsule import |
| Open Interpreter | `open-interpreter==0.4.2` | `omnisolo/harness-worker-open-interpreter:0.4.2` | `omnisolo-openai-shim` | prompt, stream, cancel, steer, workspace, capsule import |
| Plandex | `plandex-ai/plandex:cli/v2.2.1` | `omnisolo/harness-worker-plandex:2.2.1` | `omnisolo-openai-shim` | prompt, stream, cancel, steer, workspace, capsule import |

The shim accepts only the four registered harness IDs, rejects unallowlisted
arguments, and translates every provider turn to the loopback Responses route.
It returns a synthetic session ID because the provider facade owns the model
conversation; portable capsule context is inserted once at the next accepted
prompt.

## Shared Local Services

Every native and shimmed harness can receive the same token-free
`omnisolo.local_service_bundle.v1` binding set. The bundle contains references,
scope identities, capabilities, generations, and configuration digests; it
does not contain API keys, cookies, or service authority.

| Service | Binding ID | Scope | Cross-harness rule |
| --- | --- | --- | --- |
| Memory | `omnisolo.memory` | workspace | Current memory stores and a configured Mem0 adapter remain behind this shared service reference. |
| MCP | `omnisolo.mcp` | project | Catalog and invocation are capability-checked per tenant/project. |
| Workspace | `omnisolo.workspace` | workspace | Harnesses share the workspace identity, not process-local state. |
| Artifact | `omnisolo.artifact` | workspace | Artifact access remains scoped to the workspace binding. |
| Browser | `omnisolo.browser` | task | Browser state cannot cross task scope without a new authorization. |
| Cache | `omnisolo.cache` | project | Cache access is shared only inside the project boundary. |
| Integration | `omnisolo.integration` | project | External integration calls remain capability- and tenant-scoped. |
| Provider facade | `omnisolo.provider_facade` | attempt | Only the attempt-owned loopback route can use the upstream credential. |

The same service ID and configuration digest are reused when two harnesses are
bound to one session/workspace; binding IDs remain distinct so revocation and
lease generations stay harness/attempt specific. A capsule transfers these
references, never the backing store or its authority.

The deployment contract is `OPENAI_API_KEY`, `OPENAI_API_BASE_URL`,
`OPENAI_MODEL`, and `OPENAI_REASONING_EFFORT`. Defaults are
`gpt-5.6-luna` and `max`; credentials are injected only into worker/child
environments and are excluded from portable sessions and capsules.

Every model-backed native attempt begins with one durable
`inference.model_binding` event. It records the harness, provider route, model,
requested reasoning effort, API dialect, limits, capabilities, and immutable
binding revision/digest. It never records endpoint credentials. A harness that
cannot realize the requested reasoning level must additionally emit an explicit
`capability.downgraded` event.

Workers handle `SIGTERM` by gracefully stopping both gRPC admission and health
serving while active streams drain. Compose and Helm provide a configurable
120-second termination window so harness processes are not cut off at the
default short container timeout.

Codex imports portable records through native app-server item injection. Harnesses
without a native history-import API receive the same integrity-verified capsule as
a structured historical-context envelope on the first accepted prompt; subsequent
prompts use the native session history and do not resend the envelope.

The opt-in real-provider matrix first verifies the model catalog and one real
Responses turn, then builds every pinned image and sends a native session plus
prompt through each worker. It validates event correlation and ordering, model
provenance, reasoning translation or explicit downgrade, usage, terminal
success, clean session deletion, and credential redaction. Its final output is
`omnisolo.live_harness_matrix.v1` JSON. The host-only key alias is mapped to the
public child contract without mounting any harness home:

```bash
SUB2API_API_KEY="$SUB2API_API_KEY" \
OPENAI_API_BASE_URL=https://llmapi.omnisolo.co/v1 \
scripts/test-live-harness-matrix.sh
```
