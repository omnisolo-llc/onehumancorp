# OHC AI Agent Context Consolidation System

## Overview

The context consolidation system ("AutoDream") provides the Persistent Memory Layer for OHC's AI agents. It ensures that the various AI departments retain knowledge across sessions, enabling a continuous and highly contextualized experience for business owners without overwhelming prompt limits or requiring constant retraining.

## Design Principles

-   **Dual-Mode Operation:** Supports both Cloud mode using PostgreSQL with the `pgvector` extension and Standalone mode using SQLite with in-memory dot product implementations.
-   **Strict Tenant Isolation:** All memory operations and semantic searches are scoped to a specific `organization_id`. An organization’s memory never leaks across boundaries.
-   **Conservative Pruning:** Stale context removal is highly conservative. Context is evaluated against explicit time bounds, and when in doubt, the context is retained.
-   **Asynchronous Processing:** Memory operations happen in background workers to prevent blocking the main AI request paths.

## Architecture

### 1. Persistent Memory Layer

The memory layer relies heavily on the `autodream_memories_master` table, wrapped by `VectorRepository`.

When an AI department processes customer interactions or internal tasks, the logs and actions are passed to the `Service.Consolidate` routine. The pipeline:
1. Generates a summary of key technical decisions and facts via the LLM provider.
2. Embeds the summary string into a float vector array using the LLM provider.
3. Inserts or Updates the record into the `autodream_memories_master` table.

The repository implementation seamlessly switches between `pgvector` operators (using `1 - (embedding <=> $2::vector)` for cosine similarity calculation on the database side) for production cloud deployments and a fallback in-memory slice dot-product fallback computation (`cosineSimilarity(a, b []float32)`) for SQLite local sandbox runs.

### 2. Conflict Resolution Strategy

Conflicts occur when the same conceptual fact is stored multiple times with divergent values (e.g., conflicting prices, preferences, or notes). The resolution logic is built into the `Consolidate` flow:

- During consolidation, before saving the new memory, the system executes a Semantic Search against the tenant's existing `TASK_SUMMARY` memory records.
- **Threshold Matching:** It isolates the top existing semantic match. If the existing memory exceeds a semantic similarity score of `0.90`, the system classifies this as a conceptual collision/update.
- **LLM Merging:** Instead of blindly appending a contradictory new fact or statically overwriting the record, the LLM is prompted to dynamically merge the two context items (`Old` vs `New`), keeping the newer information as the source of truth.
- **Upsert:** The newly resolved summary and embedding vector replace the previous record, resolving the conflict while maintaining the recency timestamp and task tracking.

### 3. Stale Context Pruning

To keep the active agent working memory clean and fast, older ephemeral memories are periodically reviewed.

The `PruneStaleContext` worker method deletes only `TASK_SUMMARY` type records that surpass an explicitly provided duration threshold relative to their `created_at` timestamp. This allows the system to easily sweep away temporary operational scratchpads while long-term permanent business facts remain un-pruned or are assigned different `memory_type` classifications.

### 4. Cross-Department Context Sharing

Every memory is stamped with a tenant ID (`organization_id`) and written to the central, cross-functional memory table `autodream_memories_master`, rather than isolated department-specific tables.

This allows the Business Advisory department to query the universal Vector Repository and retrieve context originally synthesized by Customer Success or Operations, bridging domain silos automatically via semantic vector matching on the user’s queries.
