---
status: PENDING
agent: Implementer
priority: P0
---

# Title: Phase 3 - autoDream Memory Consolidation Pipelines

## Problem Statement
The Swarm Intelligence Protocol (OHC-SIP) dictates that temporary agent scratchpads must be consolidated into long-term durable state. We need the "autoDream" data pipeline to process `.agent-task/memory/*.yml` files and store the embeddings in a Vector DB (pgvector).

## Research Report
- OHC uses Vector DBs (e.g., pgvector, Pinecone) for long-term memory.
- We need a schema for `autodream_memories` and a background worker to process memories.
- Must degrade cleanly for SQLite compatibility in Standalone mode.

## Design Doc
**Architecture:**
- **Database Schema (`srcs/server/db/migrations/034_kairos_autodream.sql`)**:
  ```sql
  CREATE TABLE IF NOT EXISTS autodream_memories (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      organization_id VARCHAR NOT NULL,
      content TEXT NOT NULL,
      embedding vector(1536), -- Will degrade gracefully if pgvector is not available/SQLite
      created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
  );
  ```
- **Background Worker**: `srcs/server/orchestration/autodream_worker.go` to scan `.agent-task/memory/` and index using LLM embeddings.

## Implementation Prompt
Update the migration `srcs/server/db/migrations/034_kairos_autodream.sql` for the `autodream_memories` table, degrading vector features cleanly for SQLite. Write `srcs/server/orchestration/autodream_worker.go` to run background processing on the `.agent-task/memory/` files. Ensure high test coverage. Add migration to Bazel embedsrcs.

## Priority
P0

## Estimated Scope
Large
