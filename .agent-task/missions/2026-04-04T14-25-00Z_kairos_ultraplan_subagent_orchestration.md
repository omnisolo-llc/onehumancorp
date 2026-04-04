---
status: PENDING
agent: Implementer
---

# Title: KAIROS Orchestration: UltraPlan Deliberation & Sub-Agent Queuing

## Problem Statement
The OHC Hybrid Agentic OS requires a robust distributed system to decompose high-level requests into manageable tasks for the agent swarm. Currently, the system lacks a fully integrated KAIROS orchestrator that handles complex architectural changes via deep-deliberation cycles (UltraPlan) and scalable sub-agent orchestration. A comprehensive robust background queuing framework abstraction (e.g., mimicking BullMQ or Celery) must be put in place to handle these tasks resiliently across Hybrid modes (Cloud Redis & Standalone SQLite).

## Research Report
An analysis of the existing codebase (`srcs/server/orchestration/tasks.go` and `srcs/server/db/migrations/`) demonstrates partial task tracking but lacks the true structural support needed for "UltraPlan" deliberative loops and multi-tenant sub-agent processing:
1.  **UltraPlan State Machine**: We lack a `swarm_ultra_plans` database schema to track deep-deliberation parent states and bind them to smaller `swarm_tasks` via a `parent_plan_id`.
2.  **Sub-Agent Orchestration**: OHC swarm workers need an interface abstraction (`TaskQueue`) mimicking BullMQ/Celery. In Cloud-Native Mode, this must utilize Redis lists or sorted sets via `rueidis`. In Standalone Mode, it must rely on SQLite local polling loops.
3.  **Teammate Mesh Integration**: State changes within the `UltraPlan` lifecycle must be broadcast real-time via Centrifuge WebSockets using channels like `mesh:ultraplan:<plan_id>`.

## Design Doc
1.  **Database Migrations**: Add `srcs/server/db/migrations/010_ultraplan.sql` to include a `swarm_ultra_plans` table and `swarm_dream_epochs` for AutoDream consolidation. Also, modify `swarm_tasks` queries to respect `parent_plan_id`.
2.  **Queue Interface**: Abstract a queue model inside `srcs/server/orchestration/queue.go` offering:
    *   `Enqueue(ctx, queueName, payload)`
    *   `EnqueueDelayed(ctx, queueName, payload, delay)`
    *   `Poll(ctx, queueName)`
    *   `Complete(ctx, queueName, taskID)`
3.  **Sub-Agent Orchestrator Loop**: Build a capacity-managed worker loop `SubAgentOrchestrator` to automatically consume from the `TaskQueue` and spawn isolated sub-agents.
4.  **UltraPlan Manager**: Implement `UltraPlanManager` in `srcs/server/orchestration/ultraplan.go` to handle state transitions between `DELIBERATING`, `EXECUTING`, `COMPLETED`, and `FAILED` while broadcasting via the mesh.
5.  **Aesthetic Core (Doc/Log alignment)**: Ensure telemetry and metrics strictly follow the "Premium Feel" where appropriate by exposing granular dashboard metrics.

## Implementation Prompt
**Role**: Implementer
**Task**: Build out the missing KAIROS Orchestration structural primitives for UltraPlan and Queuing.

1.  **Database Additions**:
    *   Verify or create `010_ultraplan.sql` in `srcs/server/db/migrations/`.
    *   Table `swarm_ultra_plans` should have fields: `id` (UUID), `mission_id`, `status` (DELIBERATING, EXECUTING, COMPLETED, FAILED), `state_machine` (JSONB), and timestamps.
    *   Ensure the `010_ultraplan.sql` file is added to `embedsrcs` within `srcs/server/db/BUILD.bazel`.
2.  **UltraPlan API**:
    *   Implement `srcs/server/orchestration/ultraplan.go` defining the `UltraPlanManager`.
    *   Implement `CreatePlan`, `UpdatePlanStatus`, and `GetUltraPlan`.
    *   Broadcast status updates on the Teammate Mesh using the existing Centrifuge logic.
3.  **Sub-Agent Queue Abstraction**:
    *   Implement `srcs/server/orchestration/queue.go` containing a `TaskQueue` interface.
    *   Provide two implementations: `RedisTaskQueue` (using `rueidis`) and `SQLiteTaskQueue`.
    *   Create a background loop `SubAgentOrchestrator` in `sub_agent_worker.go` that utilizes the queue to simulate isolated worker processing.
4.  **Testing Requirements**:
    *   Write `ultraplan_test.go` and `queue_test.go`.
    *   Maintain >90% coverage for new orchestration code.
    *   Test both Standalone (SQLite) and Cloud (Redis) code paths.
5.  **Build & Verify**:
    *   Execute `bazelisk test //srcs/server/orchestration/...` to confirm functionality. Ensure tests run successfully locally.

## Priority
P0

## Estimated Scope
Large
