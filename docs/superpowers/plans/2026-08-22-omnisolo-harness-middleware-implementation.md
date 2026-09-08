# OmniSolo Harness Middleware Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the first production-capable OmniSolo harness middleware foundation: versioned canonical session/task state, fenced event ingestion, portable capsules, model-runtime admission, and worker envelopes with full deterministic test coverage.

**Architecture:** Add a focused `server_harness::middleware` module that owns protocol-neutral data contracts and pure state machines. The existing OmniSolo agent and future adapters consume these contracts; no new Codex-specific runner is created. Durable database and external harness adapters are integrated only after the pure contracts prove their invariants.

**Tech Stack:** Rust 2024, serde/serde_json, chrono, UUID, SHA-256, Tokio tests, protobuf/tonic for the worker wire contract, and the existing PostgreSQL/MySQL/SQLite migration conventions.

---

## Scope and File Map

The implementation is intentionally split by invariant boundary:

- `src/server/harness/middleware/types.rs`: identifiers, actor projections, session/task/turn/attempt records, model and runtime descriptors, content, tool, interaction, workspace, and artifact records.
- `src/server/harness/middleware/lifecycle.rs`: legal state transitions and versioned compare-and-swap projections.
- `src/server/harness/middleware/lease.rs`: renewable attempt/binding leases and fencing validation.
- `src/server/harness/middleware/events.rs`: durable versus delivery sequences, event parent validation, branch heads, idempotency, and source-versus-ingest provenance.
- `src/server/harness/middleware/capsule.rs`: portable allowlist, redaction, ancestor closure, portable checkpoints, and loss reports.
- `src/server/harness/middleware/inference.rs`: model runtime workers, capacity leases, inference admission, uncertain completion, and usage reconciliation.
- `src/server/harness/middleware/worker.rs`: worker-control, session-operation, and attempt-command envelopes plus validation.
- `src/server/harness/middleware/mod.rs`: public exports and module-level contract tests.
- `src/server/harness/mod.rs`: expose the middleware module.
- `src/server/harness/Cargo.toml`: add only dependencies required by the contracts.
- `src/server/harness/BUILD.bazel`: keep Bazel source/dependency coverage aligned with Cargo.
- `src/proto/harness_middleware.proto`: versioned worker and inference protocol definitions.
- `src/proto/BUILD.bazel` and `src/server/ohc/{build.rs,mod.rs,BUILD.bazel}`: generate and expose the new protobuf types.
- `src/server/migrations/`: add canonical tables and fence-safe constraints after pure storage semantics are fixed.
- `src/server/harness/middleware_tests.rs`: cross-module integration tests that exercise handoff, replay, stale workers, capsules, and inference recovery.

Every production function added in these files receives a direct unit test or is covered by an integration test that invokes the real function. Test fixtures contain no credentials, live process handles, or provider calls.

### Task 1: Middleware Type Foundation

**Files:**
- Create: `src/server/harness/middleware/mod.rs`
- Create: `src/server/harness/middleware/types.rs`
- Modify: `src/server/harness/mod.rs`
- Modify: `src/server/harness/Cargo.toml`
- Modify: `src/server/harness/BUILD.bazel`
- Test: `src/server/harness/middleware/types.rs` inline tests

- [x] **Step 1: Write failing serialization and identity tests**

Add tests for stable string IDs, actor stripping, typed content, optional fields, and round-trip JSON. The first test must prove a portable historical actor cannot serialize authenticated principal references or native aliases:

```rust
#[test]
fn historical_actor_is_authority_free() {
    let actor = ActorDescriptor::service("svc", "Automation", "principal-1")
        .with_native_alias("codex", "thread-agent-1");
    let historical = HistoricalActor::from_descriptor(&actor);
    let value = serde_json::to_value(historical).unwrap();
    assert_eq!(value["actor_id"], "svc");
    assert!(value.get("principal_ref").is_none());
    assert!(value.get("native_aliases").is_none());
}
```

