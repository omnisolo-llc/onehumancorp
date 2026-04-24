1. **Create Database Migration:**
   - Run `cat << 'MIG' > src/server/db/migrations/20260427020001_shared_tasks_indexes.sql ... MIG` to add `locked_until` (via ALTER TABLE if missing) and the indices on `status` and `locked_until` on `shared_tasks`.
   - Add a verification step: `ls -la src/server/db/migrations/20260427020001_shared_tasks_indexes.sql` to verify creation.
   - Run a Python script or `sed` to update `embedsrcs` in `src/server/db/BUILD.bazel` to include this new migration.
   - Add a verification step: `cat src/server/db/BUILD.bazel | grep 20260427020001_shared_tasks_indexes.sql` to confirm.

2. **Update TaskManager Enhancements:**
   - Use a Python script to inject `tm.stateMachine.TransitionWithTx(ctx, tx, taskID, "SHARED_TASK", statemachine.StateCompleted, agentID, "Task completed successfully")` in `CompleteTaskWithResult` inside `src/server/orchestration/tasks.go`.
   - Use a Python script to replace the status update in `ReviewTask` to utilize `tm.stateMachine.Transition(ctx, taskID, "SHARED_TASK", statemachine.StateReview, agentID, "Agent requested review")`. Oh, wait, `ReviewTask` is ALREADY doing `err = tm.stateMachine.Transition(ctx, taskID, "SHARED_TASK", statemachine.StateReview, agentID, "Agent requested review")`. Let me verify `tasks.go` again to see what exactly needs modification.

3. **Modify API Endpoints:**
   - Use a Python script to add a `requireSPIFFE` HTTP middleware in `src/server/orchestration/service.go`. The middleware will check the `Authorization` header for a bearer token, and validate it starts with `spiffe://` using `interop.ValidateSPIFFEID`.
   - Wrap the HTTP handlers defined in `RegisterTaskHTTPHandlers` with this new middleware.
   - Add a verification step: `cat src/server/orchestration/service.go | grep -C 5 requireSPIFFE` to verify changes.

4. **Update Tests:**
   - Run `cat << 'TEST' > src/server/orchestration/patch_tasks_test.py ... TEST` and execute it to inject `TestSharedTask_StateMachine` in `src/server/orchestration/tasks_test.go` covering all transitions (`PENDING` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED`).
   - Add a verification step: `grep -nri "TestSharedTask_StateMachine" src/server/orchestration/tasks_test.go` to confirm injection.

5. **Expose Prometheus Metrics:**
   - Write a python script to ensure `telemetry.RecordSwarmTaskTransition` is properly used inside `tasks.go` right after state transitions, and ensure we use `metric.WithAttributes` properly in `telemetry.go`.
   - Add verification step: `cat src/server/orchestration/tasks.go | grep RecordSwarmTaskTransition` to confirm changes.

6. **Test the changes:**
   - Run `bazelisk test //src/server/orchestration/... //src/server/db/...` to verify the logic.

7. **Pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Completion:**
   - Output a final unstructured message containing issue_id.
