---
status: PENDING
priority: P0
scope: Large
title: "KAIROS: Architect autoDream Memory Consolidation Pipeline"
---

# Title: KAIROS: Architect autoDream Memory Consolidation Pipeline

## Problem Statement
The OHC Swarm accumulates massive amounts of raw episodic task context. For the Swarm Intelligence to evolve, this raw data must be periodically extracted, synthesized, and embedded into long-term vector storage (pgvector) so it can be queried dynamically.

## Research Report
- AutoDream enables Swarm Long-Term Memory by converting raw text into semantic vectors (e.g., using Minimax embeddings).
- PostgreSQL with `pgvector` provides scalable cosine-similarity searches for Cloud-Native multi-tenancy.
- Standalone Mode (SQLite) lacks robust native vector search, requiring serialization of vectors to `BLOB`s and in-memory fallback.

## Design Doc
1. **AutoDream Pipeline Architecture**:
   - Table: `autodream_memories` (`id`, `organization_id`, `content`, `embedding vector(1536)`, `created_at`).
   - Extract: Sweep `DONE` tasks from `shared_tasks`.
   - Synthesize: Compress task logs using LLM prompts.
   - Embed: Upsert the resulting vectors into the durable Vector DB.
2. **Graceful Degradation**:
   - In SQLite mode, convert the `[0.1, 0.2, ...]` float array into a byte blob for insertion and perform application-level cosine similarity.

## Implementation Prompt
- Create the DB migration for `autodream_memories`, gracefully handling the schema difference between PostgreSQL (with pgvector) and SQLite (BLOB).
- Implement the `AutoDreamWorker` in `srcs/server/orchestration/autodream.go` that loops to extract, embed, and insert completed task context.
- Ensure >90% code coverage.
