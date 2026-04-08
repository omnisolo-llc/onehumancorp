1. Add columns `sync_status` and `last_sync_at` to the `swarm_memory_embeddings` table.
   - I'll create `srcs/server/db/migrations/032_hybrid_rag_sync_metadata.sql` (or whatever the next number is).
   - Use `ALTER TABLE swarm_memory_embeddings ADD COLUMN ...`.
   - Update `swarm_memory_embeddings` from `005_sip.sql` directly or via migration? The prompt asked for migration `0005_add_hybrid_sync_metadata.sql` but `005` already exists. I'll create a new migration like `032_hybrid_rag_sync_metadata.sql` as it will be executed sequentially.

2. Implement the Go Interface `RAGSyncService`
   - I will create `srcs/server/hub/rag_sync.go`.
   - Define the structs, interfaces, and open telemetry counters.

3. Write tests.
   - Create `srcs/server/hub/rag_sync_test.go` and ensure tests pass.

4. Make sure tests pass and complete pre-commit instructions.
