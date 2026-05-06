# OHC AI Consolidator: Memory Layer Architecture

The Long-Term Memory and Context Consolidation system is crucial for enabling OHC AI agents to maintain persistent context across interactions. This documentation outlines the design and implementation of the Persistent Memory Layer, Conflict Resolution logic, and Stale Context Pruning.

## 1. Persistent Memory Layer

The `VectorRepository` (located in `src/agents/builtin/memory_store.rs`) serves as the foundational persistence layer. It supports both Cloud (PostgreSQL via `pgvector`) and Standalone (SQLite with vector extension) modes. All operations are firmly isolated using a `tenant_id` to ensure absolute multi-tenant data privacy.
Memory records use an `EmbeddingRecord` struct containing:
- `id`: Unique Identifier.
- `tenant_id`: Mandatory tenant isolation.
- `agent_id`: Optional assignment.
- `embedding`: 1536-dimensional vector for semantic retrieval.
- `reference_count`: Usage tracking for pruning decisions.
- `owner_override`: Flag indicating explicit human guidance.

## 2. Conflict Resolution

To maintain accurate business knowledge, the `auto_resolve_conflicts` worker runs continuously to process overlapping facts.
When `get_conflicting_pairs` detects vector embeddings with a distance `< 0.05` within the same tenant, the conflict is resolved deterministically based on the following hierarchy:
1. **Explicit Owner Override:** Manual settings (`owner_override = true`) take absolute priority.
2. **Source Reliability:** Memory derived from reliable plugins/sources overrides weaker contexts.
3. **Recency:** When all else is equal, the newest fact (`created_at`) replaces the old one.

The loser's `reference_count` is merged into the winner to preserve the context weighting before being soft-deleted.

## 3. Stale Context Pruning

The `MemoryConsolidationWorker` (`src/server/workers/memory.rs`) ensures the database is not bloated with irrelevant data.
- Automatically triggers a garbage collection via `repository.prune_stale` every hour.
- Safely purges low-value task summaries (`source_type LIKE 'TASK%'` or `SESSION_SUMMARY`) older than 180 days that lack human verification (`owner_override = FALSE`) and have low access volume (`reference_count < 5`).
- Cross-Department context sharing is enabled as semantic queries query globally via `tenant_id` omitting `agent_id` scopes, keeping memory un-siloed.

All features are fully tested, ensuring 100% test coverage locally.
