# Production Backend and Agent Optimization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish reproducible production-agent baselines and implement confirmed security, performance, and token-efficiency fixes without reducing deterministic response quality.

**Architecture:** Follow a deterministic agent request through prompt construction, provider invocation, tool execution, authentication, and tenant boundaries. Add small reusable policy modules for request profiling, outbound-network validation, workspace confinement, and circuit breaking; then remove duplicated or unsafe behavior at existing call sites. Record comparable baseline and optimized evidence in a checked-in report.

**Tech Stack:** Rust 2024, Tokio, Axum/Tonic, Reqwest, SQLx, Serde, Bazel/rules_rust, Cargo workspace tests, repository lint scripts.

---

## File structure

- Create `src/agents/builtin/request_profile.rs`: deterministic request-size and token-source accounting with no provider dependency.
- Create `src/agents/builtin/tests/production_agent_path.rs`: quality fixture for a complete fake-provider agent turn.
- Create `src/agents/builtin/tools/network_policy.rs`: scheme, host, resolved-address, redirect, and response-size policy shared by outbound tools.
- Create `src/agents/builtin/tools/workspace_path.rs`: canonical workspace confinement shared by file tools.
- Create `src/agents/builtin/llm/circuit_breaker.rs`: per-client circuit-breaker state and decisions.
- Create `src/agents/builtin/examples/agent_path_baseline.rs`: reproducible JSON benchmark runner.
- Create `docs/reports/production_agent_optimization_report.md`: commands, confirmed findings, before/after measurements, and remaining risks.
- Modify `src/agents/builtin/prompt_construction.rs`: stop serializing native tool schemas into the system prompt and expose component sizes.
- Modify `src/agents/builtin/agent.rs`: attach request profiles to existing LLM spans and preserve quality assertions.
- Modify `src/agents/builtin/llm/openai.rs`: remove the truncated application response cache and use per-client circuit breaking.
- Modify `src/agents/builtin/llm/{anthropic,gemini,ollama}.rs`: use per-client circuit breaking.
- Modify `src/agents/builtin/tools/{webfetch,agent_protocol,read,write}.rs`: enforce the shared network and workspace policies and bound reads.
- Modify `src/agents/builtin/{auth,lib}.rs` and `src/server/lib.rs`: reject incomplete authentication configuration.
- Modify Bazel and Cargo manifests adjacent to those files so both build systems compile the same sources.

### Task 1: Freeze the current baseline and quality contract

**Files:**
- Create: `src/agents/builtin/request_profile.rs`
- Create: `src/agents/builtin/tests/production_agent_path.rs`
- Modify: `src/agents/builtin/lib.rs`
- Modify: `src/agents/builtin/BUILD.bazel`

- [ ] **Step 1: Run the current focused suites and preserve their output**

Run:

```bash
cargo test -p ohc_builtin_agent_core -p ohc_builtin_agent_llm -p ohc_builtin_agent_tools -p ohc_builtin_agent --lib
```

Expected: all currently passing tests pass. Record existing failures verbatim under `Baseline caveats` in the report created in Task 9; do not classify a pre-existing failure as a regression.

- [ ] **Step 2: Write failing request-profile tests**

Create `src/agents/builtin/request_profile.rs` with the test module first:

