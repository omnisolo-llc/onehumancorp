1. **Update .agent-task/missions/2026-04-07T08-02-24Z.md:** Set `status: IN_PROGRESS` and `agent: Jules`.
2. **Database Migration (`srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql`):**
   - Add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `swarm_memory_embeddings` (acting as the RAG memory context table). Use `ALTER TABLE ADD COLUMN` for broad compatibility without `IF NOT EXISTS` for SQLite compat.
   - Update `srcs/server/db/BUILD.bazel` to include this migration file in the `embedsrcs`.
3. **Go Interface Definition (`srcs/server/hub/rag_sync.go`):**
   - Define `SyncStatus` constants (`pending`, `synced`, `error`).
   - Define `RAGSyncRecord` struct with `ID`, `Context`, `Vector []byte`, `SyncStatus`, and `LastSyncAt`.
   - Define `RAGSyncService` interface with `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`.
   - Add OpenTelemetry metrics `rag_records_synced_total` and `rag_sync_errors_total` initialized during setup using injected `meter`.
   - Implement the `RAGSyncService` using an injected `db.Provider` so it supports SQLite and PostgreSQL. Handle conflict resolution logic (UPSERT for postgres).
4. **Unit Tests (`srcs/server/hub/rag_sync_test.go`):**
   - Create tests using an SQLite in-memory DB or mocked interface to verify `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`.
   - Test metrics initialization without panics.
5. **Run tests & Complete Pre-commit Steps:**
   - Execute `bazelisk test //...`.
   - Call `pre_commit_instructions` and complete pre-commit checks to ensure proper testing, verification, review, and reflection are done.
6. **Finalize:**
   - Mark the mission as `status: DONE`.
   - Submit the changes using a descriptive PR.
