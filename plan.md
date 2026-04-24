1. **Parallel Execution Optimization:** Update `snapshotLocked()` in `src/server/dashboard/server.go` to use `sync.WaitGroup` to perform parallel execution. We will fetch `PeekTasks`, `s.tracker.Summary`, and `agents/meetings` in parallel.
2. Ensure that we include `sync` in `src/server/dashboard/server.go` imports.
3. Keep the `src/server/dashboard/server_perf_test.go` benchmark we created and run the benchmark.
4. Run `bazelisk test //...` to ensure everything is fine.
5. Complete pre-commit steps.
6. Submit change.