Also cover `ContentPart` variants, digest-bearing artifacts, model/runtime descriptors, task-level attempts with no turn, and explicit `None` versus empty collections.

- [x] **Step 2: Run the focused tests and verify RED**

Run:

```bash
cargo test -p server_harness middleware::types -- --nocapture
```

Expected: compilation or test failures because the middleware module and types do not exist yet.

- [x] **Step 3: Implement the minimal typed contract**

Define serde-compatible types with explicit tagged unions and no `serde_json::Value` in fields whose semantics are canonical. Use `String` IDs at the boundary, `DateTime<Utc>` for timestamps, `Option<T>` for absent values, and `BTreeMap` for deterministic extension maps. Include:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalActor {
    pub actor_id: String,
    pub historical_role: HistoricalRole,
    pub display_label: Option<String>,
    pub delegation_parent_id: Option<String>,
}
```

`HistoricalActor` must be constructed only from the authority-free projection. Keep native IDs, principal references, instructions, credentials, and live handles out of it by type design. Add `ModelRuntimeDescriptor` fields for managed APIs, OpenAI-compatible endpoints, local processes, Kubernetes services, serving engine, model revision, GPU/resource profile, placement, capacity, and health without requiring any specific engine.

- [x] **Step 4: Run focused tests and inspect serialized fixtures**

Run the focused test command again and a deterministic fixture command:

```bash
cargo test -p server_harness middleware::types -- --nocapture
cargo test -p server_harness --doc
```

Expected: all type tests pass and JSON output contains no unstable map ordering.

- [x] **Step 5: Commit the type foundation**

```bash
git add src/server/harness/middleware src/server/harness/mod.rs src/server/harness/Cargo.toml src/server/harness/BUILD.bazel
git commit -m "feat: add harness middleware canonical types"
```

### Task 2: Lifecycle and Lease Fencing

**Files:**
- Create: `src/server/harness/middleware/lifecycle.rs`
- Create: `src/server/harness/middleware/lease.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: inline lifecycle and lease tests

- [x] **Step 1: Write failing transition and fencing tests**

Cover legal transitions, terminal immutability, compare-and-swap, stale generation rejection, lease expiry, and a partitioned worker after reassignment:

```rust
#[test]
fn stale_worker_cannot_mutate_after_reassignment() {
    let mut lease = Lease::active("attempt-1", 7);
    lease.reassign();
    assert_eq!(lease.validate(FenceToken::new(7)), Err(FenceError::StaleGeneration));
    assert_eq!(lease.validate(FenceToken::new(8)), Ok(()));
}
```

Test every canonical state family from the design, including task-level handoff, open turn across failed attempts, binding transitions, and exact duplicate terminal events.

- [x] **Step 2: Run tests to verify RED**

```bash
cargo test -p server_harness middleware::lifecycle middleware::lease -- --nocapture
```

Expected: missing types/functions or failing assertions.

- [x] **Step 3: Implement transition tables and fence validation**

Use typed enums for session, task, turn, attempt, tool, interaction, process, binding, lease, and handoff states. Each projection stores `state_version`; `transition(expected_version, next)` rejects stale versions and illegal edges. Make terminal states immutable except for a new retry/recovery operation. A `Lease` exposes a monotonically increasing generation and `FenceToken`; validation compares both attempt ownership and generation.

- [x] **Step 4: Run focused tests and property-like transition coverage**

```bash
cargo test -p server_harness middleware::lifecycle middleware::lease -- --nocapture
```

Expected: all transition paths and stale-worker cases pass with no warnings from the new module.

- [x] **Step 5: Commit lifecycle primitives**

```bash
git add src/server/harness/middleware/lifecycle.rs src/server/harness/middleware/lease.rs src/server/harness/middleware/mod.rs
git commit -m "feat: add harness lifecycle and lease fencing"
```

### Task 3: Durable Event Semantics

