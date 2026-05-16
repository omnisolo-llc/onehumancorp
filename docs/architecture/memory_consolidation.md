# OHC AI Agent Memory Consolidation System

## 1. Persistent Memory Layer
The layer runs across Cloud (PostgreSQL + `vector`) and Standalone (SQLite) modes. It leverages `consolidated_memory` schema storing `tenant_id`, `agent_id`, `embedding`, and `content`. All search operations enforce `tenant_id` scopes to ensure business isolation.

## 2. Conflict Resolution
Conflicts are detected by identical or near-identical embeddings via cosine distance `< 0.05`.
Resolution Strategy (`auto_resolve_conflicts`):
- Explicit `owner_override` wins.
- Higher `reliability_score` wins.
- More recent `created_at` wins.
The loser is deleted, and the winner's `reference_count` is incremented.

## 3. Stale Context Pruning
A background `ConsolidationWorker` periodically calls `prune_stale()`. Context is deleted if:
- It exceeds the `pruning_threshold_days` (default 180).
- AND it is not explicitly overridden (`owner_override = FALSE`).
- AND it has fewer than 5 references.

## 4. Cross-Department Context Sharing
Memory is persisted in a unified `consolidated_memory` pool accessible by any agent within the same `tenant_id`. Searches via `cross_department_search` allow agents (e.g. Sales) to pull context originally embedded by another agent (e.g. Operations) using semantic vector distances.
