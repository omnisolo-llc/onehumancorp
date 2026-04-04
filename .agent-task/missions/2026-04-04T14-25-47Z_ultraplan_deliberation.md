---
status: PENDING
agent: Implementer
---

# Title: Implement UltraPlan Deliberation & Sub-Agent Orchestration

## Problem Statement
The KAIROS Orchestrator requires deep-deliberation cycles for complex architectural changes (e.g., Database migrations, Auth overhaul) within the OHC platform. Currently, the system lacks the state machine tracking and scalable background queuing logic (like BullMQ or Celery) needed to spawn isolated sub-agents in a production environment for these deep-deliberation cycles.

## Research Report
Based on the Master Architecture Vision and Swarm Intelligence Protocol:
- **UltraPlan Deliberation**: Requires a state machine (`ultraplan_states` table) to track the deliberation phases (`PROPOSAL` -> `DELIBERATING` -> `APPROVED`).
- **Sub-Agent Orchestration**: We need robust background queuing logic. In Cloud Mode, this means utilizing PostgreSQL distributed locks to dispatch isolated agent tasks, similar to BullMQ or Celery. In Standalone Mode, this must degrade gracefully to local Goroutine pools.
- **Teammate Mesh Integration**: State transitions and sub-agent spawning events must be broadcasted over the Teammate Mesh to maintain realtime observability.

## Design Doc
1. **Schema Updates**: Create `ultraplan_states` table to track deliberation cycles.
2. **State Machine Logic**: Implement deliberation transition logic in `srcs/server/orchestration/`. Ensure distributed lock usage via `FOR UPDATE SKIP LOCKED` in Postgres.
3. **Queuing Abstraction**: Implement an orchestration queuing layer (`SubAgentQueue`).
    - **Cloud**: Postgres-backed queue using distributed locks.
    - **Standalone**: Channel-backed Goroutine pool (check `pool.IsSQLite()`).
4. **Visual Excellence**: Any exposed dashboard metrics must follow the Premium Feel mandate (`backdrop-filter: blur(20px) saturate(200%)`).

## Implementation Prompt
1. Read `docs/kairos_orchestration_design.md` for context on UltraPlan Deliberation and Sub-Agent Orchestration.
2. Add a new migration in `srcs/server/db/migrations/` (e.g., `015_ultraplan_deliberation.sql`) to create the `ultraplan_states` table. Ensure the file is added to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
3. In `srcs/server/orchestration/`, implement the state machine for deep-deliberation cycles. Use explicit transactions (`tx, err := pool.Begin(ctx)`) when acquiring locks with `FOR UPDATE SKIP LOCKED`.
4. Implement the `SubAgentQueue` component. Ensure it checks `pool.IsSQLite()` to safely switch between local Goroutine processing and Postgres distributed queues.
5. Broadcast task updates to the Teammate Mesh API when deliberation states change.
6. Write unit tests ensuring >90% coverage for the queueing fallback logic and the state transitions.
7. Verify all changes pass tests using Bazel (`bazel test //...`).

## Priority
P1

## Estimated Scope
Large
