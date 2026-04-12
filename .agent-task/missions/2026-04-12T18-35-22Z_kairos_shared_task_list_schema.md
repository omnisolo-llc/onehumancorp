# Title: KAIROS Shared Task List Schema & Orchestration

## Problem Statement
The OHC Hybrid Architecture requires a highly available, deeply deliberative "Shared Task List" capability. Currently, basic tracking exists in `srcs/server/db/migrations/20260410_kairos_tasks.sql` and `srcs/server/orchestration/tasks.go` (TaskManager), but it lacks scalable backend database designs for complex State Machine Transitions, DAG dependencies tracking, and the Teammate Mesh realtime updates needed for true OHC Swarm orchestration. The agentic OS requires these complex database pipelines designed for Cloud-Native PostgreSQL and Standalone SQLite graceful degradation.

## Research Report
- Evaluated `srcs/server/db/migrations/20260410_kairos_tasks.sql` and `tasks.go`.
- Currently, `SharedTask` struct handles basic metadata (`Dependencies []string`, `Priority`, `Status`).
- Need explicit database schemas (e.g., PostgreSQL JSONB or normalized dependency tables) for tracking multi-agent dependencies and DAG cycles.
- Standalone mode requires robust `sync.Mutex` fallback since PostgreSQL `FOR UPDATE SKIP LOCKED` is unavailable.

## Design Doc
**Architecture & DB Schema Changes:**
1. **DAG Dependency Table:** Add a new sequentially numbered Goose migration creating a `task_dependencies` table (`task_id`, `depends_on_task_id`) to properly query directed acyclic graph paths instead of parsing JSON arrays.
2. **State Machine Logging:** Add a `task_transitions_log` table for auditing when a task moves from `PENDING` -> `IN_PROGRESS` -> `BLOCKED`, mapped back to `statemachine.StateMachine`.
3. **Queue Interface Enhancement:** Expand `Queue.TaskQueue` and `TaskManager` in `tasks.go` to expose a `PopNextUnblockedTask()` method using recursive CTEs in Postgres (Cloud) and application-level graph traversal in SQLite (Standalone).

## Implementation Prompt
Hello Implementer agent! Please execute the following KAIROS Shared Task List upgrades:
1. Create a new sequentially numbered Goose migration in `srcs/server/db/migrations/` that defines `task_dependencies` and `task_transitions_log` tables. Ensure it contains proper SQLite fallback syntax where necessary.
2. Update `srcs/server/orchestration/tasks.go` to implement `PopNextUnblockedTask()` logic, detecting cycles and ensuring the Teammate Mesh `Broadcast` is triggered upon assignment.
3. Write test coverage in `srcs/server/orchestration/tasks_test.go` (create the file if it does not already exist) simulating 3 agents with interlocking dependencies to ensure proper unblocking logic.

## Priority
P0

## Estimated Scope
Medium
