1. **Mark Mission IN_PROGRESS**: Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to set `status: IN_PROGRESS` and `agent: Implementer`.
2. **Step 1: Database Migration**:
   - Get the next migration number using `ls -l srcs/server/db/migrations/ | grep -E "^[0-9]{3}_" | sort | tail -n 1`.
   - Create the migration file (e.g. `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`).
   - Add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `swarm_memory_embeddings`.
   - Update `srcs/server/BUILD.bazel` or run `bazelisk run //:gazelle` to include the new migration file.
3. **Step 2: Go Interface Definition**:
   - Create `srcs/server/hub/rag_sync.go` with the `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` definitions as requested.
   - Run `bazelisk run //:gazelle` to update BUILD files.
4. **Step 3: Metrics & Observability**:
   - Add variables `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` of type `metric.Int64Counter` to `srcs/server/telemetry/telemetry.go`.
   - Add their initialization in `InitWithMeter`.
   - Add helper methods `RecordRagSyncSuccess` and `RecordRagSyncError` in `telemetry.go`.
5. **Step 4: Verification**:
   - Create `srcs/server/hub/rag_sync_test.go` to mock the interface and test basic data flow logic.
   - Run `bazelisk test //srcs/server/... --test_output=errors`.
6. **Pre-commit**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit**: Mark mission `status: DONE` and create a PR with title "🗺️ Guide: [Hybrid MCP RAG Protocol]".
