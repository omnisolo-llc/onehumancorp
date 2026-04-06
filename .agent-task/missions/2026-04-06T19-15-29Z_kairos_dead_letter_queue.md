---
Title: "KAIROS Dead Letter Queue (DLQ) & Poison Pill Handling"
Priority: "P1"
Estimated Scope: "Medium"
---

# Problem Statement
The KAIROS Sub-Agent Queue currently routes tasks that exceed their retry limits to a generic "Dead Letter Queue" (DLQ) concept, but lacks a formalized backend API and Database schema to inspect, replay, or delete these poison pill tasks. Without a robust DLQ mechanism, operations teams cannot diagnose consistently failing agent tasks or recover lost sub-agent workloads, undermining the reliability of the OHC Swarm.

# Research Report
- Based on `docs/features/kairos/sub_agent_queue.md`, tasks that exhaust retries are routed to the DLQ.
- Current Cloud-Native (Redis) and Standalone (SQLite) queue implementations in `srcs/server/orchestration/queue/` lack a dedicated DLQ table or Redis structure.
- Best practices for DLQs require:
  1. Capturing the original task payload.
  2. Recording the complete stack trace or final error reason.
  3. Tracking the timestamp of failure and the executing agent.
  4. Providing an API to replay the task (moving it back to the active queue).

# Design Doc
**Architecture:**
- **Database Schema (PostgreSQL/SQLite):**
```sql
CREATE TABLE IF NOT EXISTS kairos_dlq_jobs (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    original_job_id TEXT NOT NULL,
    agent_role TEXT,
    payload JSONB,
    last_error TEXT,
    failed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_kairos_dlq_org ON kairos_dlq_jobs(organization_id);
```

- **Queue Integration:**
  Modify `srcs/server/orchestration/queue/queue.go` and its implementations (`postgres_queue.go`, `redis_queue.go`, `sqlite_queue.go`). When `Attempts >= MaxAttempts`, instead of just dropping the job, insert it into the DLQ storage.

- **DLQ Management API:**
  - `GET /api/v1/queue/dlq` - List poison pill tasks.
  - `POST /api/v1/queue/dlq/{id}/replay` - Move task from DLQ back to active queue, resetting attempts.
  - `DELETE /api/v1/queue/dlq/{id}` - Permanently discard task.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the "KAIROS Dead Letter Queue (DLQ)" feature.
1. Create a database migration for `kairos_dlq_jobs` in `srcs/server/db/migrations/` (e.g., `031_kairos_dlq.sql`).
2. Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Update the `TaskQueue` interface in `srcs/server/orchestration/queue/queue.go` to support a `MoveToDLQ(ctx context.Context, job *Job, lastError string) error` method.
4. Implement `MoveToDLQ` for Redis, PostgreSQL, and SQLite queue backends.
5. Create a `DLQManager` service with Replay and Discard functionality.
6. Instrument DLQ operations with OpenTelemetry metrics (e.g., `telemetry.RecordDLQTaskAdded(ctx)`).
7. Create unit tests for the DLQ service and queue backend implementations. Use `bazelisk test //srcs/server/orchestration/queue/...` to verify.
8. Ensure you run the `bazelisk run //:gazelle` to update dependencies.

# Visual Excellence Guidelines
Any frontend representation of the KAIROS DLQ later created must apply:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