```rust
use crate::types::{ChatRequest, Role};

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct RequestProfile {
    pub system_chars: usize,
    pub history_chars: usize,
    pub tool_result_chars: usize,
    pub tool_schema_chars: usize,
    pub message_count: usize,
    pub tool_count: usize,
    pub estimated_input_tokens: usize,
}

pub fn profile_request(req: &ChatRequest) -> RequestProfile {
    let history_chars = req.messages.iter().map(|m| m.content.chars().count()).sum();
    let tool_result_chars = req.messages.iter().flat_map(|m| &m.tool_results)
        .map(|r| r.content.chars().count() + r.error.chars().count()).sum();
    let tool_schema_chars = req.tools.iter().map(|t| {
        t.name.chars().count() + t.description.chars().count() + t.parameters.to_string().chars().count()
    }).sum();
    let total_chars = req.system.chars().count() + history_chars + tool_result_chars + tool_schema_chars;
    RequestProfile {
        system_chars: req.system.chars().count(),
        history_chars,
        tool_result_chars,
        tool_schema_chars,
        message_count: req.messages.len(),
        tool_count: req.tools.len(),
        estimated_input_tokens: total_chars.div_ceil(4),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, ToolDefinition, ToolResult};

    #[test]
    fn attributes_request_content_without_storing_content() {
        let req = ChatRequest {
            model: "fixture".into(),
            system: "system".into(),
            messages: vec![Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult { tool_call_id: "1".into(), content: "result".into(), error: String::new() }],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![ToolDefinition { name: "Lookup".into(), description: "Lookup facts".into(), parameters: serde_json::json!({"type":"object"}) }],
            max_tokens: 128,
            temperature: 0.0,
        };
        let profile = profile_request(&req);
        assert_eq!(profile.system_chars, 6);
        assert_eq!(profile.tool_result_chars, 6);
        assert_eq!(profile.message_count, 1);
        assert_eq!(profile.tool_count, 1);
        assert!(profile.estimated_input_tokens > 0);
        assert!(!serde_json::to_string(&profile).unwrap().contains("Lookup facts"));
    }
}
```

- [ ] **Step 3: Verify the new module is not yet wired**

Run:

```bash
cargo test -p ohc_builtin_agent_core request_profile
```

Expected: FAIL because `request_profile` is not exported by the core crate.

- [ ] **Step 4: Export the profile module in both build systems**

Add to `src/agents/builtin/core.rs`:

```rust
pub mod request_profile;
```

Add `"request_profile.rs",` to the `core` target's `srcs` in `src/agents/builtin/BUILD.bazel`.

- [ ] **Step 5: Add a deterministic full-turn quality fixture**

Create `src/agents/builtin/tests/production_agent_path.rs` containing a fake `LlmClient` that records each `ChatRequest`, returns `ChatResponse { message: Message::assistant("The verified answer is 42."), usage: Usage { input_tokens: 120, output_tokens: 8, ..Default::default() }, stop_reason: "stop".into(), response_id: Some("fixture-response".into()) }`, runs `Agent::run_tao_orchestration_loop` with `AgentRunConfig { max_iterations: 1, enable_lost_in_the_middle_prevention: false, ..Default::default() }`, and asserts:

```rust
assert_eq!(answer, "The verified answer is 42.");
assert_eq!(requests.lock().await.len(), 1);
assert_eq!(requests.lock().await[0].messages.last().unwrap().content, "What is six times seven?");
```

Register this integration test using the existing `rust_test` conventions in `src/agents/builtin/BUILD.bazel`.

- [ ] **Step 6: Run the quality contract**

Run:

```bash
cargo test -p ohc_builtin_agent request_profile
cargo test -p ohc_builtin_agent --test production_agent_path
```

Expected: PASS; the fixture makes no external network or database calls.

- [ ] **Step 7: Commit the baseline contract**

```bash
git add src/agents/builtin/request_profile.rs src/agents/builtin/tests/production_agent_path.rs src/agents/builtin/core.rs src/agents/builtin/BUILD.bazel
git commit -m "test: establish production agent quality baseline"
```

### Task 2: Remove duplicated native tool schemas from system prompts

**Files:**
- Modify: `src/agents/builtin/prompt_construction.rs`
- Modify: `src/agents/builtin/agent.rs`
- Test: `src/agents/builtin/prompt_construction.rs`
- Test: `src/agents/builtin/tests/production_agent_path.rs`

- [ ] **Step 1: Add failing duplication and quality tests**

Add to the existing prompt-construction tests:

