<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Distributed State Machine Walkthrough

Welcome to the visual guide for the **KAIROS Distributed State Machine**. This subsystem tracks Teammate Mesh dependencies and prevents race conditions across the OHC Swarm.

## 1. Architectural Flow

The state machine uses distributed locks depending on the current mode (`OHC_STANDALONE`).

```mermaid
graph TD
    Agent[Agent Request Transition] --> SM[Distributed State Machine]
    SM --> Router{Is OHC_STANDALONE=true?}

    Router -->|Yes| SQLiteLock[SQLite native locking / application-level mutex]
    SQLiteLock --> Transition[Execute Transition]

    Router -->|No| RedisLock[Redis Distributed Lock]
    RedisLock --> Transition

    Transition --> Broadcast[Broadcast to Teammate Mesh APIs]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,SM,SQLiteLock,RedisLock,Transition,Broadcast premium;
```

## 2. State Transitions
Tasks start in `PENDING` and transition to `IN_PROGRESS`, and finally to `COMPLETED` or `FAILED`. Every valid transition emits an event.

</div>
