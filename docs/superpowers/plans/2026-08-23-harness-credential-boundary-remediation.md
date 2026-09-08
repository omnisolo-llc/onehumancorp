# Harness Credential Boundary Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prevent credentials and unrelated parent-process secrets from entering portable capsules, durable harness events, or third-party harness child processes.

**Architecture:** Add one credential-only recursive sanitizer to the harness type layer and enforce it at capsule projection, native protocol decoding, and final event admission. Add one process-spawn environment helper that clears ambient variables, restores only documented non-secret platform basics, and overlays the explicit harness environment.

**Tech Stack:** Rust, serde/serde_json, Tokio process management, Cargo integration tests.

---

### Task 1: Shared credential-only sanitizer

**Files:**
- Modify: `src/server/harness/middleware/types.rs`
- Test: `src/server/harness/middleware/types.rs`

- [ ] **Step 1: Write failing table-driven sanitizer tests**

Add tests that build nested objects containing opaque synthetic canaries under `api_key`, `accessToken`, `refresh-token`, `clientSecret`, `authorization`, `cookie`, and `password`, plus bearer/assignment strings under ordinary keys. Assert sensitive object keys are absent, secret-like scalar values become `[REDACTED]`, safe `name`, `payload`, model, and usage fields remain unchanged, and the input value is not mutated.

- [ ] **Step 2: Run the focused tests and confirm RED**

Run:

```bash
cargo test -p server_harness middleware::types::tests::credential_sanitizer -- --nocapture
```

Expected: compilation failure because `sanitize_credential_value` and `sanitize_credential_map` do not exist.

- [ ] **Step 3: Add the shared sanitizer**

Expose credential-only helpers inside the harness crate:

```rust
pub(crate) fn sanitize_credential_value(value: &Value) -> Value {
    match value {
        Value::String(text) if sensitive_json_text(text) => {
            Value::String("[REDACTED]".to_owned())
        }
        Value::Array(values) => {
            Value::Array(values.iter().map(sanitize_credential_value).collect())
        }
        Value::Object(values) => Value::Object(
            values
                .iter()
                .filter(|(key, _)| !sensitive_json_key(key))
                .map(|(key, value)| (key.clone(), sanitize_credential_value(value)))
                .collect(),
        ),
        other => other.clone(),
    }
}

pub(crate) fn sanitize_credential_map(metadata: &JsonMap) -> JsonMap {
    metadata
        .iter()
        .filter(|(key, _)| !sensitive_json_key(key))
        .map(|(key, value)| (key.clone(), sanitize_credential_value(value)))
        .collect()
}
```

Route resolved-model metadata serialization through these helpers so one key classifier and value policy controls both paths.

- [ ] **Step 4: Run focused and type tests and confirm GREEN**

```bash
cargo test -p server_harness middleware::types -- --nocapture
```

Expected: all type tests pass.

### Task 2: Capsule map and annotation redaction

**Files:**
- Modify: `src/server/harness/middleware/capsule.rs`
- Test: `src/server/harness/middleware/capsule.rs`

- [ ] **Step 1: Write a failing all-map opaque-canary test**

Create a capsule source containing `("api_key", "CANARY-OPAQUE-7KQ9")` in session tags, tool-result metadata, attempt metadata, artifact and artifact-part metadata, model metadata, checkpoint usage, and `ContentAnnotation.data`. Compile the capsule and assert its serialized JSON contains neither `api_key` nor `CANARY-OPAQUE-7KQ9`, while `loss_report.entries` contains each rejected source path.

- [ ] **Step 2: Run the focused test and confirm RED**

```bash
cargo test -p server_harness middleware::capsule::tests::opaque_sensitive_keys_are_removed_from_every_portable_map -- --nocapture
```

Expected: failure showing the synthetic canary in the capsule.

- [ ] **Step 3: Make map wrappers key-aware and sanitize annotations**

Change `redact_string_map` and `redact_json_map` to use `filter_map`. For every `non_portable_json_key(key)`, append the same portable-redaction `LossEntry` used by recursive objects and omit the entry; otherwise redact the value recursively. Replace cloned text/reasoning annotations with projected annotations whose `data` uses `redact_json_map` and whose labels/text fields pass through `redact_text` where applicable.

- [ ] **Step 4: Run capsule tests and confirm GREEN**

```bash
cargo test -p server_harness middleware::capsule -- --nocapture
```

Expected: all capsule tests pass, including integrity/digest tests.

### Task 3: Native-event sanitization and final event admission

**Files:**
- Modify: `src/server/harness/middleware/codex_app_server.rs`
- Modify: `src/server/harness/middleware/grpc.rs`
- Test: `src/server/harness/tests/codex_app_server.rs`
- Test: `src/server/harness/tests/worker_grpc.rs`

- [ ] **Step 1: Write failing codec and replay tests**

Decode a durable Codex `warning` notification containing nested `api_key: CANARY-NOTIFY-7KQ9` and a bearer string. Assert neither survives in the event payload. In the worker gRPC test, inject an adapter event with the same payload, consume the initial delivery and a reconcile replay, and assert both are credential-safe while ordinary payload fields remain.