```rust
struct NeverExecutor;

#[async_trait::async_trait]
impl crate::tools::ToolExecutor for NeverExecutor {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, crate::types::ToolError> {
        Err(crate::types::ToolError::Unexpected("not executed".into()))
    }
}

#[test]
fn native_tool_schema_is_not_duplicated_in_system_text() {
    let cfg = AgentRunConfig { server_system_message: "Be accurate".into(), ..Default::default() };
    let tool = crate::tools::Tool {
        name: "Lookup".into(),
        description: "Lookup authoritative facts".into(),
        is_read_only: true,
        parameters: serde_json::json!({"type":"object","properties":{"id":{"type":"string"}}}),
        execute: std::sync::Arc::new(NeverExecutor),
    };
    let prompt = StrictHierarchicalPromptBuilder::new(&cfg, &[tool], None, None).build();
    assert!(!prompt.contains("<tool_definitions>"));
    assert!(!prompt.contains("Lookup authoritative facts"));
}
```

Extend the full-turn fixture to include the same `Lookup` tool and assert `requests[0].tools[0].name == "Lookup"`. This proves native schemas remain available to the model.

- [ ] **Step 2: Run the focused tests to verify the duplication test fails**

Run:

```bash
cargo test -p ohc_builtin_agent native_tool_schema_is_not_duplicated_in_system_text
```

Expected: FAIL because the current builder emits `<tool_definitions>`.

- [ ] **Step 3: Remove the duplicated representation**

In `StrictHierarchicalPromptBuilder`, remove the `tool_definitions: String` field, its construction loop, capacity contribution, and `<tool_definitions>` block. Keep the `tools` parameter as `_tools: &[crate::tools::Tool]` for this commit so call sites do not churn; add a doc comment that native provider schemas are the sole tool-definition representation.

Update exact-string prompt tests to omit the `<tool_definitions>` section. Do not change `ChatRequest.tools` construction in any agent loop.

- [ ] **Step 4: Record request profiles on the existing LLM span**

Immediately before `self.llm.chat(req)` in the primary TAO loop, compute:

```rust
let request_profile = crate::request_profile::profile_request(&req);
llm_span.record("estimated_input_tokens", request_profile.estimated_input_tokens as i64);
llm_span.record("system_chars", request_profile.system_chars as i64);
llm_span.record("history_chars", request_profile.history_chars as i64);
llm_span.record("tool_schema_chars", request_profile.tool_schema_chars as i64);
```

Declare those fields as `tracing::field::Empty` when the span is created. Do not record prompt or tool content.

- [ ] **Step 5: Verify quality and quantify the reduction**

Run:

```bash
cargo test -p ohc_builtin_agent prompt_construction
cargo test -p ohc_builtin_agent --test production_agent_path
```

Expected: PASS. Capture the old and new `system_chars` and `estimated_input_tokens` from the deterministic fixture for Task 9.

- [ ] **Step 6: Commit**

```bash
git add src/agents/builtin/prompt_construction.rs src/agents/builtin/agent.rs src/agents/builtin/tests/production_agent_path.rs
git commit -m "perf: avoid duplicating native tool schemas"
```

### Task 3: Remove the unsafe truncated OpenAI response cache

**Files:**
- Modify: `src/agents/builtin/llm/openai.rs`
- Modify: `src/agents/builtin/llm/Cargo.toml`
- Modify: `src/agents/builtin/llm/BUILD.bazel`
- Test: `src/agents/builtin/llm/openai.rs`

- [ ] **Step 1: Add a failing regression test using the existing mock HTTP test pattern**

Add a test that starts a local one-shot HTTP server with two sequential responses, builds `OpenAIClient::with_base_url("test-key", server_url)`, sends two requests whose serialized prefixes are identical but whose final user content differs, and asserts:

```rust
assert_eq!(first.message.content, "first response");
assert_eq!(second.message.content, "second response");
assert_eq!(request_count.load(std::sync::atomic::Ordering::SeqCst), 2);
```

Make the shared prefix exceed 8,000 characters so the current `truncate_context(..., 2000)` cache key collides. Give the second server response one tool call and assert that tool call is preserved.

