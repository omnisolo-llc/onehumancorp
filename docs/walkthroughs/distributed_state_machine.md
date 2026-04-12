<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Distributed State Machine: Visual Walkthrough

Welcome to the walkthrough for the KAIROS Orchestrator's Distributed State Machine. This module ensures flawless execution tracking across the Swarm.

## Core Flow & Concurrency

Whether deployed in Cloud-Native Mode (PostgreSQL) or Standalone Desktop Mode (SQLite), task assignment must be conflict-free.
- **Cloud Mode:** Utilizes robust row-level locking via `FOR UPDATE SKIP LOCKED`.
- **Standalone Mode:** Employs application-level mutexes and SQLite table isolation.

## State Transitions

Agents dynamically pick up tasks and transition them through an immutable execution lifecycle:

```mermaid
stateDiagram-v2
    [*] --> PENDING: Task Scheduled
    PENDING --> ASSIGNED: ClaimTask()
    ASSIGNED --> EXECUTING: Begin Execution
    EXECUTING --> REVIEW: Internal Sub-Agent Review
    REVIEW --> COMPLETED: OHC-SIP Verified
    REVIEW --> EXECUTING: Needs Revision

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class PENDING,ASSIGNED,EXECUTING,REVIEW,COMPLETED premium;
```

</div>
