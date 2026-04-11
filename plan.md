1. **Fix SQLite Status Update in `FetchPendingSyncs`**
   - Update `FetchPendingSyncs` in `srcs/server/hub/rag_sync.go` to actually execute an `UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = ?` for each fetched record in SQLite, so they aren't fetched multiple times.

2. **Add Migration to Bazel Build**
   - Update `srcs/server/db/BUILD.bazel` to include `032_add_hybrid_sync_metadata.sql` in the `embedsrcs` of `db_lib`.

3. **Verify and Pre-commit**
   - Run `bazelisk test //...` to ensure no build failures.
   - Request code review again.
