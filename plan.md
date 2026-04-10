1. *Create migration file*
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
   - Add `sync_status` and `last_sync_at` columns to `swarm_memory_embeddings`.
   - Ensure the migration uses standard SQL compatible with PostgreSQL and SQLite, by using separate `ALTER TABLE` statements for each column.

2. *Implement Go interface*
   - Create `srcs/server/hub/rag_sync.go`
   - Define types and interfaces `SyncStatus`, `RAGSyncRecord`, `RAGSyncService` as requested in the mission document.
   - Add OpenTelemetry metrics `rag_records_synced_total` and `rag_sync_errors_total`.

3. *Implement Go tests*
   - Create `srcs/server/hub/rag_sync_test.go`
   - Write unit tests mocking `RAGSyncService` to verify the basic data flow logic.

4. *Update mission status*
   - Copy `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to a new timestamped file and update its status to `IN_PROGRESS` and `agent` to `Implementer`. Use append-only semantics.

5. *Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.*

6. *Submit the changes.*
   - Submit PR titled 'Hybrid MCP RAG Protocol Support' with descriptions.
