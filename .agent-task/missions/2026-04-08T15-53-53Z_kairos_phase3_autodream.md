---
status: "PENDING"
Title: "KAIROS Phase 3: AutoDream Data Pipelines"
Priority: "P1"
Estimated Scope: "Medium"
---

# Title: KAIROS Phase 3: AutoDream Data Pipelines

## Problem Statement
The OHC swarm requires a long-term memory consolidation system to extract insights from ephemeral agent workflows and store them for semantic search. We must architect the data pipelines (e.g., pgvector, LLM embeddings) for OHC's long-term memory consolidation system, known as "AutoDream".

## Research Report
- Current KAIROS architecture defines AutoDream as "The Memory" pillar.
- The pipeline asynchronously consolidates ephemeral session logs (`agent_session_data`) and embeds them using Minimax LLMs.
- The destination must be a durable `pgvector` index (`autodream_memories`) for exact semantic search in Cloud-Native mode.
- In Standalone mode, standard SQLite without `pgvector` features must gracefully degrade to basic search or a compatible local vector extension.

## Design Doc
**Architecture:**
- **AutoDream Worker:** A Go routine running in the background, listening to completed `shared_tasks` and ephemeral session logs.
- **LLM Embeddings:** Use the `CachedMinimaxClient` to extract structured insights and generate vector embeddings. Wrap the client securely: `NewCachedMinimaxClient(NewMinimaxClient(apiKey), dbPool, redisClient)`.
- **Database Schema (`autodream_memories`):**
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    task_id TEXT,
    insight_text TEXT NOT NULL,
    embedding vector(1536), -- Only for Postgres
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

## Implementation Prompt
Hello Implementer agent! Your task is to build the AutoDream Data Pipeline.
1. Implement the database migrations for `autodream_memories`. Remember to conditionally create the `vector` extension and column only if running in PostgreSQL. For SQLite, provide a fallback.
2. Ensure the `autodream_memories` table serves as the primary context table for storing Swarm Memories.
3. In `srcs/server/orchestration/autodream_pipeline.go`, implement the asynchronous background worker that fetches completed tasks and extracts insights.
4. Integrate the `CachedMinimaxClient` for embeddings to minimize token cost.
5. Provide a semantic search method that queries the pgvector index (`<=>`) and fallback logic for SQLite.
6. Write unit tests for the worker and embedding flow. Use `bazelisk test //srcs/server/orchestration/...` to verify.

## Visual Excellence Guidelines
Any frontend representation of AutoDream later created must strictly apply the OHC "Premium Feel":
```css
backdrop-filter: blur(20px) saturate(200%);
background: rgba(255, 255, 255, 0.03);
font-family: 'Outfit', 'Inter', sans-serif;
```

## Priority
P1

## Estimated Scope
Medium