**Files:**
- Create: `src/server/harness/middleware/events.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: inline event-log tests

- [x] **Step 1: Write failing event-log tests**

Cover durable sequence allocation, transient delivery sequence gaps, duplicate idempotency, stale ingest rejection, source-attempt preservation, branch head compare-and-swap, ancestor closure, cycles, cross-session parents, and required-event durability.

- [x] **Step 2: Run RED tests**

```bash
cargo test -p server_harness middleware::events -- --nocapture
```

- [x] **Step 3: Implement the in-memory authoritative event store**

Define `EventEnvelope` with separate `source_attempt_id` and `ingest_attempt_id`, lease generation/fence token, durable/delivery sequences, branch/parent IDs, idempotency key, replay requirement, provenance, and payload. `EventStore::append` must validate the current lease before assigning a durable sequence. Required replay events must be durable; transient events receive only delivery sequence. Parent checks require same session, earlier sequence, existence, and acyclicity. Branch head updates use expected-head compare-and-swap.

- [x] **Step 4: Run event tests and a replay fixture**

```bash
cargo test -p server_harness middleware::events -- --nocapture
```

Verify replay uses durable sequence ranges, ignores dropped transient deltas, and rejects missing ancestor closure.

- [x] **Step 5: Commit event semantics**

```bash
git add src/server/harness/middleware/events.rs src/server/harness/middleware/mod.rs
git commit -m "feat: add fenced canonical event log"
```

### Task 4: Portable Capsule and Handoff Compiler

**Files:**
- Create: `src/server/harness/middleware/capsule.rs`
- Modify: `src/server/harness/middleware/events.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: inline capsule tests and `src/server/harness/middleware_tests.rs`

- [x] **Step 1: Write failing capsule tests**

Test allowlisted record generation, stripping of native records and authority fields, secret/raw-reasoning canaries in instructions/tool outputs/artifacts/extensions, ancestor closure, portable context checkpoints, severity-based loss blocking, report-digest acknowledgement, and deterministic rendering of unsupported records as historical data.

- [x] **Step 2: Run RED tests**

```bash
cargo test -p server_harness middleware::capsule middleware_tests -- --nocapture
```

- [x] **Step 3: Implement the allowlist and loss report**

Define `SessionCapsule`, `PortableContextCheckpoint`, `LossReport`, and `LossEntry`. Build capsules from explicit canonical record variants rather than serializing arbitrary session structs. Reject unknown record kinds/fields, native-record references, live grants, credentials, raw chain-of-thought, and privileged imported actor content. Bind the capsule to a durable event range, branch closure, workspace/artifact digests, redaction policy version, and manifest digest.

- [x] **Step 4: Implement handoff phases in memory**

Define `HandoffOperation` and the durable phase transitions `requested`, `fencing`, `quiescing`, `snapshotting`, `compiling`, `awaiting_loss_ack`, `target_creating`, `activating`, `completed`, `failed`, and `cancelled`. Add scope-aware fencing and require a report-digest-bound interaction before activation when acknowledgement is required.

- [x] **Step 5: Run integration tests**

```bash
cargo test -p server_harness middleware::capsule middleware_tests -- --nocapture
```

The tests must prove no source and target writable bindings coexist and that changing a loss report invalidates the previous acknowledgement.

- [x] **Step 6: Commit capsule and handoff logic**

```bash
git add src/server/harness/middleware/capsule.rs src/server/harness/middleware/events.rs src/server/harness/middleware/mod.rs src/server/harness/middleware_tests.rs
git commit -m "feat: add portable harness session capsules"
```

### Task 5: Model Runtime and Inference Admission

