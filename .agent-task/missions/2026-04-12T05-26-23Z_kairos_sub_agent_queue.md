---
status: DONE
agent: Implementer
priority: P0
---

# Title: Implement KAIROS Orchestration: Sub-Agent Orchestration Queue

## Problem Statement
The KAIROS Orchestrator requires a robust background queuing logic to spawn isolated sub-agents in a production environment. Currently, complex tasks cannot be safely delegated to background workers with guaranteed execution, fault tolerance, and status tracking across our Hybrid Architecture. We need to implement the `sub_agent_queue` schema and its accompanying orchestration logic.

## Research Report
- Based on `KAIROS_AI_OS_ARCHITECTURE.md` and the Sub-Agent Queue Design doc, the system requires a state-machine driven backend to manage sub-agent jobs.
- **Cloud-Native Mode**: PostgreSQL's `FOR UPDATE SKIP LOCKED` and Redis are necessary for horizontal scalability.
- **Standalone Mode**: SQLite with application-level locking (Mutex) must be used as a graceful fallback.

## Design Doc
See `docs/architecture/kairos_sub_agent_queue_design.md` for full architectural details.

**Database Schema:**
```sql
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
```

## Implementation Prompt
Hello Implementer agent! Please execute the implementation for the Sub-Agent Orchestration Queue:
1. Create a new SQL migration file (e.g., `034_kairos_sub_agent_queue.sql`) in `srcs/server/db/migrations/` creating the `sub_agent_queue` schema. Remember to adapt the UUID and TIMESTAMPTZ fields to be compatible with SQLite (e.g., use TEXT for ID and TIMESTAMPTZ).
2. Explicitly add the new migration file to the `embedsrcs` list in `srcs/server/db/BUILD.bazel`.
3. Create `srcs/server/orchestration/queue/queue.go` (and accompanying files) to implement the Queue Manager.
4. The Queue Manager must implement an `Enqueue` and `Poll` method.
5. In `Poll`, use conditional logic (`dbWrapper.Provider().IsSQLite()`) to choose between PostgreSQL `FOR UPDATE SKIP LOCKED` and SQLite application-level `sync.Mutex` locking.
6. Instrument the queue operations with OpenTelemetry metrics.
7. Create comprehensive test coverage in `queue_test.go`, mocking the database to ensure >90% coverage.
8. Verify functionality by running: `bazelisk test //srcs/server/orchestration/...`

## Priority
P0

## Estimated Scope
Large