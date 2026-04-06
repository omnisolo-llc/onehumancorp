---
Title: "KAIROS Phase 3: AutoDream Data Pipelines for Memory Consolidation"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
The OHC Swarm generates vast amounts of ephemeral context (`agent_session_data`) that must be converted into long-term reasoning capabilities via the AutoDream background pipeline. Agents require a robust schema to embed and synthesize memory to achieve exact semantic search across vector stores.

# Research Report
- **pgvector Integration:** PostgreSQL with `pgvector` will serve as the persistent vector database for the Cloud-Native mode, enabling exact Nearest Neighbor search.
- **SQLite Fallback:** Standalone mode requires a graceful degradation strategy, such as text extraction/recency-based search if vector extensions are unavailable.
- **LLM Embeddings:** Minimax LLMs compress session logs and intermediate artifacts to inject truth into `autodream_memories`.

# Design Doc
**Architecture & Schema:**
- **Table:** `autodream_memories` must contain `id`, `organization_id TEXT`, `agent_id TEXT`, `source_type TEXT`, `content TEXT`, `embedding vector(1536)`, and `source_mission_id TEXT`.
- **Pipeline Orchestrator:** The `AutoDreamWorker` periodically sweeps completed shared tasks and memory contexts.

```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
You are an Implementer agent. Execute the following:
1. Revise the SQL migration for `autodream_memories` to include `organization_id`, `agent_id`, and `source_type`.
2. Modify `AutoDreamWorker` in `srcs/server/orchestration/autodream_worker.go` to use `FOR UPDATE SKIP LOCKED` and process batch limits (e.g., `LIMIT 500`) when inserting context.
3. Update `autodream.go` inserts to correspond to the updated database columns. Ensure `srcs/server/orchestration/...` tests pass flawlessly.

# Visual Excellence Mandate
Any UI surfacing AutoDream logic must apply:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
