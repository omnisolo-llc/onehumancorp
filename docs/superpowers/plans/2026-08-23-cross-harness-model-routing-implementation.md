# Cross-Harness Model Routing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Route a portable session/task model selection into every advertised OmniSolo harness and prove all eight harnesses execute `gpt-5.6-luna` through Sub2API.

**Architecture:** Add a detached, non-secret resolved model selection to the canonical request and capsule path, resolve deployment defaults in the worker, and translate that selection through native stdio, ACP, HTTP, or SDK-sidecar adapters. Keep each harness behind `HarnessAdapter`, preserve native protocol records, and run deterministic contract tests before an opt-in real API matrix.

**Tech Stack:** Rust 2024, Tokio, serde/serde_json, reqwest, Axum, JSON-RPC/JSONL, tonic gRPC, PostgreSQL/MySQL migrations, Bash deployment tests, Docker Compose, Helm, upstream harness CLIs/SDKs.

---

### Task 1: Portable resolved model selection

**Files:**
- Modify: `src/server/harness/middleware/types.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Modify: `src/server/harness/middleware/capsule.rs`
- Modify: `src/server/migrations/219_harness_middleware_records.sql`
- Modify: `src/server/db/migrations/219_harness_middleware_records_mysql.sql`
- Modify: `src/server/harness/tests/schema_contract.rs`
- Modify: `src/server/harness/tests/external_adapters.rs`

- [ ] **Step 1: Write failing serialization, transfer, and schema tests**

Add assertions constructing this typed value and proving it survives request and capsule JSON while excluding credentials:

```rust
let selection = ResolvedModelSelection {
    provider_route: "openai-compatible".into(),
    model_id: "gpt-5.6-luna".into(),
    reasoning_effort: Some(ReasoningEffort::Max),
    api_dialect: ModelApiDialect::OpenAiResponses,
    context_window: None,
    max_output_tokens: None,
    capabilities: BTreeSet::from(["tools".into(), "reasoning".into()]),
    binding_revision: "binding-v1".into(),
    binding_digest: "sha256:test".into(),
    metadata: JsonMap::new(),
};
```

Assert PostgreSQL and MySQL migration contracts expose `resolved_model JSONB`
and `resolved_model JSON`, respectively.

- [ ] **Step 2: Run focused tests and observe the missing-type/column failures**

Run: `cargo test -p server_harness --test external_adapters process_adapter_forwards_protocol_and_transfer_context_to_the_worker -- --nocapture`

Run: `cargo test -p server_harness --test schema_contract -- --nocapture`

Expected: FAIL because `ResolvedModelSelection` and `resolved_model` do not exist.

- [ ] **Step 3: Implement the canonical types and request/capsule projection**

Add `ReasoningEffort`, `ModelApiDialect`, and `ResolvedModelSelection` in
`types.rs`; add `resolved_model: Option<ResolvedModelSelection>` to
`HarnessSessionRequest`; add a builder `with_resolved_model`; project only its
non-secret fields into capsules; add nullable `resolved_model` columns to both
SQL dialects.

- [ ] **Step 4: Run focused tests to green**

Run the two commands from Step 2. Expected: PASS.

- [ ] **Step 5: Commit the model-selection contract**

```bash
git add src/server/harness/middleware/types.rs src/server/harness/middleware/harness.rs src/server/harness/middleware/capsule.rs src/server/migrations/219_harness_middleware_records.sql src/server/db/migrations/219_harness_middleware_records_mysql.sql src/server/harness/tests/schema_contract.rs src/server/harness/tests/external_adapters.rs
git commit -m "feat: add portable harness model selection"
```

### Task 2: Worker configuration and native translation inputs

**Files:**
- Modify: `src/server/harness_worker/lib.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Test: `src/server/harness_worker/lib.rs`

- [ ] **Step 1: Write failing worker configuration tests**

Test precedence and defaults with a pure input struct rather than mutating the
global process environment:

```rust
assert_eq!(config.api_key.as_deref(), Some("new-key"));
assert_eq!(config.api_base_url.as_deref(), Some("https://llmapi.omnisolo.co/v1"));
assert_eq!(config.model, "gpt-5.6-luna");
assert_eq!(config.reasoning_effort, ReasoningEffort::Max);
assert!(!format!("{config:?}").contains("new-key"));
```

Also test new variables winning over deprecated `OMNISOLO_HARNESS_*` inputs,
empty-value handling, invalid URL, invalid effort, and no secret in arguments.

- [ ] **Step 2: Run worker tests and observe failures**

Run: `cargo test -p omnisolo_harness_worker --lib -- --nocapture`

Expected: FAIL because the worker still reads only OmniSolo-prefixed key/base
variables and has no model/effort fields.

- [ ] **Step 3: Implement resolved worker defaults**

Read `OPENAI_API_KEY`, `OPENAI_API_BASE_URL`, `OPENAI_MODEL`, and
`OPENAI_REASONING_EFFORT`; default model/effort to `gpt-5.6-luna`/`max`; retain
the old key/base names only as lower-priority fallbacks; inject the key into the
child environment; store model defaults in `ProcessHarnessSpec` without putting
the key in `args`.

- [ ] **Step 4: Run worker tests to green**

Run: `cargo test -p omnisolo_harness_worker --lib -- --nocapture`

Expected: PASS with no credential text in output.

- [ ] **Step 5: Commit worker configuration**

```bash
git add src/server/harness_worker/lib.rs src/server/harness/middleware/harness.rs
git commit -m "feat: configure harness model defaults"
```

### Task 3: Shared native transport dispatch

