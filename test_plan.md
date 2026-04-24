1. **Fix duplicate declarations in test files.**
   - Run `sed` to rename the conflicting `mockPgProvider` to `mockPgProviderAutoDreamTest` in `srcs/server/orchestration/autodream_test.go` and `mockPGProvider` to `mockPGProviderTasksDBTest` in `srcs/server/orchestration/tasks_db_test.go`.
   - Remove undefined protobuf method definitions in `srcs/server/orchestration/service.go`.

2. **Integrate StateMachine into `tasks.go` and `tasks_db.go`.**
   - Run `sed` or Python scripts via `run_in_bash_session` to replace raw SQL `UPDATE shared_tasks SET status = ...` strings in `tasks.go`, `tasks_db.go`, `sub_agent.go`, `queue/kairos_queue.go`, and `autodream/kairos_autodream.go`.
   - Ensure these updates are replaced with or accompanied by calls to `sm.TransitionWithTx` or `sm.Transition` via the `TaskManager` or `SharedTaskOrchestrator` methods.
   - Specifically, replace `tasks.go` UpdateTask's hardcoded status query with `sm.TransitionWithTx`.
   - In `tasks_db.go`, update `ClaimTask` and `TransitionTask` to modify other fields but let state be handled by the transitions.

3. **Verify refactoring**
   - Run `go build ./srcs/server/orchestration/...` or `./bazelisk build //srcs/server/orchestration/...` to confirm compilation.

4. **Write unit tests in `machine_test.go`.**
   - Check if `srcs/server/orchestration/statemachine/machine_test.go` has `TestStateMachine_Transition`. If so, ensure it tests concurrent state updates properly by utilizing `run_in_bash_session` if modifications are needed.

5. **Test execution**
   - Run `./bazelisk test //srcs/server/orchestration/...` to confirm functionality.

6. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit the change.**
   - Once all tests pass, submit the change with a descriptive commit message with the issue tracking id `3997`.
