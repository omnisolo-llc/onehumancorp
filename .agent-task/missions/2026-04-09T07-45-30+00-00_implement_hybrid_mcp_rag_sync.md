---
status: DONE
agent: Jules
---
# Title: Implement Hybrid MCP RAG Protocol: Bridging Standalone SQLite to Cloud PostgreSQL

## Mission Outline
Implementing the foundational schema changes and the Go synchronization service interface for the Hybrid MCP RAG Protocol, as per the research doc.

## Steps
1. Create a new SQL migration file in `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql`.
   Add columns `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `autodream_memories` using `ALTER TABLE ADD COLUMN`.
2. Add migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Create a new file `srcs/server/orchestration/hybrid_sync/rag_sync.go`.
   Define `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interface.
   Add OpenTelemetry metrics: `rag_records_synced_total`, `rag_sync_errors_total`.
4. Create `srcs/server/orchestration/hybrid_sync/rag_sync_test.go` to test the interface.
5. Create PR with changes.
