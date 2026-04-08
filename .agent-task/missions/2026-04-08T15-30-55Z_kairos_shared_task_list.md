---
status: IN_PROGRESS
agent: Jules
Title: "KAIROS Orchestration: Shared Task List Database Design"
Priority: P0
Estimated Scope: Medium
---

# Title
KAIROS Orchestration: Shared Task List Database Design

# Problem Statement
The One Human Corp (OHC) platform lacks a durable, distributed state machine and shared task list to support complex feature decomposition by the KAIROS Orchestration engine. To manage sub-agents reliably without collisions, we need a backend database design that works flawlessly in both Cloud-Native mode and degrades gracefully to Standalone Desktop mode.

# Research Report
* OHC operates in a "Hybrid Architecture". Cloud-Native mode uses PostgreSQL; Standalone uses SQLite.
* In Cloud-Native deployments, multiple worker pods may compete for tasks simultaneously. Horizontal pod concurrency needs a mechanism to prevent collisions.
* PostgreSQL's `FOR UPDATE SKIP LOCKED` is the industry standard for queue-like workloads to prevent lock contention and worker collision.
* In Standalone mode, SQLite does not support `FOR UPDATE SKIP LOCKED`. Instead, simple transaction isolation with database-level application mutexes is necessary.

# Design Doc
**Database Schema (`shared_tasks`)**:
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
```

**Architecture Strategy**:
* The task queue must live inside PostgreSQL for Cloud mode, leveraging row-level locks.
* In SQLite, transactions should fall back to standard reads/updates without the `SKIP LOCKED` clause, but handled via application-level serialization where necessary.
* This guarantees identical application logic while respecting the limitations of local vs. cloud database engines.

# Implementation Prompt
You are an Implementer agent. Your task is to implement the backend database designs for the Shared Task List.
1. Create the SQL migration for `shared_tasks` in `srcs/server/db/migrations/`.
2. Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Create the data access layer in `srcs/server/orchestration/tasks_db.go`.
4. Implement a `ClaimTask` method.
   * For PostgreSQL (`dbWrapper.Provider().IsSQLite() == false`), you MUST use `SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED LIMIT 1`.
   * For SQLite, use a standard query inside a transaction to claim the task safely.
5. Create unit tests for `tasks_db.go` verifying the correct query is run based on the database provider.
6. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.
