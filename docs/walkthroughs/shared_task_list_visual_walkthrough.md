<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Shared Task List: Visual Walkthrough

This document provides a visual representation of the KAIROS Shared Task List state machine and lifecycle within the OHC Hybrid Architecture.

## State Machine Overview

The Shared Task List handles the complex Directed Acyclic Graph (DAG) dependencies for agentic workflows, orchestrating tasks across both Cloud-Native (PostgreSQL + Redis) and Standalone (SQLite) modes.

```mermaid
sequenceDiagram
    participant User
    participant Orchestrator
    participant Database as Shared Task DB (PG/SQLite)
    participant Mesh as Teammate Mesh (Redis/Channel)
    participant Agent

    User->>Orchestrator: Submit High-Level Goal
    Orchestrator->>Orchestrator: Decompose into DAG
    Orchestrator->>Database: INSERT sub-tasks (status: PENDING)
    Orchestrator->>Mesh: Publish(TaskAvailableEvent)
    Mesh-->>Agent: Subscribes & receives event
    Agent->>Database: Attempt Claim (Mutex / FOR UPDATE SKIP LOCKED)
    alt Claim Success
        Database-->>Agent: Returns Task
        Agent->>Database: UPDATE task (status: IN_PROGRESS)
        Agent->>Agent: Execute Task Logic
        Agent->>Database: UPDATE task (status: DONE)
        Agent->>Mesh: Publish(TaskCompletedEvent)
        Orchestrator->>Orchestrator: Check DAG dependencies
        alt Next Tasks Ready
            Orchestrator->>Mesh: Publish(TaskAvailableEvent)
        end
    else Claim Failed (Locked/Claimed)
        Database-->>Agent: Returns ErrNoRows / Locked
        Agent->>Agent: Backoff & Retry later
    end
```

</div>
