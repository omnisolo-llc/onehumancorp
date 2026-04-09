---
status: "PENDING"
Title: "Implement AutoDream Vector Data Pipelines (KAIROS Orchestration Phase 3)"
Priority: "P1"
Estimated Scope: "Medium"
---

# Problem Statement
We need a semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth for the swarm's long-term memory.

# Research Report
The KAIROS Orchestration Design Doc mandates:
- **Pipeline Logic**: Background workers monitor agent session data and trigger Minimax/LLM summarization jobs, transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories`.
- **Cloud Mode**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`).
- **Local Degradation**: In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).

# Design Doc
1. **Schema Updates**: Ensure `autodream_memories` table supports vector data in PostgreSQL.
2. **AutoDream Worker**: Implement the daemon `AutoDreamWorker` in Go to monitor ephemeral context and call the LLM API.
3. **LLM Summarization**: Send short-term contexts to Minimax or configured LLMs to generate embeddings and consolidated text.
4. **pgvector Integration**: Write embeddings to the database. Use `pgvector` operations in Cloud Mode and standard text operations in Standalone SQLite Mode.

# Implementation Prompt
1. Ensure the `autodream_memories` schema exists.
2. Review `autodream.go` and `autodream_worker.go` in `srcs/server/orchestration/`. Ensure its background routines safely execute database transactions.
3. Implement the LLM API call for embedding generation. Do not perform external network calls inside active database transactions (especially `FOR UPDATE SKIP LOCKED`).
4. Ensure `autodream_memories` uses vector similarity in PG and `ORDER BY created_at DESC` in SQLite fallback.
