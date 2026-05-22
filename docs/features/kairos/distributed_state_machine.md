<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Distributed State Machine

The KAIROS Distributed State Machine governs the lifecycle of tasks across the swarm, ensuring hybrid consistency.

## State Transitions

```mermaid
stateDiagram-v2
    [*] --> PENDING
    PENDING --> IN_PROGRESS : Claimed by Agent
    IN_PROGRESS --> COMPLETED : Successfully Executed
    IN_PROGRESS --> FAILED : Execution Error
    FAILED --> PENDING : Retried
```

## Details
- Utilizes `FOR UPDATE SKIP LOCKED` on PostgreSQL.
- Falls back to application mutexes on SQLite.

</div>
