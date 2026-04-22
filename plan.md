1. **Understand Goal**: We need to extend the `SharedTaskRepo` in `srcs/server/orchestration/kairos/kairos_shared_task.go` to support claiming a task (`ClaimTask`) and transitioning a task (`TransitionTask`) atomically. Claiming should use `FOR UPDATE SKIP LOCKED` for PostgreSQL and a sync.Mutex wrapper (or basic queries with mutex locking) for SQLite. The `TransitionTask` should log a state transition record in `state_machine_transitions`.

2. **File Modifications**:
   - `srcs/server/orchestration/kairos/kairos_shared_task.go`: Add `ClaimTask` and `TransitionTask` methods. Add a `mu sync.Mutex` to the repo to handle SQLite locking if needed, similar to how it's done elsewhere.
   - `srcs/server/orchestration/kairos/kairos_shared_task_test.go`: Add tests for `ClaimTask` and `TransitionTask`.

3. **Details of Methods**:
   - `ClaimTask(ctx context.Context, agentID string) (*SharedTask, error)`
     - Uses `tx, err := r.provider.Begin(ctx)`
     - For SQLite:
       - Uses `r.mu.Lock()` and `defer r.mu.Unlock()` (if we add `mu` to repo). Actually `db.Provider` has `IsSQLite()`.
       - Queries `SELECT id FROM shared_tasks WHERE status = 'PENDING' LIMIT 1`
     - For PostgreSQL:
       - Queries `SELECT id FROM shared_tasks WHERE status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED`
     - Updates status to 'IN_PROGRESS' and sets `agent_id = $1`.
     - Logs transition in `state_machine_transitions` from `PENDING` to `IN_PROGRESS`.
   - `TransitionTask(ctx context.Context, taskID string, agentID string, fromState string, toState string, reason string) error`
     - Begins Tx.
     - Fetches current state `SELECT status FROM shared_tasks WHERE id = $1`
     - Asserts current state == `fromState`.
     - `UPDATE shared_tasks SET status = $toState, agent_id = $agentID WHERE id = $taskID`
     - Logs transition to `state_machine_transitions`.
   - `insertTransition` helper method.

4. **Verify Implementation**:
   - Run tests: `bazelisk test //srcs/server/orchestration/kairos/...`

5. **Pre-commit Checks**: Run the pre-commit instructions.
