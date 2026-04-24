# Architecture & Plan

1. **Design and Implement Persistent Memory Layer (`SemanticSearch`)**
   - Update `src/server/memory/vector_repository.go` to fully implement `SemanticSearch`.
   - The method needs to be aware of the database type. `db.Provider` has an `IsSQLite()` method.
   - For PostgreSQL, use the pgvector extension: `ORDER BY embedding <-> $2::vector ASC`. Format the query embeddings correctly.
   - For SQLite, fallback to full-text or simply latest retrieval (since cosine distance UDFs are tricky in this setup and we're missing C extensions, we will gracefully degrade to `ORDER BY created_at DESC`).
   - Write tests for `SemanticSearch` in a new `src/server/memory/vector_repository_test.go`.

2. **Design and Implement Stale Context Pruning**
   - Add a `Prune(ctx context.Context, organizationID string, olderThan time.Time) error` method to `VectorRepository`.
   - This method will run a `DELETE FROM autodream_memories_master WHERE organization_id = $1 AND created_at < $2`.
   - In `src/server/memory/autodream/service.go`, add a background cleanup mechanism or a method that can be invoked periodically to prune memories older than e.g. 6 months. Actually, we will add a `PruneStaleMemories` function to `Service`.

3. **Design and Implement Conflict Resolution**
   - When a new memory is being inserted (e.g. "Maya's cake price is $55"), we first `SemanticSearch` for similar recent memories. If a highly similar memory exists and they conflict, we should resolve it. But LLM is needed for this.
   - A simpler approach requested: Detect when the same fact is stored and resolve by recency (overwrite/update the old fact).
   - We will add `UpsertWithConflictResolution` to `autodream.Service`. It uses `SemanticSearch` to find similar existing records. It passes them to the LLM to identify conflicts. If conflicts exist, the LLM generates a consolidated memory, we prune the old conflicting records, and insert the new one.
   - We will implement this in `src/server/memory/autodream/service.go` by updating `Consolidate`. The LLM prompt can be updated to include previous relevant context, or a new `ResolveConflicts` step can be added.

4. **Add Tests**
   - Provide 100% test coverage for `src/server/memory/vector_repository.go`.
   - Write a mock LLM test for conflict resolution in `src/server/memory/autodream/service_test.go`.

5. **Pre Commit Steps**
   - Complete pre-commit steps to make sure proper testing, verifications, reviews, and reflections are done.
