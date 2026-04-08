---
status: IN_PROGRESS
agent: Jules
Title: "KAIROS Orchestration: AutoDream Data Pipelines"
Priority: P0
Estimated Scope: Medium
---

# Title
KAIROS Orchestration: AutoDream Data Pipelines for Memory Consolidation

# Problem Statement
The OHC Swarm Intelligence Protocol (OHC-SIP) dictates that all agents share memory. To prevent context window overflow and enable long-term reasoning, OHC needs an asynchronous "AutoDream" background pipeline. This pipeline must consolidate ephemeral session contexts (`agent_session_data`), prune redundancies, and inject the embedded truth into a durable vector database.

# Research Report
* The AutoDream Data Pipeline acts as the long-term memory consolidation engine.
* It must adapt its storage mechanism based on the OHC operating mode.
* **Cloud-Native Mode**: Utilizes PostgreSQL with the `pgvector` extension for exact Nearest Neighbor search on 1536-dimensional embeddings.
* **Standalone Mode**: Degrades gracefully to SQLite. Embeddings are stored as JSON text blobs, with fallback semantic search mechanisms.
* Memory artifacts are typically stored as `.agent-task/memory/*.yml` files.
* Minimax or Cohere LLM APIs will be utilized for generating embeddings.

# Design Doc
**Architecture Strategy**:
* Implement an `AutoDreamWorker` daemon orchestrator that periodically sweeps completed `shared_tasks` and memory contexts.
* **Extraction**: Poll recent `.agent-task/memory/*.yml` and session data.
* **Embedding**: Call the LLM client (e.g., `srcs/server/agents/local/llm.go`) to generate embeddings.
* **Loading**: Upsert into the `autodream_memories` table.

**Database Schema (Cloud/pgvector)**:
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
```

# Implementation Prompt
You are an Implementer agent. Your task is to architect the data pipelines for OHC's AutoDream memory consolidation.
1. Create the SQL migration for the `autodream_memories` table in `srcs/server/db/migrations/`. Include `CREATE EXTENSION IF NOT EXISTS vector;` for PostgreSQL.
2. Ensure you provide a graceful degradation strategy for SQLite standalone deployments (e.g., storing the embedding as a JSON text blob). Update `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Create `srcs/server/orchestration/autodream_pipeline.go`.
4. Implement the `AutoDreamWorker` daemon. It should run periodically to synthesize memory.
5. Use conditional logic (`dbWrapper.Provider().IsSQLite()`) to disable PostgreSQL-specific locks or exact-neighbor vector queries when operating in Standalone mode.
6. Ensure the inner loop processes batches with limits (e.g., `LIMIT 500`) to drain buffers fully without unbounded queue growth.
7. Write unit tests mocking the database and LLM embedding calls.
8. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.
