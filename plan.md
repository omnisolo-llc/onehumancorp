1. **Mark Mission as In Progress**:
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` by using `replace_with_git_merge_diff` to exactly change the frontmatter `status` from `PENDING` to `IN_PROGRESS` and `agent` from `Researcher` to `Implementer`.
   - Use `read_file` to verify the file was updated correctly.
2. **Database Migration**:
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` using `write_file`.
   - The mission explicitly says: "Add the following columns to the `rag_memories` table (assuming such a table exists, or the primary context table): `sync_status VARCHAR(50) DEFAULT 'pending'`, `last_sync_at TIMESTAMP NULL`".
   - Using the primary context table `autodream_memories` (as discovered in `007_teammate_mesh_and_autodream.sql`), the SQL will be:
     ```sql
     ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
     ```
   - Use `read_file` to verify the migration file was created successfully.
3. **Go Interface Definition**:
   - Run `mkdir -p srcs/server/hub` using `run_in_bash_session`.
   - Create `srcs/server/hub/rag_sync.go` using `write_file` with the exact code snippet for `SyncStatus`, `RAGSyncRecord` and `RAGSyncService` interfaces from the mission file (verified to contain `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`).
   - Use `read_file` to verify the file was created successfully.
4. **Metrics & Observability**:
   - Use `replace_with_git_merge_diff` tool to update `srcs/server/telemetry/telemetry.go`. In `InitWithMeter` (which was read and explored), add variables `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` (both are now explicitly confirmed by `sed -n '100,117p' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` which states "add OpenTelemetry counters for `rag_records_synced_total` and `rag_sync_errors_total`").
   - Define `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` globally as `metric.Int64Counter` in `srcs/server/telemetry/telemetry.go`.
   - Use `read_file` to verify the telemetry file was updated successfully.
5. **Testing**:
   - Create `srcs/server/hub/rag_sync_test.go` using `write_file` to test the basic data flow logic: instantiate a mock `RAGSyncService` implementing `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`, and simulate fetching and marking records.
   - Use `read_file` to verify the file was created successfully.
   - Run tests system-wide using `run_in_bash_session` with `bazelisk test //srcs/server/...` and `bazelisk test //srcs/app/...` to verify there are no regressions.
6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit changes**:
   - Mark the mission file `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` as `status: DONE` using `replace_with_git_merge_diff`.
   - Submit the PR with the exact PR title: `Implement Hybrid MCP RAG Protocol Sync Interface`.
