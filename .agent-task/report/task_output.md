# sqlite-vec: Hybrid Vector Search Parity

## Title
sqlite-vec 🧩 (Hybrid Vector Search Parity)

## Problem Statement
The OHC Hybrid Architecture currently relies on `pgvector` for efficient vector similarity search (cosine distance, embeddings, RAG) in the Cloud-Native Postgres environment. However, the Standalone Desktop Mode uses SQLite, which does not natively support `pgvector`. This results in feature disparity where local vector searches either fail or fall back to inefficient in-memory array scans. We need a lightweight, fully compatible vector search extension for SQLite to achieve true hybrid parity for the AutoDream pipeline and RAG synthesis.

## Research Report
- **Goal**: Integrate `sqlite-vec` into the SQLite-backed Standalone SIPDB to provide native vector search capabilities equivalent to `pgvector`.
- **Capabilities**:
  - **Vector Storage**: Efficient storage of `FLOAT` vectors using a specialized virtual table format.
  - **Similarity Search**: Native support for Cosine Similarity, L2 Distance (Euclidean), and Inner Product.
  - **Lightweight**: Written entirely in C with zero external dependencies, making it perfect for the embedded Standalone Desktop Mode.
- **Architecture Validation**:
  - `sqlite-vec` (the successor to `sqlite-vss`) supports multi-dimensional embeddings (e.g., 1536 dims for OpenAI/Ollama models) efficiently.
  - It seamlessly plugs into standard `database/sql` using `mattn/go-sqlite3` by loading the extension dynamically during connection initialization.
  - Unlike in-memory fallback scans, `sqlite-vec` enables KAIROS orchestrated AutoDream worker nodes to execute exact and approximate nearest neighbor (ANN) queries completely on the local device without a round-trip to the cloud.

## Design Doc
1. **Dependency Update**: Add the necessary CGO flags or dynamic extension loading mechanisms in `srcs/server/db/sqlite_provider.go` to load `sqlite-vec`.
2. **Database Schema**:
   - Update `migrations` to use dialect-specific logic.
   - When running against SQLite, create virtual tables using `USING vec0(embedding float[1536])` for the `swarm_memory_embeddings` and `autodream_memories` tables.
3. **Query Engine Update**:
   - Update the local `db.Provider` and `orchestration.autodream` queries to conditionally use `vec_distance_cosine(embedding, ?)` for SQLite, whereas Postgres continues to use `embedding <=> ?`.
4. **Lifecycle Hooks**:
   - Ensure the Go wrapper properly initializes the extension upon every new connection pool creation.

## Implementation Prompt
"Implement `sqlite-vec` integration for the Standalone SQLite database in `srcs/server/db/sqlite_provider.go`.
1. Load the `sqlite-vec` extension upon initialization.
2. Update the migration scripts (`docs/research/` or `srcs/server/db/migrations/`) to create virtual tables `vec0` for embeddings when the dialect is SQLite.
3. Modify `srcs/server/pipeline/autodream_pipeline.go` and `srcs/server/orchestration/autodream.go` to perform cosine similarity searches using `vec_distance_cosine` instead of falling back to in-memory loops.
4. Ensure all unit tests in `srcs/server/orchestration/` pass with the new native search."

## Priority
P1

## Estimated Scope
Medium
