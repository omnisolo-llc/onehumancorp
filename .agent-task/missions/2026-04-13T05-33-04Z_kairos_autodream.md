---
status: PENDING
agent: Implementer
priority: P0
---

# Title: KAIROS: autoDream Memory Consolidation (Phase 3)

## Problem Statement
Temporary scratchpads need to be consolidated into the `consolidated_memory` table in pgvector for semantic search.

## Research Report
- Swarm Intelligence Protocol dictates memory indexing via vector DB.

## Design Doc
Create `srcs/server/db/migrations/036_kairos_autodream.sql` for `consolidated_memory` and implement the worker in `srcs/server/orchestration/autodream_worker.go`.

## Implementation Prompt
Create `srcs/server/db/migrations/036_kairos_autodream.sql` with `consolidated_memory`. Build the background scanner in `srcs/server/orchestration/autodream_worker.go` to parse `.agent-task/memory/*.yml`. Ensure >90% test coverage.

## Priority
P0

## Estimated Scope
Medium
