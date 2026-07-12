# Agent Tenant Capability Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ensure built-in agent tools and memory can access only the organization assigned to the agent process, never a tenant selected by model output or mutable process state during a task.

**Architecture:** Add an immutable, validated `TenantContext` to the tools crate and inject it into every tenant-aware tool and `AgentServiceImpl`. Standalone mode may explicitly use the local `system` tenant, while cloud/cluster startup must receive a non-system `OHC_ORGANIZATION_ID`. Booking and quote transactions set their PostgreSQL tenant context from this capability, and memory lookup/write use the same captured value.

**Tech Stack:** Rust 2024, Tokio, Tonic, SQLx/PostgreSQL, Serde, Cargo, Bazel.

---

### Task 1: Define and enforce the process tenant capability

**Files:**
- Create: `src/agents/builtin/tools/tenant.rs`
- Modify: `src/agents/builtin/tools/mod.rs`
- Modify: `src/agents/builtin/tools/BUILD.bazel`
- Modify: `src/agents/builtin/service.rs`
- Modify: `src/agents/builtin/lib.rs`

- [x] **Step 1: Write failing tenant validation and startup-policy tests**

Add tests proving `TenantContext::new` trims and rejects empty IDs, and a pure startup resolver proving cloud/cluster rejects missing or `system` organization IDs while standalone accepts `system`.

- [x] **Step 2: Run tests to verify the capability API is missing**

Run: `cargo test -p ohc_builtin_agent_tools tenant --lib`

Expected: FAIL because `tenant` and `TenantContext` do not exist.

- [x] **Step 3: Implement the minimal immutable capability and startup resolver**

Implement a cloneable `TenantContext` with a private `Arc<str>`, `new`, `system`, and `as_str`. Add `resolve_process_tenant(execution_mode, configured_org)` in `lib.rs`; `standalone` may default to `system`, while all other modes require a non-empty, non-system ID. Construct `AgentServiceImpl` with a new `new_for_tenant` constructor and retain `new` only as a local/test system-tenant convenience.

- [x] **Step 4: Run focused tests and verify they pass**

Run: `cargo test -p ohc_builtin_agent_tools tenant --lib && cargo test -p ohc_builtin_agent resolve_process_tenant --lib`

Expected: all focused tests PASS.

- [x] **Step 5: Commit the capability boundary**

```bash
git add src/agents/builtin/tools/tenant.rs src/agents/builtin/tools/mod.rs src/agents/builtin/tools/BUILD.bazel src/agents/builtin/service.rs src/agents/builtin/lib.rs
git commit -m "security: bind builtin agents to a tenant capability"
```

### Task 2: Remove model-controlled tenant IDs from booking and quote tools

**Files:**
- Modify: `src/agents/builtin/tools/booking.rs`
- Modify: `src/agents/builtin/tools/quote.rs`
- Modify: `src/agents/builtin/tools/mod.rs`

- [x] **Step 1: Write failing schema tests**

Add unit tests that build all six booking tools and the quote tool with `TenantContext::new("org-a")`, then assert their JSON Schemas contain no `tenant_id` property or required entry. Also assert tenant-aware tools are created with the injected organization.

- [x] **Step 2: Run tests to verify model-visible tenant fields still exist**

Run: `cargo test -p ohc_builtin_agent_tools tenant_aware_tool_schemas --lib`

Expected: FAIL because current booking and quote schemas expose and require `tenant_id`.

- [x] **Step 3: Inject the capability into tool executors**

Remove `tenant_id` from every booking and quote argument struct. Add `TenantContext` to each executor and constructor, use it for SQL predicates/inserts and `set_config('app.current_tenant', ..., true)`, and ensure reschedule reads and updates include an explicit tenant predicate. Reuse a lazy quote PostgreSQL pool rather than opening a new pool per invocation. Pass the capability through `all_tools` from `AgentServiceImpl::build_tools`.

- [x] **Step 4: Run schema and tools-crate tests**

Run: `cargo test -p ohc_builtin_agent_tools tenant_aware_tool_schemas --lib && cargo test -p ohc_builtin_agent_tools --lib`

Expected: all tests PASS.

- [x] **Step 5: Commit tenant-safe tools**

```bash
git add src/agents/builtin/tools/booking.rs src/agents/builtin/tools/quote.rs src/agents/builtin/tools/mod.rs
git commit -m "security: remove tenant selection from agent tools"
```

### Task 3: Bind memory reads and writes to the captured tenant

**Files:**
- Modify: `src/agents/builtin/service.rs`

- [x] **Step 1: Write failing service tenant tests**

Add tests constructing `AgentServiceImpl::new_for_tenant(..., "org-a")` and asserting both run configuration memory queries and completion records source their tenant from the service capability rather than `OHC_ORGANIZATION_ID`. Extract the record construction into a pure helper so the write-side assertion does not require PostgreSQL.

- [x] **Step 2: Run tests to verify service memory still reads the environment**

Run: `cargo test -p ohc_builtin_agent service_uses_captured_tenant --lib`

Expected: FAIL because the service currently reads `OHC_ORGANIZATION_ID` during each task.

- [x] **Step 3: Replace task-time environment reads**

Use `self.tenant.as_str()` for semantic search. Clone the immutable context before spawning the task and build `EmbeddingRecord.tenant_id` from it. Do not read `OHC_ORGANIZATION_ID` anywhere in `service.rs`.

- [x] **Step 4: Run focused and regression verification**

Run: `cargo test -p ohc_builtin_agent service_uses_captured_tenant --lib && cargo test -p ohc_builtin_agent --lib && bazel test //src/agents/builtin:ohc_builtin_agent_lib_unit_test`

Expected: all tests PASS.

- [x] **Step 5: Commit memory isolation**

```bash
git add src/agents/builtin/service.rs
git commit -m "security: scope agent memory to its tenant capability"
```

### Task 4: Verify and document remediation evidence

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`

- [x] **Step 1: Run formatting and static checks**

Run targeted `rustfmt` on the changed Rust files, then run `cargo check -p ohc_builtin_agent_tools -p ohc_builtin_agent`.

Expected: formatting succeeds and both crates check successfully.

- [x] **Step 2: Run final tenant-boundary searches**

Run: `rg -n 'OHC_ORGANIZATION_ID|pub tenant_id|"tenant_id"' src/agents/builtin/service.rs src/agents/builtin/tools/booking.rs src/agents/builtin/tools/quote.rs`

Expected: no task-time environment reads and no model argument/schema tenant fields; tenant IDs may remain only in trusted SQL/result fields.

- [x] **Step 3: Record finding status and evidence**

Mark audit findings F-03 and F-05 remediated with the capability, schema, memory, Cargo, and Bazel test evidence. Preserve the original finding text and add status rather than rewriting history.

- [x] **Step 4: Commit the report update**

```bash
git add docs/reports/production_agent_optimization_report.md
git commit -m "docs: record agent tenant capability remediation"
```
