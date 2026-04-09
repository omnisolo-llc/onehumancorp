1. **Fix `tasks_db_test.go` to be Hybrid Aware and Thread-Safe**
   - Use `cat << 'EOF' > srcs/server/orchestration/tasks_db_test.go` to provide the exact literal Go test code.
   - Use `t.Setenv()` instead of `os.Setenv()`.
   - The test will explicitly run both a standalone test and a postgres test (hybrid-aware).
   - Add verification step using `cat srcs/server/orchestration/tasks_db_test.go`.

2. **Run All Tests**
   - Execute `export PATH=$PATH:$HOME/go/bin:/usr/local/go/bin:/usr/local/bin:/usr/bin:/bin; bazelisk test //srcs/server/orchestration/...` to ensure all tests pass.

3. **Pre-commit Steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
