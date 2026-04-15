<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Master Design Doc: KAIROS AI OS Orchestration (Phase 4)
**Author:** Principal Product Architect & KAIROS Orchestrator
**Status:** Approved

## 1. Overview
The OHC Hybrid Agentic OS requires an autonomous, resilient backbone to seamlessly decompose massive human goals into isolated, parallel agentic workflows. **KAIROS Orchestration** is this unified architecture, driving "Shared Task Lists", "Teammate Mesh", and "AutoDream" pipelines across both Kubernetes/PostgreSQL clouds and local SQLite standalone footprints.

## 2. The KAIROS Triad
The absolute autonomy of the OHC Swarm rests on three pillars:

1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

## 3. Architecture Visualization

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List / DB)]
        AD[AutoDream Pipeline]
        V[(pgvector Memories)]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V premium;
```

## 4. Sequence Diagrams

### 4.1 Task Decomposition (Shared Task List)
```mermaid
sequenceDiagram
    participant KAIROS
    participant TaskDB as PostgreSQL (TaskDB)
    participant Implementer

    KAIROS->>TaskDB: INSERT INTO shared_tasks (status='PENDING', priority='P0')
    KAIROS->>TaskDB: INSERT INTO task_dependencies (task_id, depends_on)
    Note right of KAIROS: Task is now pending and waiting for its DAG dependencies.
    Implementer->>TaskDB: SELECT id FROM shared_tasks WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer: Return task row
    Implementer->>TaskDB: UPDATE shared_tasks SET status='IN_PROGRESS' WHERE id=?
    Implementer->>KAIROS: Publish TASK_CLAIMED event via Mesh
```

### 4.2 Teammate Mesh Sub-Agent Queuing
```mermaid
sequenceDiagram
    participant WorkerAgent
    participant CentrifugeMesh
    participant TargetAgent

    WorkerAgent->>CentrifugeMesh: POST /api/mesh/broadcast {topic: 'task.assigned', agent_id: 'Scribe'}
    CentrifugeMesh-->>TargetAgent: StreamMeshEvents(EventStreamRequest)
    Note right of TargetAgent: Processes incoming MeshEvent
```

### 4.3 Memory Consolidation (AutoDream)
```mermaid
sequenceDiagram
    participant WorkerAgent
    participant LocalMemory as OHC_MEMORY_DIR/*.yml
    participant AutoDreamDaemon
    participant PgVector

    WorkerAgent->>LocalMemory: Write ephemeral logs
    AutoDreamDaemon->>LocalMemory: Poll and parse YAML files
    AutoDreamDaemon->>PgVector: Generate Embedding and Upsert to autodream_memories
```

## 5. Fallback Logic (Hybrid Mode)
- **Cloud-Native Mode:** Uses PostgreSQL `FOR UPDATE SKIP LOCKED` for task orchestration, `pgvector` for vector similarity searches, and Redis `rueidis` for high-throughput messaging.
- **Standalone Mode:** Uses SQLite application-level Mutex locking for task distribution, Go channels for in-memory Pub/Sub, and text-based or local file storage if `pgvector` is unsupported on the local host.

</div>
