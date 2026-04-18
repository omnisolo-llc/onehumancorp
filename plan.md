1. Use a temporary Python script via `run_in_bash_session` to modify `cleanup_tmp_files` in `srcs/server/standalone_ohc.sh` to remove temporary files that include `Linear` and `linear` in their name.
2. Use a temporary Python script via `run_in_bash_session` to remove the insecure fallback code in `srcs/server/dashboard/tenant.go`.
3. Use a temporary Python script via `run_in_bash_session` to remove the fallback logic in `srcs/server/agents/registry.go`.
4. Use a temporary Python script via `run_in_bash_session` to update the fallback tests in `srcs/server/dashboard/server_test.go`.
5. Read the files `srcs/server/standalone_ohc.sh`, `srcs/server/dashboard/tenant.go`, `srcs/server/agents/registry.go`, and `srcs/server/dashboard/server_test.go` using `cat` or `read_file` to confirm the cleanup logic changes and the removal of the fallback logic were correctly applied.
6. Run all relevant tests using `./bazelisk test //...` to ensure the modifications are correct and have not introduced regressions.
7. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8. Use the `submit` tool to submit the changes with branch name `maintainer-cleanup-tmp-files` and pull request title `🧹 Maintainer: [cleanup] Remove insecure Fast-and-Loose code in tenant registry to prevent random tenant fallback and fix standalone wrapper cleanup`.
