1. **Create Database Migration:**
   - Write `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` to alter `swarm_memory_embeddings` (since `rag_memories` does not exist; `swarm_memory_embeddings` or `swarm_memory` is the likely RAG target context table). Wait, let's re-read the mission prompt closely: "Add the following columns to the `rag_memories` table (assuming such a table exists, or the primary context table): `sync_status VARCHAR(50) DEFAULT 'pending'`, `last_sync_at TIMESTAMP NULL`". The primary context table for vector embeddings and RAG is `swarm_memory_embeddings` based on `005_sip.sql`.
   - The migration file should use `ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';` and `ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;`.

2. **Update BUILD.bazel for DB Migration:**
   - Update `srcs/server/db/BUILD.bazel` to include `migrations/032_hybrid_sync_metadata.sql` in `embedsrcs`.

3. **Create Go Interface Definition:**
   - Create a new file `srcs/server/hub/rag_sync.go`.
   - Add the specified interface, structs, and OpenTelemetry metric definitions.

4. **Add Tests:**
   - Create `srcs/server/hub/rag_sync_test.go` to mock the interface and verify data flow logic.

5. **Update hub BUILD.bazel:**
   - Create `srcs/server/hub/BUILD.bazel` to register the new `rag_sync.go` and `rag_sync_test.go`. Ensure it depends on otel metrics appropriately.

6. **Complete pre commit steps:**
   - Ensure proper testing, verification, review, and reflection are done.

7. **Submit the change.**
   - Once all tests pass, submit the change with a descriptive commit message.
