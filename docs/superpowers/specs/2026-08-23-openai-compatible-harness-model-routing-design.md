# OmniSolo OpenAI-Compatible Harness and Model Routing Design

**Status:** Approved design; pending written-spec review.

**Extends:** `2026-08-22-omnisolo-harness-middleware-design.md`.

## Objective

Give every OmniSolo harness worker one public OpenAI-compatible deployment
contract, preserve model choice across session/task routing and handoff, and
prove every advertised harness can execute a real turn through Sub2API.

This work must turn existing protocol placeholders into working integrations.
A registry descriptor is not "supported" until its deterministic adapter suite
and opt-in real API test both pass.

## Public Configuration

The public worker defaults are:

| Variable | Meaning | Default |
| --- | --- | --- |
| `OPENAI_API_KEY` | Secret used for the OpenAI-compatible endpoint | none |
| `OPENAI_API_BASE_URL` | API root before provider paths | none |
| `OPENAI_MODEL` | Default model ID for new sessions/tasks | `gpt-5.6-luna` |
| `OPENAI_REASONING_EFFORT` | Default reasoning effort | `max` |

For the OmniSolo Sub2API deployment, `OPENAI_API_BASE_URL` is
`https://llmapi.omnisolo.co/v1`. The worker may derive a harness-specific path
only when the native protocol requires it. Public callers do not configure
Codex's `/backend-api/codex` alias.

The existing `OMNISOLO_HARNESS_API_KEY` and `OMNISOLO_HARNESS_BASE_URL`
variables remain deprecated read-only fallbacks for one release. New Compose,
Helm, documentation, and tests use only the public variables above. A new
variable wins when both forms are set. Empty values are treated as absent.

Secrets remain deployment data. API keys never enter session capsules, model
descriptors, model bindings, logs, Debug output, command arguments, generated
configuration files, or durable events. Workers inject credentials only into
the harness child process or protected in-memory SDK configuration.

## Canonical Model Selection

Model intent and realized routing remain distinct:

- `RuntimeConfigSnapshot.requested_model` records portable model intent,
  including model ID, family/capability requirements, limits, and non-secret
  provider metadata.
- `ModelBinding` records the concrete model/runtime selected for one attempt.
- `HarnessSessionRequest` carries a detached resolved model selection in
  addition to `model_binding_id`; a remote worker must not query the control
  plane database to discover the model name.
- `SessionCapsule` carries requested model intent and historical non-secret
  bindings, never credentials or an endpoint authorization value.

Selection precedence is:

1. Task runtime snapshot.
2. Session runtime snapshot.
3. Worker `OPENAI_MODEL` and `OPENAI_REASONING_EFFORT` defaults.

The resolved transport value contains at least provider route, model ID,
reasoning effort, context/output limits, API dialect, capabilities, and model
binding revision/digest. It is immutable for an attempt. Changing the model
creates a new attempt binding; it does not rewrite prior events.

The initial default is model ID `gpt-5.6-luna` with reasoning effort `max`.
These are separate fields. An adapter must translate both, reject an invalid
combination, or emit a typed capability downgrade requiring policy acceptance.
It must never silently discard `max`.

## Adapter Matrix

The supported set is the eight harnesses currently advertised by
`HarnessRegistry`. Each adapter uses the upstream programmatic interface rather
than scraping terminal output.

| Harness | Required transport | OpenAI-compatible translation |
| --- | --- | --- |
| OmniSolo | Existing in-process adapter | Use resolved `ModelBinding` through the existing inference/provider boundary |
| Codex | Native app-server v2 JSON-RPC over stdio | Configure a generated provider route, pass model on native thread/turn fields, and map reasoning effort when app-server supports it |
| OpenCode | Headless HTTP/OpenAPI server plus event stream | Generate an isolated custom provider using the OpenAI-compatible Responses package and select `provider/model` per session request |
| DeepSeek Harness | Official `dsh` SDK newline-delimited JSON-RPC server | Map key/base to child `DEEPSEEK_*`, register the selected model in the isolated provider catalog, and send provider/model during `initialize` |
| Pi | Native `pi --mode rpc --no-session` JSONL protocol | Generate an isolated `models.json`, use an OpenAI Responses-compatible provider, then apply model and `xhigh`/native maximum reasoning through RPC |
| Kimi | Native `kimi acp` JSON-RPC over stdio | Generate an isolated OpenAI Responses provider/model config, select the ACP model option, and map maximum thinking when representable |
| OpenHands | OpenHands Agent Server REST/event API | Construct an SDK `LLM`/server config with `openai/<model>`, base URL, key, and supported reasoning parameters |
| AgentBoardTT OpenHarness | Python SDK streaming API behind an OmniSolo sidecar protocol | Pass provider `openai`, base URL, selected model, and reasoning settings supported by the SDK |

The existing `OpenCodeJsonRpc`, `KimiJsonRpc`, and `OpenHarnessAcp` labels do not
match the current upstream transports. They must be migrated to accurate
protocol kinds while retaining deserialization aliases for persisted records.
OpenHands remains a network adapter and must use readiness checks before
admitting work.

All upstream package/binary versions are pinned in worker images and test
fixtures. DeepSeek Harness is a developer preview, so its codec is revisioned
and compatibility failures quarantine that worker version instead of guessing
at changed wire semantics.

## Process and Deployment Isolation

Each harness pool runs independently from the OmniSolo gateway and from other
harnesses. Docker Compose and Helm expose the same environment contract.