- [ ] **Step 2: Verify the regression fails on the application cache**

Run:

```bash
cargo test -p ohc_builtin_agent_llm truncated_prompt_prefixes_do_not_share_responses -- --nocapture
```

Expected: FAIL because the second request returns the first cached text and does not reach the server.

- [ ] **Step 3: Remove application-level response caching**

Delete `PromptCache` imports, the `cache` field, its initialization, `optimized_prompt`, the cache lookup, and the cache write from `OpenAIClient`. Retain provider-native prompt caching and usage fields. Remove `server_pricing` from the LLM crate's Cargo and Bazel dependencies only if no remaining LLM source imports it.

- [ ] **Step 4: Run provider and quality tests**

Run:

```bash
cargo test -p ohc_builtin_agent_llm
cargo test -p ohc_builtin_agent --test production_agent_path
```

Expected: PASS; two distinct requests always receive distinct provider responses, including tool calls.

- [ ] **Step 5: Commit**

```bash
git add src/agents/builtin/llm/openai.rs src/agents/builtin/llm/Cargo.toml src/agents/builtin/llm/BUILD.bazel
git commit -m "fix: remove unsafe truncated LLM response cache"
```

### Task 4: Enforce outbound-network policy and bounded WebFetch bodies

**Files:**
- Create: `src/agents/builtin/tools/network_policy.rs`
- Modify: `src/agents/builtin/tools/mod.rs`
- Modify: `src/agents/builtin/tools/BUILD.bazel`
- Modify: `src/agents/builtin/tools/webfetch.rs`
- Modify: `src/agents/builtin/tools/webfetch_test.rs`
- Modify: `src/agents/builtin/tools/agent_protocol.rs`

- [ ] **Step 1: Write failing network-policy tests**

Define `validate_url(url: &url::Url, allow_private: bool) -> Result<(), ToolError>` and tests that reject `file:///etc/passwd`, `http://localhost`, loopback, unspecified, link-local, private IPv4, IPv6 loopback, unique-local IPv6, and IPv4-mapped IPv6 private addresses. Accept `https://example.com/path`. The implementation must use `IpAddr` predicates rather than string prefixes:

```rust
fn blocked_ip(ip: std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local()
            || v4.is_unspecified() || v4.is_broadcast() || v4.is_multicast(),
        std::net::IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified()
            || v6.is_unique_local() || v6.is_unicast_link_local() || v6.is_multicast()
            || v6.to_ipv4_mapped().is_some_and(|v4| blocked_ip(v4.into())),
    }
}
```

- [ ] **Step 2: Verify policy tests fail before wiring**

Run:

```bash
cargo test -p ohc_builtin_agent_tools network_policy
```

Expected: FAIL because the module is not exported.

- [ ] **Step 3: Implement initial and resolved-address validation**

Parse only `http` and `https`; reject credentials in URLs; reject blocked literal IPs and case-insensitive `localhost`. Before connecting, call `tokio::net::lookup_host((host, port))`, reject an empty result, and reject the request if any resolved address is blocked. Use an explicit `OHC_AGENT_ALLOW_PRIVATE_NETWORK=true` override, defaulting to false, and emit a warning when enabled.

Export the module from `tools/mod.rs` and add it to Cargo/Bazel sources and dependencies.

- [ ] **Step 4: Rewrite WebFetch to cap bytes before allocation and validate redirects**

Build the client with `redirect(reqwest::redirect::Policy::none())`. For each hop, validate the URL and resolved addresses, accept at most five redirects, and resolve relative `Location` headers with `Url::join`. Read the body through `Response::chunk()` into a `Vec<u8>` and stop at `1_048_576` bytes:

```rust
while let Some(chunk) = response.chunk().await.map_err(network_error)? {
    if body.len() + chunk.len() > MAX_RESPONSE_BYTES {
        return Err(ToolError::LlmRecoverable("webfetch: response exceeds 1 MiB".into()));
    }
    body.extend_from_slice(&chunk);
}
```

