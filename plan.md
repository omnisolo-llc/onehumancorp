1. **Parallel Execution Optimization in `snapshotLocked`**:
   The `snapshotLocked` function sequentially fetches `Agents`, `Meetings`, `Costs`, and `TaskQueue`. Since these operations are independent read operations, we can fetch them concurrently to improve the response time of the `/api/dashboard` endpoint, which is heavily used by clients.

   To safely do this without breaking existing behavior, we will:
   - Use a `sync.WaitGroup` to launch goroutines for gathering the four independent sets of data:
     - `Agents` (via `s.orgAgentsLocked()`)
     - `Meetings` (via `s.orgMeetingsLocked()`)
     - `Costs` (via `s.tracker.Summary(s.org.ID)`)
     - `TaskQueue` (via `s.hub.TaskManager().PeekTasks()`)
   - We must take care with `orgMeetingsLocked()` and `tracker.Summary()` as they might acquire internal locks, but read-only concurrent access is generally supported in Go's `sync.RWMutex` pattern used across the codebase.

2. **Benchmark the Change**:
   We will write a benchmark function `BenchmarkDashboardSnapshot` in `server_test.go` to measure the difference before and after the optimization, simulating Cloud vs Standalone mode if necessary.

3. **Pre-commit Steps**:
   Ensure all formatting is correct, no data races exist, and run the `pre_commit_instructions` tests, specifically checking `bazelisk test //srcs/server/dashboard/...`.

4. **Submit**:
   Submit the changes with appropriate benchmark results in the description.
