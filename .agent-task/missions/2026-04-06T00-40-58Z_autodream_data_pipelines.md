---
title: "autoDream Data Pipelines (Long-term Memory Consolidation)"
status: PENDING
priority: "P0"
estimated_scope: "Large"
---

# Problem Statement
OHC's Swarm Intelligence Protocol dictates that all agents share memory. To prevent context window overflow and enable long-term reasoning, OHC needs an "autoDream" background pipeline. This pipeline must asynchronously consolidate episodic memory (`agent_session_data`), prune redundancies, resolve conflicts, and inject truth via embeddings into a vector database (e.g., pgvector).

# Research Report
- Based on `srcs/server/orchestration/autodream.go`, the core AutoDream pipelines (memory pruning, conflict resolution, truth injection) are already fully implemented.
- We need to architect the data pipelines to feed this existing system at scale, transitioning from simple batch jobs to a continuous, durable stream processing architecture.
- **pgvector Integration:** PostgreSQL with `pgvector` will serve as the persistent vector database.
- **LLM Embeddings:** Utilize existing LLM clients (`srcs/server/agents/local/llm.go`) to generate embeddings for consolidated memory.

# Design Doc
**Architecture:**
- **Pipeline Orchestrator:** A background worker (`AutoDreamPipeline`) that orchestrates the flow: Raw Memory -> Chunking -> Summarization -> Embedding -> pgvector Insertion.
- **Data Source:** `agent_session_data` (SQLite/PostgreSQL) and `.agent-task/memory/*.yml` files.
- **Vector Storage Schema (pgvector):**
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536), -- Assuming standard 1536 dim embeddings
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON consolidated_memory USING hnsw (embedding vector_l2_ops);
```

**Workflow:**
1. **Extraction:** Poll recent `agent_session_data`.
2. **Consolidation:** Feed data to the existing `AutoDream` logic in `srcs/server/orchestration/autodream.go`.
3. **Embedding:** Call LLM client to generate embeddings for the consolidated summaries.
4. **Loading:** Upsert into `consolidated_memory` using pgvector.

# Implementation Prompt
You are an Implementer agent. Your task is to build the autoDream Data Pipelines.
1. Create `srcs/server/pipeline/autodream_pipeline.go`.
2. Define the `AutoDreamPipeline` struct and its execution loop.
3. Implement the PostgreSQL schema migration to add the `consolidated_memory` table and `pgvector` extension (ensure a fallback for SQLite if possible, or skip vector ops in standalone if SQLite vector isn't available).
4. Integrate with `srcs/server/orchestration/autodream.go` to utilize the existing pruning and conflict resolution logic.
5. Use the LLM clients in `srcs/server/agents/local/llm.go` to generate embeddings.
6. Write to the `consolidated_memory` table.
7. Write unit tests mocking the database and LLM calls. Ensure >90% coverage.
8. Verify functionality by writing a test using Bazel: `bazelisk test //srcs/server/pipeline/...`
