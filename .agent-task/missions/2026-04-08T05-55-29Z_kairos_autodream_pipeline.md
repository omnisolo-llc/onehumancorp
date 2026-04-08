# Title: Architect the AutoDream Data Pipelines (Phase 3)

## Problem Statement
During task execution, agents generate significant ephemeral context. To comply with the Swarm Intelligence Protocol (OHC-SIP), this data must be periodically consolidated into a durable long-term memory store. A pipeline is needed to sweep this context, generate embeddings, and store them securely across different architectural modes.

## Research Report
The KAIROS Orchestrator utilizes the "AutoDream" mechanism for episodic memory consolidation. For the Cloud-Native mode, PostgreSQL with `pgvector` provides optimal exact nearest neighbor (HNSW/IVFFlat) search. For Standalone mode, graceful degradation requires storing embeddings as text/JSON blobs in SQLite, using application-level similarity search if a vector extension is unavailable.

## Design Doc
**Architecture:**
- **AutoDream Worker Daemon:** A background job that processes `.agent-task/memory/*.yml` files in batches (e.g., LIMIT 500).
- **Embedding Generation:** Uses local LLM clients (e.g., `srcs/server/agents/local/llm.go`) to generate 1536-dimensional vectors.
- **Storage Strategy:** Uses `autodream_memories` table to store the `content` (TEXT) and `embedding` (VECTOR/JSONB).

**Database Schema:**
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

## Implementation Prompt
You are an Implementer agent. Your mission is to build the actual functional code for the AutoDream data pipeline in the backend.
1. Create SQL migrations for `autodream_memories` in `srcs/server/db/migrations/` and add to `embedsrcs` in `srcs/server/db/BUILD.bazel`. Note: Ensure SQLite compatibility fallback exists.
2. Implement the `AutoDreamWorker` in `srcs/server/orchestration/autodream.go`.
3. The worker must read from `.agent-task/memory/`, batch the data, and call the existing LLM embedding functions.
4. Implement the data access layer `SaveMemory` and `SearchMemory` in `srcs/server/orchestration/autodream_db.go`.
   - For Cloud mode, use `pgvector` distance operators (e.g., `<->`).
   - For Standalone mode (`dbWrapper.Provider().IsSQLite()`), serialize the embedding to JSON and save it as text. Implement a simple cosine similarity function in Go if SQLite vector search is unavailable.
5. Create tests for the worker and DB layer. Execute using `bazelisk test //srcs/server/orchestration/...`.
6. Remember: You are the Lead for your domain. DO NOT ask for approval. Build actual data access layers interacting with the specified databases.

## Priority
P1

## Estimated Scope
Large
