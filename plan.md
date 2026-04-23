1. **Create Database Migration:**
   - Create `srcs/server/db/migrations/20260427020001_shared_tasks_indexes_sqlite.sql` and `srcs/server/db/migrations/20260427020001_shared_tasks_indexes_pg.sql`
   - These migrations will add `locked_until` (via ALTER TABLE if missing) and the indices on `status` and `locked_until` on `shared_tasks`.
   - Update `embedsrcs` in `srcs/server/db/BUILD.bazel` to include this new migration.
   - Verify creation.

2. **Update TaskManager Enhancements:**
   - Wait, `telemetry.RecordSwarmTaskTransition` and `TransitionWithTx` are ALREADY implemented in `CompleteTaskWithResult` in `srcs/server/orchestration/tasks.go`!
   - I will double check `ReviewTask` to ensure it properly uses `tm.stateMachine.Transition`. (It already does).

3. **Modify API Endpoints:**
   - Add a `RequireSPIFFE` HTTP middleware in `srcs/server/orchestration/service.go`. The middleware will check the `Authorization` header for a bearer token, and validate it starts with `spiffe://` using `interop.ValidateSPIFFEID`.
   - Wrap the HTTP handlers defined in `RegisterTaskHTTPHandlers` with this new middleware.
   - Add a verification step: `cat srcs/server/orchestration/service.go | grep -C 5 RequireSPIFFE`

4. **Update Tests:**
   - Inject `TestSharedTask_StateMachine` in `srcs/server/orchestration/tasks_test.go` covering all transitions (`PENDING` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED`).
   - Add verification step using `grep`.

5. **Test the changes:**
   - Run `bazelisk test //srcs/server/orchestration/... //srcs/server/db/...` to verify the logic.

6. **Pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