**Files:**
- Modify: `src/server/harness/middleware/harness.rs`
- Modify: `src/server/harness/middleware/protocol.rs`
- Create: `src/server/harness/middleware/http_runtime.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: `src/server/harness/tests/external_adapters.rs`

- [ ] **Step 1: Write failing adapter-dispatch tests**

Assert each protocol kind selects its native runtime family:

```rust
assert_eq!(runtime_family(HarnessProtocolKind::CodexAppServer), RuntimeFamily::JsonRpc);
assert_eq!(runtime_family(HarnessProtocolKind::DeepSeekJsonRpc), RuntimeFamily::JsonRpc);
assert_eq!(runtime_family(HarnessProtocolKind::KimiAcp), RuntimeFamily::JsonRpc);
assert_eq!(runtime_family(HarnessProtocolKind::PiRpc), RuntimeFamily::JsonLines);
assert_eq!(runtime_family(HarnessProtocolKind::OpenCodeHttp), RuntimeFamily::Http);
assert_eq!(runtime_family(HarnessProtocolKind::OpenHandsHttp), RuntimeFamily::Http);
assert_eq!(runtime_family(HarnessProtocolKind::OpenHarnessSdk), RuntimeFamily::JsonLines);
```

Add serde alias tests for `opencode_json_rpc`, `pi_jsonl`, `kimi_json_rpc`, and
`openharness_acp`.

- [ ] **Step 2: Run focused tests and observe failures**

Run: `cargo test -p server_harness --test external_adapters process_adapter_selects_native_codec_only_for_codex_app_server -- --nocapture`

Expected: FAIL because only Codex enters the native runtime.

- [ ] **Step 3: Implement runtime-family dispatch and HTTP lifecycle helper**

Add accurate protocol variants with serde aliases, a `RuntimeFamily` mapping,
and an HTTP runtime that starts a child/server, polls readiness with a bounded
deadline, retains the selected loopback address, and shuts the process down on
drop. Keep the legacy generic line protocol only for `Custom`.

- [ ] **Step 4: Run focused tests to green**

Run the command from Step 2 and all `external_adapters` tests. Expected: PASS.

- [ ] **Step 5: Commit transport dispatch**

```bash
git add src/server/harness/middleware/harness.rs src/server/harness/middleware/protocol.rs src/server/harness/middleware/http_runtime.rs src/server/harness/middleware/mod.rs src/server/harness/tests/external_adapters.rs
git commit -m "feat: dispatch native harness transports"
```

### Task 4: Codex model and reasoning translation

**Files:**
- Modify: `src/server/harness/middleware/codex_app_server.rs`
- Modify: `src/server/harness_worker/lib.rs`
- Modify: `src/server/harness/tests/codex_app_server.rs`
- Modify: `src/server/harness/tests/external_adapters.rs`

- [ ] **Step 1: Write failing Codex codec tests**

Assert `thread/start` receives `model: "gpt-5.6-luna"`,
`modelProvider: "omnisolo"`, and `reasoningEffort: "max"` from the typed
selection, and that worker provider overrides use the `/v1` base with Responses
wire API.

- [ ] **Step 2: Run Codex tests and observe failures**

Run: `cargo test -p server_harness --test codex_app_server -- --nocapture`

Expected: FAIL because model values currently come from `codex.*` extensions.

- [ ] **Step 3: Translate typed selection and retain extension compatibility**

Prefer `request.resolved_model`; use namespaced extensions only when the typed
field is absent; map `ReasoningEffort::Max` to Codex native `max`; keep API key
in `OPENAI_API_KEY` and provider configuration in non-secret `-c` overrides.

- [ ] **Step 4: Run Codex deterministic tests to green**

Run Codex and worker package tests. Expected: PASS.

- [ ] **Step 5: Commit Codex translation**

```bash
git add src/server/harness/middleware/codex_app_server.rs src/server/harness_worker/lib.rs src/server/harness/tests/codex_app_server.rs src/server/harness/tests/external_adapters.rs
git commit -m "feat: route model selection to codex"
```

### Task 5: DeepSeek Harness native JSON-RPC codec

**Files:**
- Create: `src/server/harness/middleware/deepseek_harness.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/deepseek_harness.rs`
- Modify: `src/server/harness/Cargo.toml`

- [ ] **Step 1: Write failing codec and fake-process E2E tests**

Cover `initialize` with provider/model/max tokens, `session/prompt`, completion
notifications, usage, cancellation, shutdown, malformed frames, and process
exit. Assert `gpt-5.6-luna` and `max` reach initialization and that
`DEEPSEEK_API_KEY`/`DEEPSEEK_BASE_URL` exist only in the child environment.

- [ ] **Step 2: Run the DeepSeek test target and observe missing codec failure**

Run: `cargo test -p server_harness --test deepseek_harness -- --nocapture`

Expected: FAIL because `DeepSeekHarnessCodec` does not exist.

- [ ] **Step 3: Implement the official SDK JSON-RPC contract**

Implement the upstream newline-delimited methods and notifications behind
`HarnessProtocolCodec`, including session ID correlation and terminal status
mapping. Pin the runtime launch command in deployment rather than downloading a
floating package during production startup.

- [ ] **Step 4: Run DeepSeek tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit DeepSeek integration**

```bash
git add src/server/harness/middleware/deepseek_harness.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/deepseek_harness.rs src/server/harness/Cargo.toml
git commit -m "feat: integrate deepseek harness json rpc"
```

### Task 6: ACP codec and Kimi adapter

**Files:**
- Create: `src/server/harness/middleware/acp.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/acp_kimi.rs`

- [ ] **Step 1: Write failing ACP/Kimi tests**

Cover `initialize`, `session/new`, `session/load`, `session/prompt`,
`session/update`, permission requests, cancellation, model config selection,
and shutdown. Verify generated isolated Kimi configuration uses an
`openai_responses` provider with the selected base/model and references the
child environment key.

- [ ] **Step 2: Run tests and observe missing ACP codec failure**

Run: `cargo test -p server_harness --test acp_kimi -- --nocapture`

Expected: FAIL because the ACP codec and `KimiAcp` dispatch do not exist.

- [ ] **Step 3: Implement ACP v1 client behavior and Kimi translation**

Implement stable ACP lifecycle and event conversion; send model selection via
the advertised model config option; map unsupported maximum effort to a typed
capability downgrade event before prompt admission.

- [ ] **Step 4: Run ACP/Kimi tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit Kimi integration**

```bash
git add src/server/harness/middleware/acp.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/acp_kimi.rs
git commit -m "feat: integrate kimi over acp"
```

### Task 7: Pi RPC adapter

**Files:**
- Create: `src/server/harness/middleware/pi_rpc.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/pi_rpc.rs`

- [ ] **Step 1: Write failing Pi JSONL contract tests**

Cover prompt, steer, follow-up, abort, model selection, thinking level,
streamed text/tool events, usage, session state, compaction, and clean exit.
Assert generated `models.json` uses `openai-responses`, model
`gpt-5.6-luna`, and maps maximum effort to Pi's `xhigh` level.

- [ ] **Step 2: Run tests and observe missing Pi runtime failure**

Run: `cargo test -p server_harness --test pi_rpc -- --nocapture`

Expected: FAIL because `PiRpcRuntime` does not exist.

- [ ] **Step 3: Implement Pi's command/event JSONL runtime**

Implement request serialization without JSON-RPC IDs, event correlation by
active prompt/session, terminal detection, cancellation, and native event
preservation. Generate an isolated Pi home and delete it after shutdown.

- [ ] **Step 4: Run Pi tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit Pi integration**

```bash
git add src/server/harness/middleware/pi_rpc.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/pi_rpc.rs
git commit -m "feat: integrate pi rpc harness"
```

### Task 8: OpenCode headless HTTP adapter

**Files:**
- Create: `src/server/harness/middleware/opencode.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/opencode_http.rs`

- [ ] **Step 1: Write failing mock-server and child lifecycle tests**

Cover readiness, create/get/delete session, synchronous prompt, async events,
abort, model/provider selection, usage/error mapping, server exit, timeout, and
loopback-only binding. Assert generated config uses the OpenAI-compatible
Responses package and references `OPENAI_API_KEY` without writing its value.

- [ ] **Step 2: Run tests and observe missing adapter failure**

Run: `cargo test -p server_harness --test opencode_http -- --nocapture`

Expected: FAIL because `OpenCodeHttpAdapter` does not exist.

- [ ] **Step 3: Implement OpenCode HTTP/OpenAPI adapter**

Launch `opencode serve` on an allocated loopback port, poll its health endpoint,
call typed session APIs with `reqwest`, consume its event stream, and map native
messages/events into canonical events. Use an isolated config/data home.

- [ ] **Step 4: Run OpenCode tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit OpenCode integration**

```bash
git add src/server/harness/middleware/opencode.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/opencode_http.rs
git commit -m "feat: integrate opencode server"
```

### Task 9: OpenHands Agent Server adapter

**Files:**
- Create: `src/server/harness/middleware/openhands.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/openhands_http.rs`

- [ ] **Step 1: Write failing REST/event contract tests**

Cover health/readiness, conversation creation/resume/delete, prompt streaming,
actions/observations, approvals, cancellation, usage, typed provider errors,
and shutdown. Assert LLM configuration uses `openai/gpt-5.6-luna`, the selected
base URL, and maximum reasoning only when accepted by the SDK.

- [ ] **Step 2: Run tests and observe missing adapter failure**

Run: `cargo test -p server_harness --test openhands_http -- --nocapture`

Expected: FAIL because `OpenHandsHttpAdapter` does not exist.

- [ ] **Step 3: Implement OpenHands REST/event adapter**

Launch or connect to the Agent Server, wait for readiness, call its REST API,
consume events, map actions and observations, and preserve native event JSON.
Classify LiteLLM errors into stable OmniSolo provider errors.

- [ ] **Step 4: Run OpenHands tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit OpenHands integration**

```bash
git add src/server/harness/middleware/openhands.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/openhands_http.rs
git commit -m "feat: integrate openhands agent server"
```

### Task 10: AgentBoardTT OpenHarness SDK sidecar

**Files:**
- Create: `src/server/harness/sidecars/openharness_bridge.py`
- Create: `src/server/harness/middleware/openharness.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Create: `src/server/harness/tests/openharness_sdk.rs`

