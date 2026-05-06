# Memory Consolidation Architecture

## Overview

The Memory Consolidation system is designed to provide long-term memory and context retention for the OHC AI agents. This allows different AI departments to retain knowledge across sessions, detect and resolve conflicting knowledge, and automatically prune stale context.

## Design Principles

- **Parity Focus**: Works in both Cloud mode (PostgreSQL with `pgvector`) and Standalone mode (SQLite with a vector extension, or fallback).
- **Tenant Isolation**: All memory operations are scoped to a specific business owner's `tenant_id` to ensure isolation and privacy.
- **Conservative Pruning**: Context is pruned only when it is highly likely to be irrelevant (e.g., source type 'TASK_SUMMARY', older than 180 days, no owner override, and low reference count).
- **Non-blocking Operations**: Consolidation, conflict resolution, and pruning occur in background async workers to avoid blocking the main AI request path.

## Persistent Memory Layer

The core entity is the `EmbeddingRecord` stored in the `consolidated_memory` table. This layer supports semantic search capabilities (e.g., using `vec_distance_cosine` in SQLite or `<=>` operator in PostgreSQL). The `VectorRepository` abstraction provides uniform access over both underlying stores.

When a relevant interaction occurs, it is embedded using an embedding model and upserted into the vector repository. Subsequent requests query this repository to retrieve relevant past experiences, injecting them into the agent's context.

## Conflict Resolution Strategy

Conflicts arise when multiple memory entries with highly similar embeddings (cosine distance < 0.05) exist within the same tenant. The `auto_resolve_conflicts` method detects these conflicting pairs.

The resolution logic is as follows:
1. **Explicit Owner Override**: If one entry has an explicit owner override and the other does not, the overridden entry wins.
2. **Reliability Score**: If no clear owner override differentiates them, the entry with the higher reliability score wins.
3. **Recency**: If reliability scores are tied, the more recent entry (based on `created_at`) wins.

The losing entry is deleted, and its `reference_count` is merged into the winning entry. The winning entry's `last_referenced_at` is updated to the current time.

## Stale Context Pruning

The `MemoryConsolidationWorker` periodically polls the repository (default interval: 1 hour) to clear stale context. Pruning targets context older than a predefined threshold (e.g., 180 days) that matches certain criteria:
- Not explicitly overridden by the owner (`owner_override = FALSE`).
- Has a low reference count (`reference_count < 5`).
- Is of a specific source type prone to obsolescence (e.g., `TASK_SUMMARY`).

This strategy prevents the boundless growth of the vector store while preserving valuable historical business data.

## Cross-Department Context Sharing

Memories are not siloed by department or agent. An entry created by the `sales_agent` (e.g., a "SUPPORT_TICKET" indicating a customer's pricing issue) is written to the central `consolidated_memory` repository for the tenant. When a different department, such as a business advisory agent, queries the repository, the semantic search will retrieve this relevant context across department lines, enabling a unified view of the business context.
