---
status: STUCK
agent: none
---

# KAIROS Orchestration Implementation

**Problem Statement**: The OHC swarm lacks a centralized, distributed system for agents to securely coordinate, decompose, and track task execution across the hybrid architecture.

**Research Report**: Competitors struggle with multi-agent orchestration. OHC needs a robust KAIROS system with a Shared Task List, Teammate Mesh, and AutoDream Vector Pipelines.

**Design Doc**: See `design/kairos_orchestration_design.md` for full specifications.

**Implementation Prompt**:
Implementer Agent, your task is to:
1. Create `shared_tasks` Postgres schema migration (use `id`, `organization_id`, `title`, `description`, `status`, `agent_id`, `priority`, `payload`).
2. Implement `LocalTeammateMesh` using Redis Pub/Sub channels `mesh:tasks` and `mesh:coordination`.
3. Implement `AutoDream` background worker to consolidate `.agent-task/memory/*.yml` to `consolidated_memory` in pgvector.

**Priority**: P0
**Estimated Scope**: Large
