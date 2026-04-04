---
status: PENDING
agent: Implementer
---

# Title: Implement Teammate Mesh APIs and Shared Task List Orchestration

## Problem Statement
The OHC (One Human Corp) platform requires a sophisticated mechanism to decompose high-level feature requests into a shared task list for the agent team. To achieve seamless collaboration among agents in a production environment, we need to architect a highly available real-time communication layer (Teammate Mesh) and a distributed state machine for tracking task dependencies.

## Research Report
Based on the provided mission details, the OHC architecture necessitates:
1. **Shared Task List**: A database-backed structure mapping high-level requests into decomposed sub-tasks.
2. **Teammate Mesh**: A real-time coordination layer via Redis Pub/Sub channels (`mesh:tasks`, `mesh:coordination`).
3. **State Tracking**: A distributed state machine with robust locking to track dependencies (DAG relationships).

## Design Doc
The task requires changes in multiple phases:
- **Phase 1: Database Architecture**:
  - Add migrations for tables: `shared_tasks` (to track distributed state) and `task_dependencies` (DAG relationships mapping dependency blocking).
- **Phase 2: Teammate Mesh APIs**:
  - Implement Redis Pub/Sub channels to broadcast task updates and coordination messages.
- **Phase 3: State Machine Logic**:
  - Use database locks (e.g., `FOR UPDATE SKIP LOCKED`) and `Teammate Mesh` to coordinate agent pickup and execution.
- **Phase 4: Agent Payload Context Injection**:
  - Define Omni-Context Sub-agent routing hooks for generating context-rich missions.

## Implementation Prompt
Dear Implementer Agent,
Please implement the foundational database structures and API services for the OHC KAIROS Orchestration shared task list.

1. Create a database migration script `srcs/server/db/migrations/020_kairos_orchestration.sql`:
   - Create table `shared_tasks` (`id UUID PRIMARY KEY`, `title TEXT`, `status TEXT`, `agent_id UUID`, `created_at TIMESTAMP`).
   - Create table `task_dependencies` (`id UUID PRIMARY KEY`, `task_id UUID`, `depends_on_task_id UUID`).
   - Remember to add this file to `embedsrcs` in `srcs/server/db/BUILD.bazel`.

2. In `srcs/server/orchestration/tasks.go` (or similar service file), implement the `SharedTaskListManager`:
   - Functions to create, update, and fetch tasks.
   - Mechanism to broadcast task state changes via the Teammate Mesh (Centrifuge/Redis). Use channel name `mesh:tasks`.

3. Ensure to use database locks for atomic status updates.

4. Add relevant OpenTelemetry metrics.

5. Update tests to reach >90% coverage for the new module.

## Priority
P0

## Estimated Scope
Large
