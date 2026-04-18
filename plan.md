1. **Remove `.agent-task` directory**:
   - The `.agent-task/` directory is abolished according to `AGENTS.md` and the instructions. Remove the `.agent-task/` directory from the root.
   - Using `run_in_bash_session` to `rm -rf .agent-task`.

2. **Clean up `Linear` references**:
   - In `srcs/server/standalone_cleanup_test.sh`, the test explicitly creates and asserts the deletion of `Linear-state.tmp` and `linear_task.tmp` files. I will use a Python script via `run_in_bash_session` to remove these lines from the test since they are obsolete and leak internal logic to the wrapper test.

3. **Cleanup `standalone_ohc.sh`**:
   - Audit `srcs/server/standalone_ohc.sh` to remove insecure "Fast-and-Loose" logic. The script uses `pkill -9 -P` and `kill -9` inside `stop_daemon()`. A graceful shutdown should simply kill the process and wait, not use SIGKILL. I will update `stop_daemon` to avoid `kill -9` to ensure multi-tenant and local state safety.
   - I will use a Python script via `run_in_bash_session` to modify `srcs/server/standalone_ohc.sh` to remove the `kill -9` statements and instead let it gracefully exit or log a timeout.

4. **Verify Tests**:
   - Run `bazelisk test //srcs/server/...` to ensure all tests pass.
   - Run `bazelisk test //srcs/tests/...` to ensure Python checks pass.

5. **Complete pre-commit steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit PR**:
   - Create a PR using the persona's standard `"🧹 Maintainer: [Hybrid Hygiene] Audit and prune standalone wrapper & obsolete state tracking"`.
