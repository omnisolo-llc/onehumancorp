---
status: PENDING
Title: "KAIROS Phase 3: Architect AutoDream Data Pipelines for OHC Memory"
Priority: P0
Estimated Scope: Large
---

# Problem Statement
As the OHC Swarm executes tasks, it generates vast amounts of intermediate artifacts, logs, and session data. For the system to continuously evolve and leverage "Swarm Intelligence", this ephemeral data must be consolidated into long-term memory. We lack the concrete data pipeline architecture (AutoDream) to compress these logs using Minimax LLMs and store them using `pgvector` for semantic retrieval by agents.

# Research Report
- AutoDream is the core memory consolidation system of the OHC AI OS.
- It must support Cloud-Native multi-tenant isolation via PostgreSQL (`pgvector`) and degrade gracefully for standalone usage using SQLite (using a compatible VSS extension or fallback keyword search).
- According to memory, we use Minimax LLMs for compressing artifacts and generating embeddings before storing them in the `autodream_memories` table.
- A background worker process must periodically pull completed/ephemeral tasks from the orchestration layer, process them, and insert them into the vector index.

# Design Doc
**Database Schema (`autodream_memories`):**
```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    source_task_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536), -- Assuming standard 1536 dimensional embeddings
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_autodream_memories_org ON autodream_memories(organization_id);
-- Example HNSW index for efficient semantic search
CREATE INDEX IF NOT EXISTS idx_autodream_embedding_hnsw ON autodream_memories USING hnsw (embedding vector_cosine_ops);
```

**Pipeline Components:**
1. **Collector:** Reads finished tasks/logs.
2. **Compressor:** Calls the Minimax LLM to summarize the task output and extract key architectural or context lessons.
3. **Embedder:** Calls the Minimax Embedding API to vectorize the summary.
4. **Indexer:** Stores the resulting vector and metadata in `pgvector`.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the "AutoDream Data Pipeline".
1. Create the database migrations for `autodream_memories` in `srcs/server/db/migrations/` enabling `pgvector`.
2. Implement the AutoDream worker loop in `srcs/server/orchestration/autodream_worker.go`. The worker should run periodically, query completed tasks, and process them.
3. Implement the `Compressor` and `Embedder` services using the existing Minimax LLM client integrations in the codebase.
4. Implement semantic search queries in the data access layer (`autodream_db.go`) using cosine similarity (`<=>`).
5. Ensure graceful degradation for SQLite if `pgvector` isn't available locally.
6. Create unit tests for your data access layer. Mock the Minimax client during tests.
7. Verify your work using `bazelisk test //srcs/server/orchestration/...`

# Visual Excellence Guidelines
If you are modifying any dashboards or UI representing AutoDream Memory, you must use:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
