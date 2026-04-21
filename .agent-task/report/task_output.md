# SQLite-vec Integration for Standalone Vector Search Parity

## Title
SQLite-vec 🧲 (Local Vector Search Parity)

## Problem Statement
The OHC Hybrid Architecture relies on `pgvector` for semantic search and memory retrieval in the Cloud-Native mode (PostgreSQL). However, the Standalone Desktop Mode uses SQLite (`modernc.org/sqlite`), which currently mocks or skips vector operations (e.g., in `AutoDreamPipeline` and `autodream.go`). This creates a gap where offline-first standalone clients cannot perform native vector similarity searches, severely degrading the capabilities of memory retrieval and the AutoDream pipeline locally.

## Research Report
- **Goal**: Achieve native vector search parity in Standalone mode by integrating `sqlite-vec` into the SQLite database provider.
- **Capabilities**:
  - `sqlite-vec` provides vector search capabilities for SQLite, functioning similarly to `pgvector` in PostgreSQL.
  - It supports fast similarity search via cosine distance, L2 distance, etc.
- **Architecture Validation**:
  - Currently, `srcs/server/db/database.go` strips out `pgvector` specific migrations and uses text-based fallback for SQLite.
  - `srcs/server/pipeline/autodream_pipeline.go` and `srcs/server/agents/autodream.go` explicitly skip vector similarity searches on SQLite because it is unsupported.
  - By loading the `sqlite-vec` extension when initializing the SQLite connection in `srcs/server/db/database.go`, the application can perform native vector operations locally.

## Design Doc
1. **Architecture Update**:
   - Update `srcs/server/db/database.go` to load the `sqlite-vec` extension upon opening a SQLite connection.
   - Adjust `srcs/server/db/database.go` migration logic to translate `pgvector` types to `sqlite-vec` compatible virtual tables or columns instead of stripping them entirely.
2. **Component Integration**:
   - `srcs/server/pipeline/autodream_pipeline.go`: Remove the `!provider.IsSQLite()` guard around vector searches.
   - `srcs/server/agents/autodream.go`: Refactor the similarity search queries to use `sqlite-vec` functions (e.g., `vec_distance_cosine`) when connected to SQLite, maintaining `pgvector`'s `<->` operator for Postgres.
3. **API Contracts**:
   - Internal DB interfaces remain the same, but the queries will execute successfully on both Postgres and SQLite.

## Implementation Prompt
"Integrate `sqlite-vec` into the OHC Standalone Desktop Mode. Modify `srcs/server/db/database.go` to load the `sqlite-vec` extension when connecting to SQLite via `modernc.org/sqlite` (or swap to a driver that supports loading the extension if `modernc` does not). Update the migration parsing logic in `RunMigrations` to translate `pgvector` types appropriately for `sqlite-vec`. Finally, update `srcs/server/pipeline/autodream_pipeline.go` and `srcs/server/agents/autodream.go` to perform true vector similarity searches on SQLite using `sqlite-vec` functions, removing the current mock/skip logic. Ensure all unit and E2E tests pass for both database modes."

## Priority
P1

## Estimated Scope
Medium
