1. **Create Database Migration:**
   - Use `run_in_bash_session` with a `cat << 'EOF'` command to create `srcs/server/db/migrations/20260428000000_alter_shared_tasks_schema.sql` containing the ALTER TABLE statements.

2. **Verify Database Migration:**
   - Use `run_in_bash_session` to run `cat srcs/server/db/migrations/20260428000000_alter_shared_tasks_schema.sql`.

3. **Update BUILD.bazel:**
   - Use `run_in_bash_session` with a Python script to insert `20260428000000_alter_shared_tasks_schema.sql` into the `srcs` list of the corresponding migration target in `srcs/server/db/BUILD.bazel`.

4. **Verify BUILD.bazel:**
   - Use `run_in_bash_session` to run `cat srcs/server/db/BUILD.bazel | grep 20260428000000_alter_shared_tasks_schema.sql`.

5. **Update DAO Models:**
   - Use `run_in_bash_session` with a `cat << 'EOF'` command to write the `SharedTask` struct definition to `srcs/server/db/models/shared_tasks.go`.

6. **Verify DAO Models:**
   - Use `run_in_bash_session` to run `cat srcs/server/db/models/shared_tasks.go`.

7. **Update DAO Repository:**
   - Use `run_in_bash_session` with a Python script to modify `srcs/server/db/shared_tasks_repo.go` to add methods for creating and claiming shared tasks (with logic for PG `FOR UPDATE SKIP LOCKED` and SQLite `UPDATE ... RETURNING`).

8. **Verify DAO Repository:**
   - Use `run_in_bash_session` to run `cat srcs/server/db/shared_tasks_repo.go`.

9. **Update API Handlers:**
   - Use `run_in_bash_session` with a `cat << 'EOF'` command to implement handler logic for creating and claiming shared tasks in `srcs/server/api/tasks/tasks.go`.

10. **Verify API Handlers:**
    - Use `run_in_bash_session` to run `cat srcs/server/api/tasks/tasks.go`.

11. **Update API Tests:**
    - Use `run_in_bash_session` with a `cat << 'EOF'` command to create `srcs/server/api/tasks/tasks_test.go` containing tests covering the creation and claiming of shared tasks via HTTP.

12. **Verify API Tests:**
    - Use `run_in_bash_session` to run `cat srcs/server/api/tasks/tasks_test.go`.

13. **Update DAO Repository Tests:**
    - Use `run_in_bash_session` with a `cat << 'EOF'` command to create `srcs/server/db/shared_tasks_repo_test.go` containing tests covering the PG and SQLite implementations of the create and claim methods in the DAO.

14. **Verify DAO Repository Tests:**
    - Use `run_in_bash_session` to run `cat srcs/server/db/shared_tasks_repo_test.go`.

15. **Run Tests:**
    - Run `bazelisk test //...` to ensure all tests pass.

16. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

17. Use the `submit` tool to commit the code with title and description. Output a final unstructured message containing issue_id: 5622.
