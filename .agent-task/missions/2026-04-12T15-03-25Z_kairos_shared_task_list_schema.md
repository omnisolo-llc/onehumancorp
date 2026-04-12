---
status: PENDING
agent: Researcher
priority: P0
---

# Title: KAIROS Orchestration: Shared Task List Database Design & Orchestration API

## Problem Statement
The OHC Swarm demands a scalable backbone to coordinate distributed agentic workloads. Currently, there is a gap in a robust distributed state machine to track tasks with exact sequence and DAG dependencies. We need a central Shared Task List database design to track dependencies robustly in Cloud-Native (PostgreSQL) and Standalone (SQLite) modes.

## Research Report
- PostgreSQL provides horizontal scaling via row-level locks `FOR UPDATE SKIP LOCKED`.
- SQLite handles Standalone mode gracefully via application-level mutex locks or explicit transaction beginning.
- OHC needs robust Background Queuing Logic built into `srcs/server/orchestration/`.
- Deep-deliberation cycles (UltraPlan) require a DAG representation. Using JSONB (PostgreSQL) or TEXT (SQLite) optimizes storage while allowing TaskManager polling.

## Design Doc
Architecture & Schema Changes:
1. Shared Task List Schema (`srcs/server/db/migrations/032_kairos_shared_tasks_v2.sql`):
   ```sql
   CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_plan_id TEXT,
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'PENDING',
       assigned_agent_id TEXT,
       dependencies JSONB,
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );
   CREATE INDEX IF NOT EXISTS idx_shared_tasks_v2_org_status ON shared_tasks_v2(organization_id, status);
   ```

2. Sequence Diagram: Task Orchestration
   ```mermaid
   sequenceDiagram
       participant Agent
       participant TaskManager
       participant DB as Shared Task List (Postgres/SQLite)
       Agent->>TaskManager: PollTasks(limit)
       TaskManager->>DB: SELECT ... FOR UPDATE SKIP LOCKED
       DB-->>TaskManager: Return Tasks
       TaskManager->>DB: UPDATE status='IN_PROGRESS', agent_id=...
       TaskManager-->>Agent: Claimed Tasks
   ```

## Implementation Prompt
Hello Implementer agent! Please execute the backend database designs and API implementation for the Shared Task List feature:
1. Create a new SQL migration file `032_kairos_shared_tasks_v2.sql` in `srcs/server/db/migrations/` creating the `shared_tasks_v2` schema.
2. Add the migration file to `srcs/server/db/BUILD.bazel`.
3. Update `srcs/server/orchestration/tasks_db.go` with a highly concurrent `ClaimTask` method. Use `FOR UPDATE SKIP LOCKED` for PostgreSQL and a `sync.Mutex` block for SQLite.
4. Create comprehensive test coverage in `tasks_test.go` and verify with `bazelisk test //...`.
5. Execute with absolute autonomy.

## Priority
P0

## Estimated Scope
Large
