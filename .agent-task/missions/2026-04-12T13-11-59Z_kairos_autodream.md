---
status: PENDING
agent: Implementer
priority: P0
estimated_scope: Large
---

# Title: OHC Hybrid OS - autoDream Memory Consolidation Pipelines

## Problem Statement
The Swarm Intelligence Protocol (OHC-SIP) dictates that temporary agent scratchpads and coordination meshes must be consolidated into long-term durable state. We need the "autoDream" data pipeline to process short-term states and store the embeddings in a Vector DB (pgvector), fulfilling the KAIROS memory consolidation mandate.

## Research Report
- OHC uses Vector DBs (e.g., pgvector, Pinecone) for long-term memory.
- We need a schema for `autodream_memories` and a background worker pipeline to process memories into the vector database.
- Must degrade cleanly to standard SQL text-matching for SQLite compatibility in Standalone mode.

## Design Doc
1. **Database Schema (`srcs/server/db/migrations/035_kairos_autodream.sql`)**:
   - Create `autodream_memories` table with `id`, `organization_id`, `content`, `embedding` (using `vector(1536)` if Postgres, text if SQLite).

2. **AutoDream Pipeline Manager:**
   - Define `MemoryConsolidator` in `srcs/server/orchestration/autodream.go`.
   - Implement `ProcessBatch()` which takes raw memory strings, generates embeddings (mocked interface for LLMs), and stores them in the DB.

## Implementation Prompt
Hello Implementer! Build the Phase 3 AutoDream pipeline. First, create the SQL migration `srcs/server/db/migrations/035_kairos_autodream.sql` for the `autodream_memories` table, ensuring it degrades vectors cleanly for SQLite. Write `srcs/server/orchestration/autodream.go` to expose a `MemoryConsolidator` that processes text into embeddings and stores them. Use a mock for the LLM embedding provider. Add the migration to Bazel `embedsrcs` and ensure high test coverage.

## Priority
P0

## Estimated Scope
Large
