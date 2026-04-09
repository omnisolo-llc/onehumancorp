1. **Apply Database Migration:**
   Create `032_hybrid_rag_sync.sql` in `srcs/server/db/migrations/` with SQL that alters the `autodream_memories` table to add `sync_status` and `last_sync_at` columns. Note: we will use simple `ALTER TABLE ADD COLUMN` statements as SQLite doesn't fully support `IF NOT EXISTS` or multiple columns in a single `ALTER TABLE` statement. Ensure we update existing rows with `sync_status = 'pending'`. Use `cat` to verify the created file. Add the file to `srcs/server/db/BUILD.bazel` using `sed`. Use `cat srcs/server/db/BUILD.bazel` to verify the modified `BUILD.bazel`.

2. **Implement Go Interface Definition:**
   Create directory `srcs/server/hub`. Create `srcs/server/hub/rag_sync.go` defining the interfaces and structs exactly as requested in the mission prompt: `SyncStatus`, `SyncStatusPending`, `SyncStatusSynced`, `SyncStatusError`, `RAGSyncRecord`, and `RAGSyncService`. Create the interface containing `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`. Use `cat` to verify the created file. Run `~/go/bin/bazelisk run //:gazelle` to generate `BUILD.bazel` for the new directory. Use `cat srcs/server/hub/BUILD.bazel` to verify.

3. **Metrics & Observability:**
   Update `srcs/server/telemetry/telemetry.go` to add OpenTelemetry counters for `rag_records_synced_total` and `rag_sync_errors_total`. Add global vars `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal`, initialize them in `InitWithMeter`, and add record functions `RecordRAGRecordsSynced` and `RecordRAGSyncError` which will check if counters are nil before adding. Use `git diff` or `cat` to verify the modifications to `telemetry.go`.

4. **Add Unit Tests:**
   Create `srcs/server/hub/rag_sync_test.go` to mock the interface and verify the basic data flow logic, as explicitly requested by the Implementation Prompt in the mission file: "Write unit tests in `rag_sync_test.go` to mock the interface and verify the basic data flow logic". We will declare a `MockRAGSyncService` implementing `RAGSyncService` and write basic tests. Use `cat` to verify the created test file. Run `~/go/bin/bazelisk run //:gazelle` to generate the test targets. Use `cat srcs/server/hub/BUILD.bazel` to verify. Note: Since the prompt ONLY asks for foundational schemas and mock interfaces ("Implement the foundational schema changes and the Go synchronization service interface... Write unit tests ... to mock the interface"), we will not build out full database interactions as instructed by the Implementation Prompt scope guideline in memory.

5. **Update Mission File:**
   Update the mission file `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` status to `DONE` and agent to `Link`. Use `sed -i -e 's/agent: Researcher/agent: Link/' -e 's/status: PENDING/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to modify frontmatter. Use `cat` to verify.

6. **Testing:**
   Run `~/go/bin/bazelisk test //srcs/server/... //srcs/app/... --test_output=errors` to verify all backend and frontend integrity safely.

7. **Complete Pre-commit Steps:**
   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit Change:**
   Submit the PR with branch name `implement-hybrid-mcp-rag-sync`.
