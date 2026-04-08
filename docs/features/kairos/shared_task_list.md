<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Shared Task List

The Shared Task List is the structural backbone of KAIROS orchestration, enabling robust agent coordination and multi-agent workflow execution within the One Human Corp (OHC) platform.

## 1. Problem Statement

To fully realize "KAIROS Mode" (task decomposition), agents need a centralized way to break down complex instructions into a shared queue. The swarm requires a mechanism to manage state dependencies, ensuring that if a sub-agent crashes, its tasks are safely retried or delegated, preventing dag-stalls.

## 2. Competitive Landscape

| Feature | OHC Hybrid (Proposed) | AutoGen | LangGraph |
| :--- | :--- | :--- | :--- |
| **State Persistence** | Hybrid (Postgres/SQLite) | In-Memory / File | Postgres Checkpointer |
| **Transition Safety** | Distributed Locks + DB | Scripted Control Flow | DAG Checkpointing |
| **Realtime Updates** | Teammate Mesh (Redis/Centrifuge) | Polling / Stdout | Graph Events |
| **Aesthetic Guarantee** | Glassmorphism (UI Mandate) | Terminal | Studio (Web) |

## 3. Architecture

### Database Schema

We define a unified schema capable of storing tasks and sub-tasks hierarchically:

```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT REFERENCES shared_tasks(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT NOT NULL,
    status TEXT NOT NULL,
    assigned_agent_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

### Orchestration Primitives
The service interacts with other KAIROS pillars:
1.  **State Machine:** Every transition (e.g., `PENDING` -> `ASSIGNED`) is routed through the Distributed State Machine Tracker to acquire a lock and ensure validity.
2.  **Teammate Mesh:** Transitions trigger Pub/Sub broadcasts to alert other agents in real-time.
3.  **Sub-Agent Queue:** Tasks without dependencies are pushed to the background queue for asynchronous pickup by idle sub-agents.
4.  **AutoDream Pipeline:** On `COMPLETED`, task metadata is swept and vectorized for episodic memory retrieval.

## 4. OHC CSS Tokens
*(Visual Reference for Front-End Implementers)*
- `backdrop-filter`: `blur(20px) saturate(200%)`
- `background`: `rgba(255, 255, 255, 0.03)`
- `font-family`: `'Outfit', 'Inter', sans-serif`

</div>
