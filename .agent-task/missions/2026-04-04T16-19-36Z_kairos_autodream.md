# Title: KAIROS Orchestration: Implement AutoDream Vector Data Pipelines

## Problem Statement
The Swarm currently generates transient memories during execution. We need a semantic memory consolidation pipeline running passively to translate ephemeral session contexts into durable, vectorized truth for the swarm's long-term memory, ensuring continuous architectural insight.

## Research Report
The KAIROS Orchestration component "AutoDream" must handle memory consolidation:
1.  **Pipeline Logic**: Background workers monitor `agent_session_data` and trigger Minimax/LLM summarization jobs (`AutoDreamWorker`), transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories`.
2.  **Cloud Mode**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`).
3.  **Local Degradation**: In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).

## Design Doc
1. **Database Schema**: Implement the `autodream_memories` table:
   ```sql
   CREATE EXTENSION IF NOT EXISTS vector;
   CREATE TABLE IF NOT EXISTS autodream_memories (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       content TEXT NOT NULL,
       embedding VECTOR(1536),
       source_mission_id TEXT
   );
   ```
2. **AutoDream Worker**: Implement a Go background worker `AutoDreamWorker` that parses `.agent-task/memory/*.yml` and reads `agent_session_data`, generates embeddings using the configured LLM provider (Minimax), and stores them in the DB.
3. **Concurrency**: Use `pool.IsSQLite()` logic to disable PostgreSQL-specific locks if in Standalone mode.

## Implementation Prompt
Hello Implementer agent! Please implement the AutoDream Data Pipelines:
1. Add/modify DB migration scripts in `srcs/server/db/migrations/` to support `autodream_memories` schemas. Make sure to translate pgvector to SQLite blobs/text where appropriate. Add any new `.sql` migrations to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
2. Add an AutoDream Worker skeleton in Go to process `COMPLETED` tasks and `.agent-task/memory/*.yml` files into `autodream_memories`.
3. Ensure concurrency uses `pool.IsSQLite()` logic to disable PostgreSQL-specific locks (like `FOR UPDATE SKIP LOCKED`) if in Standalone mode.
4. Ensure code achieves >95% unit test coverage.
5. All DB operations should return `(int64, error)` and should not call `.RowsAffected()`.
6. Verify all code passes with `bazelisk test //srcs/server/...`.

## Priority
P0

## Estimated Scope
Medium
