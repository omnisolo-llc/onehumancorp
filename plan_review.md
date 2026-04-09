The mission asks to implement the Hybrid MCP RAG Protocol, described in `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
The task specifies adding columns to `rag_memories` table (or similar context table). I'll use `swarm_memory_embeddings` from `005_sip.sql` as the base table since `rag_memories` doesn't exist.

Step 1: Create new migration `032_hybrid_rag_sync.sql`
Add columns `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `swarm_memory_embeddings` table. Since SQLite does not fully support multiple columns in a single ALTER TABLE, we will use separate ALTER TABLE statements for each new column.

Step 2: Add Bazel embed directive
Update `srcs/server/db/BUILD.bazel` to include `032_hybrid_rag_sync.sql` in `embedsrcs` for the database schema package.

Step 3: Define Go Interface
Create `srcs/server/sync/rag_sync.go` (since `hub` directory doesn't exist, I will use `sync` directory instead, or create `hub`?). Wait, the prompt says "Create a new file `srcs/server/hub/rag_sync.go`." So I will create `srcs/server/hub/` and `srcs/server/hub/rag_sync.go`.

Step 4: Observability & OpenTelemetry
In `srcs/server/hub/rag_sync.go` or telemetry, add `rag_records_synced_total` and `rag_sync_errors_total`.

Step 5: Tests
Add `srcs/server/hub/rag_sync_test.go` and implement tests.
Add BUILD.bazel for `srcs/server/hub` using gazelle.
