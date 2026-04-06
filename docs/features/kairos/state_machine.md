<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Distributed State Machine

**Component:** Orchestration Layer | **Target Audience:** Orchestration Engineers & Architects

## 1. Overview
The **KAIROS Distributed State Machine** provides strict, resilient state transition tracking for multi-agent workflows across the Teammate Mesh. It resolves the problem of brittle dependencies in complex Directed Acyclic Graph (DAG) task structures where agent failure could lead to permanently stalled workflows.

By externalizing the coordination state (using Redis distributed locks in Cloud-Native mode or database row locks in Standalone mode), the State Machine ensures tasks transition deterministically and survive worker pod failures.

## 2. State Transition Flow

The State Machine enforces allowed transitions. A typical task will progress through the following lifecycle:

```mermaid
stateDiagram-v2
    [*] --> PENDING : Task Delegated
    PENDING --> IN_PROGRESS : Agent Claims Task
    IN_PROGRESS --> REVIEW : Agent Submits Work
    REVIEW --> IN_PROGRESS : Review Failed (Rework)
    REVIEW --> COMPLETED : Review Passed
    IN_PROGRESS --> FAILED : Execution Error / Timeout
    FAILED --> PENDING : Auto-Requeue (if retries > 0)
    COMPLETED --> [*]
    FAILED --> [*] : Dead Letter
```

## 3. High Availability and Pub/Sub
Every time a state transitions, the State Machine orchestrator emits a validated event to the `Teammate Mesh` (via Centrifuge Realtime Pub/Sub). This immediately updates any connected human dashboards or listening agents.

### Example Sequence
```mermaid
sequenceDiagram
    participant Worker as Agent Worker
    participant StateMachine as KAIROS State Machine
    participant Locks as Distributed Lock (Redis/DB)
    participant Mesh as Teammate Mesh Pub/Sub

    Worker->>StateMachine: Request Transition (IN_PROGRESS -> COMPLETED)
    StateMachine->>Locks: Acquire Lock for Task ID
    Locks-->>StateMachine: Lock Acquired
    StateMachine->>StateMachine: Validate Transition Rule
    StateMachine->>StateMachine: Update Persistent State
    StateMachine->>Mesh: Broadcast TASK_TRANSITION Event
    StateMachine->>Locks: Release Lock
    Mesh-->>Worker: Acknowledge
```

## 4. Resilience and Failover
- **Timeout Monitoring:** The State Machine continuously monitors tasks in `IN_PROGRESS` or `REVIEW`. If a timeout is exceeded without an active heartbeat, the state is transitioned to `FAILED` or re-queued to `PENDING`.
- **Idempotency:** Transition requests are idempotent to prevent double-processing during network partitions.
- **Standalone Degradation:** If Redis is unavailable (Standalone mode), the State Machine seamlessly falls back to SQLite transaction locks with specific retry backoffs to handle `ohc_sqlite_lock_contention_total`.

</div>
