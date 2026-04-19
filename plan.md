1. **Update `sync_daemon.go`**:
   - In `ProcessSync`, update the SQL queries to target missions where `status = 'CLOUD_ESCALATION'` instead of `PENDING` or `BURSTING`.
     - Update PostgreSQL query: `SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = 'CLOUD_ESCALATION' LIMIT 500`
     - Update SQLite query: `SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = 0 AND status = 'CLOUD_ESCALATION' LIMIT 500`

2. **Update `sync_daemon_test.go`**:
   - Create or update the `ClearSemaphore()` cleanup function for `sync_daemon_test.go`. Ensure it exists in a `_test.go` file (e.g., `sync_daemon_test.go` or `agent_context_test.go`).
   - Modify the mock test data to insert missions with `status = 'CLOUD_ESCALATION'` instead of `PENDING`.
   - Modify the assertions to check for `CLOUD_ESCALATION` status instead of `PENDING`.

3. **Verify tests and Build**:
   - Run `bazelisk test //...` to ensure all tests pass.

4. **Complete pre-commit steps**:
   - Ensure proper testing, verification, review, and reflection are done by calling `pre_commit_instructions`.

5. **Submit the PR**:
   - Submit the PR with standard conventions. Format the title as `🧹 Maintainer: [Hybrid Security Fix] Implement Hybrid MCP RAG Protocol Daemon`. Ensure description contains 💡 What, 🎯 Why, 📊 Impact, and 🔬 Measurement. Include `issue_id: 4038` in the final message.
