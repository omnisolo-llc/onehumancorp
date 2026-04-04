# Title: KAIROS Orchestration: Implement Shared Task List

## Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. KAIROS lacks a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies across both Cloud and Standalone modes.

## Research Report
Based on OHC Hybrid Architecture (OHC-HA) principles, the components must support:
1.  **Shared Task List**: A central coordination queue is required to manage complex feature decomposition. This necessitates a distributed state machine backed by PostgreSQL and Redis to track task dependencies and statuses across the agent swarm, with graceful fallback to SQLite for Standalone Desktop Mode.
2.  **Concurrency Models**: In Cloud Mode, we must rely on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU race conditions. In Standalone Mode, we must use SQLite's native table locking capabilities via `pool.IsSQLite()` checks.

## Design Doc
1. **Schema Additions**:
   Ensure `swarm_tasks` and `shared_tasks` are created or updated correctly to support DAG dependencies.
   ```sql
   CREATE TABLE IF NOT EXISTS shared_tasks (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       mission_id TEXT NOT NULL,
       parent_plan_id TEXT,
       dependencies JSONB NOT NULL DEFAULT '[]',
       title TEXT NOT NULL,
       status TEXT NOT NULL DEFAULT 'PENDING',
       assigned_agent_id TEXT,
       payload JSONB
   );
   ```
2. **Go Models**: Update task structs to enforce sequence blocking based on dependency state.
3. **Database Handlers**: Add PostgreSQL-specific logic utilizing `FOR UPDATE SKIP LOCKED` inside explicit transactions to avoid releasing row locks too early. Degrade gracefully utilizing SQLite transactions if `pool.IsSQLite()` returns true.
4. **Visual Excellence**: Any UI components displaying the Shared Task List must adhere to the OHC Premium Feel: `backdrop-filter: blur(20px) saturate(200%)`.

## Implementation Prompt
Hello Implementer agent! Please implement the backend components for the Shared Task List:
1. Check `srcs/server/db/migrations/` to ensure `swarm_tasks` and `shared_tasks` exist. If not, create them. Make sure to update `embedsrcs` in `srcs/server/db/BUILD.bazel`.
2. Implement `ClaimTask` logic in `srcs/server/orchestration/tasks.go` or similar, ensuring you open a transaction `tx, err := pool.Begin(ctx)` for PostgreSQL locks using `FOR UPDATE SKIP LOCKED`.
3. Ensure SQLite mode handles task locking appropriately.
4. Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete.
5. All DB operations should return `(int64, error)` and should not call `.RowsAffected()`.
6. Write unit tests achieving >95% coverage, verifying both SQLite and PostgreSQL behaviors.

## Priority
P0

## Estimated Scope
Large
