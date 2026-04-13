---
status: STUCK
agent: Implementer
---

# Title: KAIROS Phase 3: AutoDream Vector Data Pipelines

## Problem Statement
Agents lack long-term semantic memory coherence. Ephemeral contexts overflow token limits and are quickly forgotten, degrading swarm performance.

## Research Report
The AutoDream pipeline must run passively to synthesize ephemeral thoughts from `agent_session_data` and `.agent-task/memory/*.yml` into durable vectorized truth. In Cloud Mode, we rely on PostgreSQL's `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`). In SQLite Standalone mode, we fallback to text-based recency logic.

## Design Doc
- **Data Consolidation Worker**: `AutoDreamPipeline` background worker sweeps completed tasks.
- **Vector Schema**: `autodream_memories` table mapping `embedding vector(1536)`.
- **Minimax Embeddings**: Interfacing with the LLM API (`srcs/server/agents/local/llm.go`) to generate embeddings for consolidated text.

## Implementation Prompt
Hello Implementer agent!
1. Review `srcs/server/db/migrations/` for `autodream_memories`. Make sure `vector` extensions and SQLite fallbacks exist. Update `embedsrcs` in `srcs/server/db/BUILD.bazel` if new migrations are added.
2. Build the `AutoDreamPipeline` background daemon to sweep completed `shared_tasks`.
3. Use the LLM endpoints to compress ephemeral context and store it.
4. Gracefully handle vector exact-neighbor queries in Postgres vs text queries in SQLite using `db.Provider.IsSQLite()`.
5. Run unit tests (`bazelisk test`) for validation.

## Priority
P0

## Estimated Scope
Medium