- [ ] **Step 1: Write failing sidecar and adapter contract tests**

Cover create/resume session, prompt streaming, text/tool/result conversion,
steering, cancellation, model/base/provider arguments, clean exit, malformed
SDK events, and secret redaction. Verify the sidecar reads the key from its
environment and never includes it in JSONL frames.

- [ ] **Step 2: Run tests and observe missing sidecar failure**

Run: `cargo test -p server_harness --test openharness_sdk -- --nocapture`

Expected: FAIL because the SDK sidecar and adapter do not exist.

- [ ] **Step 3: Implement the pinned Python SDK bridge**

Implement a newline-delimited OmniSolo sidecar protocol around
`harness.run(...)`, pass `provider="openai"`, selected model/base URL and
permission mode, retain session IDs, and convert SDK messages into typed native
events. Replace the inaccurate persisted ACP label with a serde-compatible
`OpenHarnessSdk` protocol kind.

- [ ] **Step 4: Run OpenHarness tests to green**

Run the command from Step 2. Expected: PASS.

- [ ] **Step 5: Commit OpenHarness integration**

```bash
git add src/server/harness/sidecars/openharness_bridge.py src/server/harness/middleware/openharness.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/tests/openharness_sdk.rs
git commit -m "feat: integrate agentboard openharness sdk"
```

