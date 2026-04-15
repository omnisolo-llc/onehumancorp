1. **Understand the Goal**: The task requires adding Observability Mode Parity for Postgres Transaction Retries. This means adding a `postgresRetryExhaustedCounter` and an associated `RecordPostgresRetryExhausted` function to `srcs/server/telemetry/telemetry.go`, as well as creating related tests in `srcs/server/telemetry/telemetry_test.go` and `srcs/server/telemetry/buffer_test.go`.

2. **Modify `srcs/server/telemetry/telemetry.go`**:
    *   Add `postgresRetryExhaustedCounter metric.Int64Counter` to the global variables block, near `postgresLockContentionCounter`.
    *   In `InitWithMeter(m mockableMeter)`, initialize `postgresRetryExhaustedCounter` with the name `"ohc_postgres_retry_exhausted_total"` and description `"Total times a PostgreSQL transaction failed after exhausting retries."`. Note: follow the pattern of other metric initializations (append to `errs` if error).
    *   Add the `RecordPostgresRetryExhausted` function, similar to `RecordSQLiteRetryExhausted`. It should buffer the metric as `"postgres_retry_exhausted"`, and increment `postgresRetryExhaustedCounter` with the `operation` attribute.

3. **Modify `srcs/server/telemetry/telemetry_test.go`**:
    *   In `TestRecordFunctions(t *testing.T)`, add a call to `RecordPostgresRetryExhausted(ctx, "test_op")` to ensure it doesn't panic when initialized.
    *   In `TestFallbackFunctions(t *testing.T)` (or wherever `RecordSQLiteRetryExhausted` is called, like `TestTelemetryInitialization` or the uninitialized test cases), add a call to `RecordPostgresRetryExhausted(ctx, "test_op")` to ensure it handles uninitialized state properly. In this file, there's `TestRecordFunctions` where we see `RecordSQLiteLockContention` and `RecordSQLiteRetryExhausted`. Wait, let's check `srcs/server/telemetry/telemetry_test.go` at Line 326. Yes, around line 324-326, there are tests just verifying they don't panic without initialization. I'll add `RecordPostgresRetryExhausted` there. And also maybe down below in `TestRecordFunctions` where it is initialized.

4. **Modify `srcs/server/telemetry/buffer_test.go`**:
    *   Add a test case for `RecordPostgresRetryExhausted` following the pattern for `RecordSQLiteRetryExhausted`.

5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

6. **Submit PR**: After all changes, run tests and submit.
