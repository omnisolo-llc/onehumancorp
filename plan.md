1. **Restore Priority Ordering**:
   Modify `claimTaskPostgres` and `claimTaskSQLite` to include `ORDER BY priority ASC, created_at ASC` in the `SELECT` clause to ensure priority and FIFO processing as called out in the review.

2. **Fix Mocking & 100% Coverage**:
   Create a mocked DB provider in `srcs/server/orchestration/tasks/task_decomposition_service_test.go` or write tests to explicitly test both paths. Wait, the memory says we can just use `db.NewTestProvider` but the review specifically complains that `claimTaskPostgres` is uncovered because the test provider is SQLite. I will create a `MockProvider` that returns `false` for `IsSQLite()` and implement dummy methods for `QueryRow` etc. to ensure we hit the `claimTaskPostgres` branch in our test suite, thus achieving 100% coverage.

3. **Status Transitions**:
   Add explicit `MarkTaskDone` and `MarkTaskFailed` methods to `TaskDecompositionService` to satisfy the "Nitpick" that the design wants explicit state transitions. Add basic tests for these.

4. **Tests & Coverage Check**:
   Run `bazelisk test //srcs/server/orchestration/tasks/...` and ensure the tests pass and cover both Postgres and SQLite branches.

5. **Request Code Review**:
   Request a code review again.