### Task 11: OmniSolo provider-backed E2E path

**Files:**
- Modify: `src/server/harness/middleware/harness.rs`
- Modify: `src/server/harness/middleware/inference.rs`
- Create: `src/server/harness/tests/omnisolo_live.rs`

- [ ] **Step 1: Write failing provider-backed bridge test**

Inject a deterministic OpenAI-compatible inference fixture, execute a session
through `OmniSoloHarnessAdapterBridge`, and assert the resolved model/effort,
streamed text, terminal result, usage, and model-binding provenance.

- [ ] **Step 2: Run test and observe that the bridge does not call inference**

Run: `cargo test -p server_harness --test omnisolo_live -- --nocapture`

Expected: FAIL because the current bridge only exercises the local compatibility
adapter.

- [ ] **Step 3: Connect OmniSolo bridge to the existing inference boundary**

Resolve the attempt's `ModelBinding`, invoke the stable inference endpoint,
translate streamed content/tool calls into existing OmniSolo events, and retain
the current local adapter as an explicit test/native-development mode.

- [ ] **Step 4: Run OmniSolo tests to green**

Run the command from Step 2 and existing OmniSolo bridge tests. Expected: PASS.

- [ ] **Step 5: Commit OmniSolo inference integration**

```bash
git add src/server/harness/middleware/harness.rs src/server/harness/middleware/inference.rs src/server/harness/tests/omnisolo_live.rs
git commit -m "feat: route omnisolo harness through inference"
```

### Task 12: Worker images and independently scalable deployment

**Files:**
- Modify: `deploy/docker/Dockerfile.harness-worker`
- Modify: `deploy/docker-compose.yml`
- Modify: `deploy/helm/ohc/values.yaml`
- Modify: `deploy/helm/ohc/templates/harness-workers.yaml`
- Modify: `deploy/helm/ohc/templates/harness-workers-hpa.yaml`
- Modify: `deploy/tests/harness_worker_deployment_contract_test.sh`

- [ ] **Step 1: Write failing deployment contract assertions**

Require eight worker definitions, pinned harness versions/images, public
OpenAI-compatible environment names, Secret refs, per-pool replica/HPA values,
readiness probes, ephemeral config homes, and no host credential mounts.

- [ ] **Step 2: Run deployment tests and observe failures**

Run: `bash deploy/tests/harness_worker_deployment_contract_test.sh`

Expected: FAIL because only OmniSolo/Codex/OpenCode workers are rendered and old
environment names remain.

- [ ] **Step 3: Implement Compose/Helm/image wiring**

Add all harness pools with native executable arguments, pin versions, map
`OPENAI_API_KEY` through secrets, set the base/model/effort defaults, and keep
each pool independently scalable. Install only the harness required by each
image target or use per-harness images to avoid one oversized runtime image.

- [ ] **Step 4: Render and test deployments**

Run:

```bash
bash deploy/tests/harness_worker_deployment_contract_test.sh
docker compose -f deploy/docker-compose.yml --profile harness config >/dev/null
helm template omnisolo deploy/helm/ohc >/dev/null
```

