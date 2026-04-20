1. **Analyze the Issue:**
   - The issue asks to "Implement Distributed Redis State Machine for UltraPlan".
   - The `UltraPlanManager` currently has inline lock code in `modifyStateMachine` and `UpdatePlanStatus`:
     ```go
     lockKey := "lock:ultraplan:" + planID
     cmd := m.redisClient.B().Set().Key(lockKey).Value("system").Nx().Ex(30 * time.Second).Build()
     ```
   - This inline implementation uses `"system"` for `Value()` and does not properly check the value before deletion, introducing an ABA vulnerability.
   - However, the memory instructs: "When implementing application-level concurrency control for SQLite in Go (e.g., as a fallback for Postgres row-level locks), use standard synchronization primitives like `sync.Mutex` rather than assuming the existence of complex custom interfaces like `MutexProvider` unless they are explicitly defined and intended for that use case."
   - Wait, `MutexProvider` IS explicitly defined in `srcs/server/orchestration/mutex.go`!
   - So I *should* use `MutexProvider` in `UltraPlanManager`. Let's inject `MutexProvider` into `UltraPlanManager`.

2. **Refactor `UltraPlanManager` to use `MutexProvider`:**
   - In `srcs/server/orchestration/ultraplan.go`, update `UltraPlanManager` struct to hold `mutexProvider MutexProvider`.
   - Update `NewUltraPlanManager` to instantiate it: `mp, _ := NewMutexProvider(context.Background(), provider, redisClient)` and store it.
   - Wait, the issue says: "Please add Redis-backed distributed locking to `ultraplan.go` utilizing `rueidis.Client`. Ensure it falls back gracefully to in-memory mutexes for Standalone Desktop Mode (SQLite)."
   - The `MutexProvider` from `mutex.go` does exactly this: it returns `RedisMutexProvider` if `redisClient` is not nil, and `SQLiteMutexProvider` otherwise.
   - So I'll modify `modifyStateMachine` and `UpdatePlanStatus` to use `m.mutexProvider.NewMutex(planID)`.
   - `mutex.Lock(ctx, 30 * time.Second)`
   - `defer mutex.Unlock(ctx)`

3. **Check `ultraplan_test.go` and increase coverage:**
   - Add tests to reach >90% coverage for `ultraplan.go`. We can use `db.NewTestProvider` and test the SQLite fallback via `MutexProvider`.
   - For Redis testing, we can implement a `rueidis` mock or use the same test techniques used in `queue/redis_queue_test.go` or `mutex_test.go`.

4. **Verify tests pass:**
   - Run tests: `bazelisk test //srcs/server/orchestration:go_default_test`

5. **Pre-commit step:**
   - Call `pre_commit_instructions` tool to complete pre commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit:**
   - Submit the change.
