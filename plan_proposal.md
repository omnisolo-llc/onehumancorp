1. **SQLite UDF Initialization**:
   - In `src/server/db/database.go` (or `sqlite_provider.go`), create an `init()` function that calls `sqlite.MustRegisterDeterministicScalarFunction` to register `vec_distance_cosine` for SQLite so we can perform distance calculations on embeddings in Go, parsing the JSON byte slices and calculating cosine distance.
2. **Persistent Memory Layer (`src/server/memory/vector_repository.go`)**:
   - Add `ConsolidatedMemoryRecord` type mapping to `consolidated_memory` schema.
   - Implement `UpsertConsolidatedMemory` for writing consolidated memories.
   - Implement `SearchConsolidatedMemories` for finding relevant memories based on embedding distances (for SQLite use UDF `vec_distance_cosine`, for Pg use `<->`).
3. **Conflict Resolution (`src/server/memory/autodream/service.go`)**:
   - In `Consolidate`, fetch existing memories via `SearchConsolidatedMemories`.
   - Instead of a blind upsert, construct a new LLM prompt containing `logs` and `existing_memories` to detect conflicts, deduplicate, and resolve them based on recency/facts.
   - Insert/Update the new refined consolidated memory back.
4. **Stale Context Pruning (`src/server/memory/autodream/service.go`)**:
   - Implement `PruneStaleMemories` function. It retrieves older memories (e.g. older than 6 months), optionally asks LLM to check relevance or directly deletes them if they fall outside a predefined relevance window.
5. **Testing & Parity**:
   - Add tests for `vector_repository` and `service`, specifically testing the UDF distance registration via mock interfaces and LLM resolution logic.
   - Ensure 100% code coverage.
6. Run `bazelisk test //src/server/...`
7. Run pre-commit checks.