Expected: all commands exit 0.

- [ ] **Step 5: Commit deployment integration**

```bash
git add deploy/docker/Dockerfile.harness-worker deploy/docker-compose.yml deploy/helm/ohc/values.yaml deploy/helm/ohc/templates/harness-workers.yaml deploy/helm/ohc/templates/harness-workers-hpa.yaml deploy/tests/harness_worker_deployment_contract_test.sh
git commit -m "feat: deploy independently scalable harness pools"
```

### Task 13: Real Sub2API eight-harness matrix

**Files:**
- Create: `src/server/harness/tests/live_harness_matrix.rs`
- Create: `scripts/test-live-harness-matrix.sh`
- Modify: `src/server/harness/Cargo.toml`
- Modify: `README.md`

- [ ] **Step 1: Write the ignored live test harness and matrix assertions**

Require `OMNISOLO_RUN_LIVE_HARNESS_E2E=1` and `SUB2API_API_KEY`; map the latter
to child `OPENAI_API_KEY`; query `/v1/models`; require `gpt-5.6-luna`; run one
no-tool prompt through each harness; capture model, reasoning translation,
terminal text, usage, duration, and failure class in JSON.

- [ ] **Step 2: Run without the live flag and verify explicit skip behavior**

Run: `cargo test -p server_harness --test live_harness_matrix -- --ignored --nocapture`

Expected: PASS with each live case reporting skipped because the explicit flag
is absent; no network model turn occurs.

- [ ] **Step 3: Implement the matrix runner and per-harness launch discovery**

Use pinned executables or package runners, isolated temporary homes, bounded
timeouts, no workspace writes, and clean shutdown. Return nonzero if any of the
eight harnesses fails, is unavailable, silently drops reasoning, or bypasses its
native adapter.

- [ ] **Step 4: Run the real matrix with the host key**

Run:

```bash
OMNISOLO_RUN_LIVE_HARNESS_E2E=1 \
OPENAI_API_BASE_URL=https://llmapi.omnisolo.co/v1 \
OPENAI_MODEL=gpt-5.6-luna \
OPENAI_REASONING_EFFORT=max \
bash scripts/test-live-harness-matrix.sh
```

Expected: JSON report with `passed` for OmniSolo, Codex, OpenCode, DeepSeek,
Pi, Kimi, OpenHands, and AgentBoardTT OpenHarness; exit 0; no key in output.

- [ ] **Step 5: Commit live matrix**

```bash
git add src/server/harness/tests/live_harness_matrix.rs scripts/test-live-harness-matrix.sh src/server/harness/Cargo.toml README.md
git commit -m "test: verify every harness through sub2api"
```

### Task 14: Full verification and coverage

**Files:**
- Modify only files required to fix failures found by these commands.

- [ ] **Step 1: Format and run all deterministic tests**

```bash
cargo fmt --all -- --check
cargo test -p server_harness --all-targets -- --nocapture
cargo test -p omnisolo_harness_worker --all-targets -- --nocapture
bash deploy/tests/harness_worker_deployment_contract_test.sh
```

Expected: all commands exit 0.

- [ ] **Step 2: Run SQL and build-system verification**

```bash
cargo test -p server_ohc --test harness_migration_parity -- --nocapture
bazel test //src/server/harness/... //src/server/harness_worker/...
git diff --check
```

Expected: all commands exit 0.

- [ ] **Step 3: Run coverage and inspect every new branch**

Run package coverage with `cargo llvm-cov` for `server_harness` and
`omnisolo_harness_worker`. Add focused tests for each uncovered reachable branch
in model resolution, codecs, event/error mapping, redaction, and lifecycle code;
document only unreachable process-entry or OS-failure branches as residual risk.

- [ ] **Step 4: Re-run the live matrix after deterministic verification**

Run the command from Task 13 Step 4. Expected: all eight harnesses pass again.

- [ ] **Step 5: Record final evidence**

Record exact test counts, coverage percentages, pinned harness versions, matrix
results, and any explicit reasoning capability downgrade in the final report.
