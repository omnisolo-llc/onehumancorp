1. **Mark Mission as PENDING -> IN_PROGRESS**
   - The mission file `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` has been marked as IN_PROGRESS and agent as Implementer.

2. **Add Hybrid Sync Metadata Database Migration**
   - Create migration `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
   - The table `swarm_memory_embeddings` will have columns `sync_status` and `last_sync_at` added via standard `ALTER TABLE ADD COLUMN`.
   - Update `srcs/server/db/BUILD.bazel` to include this migration.

3. **Implement RAGSyncService Interface**
   - Create `srcs/server/hub/rag_sync.go` with the requested interface definition `RAGSyncService`, `SyncStatus`, and `RAGSyncRecord`.
   - Write a mock implementation and flow test in `srcs/server/hub/rag_sync_test.go`.
   - Add `srcs/server/hub/BUILD.bazel` to build and test this logic.

4. **Add Metrics for Sync Mechanism**
   - Update `srcs/server/telemetry/telemetry.go` to add `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` counters.
   - Initialize these metrics in `InitWithMeter`.

5. **Verify and Pre-Commit**
   - Run `bazelisk test //...` to ensure there are no build failures or broken tests.
   - Run the pre-commit instructions as mandated.

6. **Complete Mission & Submit PR**
   - Update the mission file status from `IN_PROGRESS` to `DONE`.
   - Commit the changes and submit the PR.
