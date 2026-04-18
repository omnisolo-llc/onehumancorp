1. **Analyze Code**: The performance issue in `SyncContextSync` and `SyncMissions` inside `srcs/server/orchestration/sip.go` seems to originate from limiting parallelism by manually chunking the tasks inside `CoordinatorMode` and looping sequentially inside each chunk. By changing the approach to create an individual task function for each record, we allow `CoordinatorMode` to parallelize execution efficiently up to its concurrency limit.

2. **Refactor `SyncContextSync`**:
    - Update `SyncContextSync` in `srcs/server/orchestration/sip.go`.
    - Change the `tasks` array creation to make a `func() error` for *each* record instead of looping inside `workerCount` tasks.
    - Let `workerCount = 64` (or minimum of `len(records)` and `64`).
    - Remove the inner loop, directly performing the sanitization and network call for that single record. This allows `ExecuteParallel` to correctly batch and parallelize.

3. **Refactor `SyncMissions`**:
    - Update `SyncMissions` in `srcs/server/orchestration/sip.go` exactly like `SyncContextSync`.
    - Change the `tasks` array creation to make a `func() error` for *each* mission instead of grouping them and looping inside the worker task.
    - Set `workerCount = 64` (or min of `len(missions)` and `64`).
    - Remove the inner loop inside the task func.

4. **Run Benchmarks**:
    - Use `run_in_bash_session` to execute the benchmarks in `srcs/benchmarks/sip_sync_bench_test.go` to verify the 10x performance improvement.

5. **Run Tests**:
    - Execute `./bazelisk test //...` to make sure nothing is broken.

6. **Complete pre commit steps**:
    - Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit**:
    - Commit and submit the code using `submit` tool with title "⚡ Bolt: [performance improvement]".
