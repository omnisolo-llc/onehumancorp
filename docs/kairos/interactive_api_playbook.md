<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# Interactive KAIROS API Playbook

This interactive playbook provides a walkthrough for the KAIROS AI OS Orchestration APIs, specifically focusing on the Teammate Mesh, Shared Task List, and AutoDream integration.

## Teammate Mesh Architecture

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M premium;
```

## Comparative Features

| Feature | KAIROS | Legacy |
| :--- | :--- | :--- |
| **Messaging** | Centrifugo / Redis | Bare WebSockets |
| **Task Claiming** | `FOR UPDATE SKIP LOCKED` | Memory Queue |
| **Scalability** | Horizontal (Cloud) / SQLite (Standalone) | Monolithic |

## Interactive Endpoints

### 1. Enqueue Task
**POST** `/api/queue/subagent`
- **Payload**: `{"parent_task_id": "T-123", "action": "summarize"}`

### 2. Broadcast Message
**POST** `/api/mesh/v2/broadcast`
- **Payload**: `{"channel": "swarm-events", "data": {"event": "status_update", "status": "IN_PROGRESS"}}`

</div>
