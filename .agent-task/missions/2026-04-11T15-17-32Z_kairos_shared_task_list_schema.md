---
status: FAILED
agent: Researcher
priority: P0
---

# Title: Implement KAIROS Orchestration: Shared Task List, DAG Dependencies & Sub-Agent Orchestration

## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. Currently, the system lacks a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies. This hinders complex sub-agent orchestration and UltraPlan deliberation cycles (where a high-level goal is broken down into a DAG of tasks). We need a central "Shared Task List" backend database design to track these dependencies robustly in both Cloud-Native (PostgreSQL) and Standalone (SQLite) modes.

## Research Report
- Based on `CLAUDE_OHC.md` and OHC Hybrid Architecture (OHC-HA), the backend relies on custom interfaces (`db.Provider`) mimicking `database/sql`. PostgreSQL provides horizontal scaling (via row-level locks `FOR UPDATE SKIP LOCKED`), while SQLite handles standalone gracefully (via application-level single-connection Mutex).
- OHC requires a robust Background Queuing Logic (e.g. BullMQ/Celery style) built into `srcs/server/orchestration/`.
- Deep-deliberation cycles (UltraPlan) need a DAG representation in the DB. By utilizing JSONB (Postgres) or TEXT (SQLite) to store dependencies, we optimize for storage costs while allowing `TaskManager` to poll state machines.
- State Machine Tracking: We need a distributed lock mechanism using `rueidis` (Redis) for Cloud mode and SQLite for Standalone mode to prevent race conditions during sub-agent queue polling.

## Design Doc
**Architecture & Schema Changes:**
1. **Shared Task List & UltraPlan State Machine Schema (`srcs/server/db/migrations/032_kairos_shared_tasks_v2.sql`)**:
   ```sql
   CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       parent_plan_id TEXT,
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'PENDING',
       assigned_agent_id TEXT,
       dependencies JSONB, -- Array of dependent task IDs
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );
   CREATE INDEX IF NOT EXISTS idx_shared_tasks_v2_org_status ON shared_tasks_v2(organization_id, status);
   ```

2. **Sub-Agent Orchestration Queue (`srcs/server/orchestration/tasks_db.go`)**:
   - Add a `ClaimTask` method that implements the exact state machine tracking.
   - **Cloud Mode (`!dbWrapper.Provider().IsSQLite()`)**: Use `SELECT * FROM shared_tasks_v2 WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED`.
   - **Standalone Mode**: Utilize application-level `sync.Mutex` or `db.Provider` locks for SQLite concurrency.

3. **UltraPlan DAG Deliberation (`srcs/server/orchestration/ultraplan.go`)**:
   - Ensure a task only transitions from `BLOCKED` to `PENDING` when its parent tasks listed in `dependencies` achieve a `COMPLETED` state.

## Implementation Prompt
Hello Implementer agent! Please execute the backend database designs and sequence diagrams for the Shared Task List feature:
1. Create a new SQL migration file (e.g., `032_kairos_shared_tasks_v2.sql`) in `srcs/server/db/migrations/` creating the `shared_tasks_v2` schema. Remember, SQLite does not support `IF NOT EXISTS` for column additions, but here we are creating a new table so it's fine.
2. Explicitly add the new migration file to the `embedsrcs` list in `srcs/server/db/BUILD.bazel`.
3. Update `srcs/server/orchestration/tasks_db.go` with a highly concurrent `ClaimTask` and `ResolveTaskDependencies` DAG method.
4. Integrate `redis/rueidis` distributed locks (or standard `sync.Mutex` for local) to manage the polling queue for Sub-Agent tracking.
5. Create comprehensive test coverage in `tasks_test.go` and `ultraplan_test.go`, maintaining >90% coverage.
6. Remember: Execute with absolute autonomy. Do not ask for approval.

## Priority
P0

## Estimated Scope
Large
