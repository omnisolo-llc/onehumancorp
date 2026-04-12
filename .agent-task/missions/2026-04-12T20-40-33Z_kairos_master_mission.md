---
status: DONE
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

Competitive Analysis: Competitors rely entirely on stateless API calls or slow long-polling for orchestration. OHC's Hybrid Architecture uses real-time Pub/Sub channels connected directly to persistent local and remote databases.

## Design Doc
**Phase 1 Schema (srcs/server/db/migrations/[NEXT_SEQ]_kairos_shared_tasks.sql):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
**Phase 2 Teammate Mesh:**
Implement `LocalTeammateMesh` struct in `srcs/server/orchestration/` that leverages `rueidis.Client` for cloud mode and `sync.Cond` / channels for standalone mode to broadcast swarm events.

**Phase 3 autoDream (srcs/server/db/migrations/[NEXT_SEQ]_kairos_autodream.sql):**
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
**Phase 4 Design Doc:** Comprehensive MD documents are available inside `docs/features/kairos/`.

## Implementation Prompt
Hello Implementer agent!
1. Create SQL migrations `[NEXT_SEQ]_kairos_shared_tasks.sql` and `[NEXT_SEQ]_kairos_autodream.sql`. Ensure both degrade gracefully to SQLite using VARCHAR PRIMARY KEY for ids instead of Postgres gen_random_uuid().
2. Implement `LocalTeammateMesh` component in the orchestration package.
3. Achieve >90% test coverage.

## Priority
P0

## Estimated Scope
Large
