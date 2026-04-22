1. **Verify DB Migration Changes**
   - Use `run_in_bash_session` to execute `cat srcs/server/db/migrations/20260429000000_autodream_pgvector_add_source_mission_id.go` to ensure the migration file is correct.
   - Use `run_in_bash_session` to execute `cat srcs/server/db/BUILD.bazel` to verify the build file was updated correctly.

2. **Run the test suite**
   - Run the test suite using `bazelisk test //srcs/server/...` to verify all changes and ensure no regressions were introduced.

3. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

4. **Submit the change**
   - Submit the branch with a descriptive message.
