1. **Create Benchmark Test**: Create `srcs/server/db/benchmark_test.go` to measure the latency of task creation and acquiring using `db.Exec`, `db.QueryRow`, etc., simulating both Postgres and SQLite. The tests should perform `InsertMemory` and `ClaimTask` simulations, measuring operations under concurrency.
2. **Optimize SQLite Connection Settings**: Modify `srcs/server/db/database.go` to execute `PRAGMA journal_mode=WAL`, `PRAGMA synchronous=NORMAL`, and `PRAGMA busy_timeout=5000` when initializing a SQLite connection. I also need to tune `SetMaxOpenConns` appropriately for WAL mode to handle multiple concurrent reads while writing.
3. **Optimize Postgres Settings**: Modify `srcs/server/db/database.go` to configure Postgres pool settings (`MaxConns`, etc.) using pgxpool config options for sub-second latency.
4. **Run Benchmarks**: Execute `bazelisk test //srcs/server/db/... --test_arg=-test.bench=.` to verify latency improvements and performance parity.
5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
6. **Submit PR**: Commit and submit the code matching the issue protocol.
