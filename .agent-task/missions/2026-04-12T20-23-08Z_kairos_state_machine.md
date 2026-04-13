---
status: PENDING
priority: P0
scope: Large
title: "KAIROS: Design Distributed State Machine Tracking"
---

# Title: Design Distributed State Machine Tracking

## Problem Statement
The KAIROS orchestrator requires a robust, distributed state machine (backed by database locks/Redis) to track the Teammate Mesh dependencies. This ensures that multi-agent orchestrations and long-running complex feature tasks don't fail midway or create race conditions across the Swarm.

## Research Report
- OHC agents coordinate asynchronously. Task A might rely on Task B and C.
- We need to robustly track these transitions (e.g., PENDING -> IN_PROGRESS -> DONE).
- State transitions must use Redis Distributed Locks in Cloud Mode, and SQLite native locking in Standalone Mode.
- This fulfills the "State Machine Tracking" mandate.

## Design Doc
1.  **State Machine Core:**
    - Define states and allowed transitions in `srcs/server/orchestration/state_machine.go`.
2.  **Distributed Locking Interface:**
    - `StateLock` interface (`Acquire`, `Release`).
    - `RedisLock` implementation using Redsync.
    - `SQLiteLock` using application-level mutex + select-for-update (or equivalent SQLite serialization).
3.  **Transition Execution:**
    - Logic to assert transition validity and notify the Teammate Mesh upon successful transition.

## Implementation Prompt
- Implement the `DistributedStateMachine` struct and logic in `srcs/server/orchestration/state_machine.go`.
- Implement robust distributed locking implementations for both Redis and SQLite.
- Ensure that every valid state transition emits a broadcast event to the Teammate Mesh APIs.
- Write tests to ensure >90% code coverage.
