1. **Create Database Migration:**
   - Create `srcs/server/db/migrations/032_hybrid_mcp_rag_sync.sql` using `write_file`.
   - Content: `ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';` and `ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;`. (Based on memory rules, `autodream_memories` is the correct table for Hybrid MCP RAG protocol sync metadata).
   - Verify using `cat srcs/server/db/migrations/032_hybrid_mcp_rag_sync.sql` via `run_in_bash_session`.

2. **Update BUILD.bazel for Migrations:**
   - Modify `srcs/server/db/BUILD.bazel` to include `"migrations/032_hybrid_mcp_rag_sync.sql",` in the `embedsrcs` list using `replace_with_git_merge_diff`.
   - Verify using `cat srcs/server/db/BUILD.bazel` via `run_in_bash_session`.

3. **Create Go Interface and Concrete Implementation:**
   - Modify `srcs/server/hub/rag_sync.go` using `write_file`.
   - Include `RAGSyncRecord`, `RAGSyncService` interfaces, `SyncStatus` constants exactly as defined in the mission file (`FetchPendingSyncs`, `MarkSynced`, `ProcessIncomingSync`). Include OpenTelemetry counters `rag_records_synced_total` and `rag_sync_errors_total` globally via `otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")`.
   - Create a concrete struct `DefaultRAGSyncService` that implements `RAGSyncService`. It will hold a `db.Provider` and implement the 3 required methods.
   - Verify using `cat srcs/server/hub/rag_sync.go` via `run_in_bash_session`.

4. **Create Unit Tests:**
   - Create `srcs/server/hub/rag_sync_test.go` using `write_file`.
   - Implement tests that exercise the actual concrete service struct `DefaultRAGSyncService` by injecting an in-memory SQLite dependency `db.DB{Provider: db.NewSqliteProvider(sqliteDB)}`.
   - Before running tests, ensure `DROP TABLE IF EXISTS autodream_memories;` is called, and then recreate it.
   - Write tests for `FetchPendingSyncs` to ensure it retrieves correct rows, `MarkSynced` to ensure status gets updated, and `ProcessIncomingSync` to verify records are properly saved.
   - Verify using `cat srcs/server/hub/rag_sync_test.go` via `run_in_bash_session`.

5. **Configure BUILD.bazel for Hub Package:**
   - Create `srcs/server/hub/BUILD.bazel` using `write_file`.
   - Include `rag_sync.go` in `go_library` and `rag_sync_test.go` in `go_test`, along with dependencies: `@io_opentelemetry_go_otel//:otel`, `@io_opentelemetry_go_otel_metric//:metric`, `"//srcs/server/db"`, and `@org_modernc_sqlite//:sqlite`.
   - Verify using `cat srcs/server/hub/BUILD.bazel` via `run_in_bash_session`.

6. **Run Tests:**
   - Run tests for the new package via `run_in_bash_session` using `bazelisk test //srcs/server/hub/...`.

7. **Update Mission Status:**
   - Modify `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to change `status: PENDING` to `status: DONE` using `replace_with_git_merge_diff`.
   - Verify using `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` via `run_in_bash_session`.

8. **Check Links Validation:**
   - Run `./check_links.sh` via `run_in_bash_session` to validate markdown modification.

9. **Pre-commit Steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. **Submit:**
    - Submit changes with the `default_api:submit` tool.
