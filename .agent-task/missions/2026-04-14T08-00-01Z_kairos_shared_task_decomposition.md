status: DONE
agent: Implementer
# Mission: Shared Task List Decomposition & Schema Refinement

**Title:** Shared Task List Decomposition & Schema Refinement
**Problem Statement:** Current `shared_tasks` implementations are fragmented across multiple migrations and lack native support for UltraPlan deliberation phases and robust DAG dependency validation in Standalone (SQLite) mode.
**Research Report:**
- PostgreSQL supports `FOR UPDATE SKIP LOCKED`, but SQLite needs explicit transaction management and mutexes.
- UltraPlan phases (PROPOSE, CRITIQUE, REVISE, APPROVED, EXECUTE) are currently handled as string status, but lack structured transitions in the state machine.
- DAG dependencies are stored in `task_dependencies`, but circular dependency checks are not enforced at the DB or middleware level.
**Design Doc:**
- Consolidate `shared_tasks` to include `ultraplan_phase` and `deliberation_log` (JSONB).
- Add `depth` column to `shared_tasks` to optimize DAG traversal.
- Implement a middleware check in Go for circular dependencies during `CreateTask`.
- **API Change:** `POST /api/orchestration/tasks/decompose` to take a high-level task and return a set of sub-tasks.
**Implementation Prompt:**
- Modify `srcs/server/db/migrations/050_refine_shared_tasks.sql` to add `ultraplan_phase` (VARCHAR) and `deliberation_log` (JSONB/TEXT).
- Update `srcs/server/orchestration/tasks.go` to include these fields in `SharedTask` struct.
- Implement `CheckCircularDependency(ctx context.Context, taskID string, dependencies []string) error` in `TaskManager`.
- Update `ClaimTask` to respect `ultraplan_phase == 'APPROVED'` or `status == 'PENDING'`.
- Ensure SQLite fallback for JSONB uses `TEXT`.
**Priority:** P0
**Estimated Scope:** Medium
