```yaml
research_report:
  title: "High-Performance Agentic Background Job Queue Implementation Guide"
  issue: "#23379"
  summary: "Research outlining the gap between intended ohc_job_queue schema and current sub_agent_queue implementation in src/server/queue.rs, along with necessary implementation steps."
  gap_analysis:
    intended_schema: "ohc_job_queue (id, tenant_id, parent_task_id, job_type, payload (JSONB), status, retry_count, max_retries, next_retry_at, locked_until, created_at, updated_at) with Row-Level Security (RLS) on tenant_id."
    current_implementation: "src/server/queue.rs defines `Job` struct mapping to these fields but likely queries a legacy table (`sub_agent_queue`?) or needs refactoring to fully utilize `ohc_job_queue` schema, Postgres SKIP LOCKED, and proper concurrency management."
  implementation_steps:
    - step: "Verify and Update Struct Definitions"
      description: "Ensure Rust `Job` struct exactly matches the `ohc_job_queue` schema, handling `serde_json::Value` for JSONB payload."
    - step: "Implement Postgres SKIP LOCKED pattern"
      description: "Refactor `pg_queue.rs` or `queue.rs` to use `SELECT ... FOR UPDATE SKIP LOCKED` for dequeuing jobs to ensure high performance and prevent deadlocks."
    - step: "Implement Retry Logic with Exponential Backoff"
      description: "In worker loop, handle failures by incrementing `retry_count` and updating `next_retry_at` using exponential backoff up to `max_retries`."
    - step: "Dead-Letter Queue Integration"
      description: "Move jobs that exceed `max_retries` to a dead-letter queue or mark status as 'FAILED'."
    - step: "Tenant Isolation"
      description: "Ensure all database queries enforce RLS by using `set_org_context` or setting `app.current_tenant` before executing queue operations."
```