Use `String::from_utf8_lossy`, and truncate displayed output on a character boundary with `text.char_indices().nth(10_000)` rather than `&text[..10_000]`.

- [ ] **Step 5: Reuse the same policy in Agent Protocol**

Replace its string-prefix `is_safe_url` with `network_policy::validate_and_resolve`. Disable automatic redirects on its client. Keep the existing dangerous local-network override as a backward-compatible alias for the new override and add a deprecation warning.

- [ ] **Step 6: Add HTTP regression tests**

Test private-address rejection without sending a request, oversized chunked-response rejection, redirect-to-loopback rejection, five-hop redirect enforcement, and a multibyte body longer than 10,000 characters without panic.

- [ ] **Step 7: Run and commit**

```bash
cargo test -p ohc_builtin_agent_tools webfetch
cargo test -p ohc_builtin_agent_tools agent_protocol
git add src/agents/builtin/tools/network_policy.rs src/agents/builtin/tools/mod.rs src/agents/builtin/tools/BUILD.bazel src/agents/builtin/tools/webfetch.rs src/agents/builtin/tools/webfetch_test.rs src/agents/builtin/tools/agent_protocol.rs
git commit -m "security: constrain outbound agent requests"
```

### Task 5: Confine file tools to the configured workspace and eliminate double reads

**Files:**
- Create: `src/agents/builtin/tools/workspace_path.rs`
- Modify: `src/agents/builtin/tools/mod.rs`
- Modify: `src/agents/builtin/tools/BUILD.bazel`
- Modify: `src/agents/builtin/tools/read.rs`
- Modify: `src/agents/builtin/tools/write.rs`

- [ ] **Step 1: Write failing path-policy tests**

Cover direct `..`, absolute paths, a symlink inside the workspace pointing outside it, a write through a symlinked parent, and a normal nested path. The public API is:

```rust
pub async fn existing(root: &std::path::Path, requested: &str) -> Result<std::path::PathBuf, ToolError>;
pub async fn for_write(root: &std::path::Path, requested: &str) -> Result<std::path::PathBuf, ToolError>;
```

Both functions reject `RootDir`, `ParentDir`, and platform prefixes by inspecting `Path::components`. `existing` canonicalizes root and target and requires `target.starts_with(root)`. `for_write` canonicalizes the root and nearest existing parent before directory creation, then requires that parent to stay within root.

- [ ] **Step 2: Verify tests fail, then export the module**

Run `cargo test -p ohc_builtin_agent_tools workspace_path`; expect FAIL before adding `pub mod workspace_path;` and the Bazel source entry.

- [ ] **Step 3: Require an explicit workspace root in production file tools**

At tool construction, canonicalize the supplied working directory. When it is absent, use the current directory once and store that root; do not interpret absolute user paths as unrestricted host paths. Route every Read and Write request through the shared policy.

- [ ] **Step 4: Make Read single-pass and byte-bounded**

Remove the preliminary full-file line count. In one `BufReader` pass, collect only the requested range, stop at 1,000 selected lines or 1 MiB of selected bytes, and return a pagination error as soon as an un-ranged read sees line 1,001. Preserve UTF-8 safely through `read_line`.

- [ ] **Step 5: Make Write bounded and atomic**

Reject content larger than 4 MiB. Create a sibling path with `format!(".{}.{}.tmp", file_name, uuid::Uuid::new_v4())`, open it with `tokio::fs::OpenOptions::new().write(true).create_new(true)`, write and flush the content, run optional Rust verification against that path, and rename it to the target within the same directory. Remove the temporary file on every error path. Reject an existing symlink at the destination before rename.

- [ ] **Step 6: Run security and behavior tests**

Run:

```bash
cargo test -p ohc_builtin_agent_tools read
cargo test -p ohc_builtin_agent_tools write
cargo test -p ohc_builtin_agent_tools workspace_path
```

Expected: PASS, including traversal and symlink regressions.

- [ ] **Step 7: Commit**