- [ ] **Step 2: Run focused tests and confirm RED**

```bash
cargo test -p server_harness --test codex_app_server notification_credentials_are_sanitized -- --nocapture
cargo test -p server_harness --test worker_grpc durable_event_admission_sanitizes_initial_and_replayed_payloads -- --nocapture
```

Expected: both tests fail because raw notification/event payloads are retained.

- [ ] **Step 3: Enforce both boundaries**

Replace Codex's private sanitizer with `types::sanitize_credential_value`, construct `native` from sanitized params, and sanitize item/detail/diff/plan payload fragments. In `HarnessWorkerGrpcService::append_attempt_event`, sanitize `event.payload` immediately before serializing and storing the delivery envelope:

```rust
let payload = sanitize_credential_value(&event.payload);
let envelope_payload = serde_json::to_vec(&json!({
    "event_type": event.event_type,
    "payload": payload,
    "native_cursor": event.native_cursor,
}))?;
```

Keep this credential-only so safe `payload`, `name`, usage, and model fields are not removed.

- [ ] **Step 4: Run protocol and gRPC tests and confirm GREEN**

```bash
cargo test -p server_harness --test codex_app_server -- --nocapture
cargo test -p server_harness --test worker_grpc -- --nocapture
```

Expected: all Codex and worker gRPC tests pass.

### Task 4: Child-process environment isolation

**Files:**
- Create: `src/server/harness/middleware/process_env.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Modify: `src/server/harness/middleware/json_rpc.rs`
- Modify: `src/server/harness/middleware/pi_rpc.rs`
- Modify: `src/server/harness/middleware/http_runtime.rs`
- Modify: `src/server/harness/middleware/harness.rs`
- Test: `src/server/harness/tests/json_rpc_runtime.rs`
- Test: `src/server/harness/tests/pi_rpc.rs`
- Test: `src/server/harness/tests/opencode_http.rs`
- Test: `src/server/harness/tests/external_adapters.rs`

- [ ] **Step 1: Write failing ambient-canary tests**

For generic, JSON-RPC, Pi, and HTTP runtimes, spawn a fixture from a parent test process containing `UNRELATED_DEPLOYMENT_SECRET=CANARY-AMBIENT-7KQ9`. Have the fixture report only boolean presence. Assert the ambient key is absent while an explicitly configured `OPENAI_API_KEY` and required `PATH` behavior remain available. Serialize process-spawning tests that mutate the test environment.

- [ ] **Step 2: Run focused tests and confirm RED**

```bash
cargo test -p server_harness --test json_rpc_runtime ambient_parent_environment_is_not_inherited -- --nocapture
cargo test -p server_harness --test pi_rpc ambient_parent_environment_is_not_inherited -- --nocapture
cargo test -p server_harness --test opencode_http ambient_parent_environment_is_not_inherited -- --nocapture
cargo test -p server_harness --test external_adapters ambient_parent_environment_is_not_inherited -- --nocapture
```

Expected: fixtures report the ambient key as present.

- [ ] **Step 3: Add and use one process environment helper**

Implement:

```rust
pub(crate) fn apply_isolated_environment<I, K, V>(
    command: &mut tokio::process::Command,
    explicit: I,
) where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    command.env_clear();
    for key in ["PATH", "LANG", "LC_ALL", "TMPDIR", "TEMP", "TMP", "SystemRoot", "WINDIR", "PATHEXT"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command.envs(explicit);
}
```

Call it in generic/custom, JSON-RPC, Pi, and HTTP spawn paths before `spawn()`. Do not change OpenHarness, which already uses `env_clear` and an isolated home.

- [ ] **Step 4: Run all adapter tests and confirm GREEN**

```bash
cargo test -p server_harness --test json_rpc_runtime -- --nocapture
cargo test -p server_harness --test pi_rpc -- --nocapture
cargo test -p server_harness --test opencode_http -- --nocapture
cargo test -p server_harness --test external_adapters -- --nocapture
```

Expected: all tests pass and ambient canaries remain absent.

### Task 5: Security-slice regression gate

**Files:**
- Modify only if required by failures: files from Tasks 1-4

- [ ] **Step 1: Run the complete harness package**

```bash
cargo test -p server_harness --all-targets
cargo test -p omnisolo_harness_worker --all-targets
```

Expected: all deterministic tests pass; provider-dependent tests remain explicitly ignored.

- [ ] **Step 2: Run formatting, lint baseline, and credential scan**

```bash
cargo fmt --all -- --check
git diff --check
rg -n 'CANARY-(OPAQUE|NOTIFY|AMBIENT)-7KQ9' src/server/harness src/server/harness_worker
```

Expected: formatting and diff checks pass; canaries appear only in tests.

- [ ] **Step 3: Record exact coverage honestly**

```bash
cargo llvm-cov -p server_harness --all-targets --json --output-path /tmp/omnisolo-server-harness-security-coverage.json
```

Expected: a reproducible report; do not claim 100% unless totals are exactly 100%.
