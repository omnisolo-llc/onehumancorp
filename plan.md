1.  **Database Migration**:
    *   Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
    *   Add columns `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `swarm_memory_embeddings` (acting as the RAG memory table based on existing schemas). Use separate `ALTER TABLE` statements for SQLite compatibility.
    *   Update `srcs/server/db/BUILD.bazel` to include `migrations/032_hybrid_sync_metadata.sql` in `embedsrcs`.

2.  **Go Interface Definition**:
    *   Create `srcs/server/hub/rag_sync.go`.
    *   Define the `SyncStatus`, `RAGSyncRecord` and `RAGSyncService` interface as described in the mission prompt.
    *   Ensure the vector embedding in `RAGSyncRecord` matches the requested struct `[]float32`.
    *   Create `srcs/server/hub/BUILD.bazel` to properly expose this package.

3.  **Metrics & Observability**:
    *   Update `srcs/server/telemetry/telemetry.go`.
    *   Add `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal` counters.
    *   Implement init logic in `InitWithMeter`.
    *   Add recording functions `RecordRAGRecordSynced(ctx)` and `RecordRAGSyncError(ctx)`.

4.  **Verification (Tests)**:
    *   Create `srcs/server/hub/rag_sync_test.go` and add unit tests to mock `RAGSyncService` and verify basic data flow.

5.  **Pre-commit & Submit**:
    *   Run `bazelisk test //srcs/server/...` to verify all tests pass.
    *   Mark mission `status: DONE` and verify file modification visually.
    *   Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
    *   Create a PR and submit the branch.
