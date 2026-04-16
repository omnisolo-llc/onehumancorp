<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Interactive API Playbook Walkthrough

Welcome to the interactive walkthrough for the KAIROS Orchestration APIs.

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

### 3. Hybrid Health Probe
**GET** `/api/v1/health`
- **Response**: `{"mode": "cloud", "status": "healthy", "db_ping": 15000000, "sync_backlog": 0, "stuck_missions": 0, "mesh_active": true}`

### 4. Hybrid CRDT Sync MCP Tools
**Tool Invoke**: `crdt_push`
- **Payload**: `{"entity_id": "task_12345", "mutations": [{"clock": 42, "op": "set", "path": "status", "value": "COMPLETED"}]}`

## Hybrid Architecture Visualizations

### Hybrid Health Probe Flow

```mermaid
graph TD
    A[Client Request] -->|GET /api/v1/health| B(Orchestrator Hub)
    B -.->|Ping| C[(Shared Task DB)]
    B -.->|Check Backlog| C
    B -.->|Publish mesh:health| D((Teammate Mesh))
    D -.->|pong| B
    B -->|Returns HybridHealthProbe JSON| A

    style A fill:#003366,stroke:#333,stroke-width:2px,color:#fff
    style B fill:#006699,stroke:#333,stroke-width:2px,color:#fff
    style C fill:#0099cc,stroke:#333,stroke-width:2px,color:#fff
    style D fill:#00ccff,stroke:#333,stroke-width:2px,color:#111
```

### Hybrid CRDT State Synchronization

```mermaid
graph TD
    A[Standalone Mode] -->|Local Edits| B(SQLite DB)
    B -.->|crdt_push via MCP| C{Cloud MCP Gateway}
    C -->|crdt_merge| D(PostgreSQL DB)
    D -->|crdt_pull| E[Cloud Swarm Orchestration]

    style A fill:#003366,stroke:#333,stroke-width:2px,color:#fff
    style B fill:#006699,stroke:#333,stroke-width:2px,color:#fff
    style D fill:#0099cc,stroke:#333,stroke-width:2px,color:#fff
    style E fill:#00ccff,stroke:#333,stroke-width:2px,color:#111
```

</div>
