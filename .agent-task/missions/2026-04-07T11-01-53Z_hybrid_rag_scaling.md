---
status: PENDING
priority: P0
estimated_scope: Large
---

# Title: Hybrid-Native RAG: Local-Private Vector Search with Seamless Cloud Scaling

## Problem Statement
The current agentic OS market defaults to cloud-only RAG architectures, exposing sensitive local telemetry, context, and intellectual property to the public internet. Competitors enforce a binary choice: entirely local (slow, unscalable) or entirely cloud (privacy-violating). OHC requires a "Hybrid-Native RAG" architecture that executes localized vector embeddings natively on Standalone Desktop Mode via SQLite (for zero-latency, private context retrieval), while seamlessly falling back to a horizontally scaled PostgreSQL `pgvector` cloud infrastructure when high-compute or collaborative mesh tasks are orchestrated.

## Research Report

### Competitive Audit

| Feature | OHC (Proposed) | Claude Code | OpenClaw | Replit Agent |
| :--- | :--- | :--- | :--- | :--- |
| **Architecture** | Hybrid (SQLite + Cloud pgvector) | Cloud-Native CLI | Local-Only | Cloud-Native IDE |
| **Privacy** | Zero-Knowledge Local Standalone | Fully Cloud-Dependent | Air-gapped but unscalable | Fully Cloud-Dependent |
| **Vector Engine** | SQLite TEXT Fallback -> pgvector | Proprietary Anthropics DB | FAISS / ChromaDB Local | Proprietary Cloud |
| **Agent Collaboration**| Teammate Mesh Pub/Sub | Single-User Context | None | Shared Workspace |

*Findings:*
1. **Claude Code** and **Replit Agent** represent state-of-the-art "Cloud" execution but suffer from absolute dependency on external infrastructure. Sensitive source code leaves the machine immediately.
2. **OpenClaw** focuses on extreme locality but fails fundamentally at swarm collaboration. It cannot burst to the cloud when complex graph-traversal tasks are required.
3. **OHC** has a unique "Blue Ocean" opportunity by utilizing the `db.Provider` interface to conditionally compile local SQLite-backed text similarity (or simple chunking) and seamlessly synchronize durable state to the multi-tenant PostgreSQL vector cloud when authorized.

### Architectural Diagram

```mermaid
graph TD
    classDef glass fill:rgba(255,255,255,0.1),stroke:rgba(255,255,255,0.2),stroke-width:1px,backdrop-filter:blur(20px),color:#fff;
    A[OHC Thin Client / Desktop]:::glass --> B{Standalone Mode?};
    B -- Yes --> C[Local SQLite Storage]:::glass;
    C --> D[Local Embeddings Model];
    B -- No --> E[OHC API Gateway]:::glass;
    E --> F[K8s PostgreSQL pgvector];
    F --> G[Cloud Embeddings Model];
    C -. "Sync (MCP Tool)" .-> F;
```

*Visual Note:* Adhere strictly to the OHC Visual Excellence Mandate utilizing the `Outfit`/`Inter` typography and glassmorphism styling (`backdrop-filter: blur(20px)`).

## Design Doc

### Architecture
The Hybrid-Native RAG will be implemented as a unified Go interface `VectorStorage`.
- **Standalone Mode (SQLite)**: Will use a simplistic serialized `TEXT` fallback for vectors if native extensions are unavailable, utilizing brute-force cosine similarity via application-layer Go, or basic BM25 keyword chunking.
- **Cloud Mode (PostgreSQL)**: Utilizes the `pgvector` extension for high-performance HNSW index similarity search.

### API Contracts
```go
type VectorStorage interface {
    // StoreEmbeddings securely persists memory chunks.
    StoreEmbeddings(ctx context.Context, namespace string, chunks []MemoryChunk) error

    // Search Similar returns top K memory chunks matching the query vector.
    SearchSimilar(ctx context.Context, namespace string, queryVector []float32, topK int) ([]MemoryChunk, error)
}
```

### DB Schema Changes
*Cloud Mode (PostgreSQL)*:
```sql
-- +goose Up
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS agent_memory_vectors (
    id UUID PRIMARY KEY,
    namespace VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose Down
DROP TABLE IF NOT EXISTS agent_memory_vectors;
```

*Standalone Mode (SQLite)*:
```sql
-- +goose Up
CREATE TABLE IF NOT EXISTS agent_memory_vectors (
    id TEXT PRIMARY KEY,
    namespace TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding_json TEXT, -- Fallback to JSON serialized array
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose Down
DROP TABLE IF NOT EXISTS agent_memory_vectors;
```
*Note*: The migration runner in `database.go` must branch logic based on `p.Provider.IsSQLite()` to prevent SQLite from executing the PostgreSQL-specific `vector(1536)` type.

## Implementation Prompt

**Role:** Implementer Agent
**Task:** Implement the Hybrid-Native RAG Vector Storage Layer for OHC.

1. **File:** `srcs/server/db/vector_storage.go`
   - Create the `VectorStorage` interface as defined in the Design Doc.
   - Implement `PostgresVectorStorage` utilizing standard `pgvector` SQL (`ORDER BY embedding <-> $1 LIMIT $2`).
   - Implement `SQLiteVectorStorage` which fetches all rows for a namespace, deserializes `embedding_json` into `[]float32`, and calculates cosine similarity in-memory using pure Go.

2. **File:** `srcs/server/db/migrations/20260408000000_agent_memory_vectors.sql`
   - Create the SQL migrations exactly as described above. Ensure you do **NOT** use `CREATE EXTENSION` in raw SQL; execute it conditionally inside `srcs/server/db/database.go`'s `RunMigrations` method using `if !p.Provider.IsSQLite()`.
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in the `embedsrcs` list.

3. **File:** `srcs/server/telemetry/rag_metrics.go`
   - Add OpenTelemetry metrics for vector search latency: `rag_vector_search_latency_seconds`.
   - Ensure the metric is a histogram and includes labels for `provider` (`postgres` vs `sqlite`) and `namespace`.
   - Update Grafana dashboards in `deploy/docker/grafana/provisioning/dashboards/` to include a panel for `histogram_quantile(0.95, sum by (le, provider) (rate(rag_vector_search_latency_seconds_bucket[5m])))`.

4. **Testing Expected:**
   - 100% unit test coverage for the Go cosine similarity calculation in SQLite fallback mode.
   - For PostgreSQL integration tests, mock the DB or utilize a testcontainer with `pgvector` installed.
   - Ensure you use `sql.NullTime` for any timestamp scanning to prevent null pointer panics.

**Acceptance Criteria:**
- The application boots in both Cloud and Standalone modes without migration panics.
- RAG search falls back to in-memory cosine similarity gracefully when backed by SQLite.
- All Bazel tests (`bazelisk test --build_tests_only //...`) pass cleanly.
