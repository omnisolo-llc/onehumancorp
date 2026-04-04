---
status: DONE
agent: Implementer
---

# Title: Implement AutoDream Vector Data Pipelines (KAIROS Orchestration)

## Problem Statement
We need a semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth for the swarm's long-term memory.

## Research Report
The KAIROS Orchestration Design Doc mandates:
- **Pipeline Logic**: Background workers monitor `agent_session_data` and trigger Minimax/LLM summarization jobs (`AutoDreamWorker`), transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories`.
- **Cloud Mode**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`).
- **Local Degradation**: In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).

## Design Doc
1. **Schema Updates**: Ensure `autodream_memories` table supports `VECTOR(1536)` in PostgreSQL.
2. **AutoDream Worker**: Implement the daemon `AutoDreamWorker` in Go to monitor ephemeral context and call the LLM API.
3. **LLM Summarization**: Send short-term contexts to Minimax or configured LLMs to generate embeddings and consolidated text.
4. **pgvector Integration**: Write embeddings to the database. Use `pgvector` operations in Cloud Mode and standard text operations in Standalone SQLite Mode.

## Implementation Prompt
1. Read `docs/kairos_orchestration_design.md` for context.
2. Ensure the `autodream_memories` schema exists in `srcs/server/db/migrations/` (check `007_autodream.sql` or similar).
3. Review `orchestration.NewAutoDreamWorker` in `srcs/server/orchestration/`. Ensure its background routines safely check `pool.IsSQLite()` before executing PostgreSQL-specific queue locks or distributed routines.
4. Implement the LLM API call for embedding generation. Do not perform external network calls inside active database transactions (especially `FOR UPDATE SKIP LOCKED`).
5. Ensure `autodream_memories` uses `ORDER BY embedding <-> $1` in PG and `ORDER BY created_at DESC` in SQLite fallback.
6. Write tests (>95% coverage) testing memory consolidation in both Postgres and SQLite.
7. Ensure all Bazel tests pass.

## Priority
P1

## Estimated Scope
Medium
