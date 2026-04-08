---
status: PENDING
agent:
---

# Title: KAIROS Phase 3: Implement AutoDream Vector Data Pipelines
## Problem Statement
During task execution, agents generate significant amounts of context. To prevent context window overflow and enable long-term reasoning, OHC needs the AutoDream data pipeline to sweep ephemeral session data, prune redundancies, and inject the consolidated "truth" into a durable vector database.

## Research Report
- AutoDream enables OHC-SIP (Swarm Intelligence Protocol) memory sharing.
- **Cloud-Native Mode:** Utilizes PostgreSQL with `pgvector` for exact Nearest Neighbor search on 1536-dimensional embeddings.
- **Standalone Mode:** Uses SQLite. Embeddings are stored as JSON text blobs, with fallback mechanisms.
- Batch processing is required to prevent unbound queue growth.

## Design Doc
**Database Schema (`autodream_memories`):**
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

**Architecture:**
- **AutoDreamWorker:** A daemon that processes data in batches (e.g., `LIMIT 500`).
- **Embedding Generation:** Uses existing LLM clients (`srcs/server/agents/local/llm.go`).
- **Graceful Degradation:** Use `dbWrapper.Provider().IsSQLite()` to conditionally disable PostgreSQL-specific features.

**Visual Excellence Guidelines:**
Any UI developed for this feature must enforce:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the AutoDream Data Pipeline APIs.
1. Create the SQL migration file for `autodream_memories` in `srcs/server/db/migrations/` formatted for Goose.
2. Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. Implement `AutoDreamWorker` in `srcs/server/orchestration/autodream.go` to sweep `.agent-task/memory/*.yml` files.
4. Integrate the LLM client from `srcs/server/agents/local/llm.go` to chunk, tokenize, and generate 1536-dimensional vectors.
5. Create the database insertion logic, taking into account `IsSQLite()` to store JSON text blobs in Standalone mode vs `vector(1536)` in PostgreSQL.
6. Provide unit tests using `db.NewTestProvider(t)`.
7. Verify tests pass via `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Medium
