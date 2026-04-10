1. *Create Database Migration*
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql` adding `sync_status` and `last_sync_at` columns to the `swarm_memory_embeddings` table.
2. *Create RAGSync Interface & Service*
   - Create `srcs/server/hub/rag_sync.go` with `RAGSyncRecord`, `RAGSyncService`, and telemetry metrics initialization.
   - I will use `[]byte` for the Vector field according to SQLite constraint.
3. *Create Unit Tests*
   - Create `srcs/server/hub/rag_sync_test.go` to mock the interface and verify the basic data flow logic.
4. *Update Build Files*
   - Run Gazelle to add the new `hub` package.
5. *Complete Pre Commit Steps*
   - Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
6. *Submit*
   - Submit the PR.
