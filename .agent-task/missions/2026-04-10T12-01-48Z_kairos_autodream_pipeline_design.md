---
status: "PENDING"
priority: P1
agent: "KAIROS Orchestrator"
Title: "Design Doc: OHC KAIROS AutoDream Pipeline"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
Agents lack long-term coherence. AutoDream runs passively to translate ephemeral thoughts into durable truth, preventing context window overflows.

# Research Report
*   **Data Sources**: Ephemeral context streams into `agent_session_data` and `.agent-task/memory/{timestamp}.yml`.
*   **Background Consolidation**: The `AutoDreamPipeline` orchestrator worker consumes these sources, chunking and compressing the context via a Minimax/LLM summarization call (using `srcs/server/agents/local/llm.go`).
*   **Vector Querying**: `pgvector` enables exact Nearest Neighbor (`ORDER BY embedding <-> $1`). SQLite gracefully falls back to recency sorts in standalone mode.

# Design Doc
## Data Pipeline Architecture
1. **Source**: Local `.agent-task/memory/` YAML files.
2. **Ingestion Agent**: Reads files, generates chunked text.
3. **Embedding Generation**: Calls LLM provider (e.g., Anthropic/OpenAI/Minimax) to produce vectors.
4. **Storage (pgvector)**:
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**Vector Storage Schema (pgvector) in consolidated_memory:**
```sql
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON consolidated_memory USING hnsw (embedding vector_l2_ops);
```

# Implementation Prompt
Implement the AutoDream background pipeline. The `AutoDreamPipeline` worker should monitor `.agent-task/memory/` and `agent_session_data`, chunk/summarize content using `srcs/server/agents/local/llm.go`, and insert embedded vectors into `autodream_memories` and `consolidated_memory` tables. Use `pgvector` for Postgres, but map vectors to `[]byte` in Go structs for SQLite fallback compatibility.
