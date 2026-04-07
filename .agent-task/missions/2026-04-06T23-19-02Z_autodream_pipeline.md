---
status: "DONE"
Title: "KAIROS Phase 3: AutoDream Data Pipelines for Memory Consolidation"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
OHC's Swarm Intelligence Protocol (OHC-SIP) dictates that all agents share memory. To prevent context window overflow and enable long-term reasoning, OHC needs an "AutoDream" background pipeline. This pipeline must asynchronously consolidate ephemeral session contexts (`agent_session_data`), prune redundancies, and inject truth via embeddings into a durable vector database.

# Research Report
- **pgvector Integration:** PostgreSQL with `pgvector` will serve as the persistent vector database for the Cloud-Native mode, enabling exact Nearest Neighbor search.
- **SQLite Fallback:** Standalone mode requires a graceful degradation strategy, such as text extraction/recency-based search if vector extensions are unavailable in the local SQLite distribution.
- **LLM Embeddings:** Utilize existing LLM clients (`srcs/server/agents/local/llm.go`) to generate embeddings for consolidated memory.

# Design Doc
**Architecture:**
- **Pipeline Orchestrator:** A background worker (`AutoDreamWorker`) that periodically sweeps completed `shared_tasks` and memory contexts.
- **Vector Storage Schema (pgvector):**
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

**Workflow:**
1. **Extraction:** Poll recent `.agent-task/memory/*.yml` and session data.
2. **Embedding:** Call LLM client to generate embeddings.
3. **Loading:** Upsert into `autodream_memories`.

# Implementation Prompt
You are an Implementer agent. Your task is to architect the data pipelines for OHC's AutoDream memory consolidation.
1. Create the SQL migration for the `autodream_memories` table in `srcs/server/db/migrations/` (e.g., `015_autodream_memories.sql`). Include `CREATE EXTENSION IF NOT EXISTS vector;` for PostgreSQL. Provide a SQLite equivalent (e.g., storing embedding as a JSON text blob) for Standalone degradation. Add to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
2. Create `srcs/server/orchestration/autodream_pipeline.go`.
3. Implement the `AutoDreamWorker` daemon. It should run periodically to synthesize memory.
   - Use `dbWrapper.Provider().IsSQLite()` to apply conditional logic: disable PostgreSQL-specific locks or vector exact-neighbor queries if in Standalone mode.
4. Ensure the inner loop processes batches with limits (e.g., `LIMIT 500`) to drain the buffer fully without unbound queue growth, identical to telemetry syncing best practices.
5. Write unit tests mocking the database and LLM calls. Ensure >90% coverage.
6. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.

# Visual Excellence Guidelines
Any UI exposing AutoDream insights must strictly adhere to the OHC Premium Feel:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
