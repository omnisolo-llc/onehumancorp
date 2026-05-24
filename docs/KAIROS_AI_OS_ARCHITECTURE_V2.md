<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# KAIROS AI OS Architecture V2 (Phase 4 Master Premium Design)

## Overview
The KAIROS Orchestration engine is the heart of the OHC "Hybrid Agentic OS". It synthesizes three core pillars into a highly resilient and durable foundation for agentic teamwork:
1. **Shared Task List:** A distributed state machine tracking agent task transitions to prevent deadlocks and ensure reliability.
2. **Teammate Mesh:** A low-latency, highly available communication layer for real-time pub/sub inter-agent collaboration.
3. **AutoDream Pipeline:** An intelligent memory consolidation engine that compresses session logs and embeds them into a vector index for long-term semantic truth.

---

## KAIROS Triad: Component Synthesis

### 1. Shared Task List (Distributed State Machine)
The Shared Task List operates as the core truth mechanism for tasks. It guarantees exactly-once execution of complex agentic tasks across the worker swarm. When operating in Cloud-native mode, it uses PostgreSQL as the consistency boundary.

### 2. Teammate Mesh (Inter-Agent Pub/Sub)
The Teammate Mesh provides real-time awareness and collaborative message passing. Workers subscribe to relevant channels to coordinate multi-agent tasks, share intermediate context, and broadcast state changes seamlessly.

### 3. AutoDream Pipeline (Episodic Memory to Long-Term Vector Truth)
AutoDream continuously monitors swarm execution logs and completions. It leverages Minimax LLMs to summarize episodic memories, extract semantic meaning, and embed these insights into a pgvector index, creating a durable knowledge graph.

---

## Sequence Diagrams

### Task Decomposition
When a complex mission is submitted, the Orchestrator breaks it down and delegates it across the swarm using the Shared Task List.

```mermaid
sequenceDiagram
    participant User
    participant Orchestrator
    participant SharedTaskList
    participant WorkerAgent

    User->>Orchestrator: Submit High-Level Mission
    Orchestrator->>Orchestrator: Analyze & Decompose Mission
    Orchestrator->>SharedTaskList: Persist Sub-tasks (State: PENDING)
    SharedTaskList-->>Orchestrator: Acknowledge Persistence
    WorkerAgent->>SharedTaskList: Poll/Claim Available Task
    SharedTaskList-->>WorkerAgent: Assign Task (State: IN_PROGRESS)
    WorkerAgent->>WorkerAgent: Execute Task Logic
    WorkerAgent->>SharedTaskList: Update Task (State: COMPLETED)
```

### Sub-Agent Queuing
Sub-agents are dynamically provisioned in the background to handle asynchronous, scoped workloads securely.

```mermaid
sequenceDiagram
    participant PrimaryAgent
    participant QueueManager
    participant MeshHub
    participant SubAgent

    PrimaryAgent->>QueueManager: Enqueue Sub-task Workload
    QueueManager->>QueueManager: Buffer & Prioritize
    QueueManager->>MeshHub: Broadcast Queue Event
    MeshHub-->>SubAgent: Notify Available Workload
    SubAgent->>QueueManager: Dequeue Sub-task
    SubAgent->>SubAgent: Process Workload (Isolated Context)
    SubAgent->>MeshHub: Publish Completion Event
    MeshHub-->>PrimaryAgent: Receive Sub-task Result
```

### Memory Consolidation (AutoDream)
Background processes condense raw session logs into searchable vector embeddings.

```mermaid
sequenceDiagram
    participant SharedTaskList
    participant AutoDream
    participant MinimaxLLM
    participant VectorDB

    SharedTaskList->>AutoDream: Trigger: Task Completed (Session Logs)
    AutoDream->>MinimaxLLM: Send Logs for Compression & Extraction
    MinimaxLLM-->>AutoDream: Return Semantic Summary & Entities
    AutoDream->>MinimaxLLM: Request Vector Embeddings
    MinimaxLLM-->>AutoDream: Return Embeddings
    AutoDream->>VectorDB: Upsert into pgvector Index
    VectorDB-->>AutoDream: Confirm Persistence
```

---

## Hybrid Mode Fallback Logic: Cloud vs. Standalone

OHC provides seamless cross-mode deployment between Cloud-native and Standalone Desktop environments.

### Cloud-Native Mode
- **Database:** PostgreSQL acts as the central consistency boundary and distributed state machine for the Shared Task List.
- **Mesh:** Redis provides the highly available Pub/Sub backplane for the Teammate Mesh.
- **Memory:** `pgvector` extension in PostgreSQL handles the semantic vector truth.
- **Scaling:** Stateless API pods scale horizontally, relying on the durable PostgreSQL cluster.

### Standalone Desktop Mode (Fallback)
- **Database:** Replaces PostgreSQL with a local **SQLite-backed SIPDB**. The distributed state machine degrades gracefully to local, single-node transactional locks.
- **Mesh:** Redis is disabled. The Teammate Mesh falls back to an in-memory channel multiplexer running within the local Rust backend.
- **Memory:** Vector operations either utilize local embedding fallbacks or bypass `pgvector` in favor of lightweight, on-disk similarity search optimizations compatible with SQLite.
- **Scaling:** Bounded to the local machine's resources, optimized for minimal footprint and completely offline capability.

</div>
