1. **Analyze Parity:** Find all places doing `FOR UPDATE SKIP LOCKED` or mimicking it in SQLite that don't have the `mu.TryLock` contention logic.
2. **Update `TaskManager` (`src/server/orchestration/tasks.go`)**:
   - `ClaimTask`, `PollTasks`, `PeekTasks` (wait, does peek need lock? maybe not), `CompleteTaskWithResult`, `ReviewTask`, `UpdateTask`, `DeleteTask` use `tm.mu.Lock()`. Change them to:
     ```go
     if tm.db.IsSQLite() {
         if !tm.mu.TryLock() {
             telemetry.RecordPostgresLockContention(ctx, "task_manager_...")
             tm.mu.Lock()
         }
         defer tm.mu.Unlock()
     }
     ```
3. **Update `TaskQueueService` (`src/server/orchestration/queue/kairos_queue.go`)**:
   - Add `mu sync.Mutex` to `TaskQueueService`.
   - In `claimTaskSQLite`, use the mutex block with telemetry: `telemetry.RecordPostgresLockContention(ctx, "kairos_queue_claim")`.
4. **Update `CloudStateManager` (`src/server/orchestration/state/cloud_state_manager.go`)**:
   - Add `mu sync.Mutex` to `CloudStateManager`.
   - In `ClaimTask`, use the mutex block with telemetry: `telemetry.RecordPostgresLockContention(ctx, "cloud_state_manager_claim")`.
5. **Update `StandaloneStateManager` (`src/server/orchestration/state/standalone_state_manager.go`)**:
   - Update `ClaimTask` and `TransitionState` and `MarkTaskCompleted` which currently do `m.mu.Lock()` to do the `TryLock()` and record contention: `telemetry.RecordPostgresLockContention(ctx, "standalone_state_manager_...")`.
6. **Update `SharedTaskListRepo` (`src/server/orchestration/shared_task_list_repo.go`)**:
   - Add `mu sync.Mutex` to `SharedTaskListRepo`.
   - In `getNextAvailableTaskSQLite`, use the mutex block with telemetry: `telemetry.RecordPostgresLockContention(ctx, "shared_task_list_repo_claim")`.
7. **Verify Tests**: Run `bazelisk test //src/server/orchestration/...`.
8. **Pre-commit**: Complete pre-commit.
9. **Submit**: Output YAML with `issue_id: 4871`.
