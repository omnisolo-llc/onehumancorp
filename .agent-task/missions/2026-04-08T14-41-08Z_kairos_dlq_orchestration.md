---
status: "PENDING"
Title: "KAIROS Orchestration: Implement Dead-Letter Queue (DLQ) and Retry State Machine"
Priority: "P1"
Estimated Scope: "Medium"
---

# Title: KAIROS Orchestration: Implement Dead-Letter Queue (DLQ) and Retry State Machine

## Problem Statement
The OHC Hybrid Architecture's KAIROS Orchestrator decomposes complex feature requests into sub-tasks (DAG dependencies) processed by autonomous agents. Currently, if a sub-agent task (`DELEGATED`) in the Shared Task List fails due to timeouts, API errors, or missing capabilities, the failure can cause upstream DAG dependencies to deadlock. KAIROS needs a robust Retry and Dead-Letter Queue (DLQ) mechanism to gracefully backoff, retry tasks, and eventually escalate unrecoverable tasks to a DLQ for Human CEO/Architect review instead of failing silently or stalling the entire DAG.

## Research Report
1. **Current Failure States**: Based on `docs/KAIROS_ORCHESTRATOR_DESIGN.md` and `srcs/server/orchestration/ultraplan.go`, tasks follow `PENDING -> IN_PROGRESS -> COMPLETED`, but explicit handling for automated retries and dead-letter escalation is missing.
2. **PostgreSQL & SQLite Impact**: To track retries, new columns (`retry_count`, `max_retries`) must be added to the `shared_tasks` and `swarm_tasks` schemas without breaking SQLite Standalone mode or Cloud-Native PostgreSQL modes.
3. **DAG Unblocking**: A task moved to `DEAD_LETTER` should emit a specific Mesh event (`mesh:tasks` channel) so that the `DefaultTaskOrchestrator` can optionally fail the parent `UltraPlan` rather than waiting indefinitely.
4. **Visual Excellence Guidelines**: The CEO dashboard DLQ monitor UI must be conceptualized with the OHC-HA aesthetics (`backdrop-filter: blur(20px) saturate(200%)`).

## Design Doc
1. **Schema Migrations**:
   Add columns to track retries.
   ```sql
   ALTER TABLE shared_tasks ADD COLUMN retry_count INT DEFAULT 0;
   ALTER TABLE shared_tasks ADD COLUMN max_retries INT DEFAULT 3;
   ```
   (And similarly for `swarm_tasks` if applicable).
2. **State Machine Modifications**:
   Update `srcs/server/orchestration/statemachine/machine.go` or task orchestrator to introduce `RETRYING` and `DEAD_LETTER` states.
3. **Queue Logic Updates (`task_orchestrator.go`)**:
   - In the background worker loop, if a task execution fails, increment `retry_count`.
   - If `retry_count < max_retries`, backoff exponentially and set status back to `PENDING` (or a `RETRYING` equivalent).
   - If `retry_count >= max_retries`, transition to `DEAD_LETTER`.
4. **Teammate Mesh Broadcasts**:
   - `tm.hub.PublishTaskBroadcast(task.ID, payload)` with `action: "TASK_FAILED"` and `action: "TASK_DEAD_LETTER"`.

## Implementation Prompt
Hello Implementer agent! Please execute the DLQ and Retry State Machine:
1. Create a database migration in `srcs/server/db/migrations/` (e.g., `032_task_retries_dlq.sql`) to add `retry_count` (INT, default 0) and `max_retries` (INT, default 3) to `shared_tasks`. Update `BUILD.bazel` to include it in `embedsrcs`.
2. Update the `SharedTask` struct in `srcs/server/orchestration/tasks.go` to include `RetryCount` and `MaxRetries`.
3. In `srcs/server/orchestration/sub_agent.go`, modify `completeTask` or create a `failTask` method. If execution fails (`executeWithRetry` logic), increment the retry counter in the database. If the limit is exceeded, transition the status to `DEAD_LETTER` and emit a `TASK_DEAD_LETTER` mesh event.
4. Update `srcs/server/orchestration/ultraplan.go` to listen for `DEAD_LETTER` tasks and correctly fail the parent `UltraPlan` if a critical dependency permanently fails.
5. In tests, use `db.NewTestProvider(t)` and verify the retry counter logic using an in-memory SQLite database. Achieve >90% coverage on the new retry paths in `task_orchestrator_test.go` and `sub_agent_test.go`.
6. Run `bazelisk test //srcs/server/orchestration/...` to verify your implementation.

## Priority
P1

## Estimated Scope
Medium