```bash
git add src/agents/builtin/tools/workspace_path.rs src/agents/builtin/tools/mod.rs src/agents/builtin/tools/BUILD.bazel src/agents/builtin/tools/read.rs src/agents/builtin/tools/write.rs
git commit -m "security: confine agent file tools to workspace"
```

### Task 6: Fail closed on incomplete agent authentication

**Files:**
- Modify: `src/agents/builtin/auth.rs`
- Modify: `src/agents/builtin/lib.rs`
- Modify: `src/server/lib.rs`
- Test: `src/agents/builtin/auth.rs`

- [ ] **Step 1: Add serialized environment tests**

Use `temp_env` and a test mutex to assert:

```rust
assert!(auth_mode_from_env().is_err()); // no token, no SPIFFE ID
assert!(auth_mode_from_env().is_err()); // token present, auth key absent
assert!(matches!(auth_mode_from_env().unwrap(), AuthMode::Token { .. }));
assert!(matches!(auth_mode_from_env().unwrap(), AuthMode::Spiffe { .. }));
```

The disabled mode is accepted only when `OHC_AGENT_AUTH_DISABLED=true` and `OHC_ENV` is `development` or `test`; it is rejected when `OHC_ENV=production`.

- [ ] **Step 2: Verify current behavior fails the new expectations**

Run:

```bash
cargo test -p ohc_builtin_agent_core auth_mode_requires_complete_configuration
```

Expected: FAIL because the current implementation uses `default_auth_key_change_me` and permits an empty SPIFFE ID.

- [ ] **Step 3: Return configuration errors instead of insecure defaults**

Change the signature to:

```rust
pub fn auth_mode_from_env() -> Result<AuthMode, String>
```

Require a non-empty `OHC_AGENT_AUTH_KEY` of at least 32 bytes in token mode. Require a valid non-empty `OHC_AGENT_SPIFFE_ID` in SPIFFE mode. Replace the fallback key in `hmac_token` with a key parameter so hashing cannot silently select configuration:

```rust
pub fn hmac_token(tok: &str, key: &[u8]) -> Vec<u8>
```

Use `Mac::verify_slice` for comparison. Update `run_agent` and server startup call sites to propagate the configuration error and abort startup.

- [ ] **Step 4: Run auth and startup-related tests**

Run:

```bash
cargo test -p ohc_builtin_agent_core auth
cargo test -p ohc_builtin_agent auth
cargo test --lib server_lib -- auth
```

Expected: PASS; production startup cannot select disabled auth or a default key.

- [ ] **Step 5: Commit**

```bash
git add src/agents/builtin/auth.rs src/agents/builtin/lib.rs src/server/lib.rs
git commit -m "security: fail closed on agent auth configuration"
```

### Task 7: Replace provider-global circuit breakers with per-client state

**Files:**
- Create: `src/agents/builtin/llm/circuit_breaker.rs`
- Modify: `src/agents/builtin/llm/mod.rs`
- Modify: `src/agents/builtin/llm/{openai,anthropic,gemini,ollama}.rs`
- Modify: `src/agents/builtin/llm/BUILD.bazel`

- [ ] **Step 1: Write deterministic circuit-breaker tests with paused Tokio time**

Implement and test this interface:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState { Closed, Open, HalfOpen }

pub struct CircuitBreaker {
    failures: std::sync::atomic::AtomicUsize,
    opened_at: std::sync::Mutex<Option<std::time::Instant>>,
    max_failures: usize,
    reset_timeout: std::time::Duration,
    probe_in_flight: std::sync::atomic::AtomicBool,
}

