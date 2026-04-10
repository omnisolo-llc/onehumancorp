---
status: IN_PROGRESS
agent: Researcher
---

# Title: Add DAG Unblocking Tests for KAIROS Orchestration

## Problem Statement
The `ClaimTask` and polling functionality for the Shared Task List have been updated to respect `task_dependencies`. However, there is a gap in our test coverage. We need robust unit tests specifically validating that tasks with unmet dependencies remain blocked and that they are successfully unblocked and claimed once their dependencies transition to the `COMPLETED` state, across both SQLite and PostgreSQL.

## Research Report
Per the `KAIROS_ORCHESTRATOR_DESIGN.md` Phase 1, DAG sequence enforcement is a critical requirement. The schema uses the `task_dependencies` table to track the `depends_on_task_id` for a `task_id`. Code exploration reveals that `ClaimTask` in `srcs/server/orchestration/tasks_db.go` correctly queries:
`AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = shared_tasks.id AND d.status != 'COMPLETED') = 0`
However, `srcs/server/orchestration/tasks_db_test.go` lacks tests explicitly verifying this logic.

## Design Doc
- **Target File**: `srcs/server/orchestration/tasks_db_test.go`
- **Tests to Add**:
  - `TestClaimTask_DAG_Blocked`: Create `task-1` (parent) and `task-2` (child dependent on `task-1`). Attempt to claim `task-2` and assert it returns nil (blocked).
  - `TestClaimTask_DAG_Unblocked`: Update `task-1` status to `COMPLETED`. Attempt to claim `task-2` and assert it successfully returns the task.
- **Environments**: Both SQLite (`db.NewSqliteProvider`) and PostgreSQL variants must be implemented or mocked appropriately depending on test suite capabilities.

## Implementation Prompt
1. Read `srcs/server/orchestration/tasks_db.go` to understand the `ClaimTask` query logic regarding `task_dependencies`.
2. Open `srcs/server/orchestration/tasks_db_test.go`.
3. Update the SQLite schema creation string inside `TestClaimTask_SQLite` to ensure the `task_dependencies` table is created if it wasn't already.
4. Add new test cases (e.g., inside `TestClaimTask_SQLite` or as new functions like `TestClaimTask_SQLite_DAG_Dependencies`) that explicitly insert parent and child tasks into `shared_tasks`, and insert the dependency mapping into `task_dependencies`.
5. Assert that the child task cannot be claimed while the parent is `PENDING`.
6. Execute a `dbProvider.Exec` to update the parent task to `COMPLETED`.
7. Assert that the child task can now be claimed successfully.
8. Run tests: `export PATH="$PATH:$HOME/go/bin" && bazelisk test //srcs/server/orchestration:orchestration_test --test_filter="TestClaimTask_SQLite"`.

## Priority
P1

## Estimated Scope
Small
