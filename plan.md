1. **Database Migration**
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql` with two `ALTER TABLE` statements to add `sync_status` and `last_sync_at` to `swarm_memory_embeddings`.
   - Update `srcs/server/db/BUILD.bazel` to add the new migration to `embedsrcs`.

2. **Go Interface & Metrics Definition**
   - Create directory `srcs/server/hub/`.
   - Create `srcs/server/hub/rag_sync.go` defining `RAGSyncService`, `RAGSyncRecord`, and `SyncStatus`.
   - Add OpenTelemetry metrics as global variables for `rag_records_synced_total` and `rag_sync_errors_total`.

3. **Write Unit Tests**
   - Create `srcs/server/hub/rag_sync_test.go` with a mock implementation of `RAGSyncService` to verify the logic and metrics.

4. **Update BUILD.bazel**
   - Run `bazelisk run //:gazelle` to generate `BUILD.bazel` for `srcs/server/hub/`.
   - Verify that the metrics imports are correct.

5. **Run Pre-Commit Checks**
   - Ensure proper testing, verification, review, and reflection are done by running `pre_commit_instructions` and addressing any checks.

6. **Update Mission Status & Submit**
   - Mark the mission file `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` as `DONE`.
   - Submit the changes using the `submit` tool.
