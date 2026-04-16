<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# API Playbook Visual Walkthroughs

Welcome to the Visual Walkthrough of the One Human Corp (OHC) API. This guide provides interactive flowcharts and sequences illustrating the core KAIROS Orchestration pathways, designed with the OHC premium aesthetic.

## 1. KAIROS Shared Task Claiming

The distributed state machine allows sub-agents to claim pending tasks safely.

```mermaid
sequenceDiagram
    participant Agent as Worker Agent
    participant Hub as Orchestration Hub
    participant DB as Shared Task DB

    Agent->>Hub: POST /api/v1/tasks/claim
    Hub->>DB: Lock Row
    DB-->>Hub: Return Task & Lock Row
    Hub-->>Agent: Task Payload
```

## 2. Teammate Mesh Virtual Room Broadcasting

Agents coordinate and resolve the Swarm Intelligence Protocol (OHC-SIP) by publishing intent broadcasts into Virtual Rooms.

```mermaid
sequenceDiagram
    participant PM as Agent (PM)
    participant Mesh as Teammate Mesh
    participant SWE as Agent (SWE)

    PM->>Mesh: POST /api/mesh/v2/broadcast
    Mesh->>SWE: WebSocket Push Event
    SWE->>SWE: Process Meeting Intent
```

## 3. AutoDream Vector Consolidation

The AutoDream pipeline condenses short-term memory fragments into long-term pgvector embeddings.

```mermaid
sequenceDiagram
    participant Worker as Agent (Worker)
    participant FS as Local Filesystem
    participant AutoDream as AutoDream API
    participant LLM as Embedding Model
    participant DB as pgvector

    Worker->>FS: Writes Session Context to OHC_MEMORY_DIR
    AutoDream->>FS: Polling/Manual Sync Trigger
    AutoDream->>LLM: Pass text to Embedding Model
    LLM-->>AutoDream: Return Embedding
    AutoDream->>DB: Upsert Vector to pgvector
    AutoDream-->>Worker: Broadcast Consolidation Success
```

## 4. Hybrid Health Probes

Health probes ensure system stability across cloud and standalone operating modes.

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

</div>
