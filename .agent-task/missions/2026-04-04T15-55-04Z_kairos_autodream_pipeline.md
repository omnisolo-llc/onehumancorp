---
title: "Implement AutoDream Vector Data Pipelines (KAIROS Orchestration)"
problem_statement: "We need a semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth for the swarm's long-term memory."
priority: "P1"
estimated_scope: "Medium"
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# Title: Implement AutoDream Vector Data Pipelines (KAIROS Orchestration)

## Problem Statement
We need a semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth for the swarm's long-term memory.

## Research Report
The KAIROS Orchestration Design Doc mandates:
- **Pipeline Logic**: Background workers monitor `agent_session_data` and trigger Minimax/LLM summarization jobs (`AutoDreamWorker`), transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories`. It must sweep completed `shared_tasks`.
- **Cloud Mode**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`).
- **Local Degradation**: In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).
- **Data Protection**: Never perform external network calls (Minimax/LLM API requests) inside active PostgreSQL database transactions (`FOR UPDATE SKIP LOCKED`).

## Design Doc
1. **Schema Updates**: Ensure `autodream_memories` table supports `VECTOR(1536)` in PostgreSQL (e.g. `007_teammate_mesh_and_autodream.sql`).
2. **AutoDream Worker**: Implement the daemon `AutoDreamWorker` in Go to monitor ephemeral context, sweep completed tasks (`runCompletedTaskSweeper`), and call the LLM API.
3. **LLM Summarization**: Send short-term contexts to configured Minimax endpoints to generate embeddings and consolidated text. Use prompt caching if applicable (`cache_control: {"type": "ephemeral"}`).
4. **pgvector Integration**: Write embeddings to the database using `pgvector` operations in Cloud Mode and standard text operations in Standalone SQLite Mode.

## Implementation Prompt
1. Read `docs/kairos_orchestration_design.md` for context.
2. Review `orchestration.AutoDreamWorker` in `srcs/server/orchestration/autodream.go`.
3. Implement `runCompletedTaskSweeper` as a background daemon to sweep `COMPLETED` shared tasks into `autodream_memories`.
4. Ensure its background routines safely check `pool.IsSQLite()` before executing PostgreSQL-specific queue locks.
5. Implement the LLM API call for embedding generation. Do not perform external network calls inside active database transactions. Fetch data, close the transaction/rows, perform the external call, and then open a new transaction for updates.
6. Ensure `autodream_memories` uses `ORDER BY embedding <-> $1` in PG and `ORDER BY created_at DESC` in SQLite fallback.
7. Write tests (>95% coverage) testing memory consolidation in both Postgres and SQLite. Ensure `ML-Resilience` prevents panicking if `DATABASE_URL` is unavailable or files are locked.
8. Ensure all Bazel tests pass (`bazelisk test //srcs/server/orchestration:orchestration_test`).

## Priority
P1

## Estimated Scope
Medium

</div>
