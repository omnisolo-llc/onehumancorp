# Title: [integrations] Hybrid Vector DB MCP

## Problem Statement
OHC supports both Cloud-native (Kubernetes/Postgres/Redis) and Standalone (SQLite) modes. However, agents need a unified vector database interface for episodic memory and semantic search that functions optimally in both environments. In Cloud mode, we should leverage a scalable vector database solution (e.g., pgvector, Milvus). In Standalone mode, we need a lightweight local alternative (e.g., sqlite-vss) to ensure agents can retrieve long-term context without cloud dependencies.

## Research Report
Market research indicates that typical agentic frameworks (like CrewAI, AutoGen) assume a monolithic backend for vector storage (like Pinecone or Qdrant) or rely entirely on local embeddings, making them inflexible for hybrid deployments. OHC's "Unfair Advantage" requires an MCP Tool that abstracts the vector DB interaction, dynamically routing queries and embeddings based on the `OHC_MULTITENANT` configuration, without the agent needing to implement the underlying DB connections.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/vector_db/`.
- Introduce a `VectorStoreManager` that implements the MCP Tool interface.
- Dynamically route based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize Postgres with `pgvector` for scalable, multi-tenant vector storage.
- **Standalone Mode:** Utilize `sqlite-vss` (or an equivalent local embedding store) for zero-dependency local operation.

**API Contracts:**
- `StoreEmbedding(ctx async context, collection string, id string, vector []float32, metadata map[string]interface{}) error`
- `SearchSimilar(ctx async context, collection string, vector []float32, limit int) ([]SearchResult, error)`

**Security:**
- Must validate `organization_id` in cloud mode.
- Apply `RedactInterfacePII` to metadata before storing it.

## Implementation Prompt
"Implement the Hybrid Vector DB MCP tool in `src/server/lib/integrations/vector_db/`.
1. Create `vector_db.rs` defining the `VectorStoreManager` and its MCP capabilities (`StoreEmbedding` and `SearchSimilar`).
2. Implement driver-agnostic logic. To determine if the connection is Cloud, use: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`. For Cloud, implement the logic using `pgvector`. For Standalone, use `sqlite-vss`.
3. Ensure `organization_id` filtering is rigidly applied in Cloud Mode.
4. Create tests in `vector_db_test.rs` mocking both `pgvector` and `sqlite-vss`.
5. Update or create the adjacent `BUILD.bazel` file, ensuring `srcs` array accurately reflects the new files."

## Priority
P1

## Estimated Scope
Medium
