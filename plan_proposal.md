1. **Apply Database Migration:**
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql` extending `autodream_memories` table (as this serves as the persistent RAG memory table according to previous `024` and `029` migrations). The instruction mentioned `rag_memories` but the actual table existing for vector context memories is `autodream_memories`.
   - The migration will add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMPTZ NULL` or `TIMESTAMP NULL`. (We'll use `VARCHAR` and `TIMESTAMP` for SQLite/PostgreSQL compatibility).
   - Update `srcs/server/db/BUILD.bazel` to include this new migration file in `embedsrcs`.

2. **Define Go Interface (`srcs/server/hub/rag_sync.go`):**
   - Create `srcs/server/hub/rag_sync.go`.
   - Define `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interface as requested.
   - Implement the actual interface `ragSyncService` struct with a `db.Provider` dependency.
   - Add OpenTelemetry metrics `rag_records_synced_total` and `rag_sync_errors_total` using standard `go.opentelemetry.io/otel/metric`.

3. **Implement RAGSyncService Methods:**
   - `FetchPendingSyncs`: Query `autodream_memories` for `sync_status = 'pending'`, casting the `VECTOR` as text for cross-db compatibility (Postgres handles vector, but we must use standard strings locally, or just `embedding::text` in Postgres, but the memory hints suggest `CAST(embedding AS TEXT)`).
   - `MarkSynced`: Update `sync_status = 'synced'` and `last_sync_at = CURRENT_TIMESTAMP` for given IDs.
   - `ProcessIncomingSync`: Upsert incoming records to `autodream_memories`. Use `ON CONFLICT (id) DO UPDATE SET` syntax.

4. **Implement Unit Tests:**
   - Create `srcs/server/hub/rag_sync_test.go` and `srcs/server/hub/BUILD.bazel`.
   - Test basic logic utilizing an SQLite test provider using `db.NewSqliteProvider(sqlDB)`.
   - Manually execute `CREATE TABLE` and the new schema additions during test setup since Goose isn't automatically run in tests.

5. **Update Mission File:**
   - Prepend `status: IN_PROGRESS` and `agent: Miser` to `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
   - Update the status to `DONE` after tests pass.