- One worker image/pool and HPA target per harness.
- Credentials come from Compose secrets/environment or Kubernetes Secret refs.
- Generated native config and harness homes live in per-process temporary or
  ephemeral volumes with owner-only permissions.
- No host `CODEX_HOME`, Kimi/Pi/OpenCode home, or developer credential store is
  mounted for tests or production.
- Worker readiness is false until the harness process/server has completed its
  native initialize/health exchange.
- Session affinity is used only where native state is local. Portable handoff
  remains available when the destination worker has no source-native state.
- Model runtime workers remain independently scalable from harness workers.

Generated configurations may contain environment-variable references but not
resolved key bytes. Any upstream that only accepts a key in a file receives a
short-lived `0600` file in an isolated temporary directory, deleted when the
process exits; this exception is covered by a secret-leak test.

## Errors and Capability Downgrades

Configuration fails before process launch when the endpoint, model ID, or
reasoning value is missing or malformed. Native errors map to stable classes:
authentication, unknown model, unsupported reasoning, protocol mismatch,
rate-limit, provider rejection, timeout, process/server exit, and uncertain
post-admission failure.

The real test matrix distinguishes:

- `passed`: a real model turn completed through the named harness and selected
  model;
- `skipped`: only when the live-test flag or credential is absent;
- `unsupported`: an upstream release cannot represent a required capability;
- `failed`: installation, configuration, protocol, or model execution failed.

`unsupported` is not a passing result. A harness advertised as supported must
have a passing baseline turn. Reasoning effort may be reported as a capability
downgrade only when the harness still executes the selected model and OmniSolo
records the downgrade explicitly.

## Test Strategy

Implementation follows test-driven development. Every behavior first receives
a failing test that is observed failing for the intended reason.

### Deterministic tests

- Unit tests for environment precedence, URL normalization, model/reasoning
  validation, native variable translation, config generation, and redaction.
- Serialization and MySQL/PostgreSQL contract tests for detached model
  selections, runtime snapshots, bindings, aliases, and capsule transfer.
- One native codec contract suite per harness covering initialize, session
  creation/resume/close, prompt streaming, final response, usage, cancellation,
  approvals/questions where supported, model selection, errors, and shutdown.
- Process/server E2E tests using deterministic fixtures for every transport.
- Worker gRPC E2E tests proving canonical session/task requests reach each
  native adapter and events return with model/binding provenance.
- Docker Compose rendering, Helm rendering, Secret refs, readiness, independent
  scaling, and no-host-home deployment tests.
- Secret scans over Debug output, logs, generated config, process arguments,
  events, capsules, and test artifacts.

### Real Sub2API matrix

An ignored, explicitly enabled suite uses host `SUB2API_API_KEY` only as the
source credential and maps it to `OPENAI_API_KEY` in child environments. It sets:

```text
OPENAI_API_BASE_URL=https://llmapi.omnisolo.co/v1
OPENAI_MODEL=gpt-5.6-luna
OPENAI_REASONING_EFFORT=max
```

Before running harnesses, the suite calls `GET /v1/models` and requires the
selected model to be advertised. Each harness then runs in an empty isolated
home/workspace and answers a deterministic no-tool prompt. The assertion checks
non-empty final text, terminal success, selected model provenance when exposed,
event ordering, clean shutdown, and absence of the key from captured artifacts.

The suite covers OmniSolo, Codex, OpenCode, DeepSeek Harness, Pi, Kimi,
OpenHands, and AgentBoardTT OpenHarness. It emits a machine-readable matrix and
returns nonzero unless all eight baseline turns pass. Per-harness tests remain
individually runnable for diagnosis. A bounded concurrency limit and output
caps control API cost; retries are disabled except for a single retry on an
explicit pre-admission transient failure.

Because `max` support varies, the report separately records model execution and
reasoning translation. The overall baseline fails on a silent reasoning drop.

## Completion Criteria

This work is complete only when:

1. Public configuration uses `OPENAI_API_KEY`, `OPENAI_API_BASE_URL`,
   `OPENAI_MODEL`, and `OPENAI_REASONING_EFFORT`.
2. Task/session model choice survives worker dispatch and cross-harness capsule
   transfer without carrying secrets.
3. All eight registry entries have real adapters using the transports above.
4. Deterministic unit, codec, process/server, worker, database, deployment, and
   secret-redaction tests pass.
5. The live Sub2API matrix passes all eight harnesses using `gpt-5.6-luna` and
   records how `max` was applied or explicitly downgraded.
6. Docker and Kubernetes can scale every harness and model runtime pool
   independently.
7. No harness is described as supported based only on a descriptor, mock, or
   direct API call that bypasses that harness.

## Upstream References

- Codex app-server: <https://github.com/openai/codex>
- OpenCode server and providers: <https://opencode.ai/docs/server/>,
  <https://opencode.ai/docs/providers/>
- DeepSeek Harness SDK server:
  <https://github.com/deepseek-ai/deepseek-harness/tree/master/packages/sdk/server>
- Pi RPC and models:
  <https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/sdk.md>,
  <https://github.com/badlogic/pi-mono/blob/main/packages/coding-agent/docs/models.md>
- Kimi ACP and configuration: <https://github.com/MoonshotAI/kimi-cli>,
  <https://moonshotai.github.io/kimi-code/en/configuration/env-vars.html>
- OpenHands SDK/Agent Server: <https://docs.openhands.dev/sdk/index>
- AgentBoardTT OpenHarness: <https://github.com/AgentBoardTT/openharness>
