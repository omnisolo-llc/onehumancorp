<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Distributed State Machine: Visual Walkthrough

The Distributed State Machine is a core KAIROS Orchestration component that ensures resilient multi-agent task execution across Cloud-Native (Postgres/Redis) and Standalone (SQLite) architectures.

## Execution Flow

```mermaid
stateDiagram-v2
    [*] --> STATE_PENDING
    STATE_PENDING --> STATE_ASSIGNED : Claim Task (Lock Acquired)
    STATE_ASSIGNED --> STATE_EXECUTING : Begin Execution
    STATE_EXECUTING --> STATE_WAITING_DELEGATION : Delegate Sub-tasks
    STATE_WAITING_DELEGATION --> STATE_EXECUTING : Sub-tasks Complete
    STATE_EXECUTING --> STATE_REVIEW : Needs Review
    STATE_REVIEW --> STATE_EXECUTING : Review Failed
    STATE_REVIEW --> STATE_SUCCESS : Review Passed
    STATE_EXECUTING --> STATE_TERMINATED_ERROR : Unrecoverable Error
    STATE_SUCCESS --> [*]
    STATE_TERMINATED_ERROR --> [*]
```

## Resilience and distributed locking

- **Cloud Mode:** Uses Redis `SET NX EX` or Postgres `FOR UPDATE SKIP LOCKED` for high-concurrency lock arbitration.
- **Standalone Mode:** Uses SQLite application-level mutexes for local host efficiency.

</div>
