# OHC-04: Customer inquiry → qualified quote Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the OHC-04 inquiry-to-quote ingestion API that parses an inbound message, qualifies it against OHC-03 policies, and generates a priced quote draft for owner review.

**Architecture:** Expose a new endpoint `/api/v1/inquiries` that accepts an inbound raw message. The endpoint inserts an `inquiries` record and enqueues an `inquiry_intake_agent` job on `ohc_job_queue`. A new worker `inquiry_intake_worker` reads this job, calls the LLM with the raw message and OHC-03 tenant policy to qualify it. If it matches, it inserts a drafting quote and enqueues `draft_quote_agent`.

**Tech Stack:** Rust, axum, sqlx, tokio

**Spec:** .agent-task/report/task_output.md

## Global Constraints

- Tenant isolation must be strictly enforced using `set_org_context` and `tenant_id` bounds.
- Use `ohc_job_queue` for asynchronous LLM tasks.
- Must preserve existing `make test` and `make lint` compatibility.

## Review Focus

- Malformed or extremely long raw messages crashing the API.
- The LLM qualifier misinterpreting the policy and quoting for unsupported services.

---

### Task 1: Add Inquiry API Endpoint

**Files:**
- Create: `src/server/api/inquiries.rs`
- Modify: `src/server/api/mod.rs`
- Modify: `src/server/main.rs` (if necessary to mount the router)

**Interfaces:**
- Produces: `POST /api/v1/inquiries` endpoint accepting `{"raw_message": "...", "source": "web"}`.

- [ ] **Step 1: Write the failing test**

```rust
#[tokio::test]
async fn test_create_inquiry() {
    // Standard axum test setup checking POST /api/v1/inquiries
    // Expecting 201 Created and job queued.
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --locked -p omnisolo test_create_inquiry`
Expected: FAIL

- [ ] **Step 3: Implement `inquiries::router` and `create_inquiry` handler**

Create `InquiryRequest` with `raw_message` and `source`. Insert into `inquiries` table (from migration 139). Insert `inquiry_intake_agent` job into `ohc_job_queue`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --locked -p omnisolo test_create_inquiry`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/server/api/inquiries.rs src/server/api/mod.rs
git commit -m "feat: Add POST /api/v1/inquiries endpoint"
```

### Task 2: Implement Inquiry Intake Worker

**Files:**
- Create: `src/server/workers/inquiry_intake_worker.rs`
- Modify: `src/server/workers/mod.rs`
- Modify: `src/server/main.rs` (to spawn the worker)

**Interfaces:**
- Consumes: `inquiry_intake_agent` jobs from `ohc_job_queue`.

- [ ] **Step 1: Write the failing test**

```rust
#[tokio::test]
async fn test_inquiry_intake_worker() {
    // Test that the worker picks up an inquiry job and evaluates it via AdapterLlm.
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --locked -p omnisolo test_inquiry_intake_worker`
Expected: FAIL

- [ ] **Step 3: Implement `InquiryIntakeWorker`**

Read `inquiry_intake_agent` jobs. Use `AdapterLlm` (similar to `draft_quote_worker.rs`) to evaluate if the `inquiry` matches the tenant's `ohc_03_policy` and `service_items`. If it matches, transition `inquiries.status` to `PROCESSING`, insert into `quotes` as `DRAFTING`, and enqueue `draft_quote_agent` with the `quote_id` and `inquiry`. If out of scope, transition inquiry to `CLOSED`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --locked -p omnisolo test_inquiry_intake_worker`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/server/workers/inquiry_intake_worker.rs src/server/workers/mod.rs src/server/main.rs
git commit -m "feat: Implement InquiryIntakeWorker for qualifying leads"
```
