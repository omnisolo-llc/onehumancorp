1. **Database Migration**
   - Create migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` using `cat << 'EOF' > srcs/server/db/migrations/032_hybrid_sync_metadata.sql` containing `ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';` and `ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;`.
   - Verify file was created correctly using `cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
2. **Go Interface Definition**
   - Run `mkdir -p srcs/server/hub`
   - Create `srcs/server/hub/rag_sync.go` using `cat << 'EOF' > srcs/server/hub/rag_sync.go` to define `SyncStatus` constants, `RAGSyncRecord` struct, and the `RAGSyncService` interface as specified in the mission file.
   - Create `srcs/server/hub/BUILD.bazel` using `cat << 'EOF' > srcs/server/hub/BUILD.bazel` to define the library `go_default_library` with `name = "hub"` and `importpath = "github.com/onehumancorp/mono/srcs/server/hub"`.
   - Verify files were created correctly using `cat srcs/server/hub/rag_sync.go srcs/server/hub/BUILD.bazel`.
3. **Metrics & Observability**
   - Run the script `python3 test_metrics.py` (which I previously wrote and tested) to inject `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal` variables and their initializations into `srcs/server/telemetry/telemetry.go`.
   - Create a script `telemetry_append.py` to append the helper functions: `RecordRAGRecordSynced(ctx context.Context, count int64)` and `RecordRAGSyncError(ctx context.Context, count int64)` using `cat << 'EOF' > telemetry_append.py`, then run it `python3 telemetry_append.py`.
   - Verify changes using `git diff srcs/server/telemetry/telemetry.go`.
4. **Verification**
   - Create `srcs/server/hub/rag_sync_test.go` using `cat << 'EOF' > srcs/server/hub/rag_sync_test.go` containing a mock struct `MockRAGSyncService` implementing the interface and a test function `TestMockRAGSyncService` that calls `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync` with dummy data and asserts the mock records the correct results.
   - Verify the file creation using `cat srcs/server/hub/rag_sync_test.go`.
5. **Mark mission DONE**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` status to `DONE` using `sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
   - Verify the update using `head -n 5 .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
6. **Run final tests**
   - Run `~/go/bin/bazelisk test //...` to ensure everything works and migrations run successfully.
7. **Complete pre commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8. **Submit**
   - Submit the changes.
