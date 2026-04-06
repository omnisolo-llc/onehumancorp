status: IN_PROGRESS
agent: Jules
---
title: "Distributed State Machine Tracker for Teammate Mesh Dependencies"
status: PENDING
priority: "P0"
estimated_scope: "Medium"
---

# Problem Statement
The OHC swarm relies on the Teammate Mesh for coordination. However, complex multi-agent workflows (like an Architect delegating to a Coder, who delegates to a Tester) lack a strict, resilient state machine. Without a distributed state machine, task dependencies are brittle: if an agent crashes mid-task, the DAG (Directed Acyclic Graph) of dependencies can stall permanently. We need a robust state machine backed by Redis distributed locks (cloud) and database row locks (standalone) to track and transition agent coordination states reliably.

# Research Report
- Current DAG execution relies on `task_dependencies` but lacks strict transition enforcement (e.g., handling timeouts, auto-requeuing dead states).
- State machines ensure deterministic transitions (e.g., `PENDING` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED` | `FAILED`).
- Distributed systems require externalized state for these transitions to survive worker pod failures.
- A robust distributed state machine must enforce transition rules, handle "stuck" states, and emit events to the Pub/Sub Teammate Mesh upon every state change.

# Design Doc
**Architecture:**
- Create a new package `srcs/server/orchestration/statemachine`.
- Implement `StateMachine` orchestrator.
- States: `STATE_PENDING`, `STATE_ASSIGNED`, `STATE_EXECUTING`, `STATE_WAITING_DELEGATION`, `STATE_REVIEW`, `STATE_SUCCESS`, `STATE_TERMINATED_ERROR`.
- Transitions should be strictly validated against an allowed transition matrix.

**DB Schema Changes:**
```sql
CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL, -- e.g., Task ID
    entity_type TEXT NOT NULL, -- e.g., 'SHARED_TASK'
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    reason TEXT,
    occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_sm_entity ON state_machine_transitions(entity_id, entity_type);
```

**Distributed Lock Mechanism:**
- **Cloud Mode:** Use `rueidis` `SET NX EX` to acquire an exclusive lock on the entity ID before reading current state and transitioning.
- **Standalone Mode:** Use SQLite/PostgreSQL transaction `FOR UPDATE` (if Postgres) or SQLite database lock to serialize transitions.

# Implementation Prompt
You are an Implementer agent. Your task is to build the Distributed State Machine Tracker.
1. Create `srcs/server/orchestration/statemachine/machine.go`.
2. Define the state transition graph.
3. Implement a method `Transition(ctx context.Context, entityID string, toState string, agentID string) error` that:
   - Acquires a distributed lock for `entityID` (using Redis if multi-tenant, else DB transaction).
   - Reads the current state.
   - Validates the transition.
   - Updates the underlying entity (e.g., `shared_tasks`) and records an audit log in `state_machine_transitions`.
   - Emits a Teammate Mesh broadcast via the `CentrifugeNode` hub.
4. Integrate this `StateMachine` into `srcs/server/orchestration/tasks.go` to replace the manual `UPDATE shared_tasks SET status = ...` logic.
5. Write unit tests in `machine_test.go` verifying invalid transitions fail, locks prevent concurrent invalid state mutations, and valid transitions succeed. Ensure >90% coverage.
6. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.