impl CircuitBreaker {
    pub fn new(max_failures: usize, reset_timeout: std::time::Duration) -> Self;
    pub fn allow(&self) -> bool;
    pub fn record_success(&self);
    pub fn record_failure(&self);
    pub fn state(&self) -> CircuitState;
}
```

Assert one half-open probe, reset on success, reopen on probe failure, and independence between two instances.

- [ ] **Step 2: Verify tests fail before export**

Run `cargo test -p ohc_builtin_agent_llm circuit_breaker`; expect FAIL because the shared module does not exist.

- [ ] **Step 3: Add one breaker to each client instance**

Add `circuit_breaker: CircuitBreaker` to each provider client and initialize it in constructors. Delete every `GLOBAL_CIRCUIT_BREAKER`, `OnceLock`, and duplicated breaker implementation. Provider A, endpoint A, or tenant-specific client failures must not open provider B or another client instance.

- [ ] **Step 4: Classify failures consistently**

Record breaker failures for transport failures, timeouts, HTTP 429, and HTTP 5xx. Do not open the breaker for authentication errors, invalid requests, or response parsing bugs. Record success only after a valid response is decoded. Preserve sanitized user errors and status codes.

- [ ] **Step 5: Run provider tests and commit**

```bash
cargo test -p ohc_builtin_agent_llm
git add src/agents/builtin/llm/circuit_breaker.rs src/agents/builtin/llm/mod.rs src/agents/builtin/llm/openai.rs src/agents/builtin/llm/anthropic.rs src/agents/builtin/llm/gemini.rs src/agents/builtin/llm/ollama.rs src/agents/builtin/llm/BUILD.bazel
git commit -m "perf: isolate LLM provider circuit breakers"
```

### Task 8: Audit authentication, tenant isolation, cancellation, and telemetry end to end

**Files:**
- Inspect: `src/server/api/agents/chat.rs`
- Inspect: `src/server/services/agent/service.rs`
- Inspect: `src/server/workers/agent_memory_pipeline.rs`
- Test: `src/server/auth/multitenancy_isolation.rs`
- Test: `src/server/services/agent/service.rs`
- Create: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Run targeted static searches and record each reviewed sink**

Run:

```bash
rg -n 'query\(|query_as\(|execute\(|fetch_|set_org_context|organization_id|tenant_id|spawn\(|timeout\(|tracing::(debug|info|warn|error)!' \
  src/server/api/agents src/server/services/agent src/server/workers/agent_memory_pipeline.rs \
  src/agents/builtin/{agent.rs,auth.rs,observability.rs,tools,llm}