**Files:**
- Create: `src/server/harness/middleware/inference.rs`
- Modify: `src/server/harness/middleware/types.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: inline inference tests

- [x] **Step 1: Write failing runtime admission tests**

Cover worker registration, capability matching, capacity leasing, idempotent pre-admission retry, fenced stale capacity holders, streaming, cancellation, final usage commit, and uncertain post-admission recovery.

- [x] **Step 2: Run RED tests**

```bash
cargo test -p server_harness middleware::inference -- --nocapture
```

- [x] **Step 3: Implement durable in-memory inference state**

Define runtime worker/capacity lease/request/admission/final response records and the lifecycle `queued -> capacity_leased -> admitted -> streaming -> completed|failed|cancelled|uncertain`. Admission binds request digest, model binding, runtime worker and capacity generation. Every stream/final mutation validates the capacity fence; pre-admission retry is safe, post-admission missing terminal state becomes uncertain unless the request is explicitly replayable.

- [x] **Step 4: Run inference tests**

```bash
cargo test -p server_harness middleware::inference -- --nocapture
```

- [x] **Step 5: Commit inference contracts**

```bash
git add src/server/harness/middleware/inference.rs src/server/harness/middleware/types.rs src/server/harness/middleware/mod.rs
git commit -m "feat: add fenced model runtime admission"
```

### Task 6: Worker and Inference Wire Contracts

**Files:**
- Create: `src/proto/harness_middleware.proto`
- Modify: `src/proto/BUILD.bazel`
- Modify: `src/server/ohc/build.rs`
- Modify: `src/server/ohc/mod.rs`
- Modify: `src/server/ohc/BUILD.bazel`
- Create: `src/server/harness/middleware/worker.rs`
- Modify: `src/server/harness/middleware/mod.rs`
- Test: Rust envelope tests and protobuf encode/decode tests

- [x] **Step 1: Write failing envelope tests**

Test that worker-control messages do not require session IDs, session-operation messages require operation fencing, attempt commands require task/attempt/lease fencing, task-level commands allow no turn ID, and protobuf round trips preserve unknown-safe version fields.

- [x] **Step 2: Run RED tests**

```bash
cargo test -p server_harness middleware::worker -- --nocapture
```

- [x] **Step 3: Define the versioned protobuf contract**

Use separate messages for worker control, session operations, attempt commands, event delivery, artifact references, and model-runtime inference. Include tenant/session/task/turn/attempt IDs where applicable, operation or lease generation, fencing token, capability version, correlation ID, idempotency key, replay cursor, and payload schema/version. Do not expose ACP/native wire shapes as this internal contract.

- [x] **Step 4: Add generated bindings to both Cargo and Bazel paths**

Follow the existing `server_ohc` `tonic::include_proto!` and Bazel re-export patterns. Preserve current generated modules and add a separate `harness_middleware` namespace.

- [x] **Step 5: Run focused wire tests and compile checks**

```bash
cargo test -p server_harness middleware::worker -- --nocapture
cargo check -p server_ohc -p server_harness -p ohc_builtin_agent
```

- [x] **Step 6: Commit the wire contract**

```bash
git add src/proto/harness_middleware.proto src/proto/BUILD.bazel src/server/ohc src/server/harness/middleware/worker.rs src/server/harness/middleware/mod.rs
git commit -m "feat: add harness worker protocol"
```

### Task 7: Durable Database Schema and Fence Constraints

**Files:**
- Create: `src/server/migrations/218_harness_middleware.sql`
- Create: `src/server/db/migrations/218_harness_middleware_mysql.sql`
- Create: `src/server/db/sql_middleware.rs`
- Modify: `src/server/db.rs` to route MySQL through the SQL middleware
- Test: `src/server/db` migration tests and SQL contract tests

- [x] **Step 1: Write failing schema contract tests**

Assert that the migration contains tenant keys, typed identity/order columns, session/task/turn/attempt/binding/lease tables, event parent and branch constraints, command inbox/outbox, model runtime admissions, capsule manifests, and unique active read-write binding scope constraints. Assert the migration rejects stale lease generations in its update predicates or stored procedures.

- [x] **Step 2: Run RED schema tests**

```bash
cargo test -p ohc-mono harness_middleware_schema -- --nocapture
```

- [x] **Step 3: Implement the forward-only migration**

Use the repository's tenant isolation conventions through a dialect-aware SQL middleware. Keep PostgreSQL's RLS, triggers, partial indexes, and JSONB migration; provide a native MySQL 8.0.13+ companion with JSON, InnoDB foreign keys, nullable unique sequence keys, and a migration marker protected by `GET_LOCK`. Store query-critical identity, sequence, state version, lease generation, fencing token, digest, and timestamps in typed columns on both backends. Add indexes for `(tenant_id, session_id, durable_sequence)`, active scope bindings, lease expiry, inbox idempotency, and inference admission.

- [x] **Step 4: Run migration tests against the supported local backend**

```bash
cargo test -p ohc-mono harness_middleware_schema -- --nocapture
OMNISOLO_HARNESS_MYSQL_URL=mysql://... cargo test -p ohc-mono --test harness_middleware_interop mysql_harness_middleware_migration_is_idempotent -- --ignored --nocapture
```

Expected: both dialect migrations apply, the MySQL path is idempotent across repeated pod starts, tenant isolation remains enabled, and duplicate/stale writes are rejected.

- [x] **Step 5: Commit durable schema**

```bash
git add src/server/migrations/218_harness_middleware.sql src/server/db.rs
git commit -m "feat: add harness middleware persistence schema"
```

### Task 8: OmniSolo Harness Integration and Legacy Projections

**Files:**
- Modify: `src/agents/builtin/agent.rs`
- Modify: `src/agents/builtin/lib.rs`
- Modify: `src/agents/builtin/service.rs`
- Modify: `src/server/interop/protocol.rs`
- Modify: `src/proto/agent_service.proto` and `src/proto/interop.proto` only through additive fields
- Test: `src/agents/builtin/middleware_integration_tests.rs`

- [x] **Step 1: Write failing integration tests**

Use a deterministic fake OmniSolo provider and tool executor to prove one run emits canonical session/task/turn/attempt events, preserves the legacy `RunTaskEvent` projection, records tool and usage data, and rejects stale worker output.

- [x] **Step 2: Run RED integration tests**

```bash
cargo test -p ohc_builtin_agent middleware_integration -- --nocapture
```

- [x] **Step 3: Add the OmniSolo adapter boundary**

Map current `AgentEvent` values into canonical events with generated IDs, source/ingest attempt provenance, durable sequence assignment, and the existing stream as a read projection. Keep existing behavior and public fields additive. Do not create or rename a Codex runner file.

- [x] **Step 4: Add capsule import/export hooks**

Expose explicit create/resume/handoff commands through the existing service layer, with fresh policy/capability checks and workspace/artifact references. Legacy opaque handoff bytes remain readable only through a versioned legacy importer and never become canonical state.

- [x] **Step 5: Run targeted and regression tests**

```bash
cargo test -p ohc_builtin_agent middleware_integration -- --nocapture
cargo test -p server_harness
```

- [x] **Step 6: Commit OmniSolo integration**

```bash
git add src/agents/builtin src/proto src/server/harness
git commit -m "feat: connect OmniSolo harness to canonical middleware"
```

### Task 9: Adapter Conformance Fixtures and Full Verification

**Files:**
- Create: `src/server/harness/tests/middleware_conformance.rs`
- Create: `src/server/harness/tests/fixtures/harness_middleware/*.json`
- Modify: `src/server/harness/BUILD.bazel` to include `tests/middleware_conformance.rs`
- Modify: this plan only to record completed commands

- [x] **Step 1: Add protocol-neutral conformance fixtures**

Fixtures cover durable/transient replay, branch closure, content redaction, tool effects, approvals, compaction, workspace digests, native extension exclusion, model capability matching, and loss reports.

- [x] **Step 2: Add failure-injection tests**

Exercise stale worker writes, lease reassignment, checkpoint crash windows, handoff concurrency, source/target writer exclusivity, gateway restart after uncertain model admission, and duplicate delivery. Use deterministic clocks and in-memory stores so tests are repeatable.

- [x] **Step 3: Run focused, package, and workspace verification**

```bash
cargo fmt --all -- --check
cargo test -p server_harness
cargo test -p ohc_builtin_agent --lib
cargo test -p ohc-mono --lib
cargo test --workspace
```

The existing baseline currently has unrelated failures in Stripe test imports and Restic async test closures. Those failures must remain separately identified; all new middleware packages and tests must pass independently, and no new warnings/errors may be introduced.

- [x] **Step 4: Run Bazel targets when the executable is installed**

```bash
command -v bazel
bazel test //src/server/harness:server_harness_test //src/agents/builtin:ohc_builtin_agent_lib
```

If `command -v bazel` exits non-zero, record that Bazel verification was unavailable and retain the Cargo verification output.

- [x] **Step 5: Review the complete diff and commit test fixtures**

```bash
git diff --check
git status --short
git add tests src/server/harness src/agents/builtin src/proto src/server/db
git commit -m "test: add harness middleware conformance coverage"
```

## Plan Self-Review

## Implementation Status

The implementation is complete on the `feat/omnisolo-harness-middleware` worktree. The canonical middleware, portable capsule, lease/event fencing, model-runtime admission, worker envelopes, PostgreSQL and MySQL migrations behind the SQL middleware, OmniSolo adapter, legacy projections, typed NATS capsule operations, and conformance fixtures are present. The existing OmniSolo harness remains the native adapter; no `codex_runner.rs` was introduced.

Final verification evidence:

- `cargo test -p server_harness -- --nocapture` via the final coverage run: 98 unit tests, 5 conformance tests, and 2 schema-contract tests passed.
- `cargo test -p ohc_builtin_agent middleware --lib -- --nocapture`: 7 bridge/integration tests passed.
- `cargo test -p server_ohc -- --nocapture`: 2 protobuf round-trip tests passed.
- `cargo test -p ohc-mono --test harness_middleware_interop -- --nocapture`: 2 typed capsule transport tests passed.
- `cargo check -p ohc-mono --lib`, `git diff --check`, targeted `rustfmt --check`, and `bazel query //src/proto:harness_middleware_prost` passed.
- `cargo fmt --all -- --check` remains red because the repository has extensive unrelated pre-existing formatting drift; targeted rustfmt checks for every changed standalone middleware file pass.
- `cargo llvm-cov` all-target reports cover the server middleware modules and bridge; the bridge is 100% line/function and 99% region covered. The server middleware suite covers the major state, fencing, replay, capsule, inference, worker, and integration paths.
- Local PostgreSQL migration validation applied the migration, exercised tenant RLS and cross-tenant trigger rejection, and cleaned up its temporary schema/role.
- Local MySQL 8.0.46 validation applied all 22 canonical tables, accepted the tenant-aware foreign keys and checks, rejected a cross-tenant task reference, and passed the Rust idempotency test twice against the same database.
- The repository-level migration test still cannot compile because of the unrelated existing `src/server/integrations/whatsapp_cloud/client_test.rs` `crate::client` import. The bounded Bazel test build reached analysis but timed out after 120 seconds while compiling external dependencies.

- **Spec coverage:** Tasks 1-3 cover canonical identity, lifecycle, leases, event ordering, provenance, and branches. Task 4 covers the portable capsule and handoff. Task 5 covers future open-source model runtimes. Task 6 covers independently scalable worker communication. Task 7 covers durable state. Task 8 preserves and integrates the OmniSolo harness and legacy projections. Task 9 covers failure injection and full verification.
- **Security coverage:** Credentials, live grants, native records, raw reasoning, privileged actors, stale leases, and unsnapshotted effects are tested as explicit rejection cases.
- **Test coverage:** Each production module starts with failing tests, includes focused edge cases, and ends in package plus integration verification. The plan records unrelated baseline failures so they cannot be misreported as feature regressions.
- **Naming:** No implementation is placed in a file named `codex_runner.rs`; the existing file remains untouched except where an existing caller requires an additive integration point.
- **Open-source models:** Model descriptors, runtime descriptors, capability snapshots, capacity leases, inference admission, model bindings, and usage reconciliation are implemented before selecting a serving engine.
