<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Architect Distributed State Machine

## Problem Statement
The KAIROS orchestrator needs to manage complex workflows with intricate task dependencies within the Shared Task List. Currently, there is no centralized, durable way to track the state transitions of tasks and their associated Teammate Mesh coordination messages, leading to potential race conditions and deadlocks in the hybrid environment.

## Research Report
*   **Market Analysis**: High-reliability workflow engines (e.g., AWS Step Functions, Temporal) use rigorous state machine paradigms to ensure fault tolerance. OHC needs a localized version of this that works across both Postgres/Redis (Cloud) and SQLite (Standalone).
*   **State Machine Design**: A distributed state machine for KAIROS should handle standard states (`PENDING`, `IN_PROGRESS`, `BLOCKED`, `COMPLETED`, `FAILED`) and support deterministic transitions triggered by events from the Teammate Mesh.
*   **Concurrency Control**: Leveraging Redis Distributed Locks (Cloud) or SQLite write locks (Standalone) is critical to prevent split-brain execution of the same task state transition.

## Design Doc

### 1. State Machine Definition
**States**:
*   `PENDING`: Task created, dependencies not yet met.
*   `READY`: Dependencies met, waiting for sub-agent assignment.
*   `IN_PROGRESS`: Sub-agent actively executing.
*   `BLOCKED`: Sub-agent requires external input or encountered a recoverable error.
*   `COMPLETED`: Execution successful.
*   `FAILED`: Unrecoverable error.

**Transitions**:
*   `PENDING` -> `READY` (Trigger: Dependencies resolve)
*   `READY` -> `IN_PROGRESS` (Trigger: Sub-agent assigned)
*   `IN_PROGRESS` -> `COMPLETED` (Trigger: Success report)
*   `IN_PROGRESS` -> `BLOCKED` (Trigger: Teammate Mesh negotiation request)

### 2. Distributed Locking Mechanism
**Cloud Mode**:
*   Use Redis-based Redlock algorithm via `github.com/go-redsync/redsync`.
*   Lock key format: `ohc:lock:task:{task_id}`.

**Standalone Mode**:
*   Rely on SQLite's `BEGIN EXCLUSIVE` transactions or an in-memory sync.Mutex pool for the single instance.

### 3. Transition Logic
*   Implement a `StateMachine` service that accepts a `StateTransitionRequest`.
*   Before applying the transition, the service acquires the appropriate distributed lock.
*   It validates the transition against an allowed transitions map.
*   If valid, it updates the database and publishes a state change event to the Teammate Mesh.

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Implement the Distributed State Machine for KAIROS task orchestration.
**Instructions**:
1.  **Locking Abstraction**: Create a `DistributedLock` interface in `srcs/server/orchestration/locks.go` with implementations for Redis (Cloud) and Mutex (Standalone).
2.  **State Machine Core**: Implement the `StateMachine` struct in `srcs/server/orchestration/statemachine.go`. It should wrap the database repository and the locking mechanism.
3.  **Transition Methods**: Implement methods for standard transitions (e.g., `TransitionToReady(ctx, taskID)`, `TransitionToInProgress(ctx, taskID, agentID)`).
4.  **Tests**: Write unit tests simulating concurrent transition requests to ensure the locking mechanism prevents invalid states.
**Acceptance Criteria**:
*   The State Machine correctly validates allowed transitions.
*   Concurrent transition requests for the same task do not result in race conditions.
*   Test coverage for new packages is >90%.

## Priority
P1

## Estimated Scope
Medium

</div>
