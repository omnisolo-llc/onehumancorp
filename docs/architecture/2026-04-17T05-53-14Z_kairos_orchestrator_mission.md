---
status: PENDING
agent: Principal Growth Engineer & Nova (L7)
title: Align KAIROS Implementation with Master Blueprint
priority: P0
scope: Medium
---

# Title: Align KAIROS Implementation with Master Blueprint

## Problem Statement
The current implementation of the KAIROS Orchestration schemas in `srcs/server/db/migrations/20260416050000_kairos_orchestrator.sql` uses incorrect table names (`shared_tasks_decomposition` and `autodream_memories`) instead of the required ones (`kairos_shared_tasks` and `autodream_vector_memories`) defined in the master blueprint `docs/architecture/2026-04-16T13-26-47Z_kairos_shared_task_mesh_auto_dream.md`. Furthermore, the implementer needs to locate and update the sub-agent queue table schema to `kairos_sub_agent_queue`. Additionally, `srcs/server/api/mesh/mesh.go` and its tests must be thoroughly reviewed and updated to strictly enforce the new `{"agent_id": "...", "channel": "mesh:tasks", "event_type": "...", "data": ...}` OHC-SIP payload contract for `POST /api/mesh/broadcast`.

## Research Report
I verified the codebase. The `docs/architecture/2026-04-16T13-26-47Z_kairos_shared_task_mesh_auto_dream.md` defines the exact expected table names:
- `kairos_shared_tasks`
- `autodream_vector_memories`
- `kairos_sub_agent_queue`

However, the latest migration `20260416050000_kairos_orchestrator.sql` uses `shared_tasks_decomposition` and `autodream_memories`. We need to bring the implementation in line with the final master blueprint.

## Design Doc
Please refer strictly to `docs/architecture/2026-04-16T13-26-47Z_kairos_shared_task_mesh_auto_dream.md`.

## Implementation Prompt
Hello Implementer! Your objective is to align the KAIROS backend with the master blueprint.
1. Update `srcs/server/db/migrations/20260416050000_kairos_orchestrator.sql` (and any subsequent migrations) to use the exact table names: `kairos_shared_tasks` and `autodream_vector_memories`.
2. Independently locate and update the sub-agent queue table schema to `kairos_sub_agent_queue`.
3. Review `srcs/server/api/mesh/mesh.go` and `srcs/server/api/mesh/mesh_test.go` to ensure `POST /api/mesh/broadcast` strictly accepts and tests the exact JSON structure defined in Section 2 of `docs/architecture/2026-04-16T13-26-47Z_kairos_shared_task_mesh_auto_dream.md`.
4. Add comprehensive tests to verify the schema and API payload changes.
5. Ensure `./bazelisk test //srcs/server/api/... //srcs/server/db/...` pass.
