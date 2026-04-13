---
status: PENDING
agent: Implementer
priority: P0
---

# Title: Implement KAIROS Master Loop Playbook: Orchestration Backend

## Problem Statement
The KAIROS Orchestrator serves as the core intelligence engine for the OHC Swarm. We need to implement the backend database designs and API integrations defined in the "Execution Playbook" to manage deep-deliberation cycles and long-running distributed tasks effectively across the Hybrid Architecture.

## Research Report
The KAIROS execution playbook defines four key phases:
- **Phase 1 (UltraPlan/Decomposition):** Create backend database designs (`shared_tasks`) and sequence diagrams for the Shared Task List.
- **Phase 2 (Orchestration):** Design the Realtime Teammate Mesh APIs (e.g. `mesh:tasks`, `mesh:coordination`) using `rueidis` (Redis) or Go channels.
- **Phase 3 (autoDream):** Architect data pipelines for memory consolidation using `autodream_memories` table (with `pgvector` or graceful SQLite degradation).
- **Phase 4 (Finalize):** Prepare a Premium Design Doc detailing implementation for KAIROS features.

## Design Doc
**Phase 1 Schema (srcs/server/db/migrations/035_kairos_shared_tasks.sql):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
**Phase 2 Teammate Mesh (srcs/server/orchestration/mesh.go):**
Implement `LocalTeammateMesh` struct that leverages `rueidis.Client` for cloud mode and `sync.Cond` / channels for standalone mode to broadcast swarm events.

**Phase 3 autoDream (srcs/server/db/migrations/036_kairos_autodream.sql):**
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
**Phase 4 Design Doc:** A comprehensive MD document inside `docs/features/kairos/`.

## Implementation Prompt
Hello Implementer agent!
1. Create SQL migration `035_kairos_shared_tasks.sql` for the Shared Task List and `036_kairos_autodream.sql` for the autoDream memory pipeline. Ensure both degrade gracefully to SQLite.
2. Update `srcs/server/db/BUILD.bazel` to include these migrations in `embedsrcs`.
3. Implement `LocalTeammateMesh` in `srcs/server/orchestration/mesh.go` that handles Teammate Mesh APIs (Realtime broadcast/subscribe).
4. Achieve >90% test coverage for the new packages and verify using `bazelisk test //srcs/server/orchestration/...` and `bazelisk test //srcs/server/db/...`.

## Priority
P0

## Estimated Scope
Large
