Resolves #4138 by implementing Hybrid MCP RAG Protocol sync service and database schema changes.

Added new migration file `032_hybrid_sync_metadata.sql` which adds `sync_status` and `last_sync_at` columns to `swarm_memory_embeddings` table.
Added `RAGSyncService` Go interface to handle pending syncs fetching, marking records as synced, and processing incoming cloud syncs.
Configured OpenTelemetry counters `rag_records_synced_total` and `rag_sync_errors_total` for metrics.
Added corresponding test file `rag_sync_test.go` and verified tests via local Go tooling since the `srcs` folder is `.bazelignore`d from top-level bazel build.