```

For each production path in the design, record source, trust boundary, authorization source, tenant context, timeout/cancellation behavior, logged fields, and conclusion in the report. A finding is confirmed only with a reproducing test or a direct invariant violation visible at the sink.

- [ ] **Step 2: Exercise existing tenant-isolation regressions**

Run:

```bash
cargo test --lib server_lib -- multitenancy_isolation --nocapture
cargo test --lib server_lib -- agent_memory_pipeline --nocapture
cargo test --lib server_lib -- services::agent --nocapture
cargo test --lib server_lib -- orchestration::queue --nocapture
cargo test -p ohc_builtin_agent service -- --nocapture
```

Expected: PASS or explicit environment skips. Record skips as unverified risks, not passes. In the service tests, verify `RunTaskStream` uses the bounded channel and that receiver cancellation terminates producer work; in queue tests, record enqueue/dequeue counts and tenant identifiers for the deterministic fixtures.

- [ ] **Step 3: Classify backend boundary evidence**

For each reviewed boundary, classify the result as protected by a named passing test, confirmed by a direct invariant violation, or unverified because an external service is unavailable. Record file and line references, preconditions, severity, and the smallest proposed regression name. Additional confirmed defects become focused follow-up remediation plans; they are not patched opportunistically in this audit task.

- [ ] **Step 4: Cross-check the traced path against existing controls**

Confirm that every traced database access either uses `set_org_context` in its transaction or binds and filters the authenticated tenant explicitly; every agent entry point rejects a missing tenant in cloud mode; every provider/tool call has a deadline; streaming uses bounded channels and stops work after receiver cancellation; and telemetry fields exclude prompts, credentials, tool results, and tenant data. Record exact gaps in the findings table for focused follow-up work.

- [ ] **Step 5: Scan dependencies and tracked secret candidates**

Run:

```bash
cargo audit
pnpm audit --prod
git ls-files | rg '(secret|token|password|credential|private[_-]?key|\.pem$|\.key$)'
rg -n --hidden -g '!site/**' -g '!docs/**' -g '!target/**' '(BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|default_auth_key_change_me|postgres://[^[:space:]]+:[^[:space:]@]+@)'
```

If `cargo audit` is unavailable, record that exact tooling gap and run `cargo tree -d` plus the repository's lockfile checks. Validate every candidate before assigning severity; do not copy secret contents into the report.

- [ ] **Step 6: Commit the completed audit matrix and initial findings table**

Stage only the report, then commit:

```bash
git add docs/reports/production_agent_optimization_report.md
git commit -m "security: harden production agent boundaries"
```

### Task 9: Add reproducible before/after benchmark output and final verification

**Files:**
- Create: `src/agents/builtin/examples/agent_path_baseline.rs`
- Modify: `src/agents/builtin/Cargo.toml`
- Modify: `src/agents/builtin/BUILD.bazel`
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Create the benchmark runner**

Reuse the deterministic fake provider and request fixture from Task 1. Execute 20 warm-up turns and 200 measured turns under a Tokio current-thread runtime. Serialize only aggregate data:

```rust
#[derive(serde::Serialize)]
struct BenchmarkResult {
    schema_version: u8,
    iterations: usize,
    median_micros: u128,
    p95_micros: u128,
    request_profile: ohc_builtin_agent_core::request_profile::RequestProfile,
    llm_calls_per_turn: f64,
    quality_passed: bool,
}
```

Sort durations before selecting median and p95. Assert the answer and tool schema invariants on every measured turn. Print one JSON object to stdout so results can be diffed without scraping logs.

- [ ] **Step 2: Run comparable release measurements**

Run on the baseline commit and optimized HEAD with the same machine and environment:

```bash
cargo run --release -p ohc_builtin_agent --example agent_path_baseline > /tmp/agent-baseline.json
cargo run --release -p ohc_builtin_agent --example agent_path_baseline > /tmp/agent-optimized.json
```

Expected: both JSON files report `quality_passed: true` and identical expected response behavior. Include commit hashes, compiler version, CPU description, and both JSON objects in the report. Do not claim latency improvement when run-to-run variance overlaps; the tool-schema character/token reduction is deterministic and may be reported exactly.

- [ ] **Step 3: Run formatting, lint, tests, and build-system parity**

Run:

```bash
cargo fmt --all -- --check
cargo clippy -p ohc_builtin_agent_core -p ohc_builtin_agent_llm -p ohc_builtin_agent_tools -p ohc_builtin_agent --all-targets -- -D warnings
cargo test -p ohc_builtin_agent_core -p ohc_builtin_agent_llm -p ohc_builtin_agent_tools -p ohc_builtin_agent --all-targets
cargo test --lib server_lib -- multitenancy_isolation
bazel test //src/agents/builtin:all //src/agents/builtin/llm:all //src/agents/builtin/tools:all
git diff --check
```

Expected: PASS. If an unrelated pre-existing failure remains, include its command and output summary under `Remaining risks` and show that the focused changed-module tests pass.

- [ ] **Step 4: Complete the report**

The report must contain:

- Scope and exact commits reviewed.
- Baseline environment and commands.
- Confirmed findings with severity, affected paths, evidence, and remediation commit.
- Before/after latency, request profile, token attribution, tool-call count, and quality results.
- Dependency/configuration scan results without secret values.
- Tests and build targets executed.
- Skipped or unverified paths and operational rollout notes.

- [ ] **Step 5: Final review and commit**

Search the report for unfinished markers or ambiguous empty sections and expect no matches. Review `git diff --stat` and `git status --short`, then commit:

```bash
git add src/agents/builtin/examples/agent_path_baseline.rs src/agents/builtin/Cargo.toml src/agents/builtin/BUILD.bazel docs/reports/production_agent_optimization_report.md
git commit -m "docs: report production agent optimization results"
```
