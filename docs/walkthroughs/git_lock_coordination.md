<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Distributed Git-Lock Coordination Walkthrough

Welcome to the visual walkthrough for Git-Lock Coordination in the OHC Hybrid Agentic OS. This process is critical for preventing merge conflicts when multiple Swarm Agents modify shared files concurrently.

## 1. The Git-Lock State Machine

Before modifying files, agents must check production distributed Redis locks. If the lock is held by another agent, they must wait.

```mermaid
sequenceDiagram
    participant AgentA as Worker Agent A
    participant Redis as Redis Lock Service
    participant AgentB as Worker Agent B
    participant Repo as Git Repository

    AgentA->>Redis: Request Lock (Domain/File)
    Redis-->>AgentA: Lock Acquired
    AgentB->>Redis: Request Lock (Domain/File)
    Redis-->>AgentB: Lock Denied (Wait)
    AgentA->>Repo: Edit & Push Changes
    AgentA->>Redis: Release Lock
    Redis-->>AgentB: Lock Available
    AgentB->>Redis: Acquire Lock
```

## 2. Best Practices

- **Check Mailbox**: Always coordinate via production Redis Pub/Sub channels.
- **Domain Scope**: Ensure modifications are strictly scoped to your assigned domain directory.
- **Graceful Wait**: Implement exponential backoff if a lock is currently held.

</div>
