---
Title: "KAIROS Phase 3: AutoDream Data Pipeline Architecture"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
Agents generate massive amounts of ephemeral execution logs and context during operations. Without a long-term memory consolidation system, this episodic memory is lost or bloated, preventing the Swarm from learning and evolving across sessions.

# Research Report
- AutoDream requires processing raw context, generating summarized embeddings via Minimax LLM, and storing them in an index.
- OHC uses `pgvector` for vector storage in PostgreSQL (Cloud-Native Mode). In Standalone Mode, vectors are stored as encoded JSON byte arrays in SQLite to maintain compatibility.
- Vector arrays are strictly encoded as `[]byte` in Go structs, NOT `[]float32`.

# Design Doc
**Pipeline Flow:**
1. Read raw context from ephemeral memory tables.
2. Call Minimax LLM to generate `[]float32` embeddings.
3. Serialize `[]float32` to `[]byte` via JSON encoding.
4. Upsert into `autodream_memories` table (utilizing `pgvector` operators in PG or simple lookup in SQLite).

# Implementation Prompt
Hello Implementer agent! Your mission is to build the AutoDream pipeline.
1. Create `srcs/server/orchestration/autodream_pipeline.go`.
2. Implement an asynchronous worker that polls `ephemeral_logs` and pipes text into the embedding API.
3. Crucially, ensure the final vector is serialized: `embeddingBytes, _ := json.Marshal(floatArray)`.
4. Perform inserts into `autodream_memories` using the custom `db.Provider` interface context methods (e.g., `dbProvider.Begin(ctx)`).
5. Run tests locally utilizing `t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")`.

# Visual Excellence Guidelines
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
