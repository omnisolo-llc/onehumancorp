1. **Parallel Execution Optimization in `snapshotLocked`:**
   - Modify `srcs/server/dashboard/server.go`.
   - Update `snapshotLocked` method to fetch data concurrently.
   - Currently, `orgAgentsLocked()`, `orgMeetingsLocked()`, `tracker.Summary()`, and `TaskManager().PeekTasks()` run sequentially.
   - Use `sync.WaitGroup` to execute them in parallel, since they don't depend on each other and simply read data.

2. **Benchmark:**
   - I will run tests with `go test -bench` before and after to show improvements.

3. **Pre-commit steps:**
   - Complete pre-commit steps to make sure proper testing, verifications, reviews and reflections are done.
