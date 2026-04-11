1. **Create Database Migration:**
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql`.
   - Add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMPTZ` to the `swarm_memory_embeddings` table.

2. **Update db BUILD.bazel:**
   - Add `migrations/032_add_hybrid_sync_metadata.sql` to `embedsrcs` in `srcs/server/db/BUILD.bazel`.

3. **Create Go Interface for Syncing:**
   - Create `srcs/server/hub/rag_sync.go` with the interfaces and structs mentioned in the prompt (and metrics!).
   - Note: The prompt tells us to define `[]float32` as a `[]byte` based on the ground rule: "When defining Go structs that interface with the swarm_memory_embeddings table for RAG features, map the vector_embedding field to []byte to match the underlying database schema's BYTEA (PostgreSQL) or BLOB (SQLite) column types, rather than []float32." Thus, `RAGSyncRecord` will use `[]byte` for the vector instead of `[]float32`.

4. **Update hub BUILD.bazel:**
   - Create `srcs/server/hub/BUILD.bazel` to expose the package. Wait, I should verify what's currently in `hub`, it didn't exist, I'll need to create the BUILD.bazel file. Or, wait, we need to add `hub` as a dependency in `srcs/server/BUILD.bazel`.

5. **Create tests:**
   - Create `srcs/server/hub/rag_sync_test.go` to verify the logic/mocks.

6. **Run checks:**
   - Run `bazelisk test //...`.

7. Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.

8. Submit changes via PR.
