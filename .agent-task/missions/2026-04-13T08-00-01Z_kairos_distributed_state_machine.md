---
title: "KAIROS: Distributed State Machine Tracking for Teammate Mesh"
status: DONE
priority: P0
scope: Large
agent: Implementer
agent_id: jules
---

# Title: KAIROS: Distributed State Machine Tracking for Teammate Mesh

## Problem Statement
The OHC Swarm requires a robust, distributed state machine to track complex dependencies across the Teammate Mesh. Currently, state transitions are handled locally within `SharedTaskOrchestrator` or `UltraPlanManager`, but there is no centralized, durable log that tracks the global "Swarm State" and ensures that mesh events trigger the correct state transitions across disparate agent workers. We need a unified `DistributedStateMachine` that leverages database locks (Postgres `SKIP LOCKED`) or Redis to maintain consistency.

## Research Report
- Agents currently coordinate via Redis Pub/Sub, but if an agent crashes after publishing an event but before updating its local state, the swarm enters an inconsistent state.
- A "Check-and-Set" (CAS) pattern is required for state transitions.
- The `state_machine_transitions` table already exists but is underutilized for orchestration.
- Reference: `srcs/server/orchestration/statemachine/machine.go` provides a basic structure but lacks distributed enforcement.

## Design Doc
1. **Unified State Machine Interface**:
   - `Transition(ctx, entityID, event) (newState, error)`
   - `GetState(ctx, entityID) (State, error)`
2. **Distributed Locking Logic**:
   - Cloud: Use `rueidis` distributed locks (Redlock pattern) for high-contention transitions.
   - Standalone: Use `SELECT FOR UPDATE` in a SQLite transaction with application-level mutexes.
3. **Event Integration**:
   - Every successful transition MUST broadcast an OHC-SIP compliant message to the Teammate Mesh: `{"agent_id": "...", "action": "STATE_TRANSITION", "status": "SUCCESS", "payload": {"from": "...", "to": "..."}}`.
4. **Audit Trail**:
   - Persist every transition to `state_machine_transitions` for observability and "AutoDream" consolidation.

## Implementation Prompt
Hello Implementer! Your mission is to build the `DistributedStateMachine` in `srcs/server/orchestration/dist_state_machine.go`.
1. Implement a `Manager` struct that accepts a `db.Provider` and a `rueidis.Client`.
2. Implement the `Transition` method. It must:
   - Acquire a distributed lock for the `entityID`.
   - Fetch the current state from the database.
   - Validate if the `event` is allowed for the current state.
   - Update the state in the relevant table (e.g., `shared_tasks` or `swarm_ultra_plans`).
   - Log the transition in `state_machine_transitions`.
   - Broadcast the change via the `Teammate Mesh` (using `mesh.BroadcastMeshEvent`).
3. Ensure absolute parity between Cloud-Native (Redis) and Standalone (SQLite) locking strategies.
4. Write tests in `srcs/server/orchestration/dist_state_machine_test.go` simulating concurrent transition attempts and verifying zero race conditions.
5. Verification: `go test ./srcs/server/orchestration/...`

## Priority
P0

## Estimated Scope
Large
