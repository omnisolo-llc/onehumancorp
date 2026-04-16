Parent: #4909

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [research] Architect Distributed State Machine for Teammate Mesh Dependencies

## Problem Statement
The OHC Hybrid Agentic OS requires agents to collaborate via a Teammate Mesh, where complex tasks have inter-agent dependencies. Currently, there is no robust distributed state machine to track these dependencies across the swarm, leading to race conditions or deadlocks when multiple agents attempt to progress a shared, multi-step objective.

## Research Report
An analysis of Swarm intelligence architectures and our internal KAIROS guidelines indicates a need for a distributed state machine:
- **State Management**: Needs to be backed by database locks (for persistent state) and Redis (for real-time signaling) to prevent conflicting state transitions.
- **Dependency Tracking**: Tasks must be able to specify prerequisites (e.g., "Agent B cannot start until Agent A finishes").
- **Resilience**: The system must degrade gracefully in Standalone Desktop Mode (falling back to SQLite locks) while utilizing high-performance Redis primitives in Cloud-Native Mode.

## Design Doc
1. **Schema Design**: Create a new `task_dependencies` table to link parent-child tasks and track execution order. Add state transition tracking to the existing `tasks` schema (e.g., `BLOCKED`, `READY`, `IN_PROGRESS`).
2. **API Contract**: Define gRPC/REST endpoints in `srcs/server/orchestration/state_machine.go` for `TransitionState` and `CheckDependencies`.
3. **Locking Strategy**: Implement a hybrid locking mechanism using `Redis.SetNX` for cloud mode and `SQLite BEGIN EXCLUSIVE` for local mode to guarantee atomic state transitions.

## Implementation Prompt
Hello Implementer!
1. Add the database migration scripts to create the `task_dependencies` table and update the `tasks` schema with new state enums (`BLOCKED`, `READY`).
2. Implement the state transition logic in `srcs/server/orchestration/state_machine.go`, ensuring the correct locking strategy is applied based on the runtime environment (Cloud vs. Standalone).
3. Update the Teammate Mesh event loop to listen for `task.completed` events and automatically transition dependent tasks to `READY`.
4. Run `bazel test //srcs/server/orchestration/...` to verify your implementation.

## Priority
P0

## Estimated Scope
Large

</div>
