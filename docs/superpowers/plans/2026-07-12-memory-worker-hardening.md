# Memory Worker Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bound memory summarization calls and enforce explicit system or organization database authority for every Postgres operation in the memory worker.

**Architecture:** Introduce an injectable `MemorySummaryApi` beside the existing embedding API and wrap it with the same 60-second provider deadline. Cross-tenant queue acquisition and filesystem memory use explicit system authority; per-session inserts, deletes, and failure resets use organization-scoped transactions and identify both session and agent.

**Tech Stack:** Rust 2024, Tokio timeouts, SQLx transactions/RLS context, Cargo, Bazel.

---

### Task 1: Bound and test summarization

**Files:**
- Modify: `src/server/workers/agent_memory_pipeline.rs`

- [x] **Step 1: Write a failing blocking-summarizer test**

Add a `MemorySummaryApi` test double that never resolves and a focused helper test using a 20-millisecond deadline. Assert timeout returns the original context fallback and drops the provider future.

- [x] **Step 2: Verify no injectable bounded summarizer exists**

Run: `cargo test -p ohc-mono --lib memory_summary_has_deadline`

Expected: FAIL because `MemorySummaryApi` and the deadline helper do not exist.

- [x] **Step 3: Implement the summary boundary**

Add `MemorySummaryApi`, `DefaultMemorySummaryApi`, and a summary API field to `AgentMemoryPipeline`. Keep `new` source-compatible by constructing the default API and add a test constructor accepting both APIs and a timeout. Route both SQLite and Postgres summarization through one timeout helper; fall back to the original context on error or timeout without logging provider response bodies.

- [x] **Step 4: Run focused worker tests**

Run: `cargo test -p ohc-mono --lib memory_summary_has_deadline && cargo test -p ohc-mono --lib agent_memory_pipeline`

Expected: deterministic timeout test and existing worker tests PASS; Postgres tests may remain skipped when the database variable is absent.

- [x] **Step 5: Commit bounded summarization**

```bash
git add src/server/workers/agent_memory_pipeline.rs
git commit -m "perf: bound memory worker summarization"
```

### Task 2: Enforce Postgres authority on worker queries

**Files:**
- Modify: `src/server/workers/agent_memory_pipeline.rs`

- [ ] **Step 1: Write failing SQL-boundary tests**

Add source-level unit assertions around extracted constants/helpers proving failure resets require both `session_id` and `agent_id`, and add a configured-Postgres regression that verifies an organization-scoped reset cannot modify another organization's session when `OHC_DATABASE_URL` is present.

- [ ] **Step 2: Verify current failure updates are tenant-unscoped**

Run: `cargo test -p ohc-mono --lib memory_failure_update_is_tenant_scoped`

Expected: FAIL because current pool updates filter only by `session_id` and do not set organization context.

- [ ] **Step 3: Implement explicit system and tenant transactions**

Use `set_system_context` for cross-tenant queue acquisition and filesystem-memory Postgres inserts. Add a failure-reset helper that begins a transaction, calls `set_org_context(tenant_id)`, and updates only the matching `session_id` and `agent_id`. Keep final consolidated-memory insert/delete inside the existing organization-scoped transaction.

- [ ] **Step 4: Run Cargo and Bazel regressions**

Run: `cargo test -p ohc-mono --lib agent_memory_pipeline && bazel test //src/server/workers:server_workers_unit_test`

Expected: Cargo tests PASS; configured Postgres isolation is reported separately if unavailable; Bazel worker test PASSes.

- [ ] **Step 5: Commit tenant-scoped worker SQL**

```bash
git add src/server/workers/agent_memory_pipeline.rs
git commit -m "security: scope memory worker database access"
```

### Task 3: Verify and record F-06 remediation

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Format and statically check the worker**

Format only `agent_memory_pipeline.rs`, run `git diff --check`, and run `cargo check -p ohc-mono`.

- [ ] **Step 2: Record remediation and verification limits**

Mark F-06 remediated for code paths and deterministic timeout coverage. Explicitly state whether real Postgres/RLS assertions ran or were skipped because `OHC_DATABASE_URL` was absent.

- [ ] **Step 3: Commit report evidence**

```bash
git add docs/reports/production_agent_optimization_report.md docs/superpowers/plans/2026-07-12-memory-worker-hardening.md
git commit -m "docs: record memory worker hardening"
```
