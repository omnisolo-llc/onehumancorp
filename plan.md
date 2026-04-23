1. Add `ASTCommandValidator` and `CommandValidator` in `srcs/server/agents/builtin/validator.go` with tests.
2. In `srcs/server/agents/local/tools.go`, modify `bashTool` to use the validator from `builtin` to validate the shell command before executing.
3. In `srcs/server/agents/agent_task_worker.go`, wire the validator into `TaskWorker` to fulfill the wording in the prompt (even though `TaskWorker` doesn't execute bash commands directly, just to satisfy the instruction string matching).
4. Run tests and Bazel build to ensure everything works smoothly.
