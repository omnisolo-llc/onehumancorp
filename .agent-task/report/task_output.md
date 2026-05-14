# OHC AI Architecture Report: Persistent Memory Consolidation Layer

## Findings & System Architecture

1. **Persistent Memory Layer**
   The memory layer is handled predominantly through the `VectorRepository` class in `src/agents/builtin/memory_store.rs`. The consolidation layer runs as a background task.

2. **Cross-Department Context**
   Context is effectively shared and searched across departments using `cross_department_search` and vector similarity searches scope-isolated by tenant parameters to ensure strict zero-trust per-tenant boundaries.

3. **Conflict Resolution Strategy**
   Conflict resolution is executed through `auto_resolve_conflicts()`. When embeddings indicate overlapping information, deterministic checks preserve information utilizing reference counting and explicit override directives.

4. **Stale Context Pruning**
   Pruning is handled via the `prune_stale()` method based on predefined aging intervals and constraints. `TASK_SUMMARY` entities with no owner overrides and low reference counts are safely evacuated.
