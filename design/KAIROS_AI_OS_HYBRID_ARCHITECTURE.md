<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Orchestration: Master Design Document

## 1. Vision
The One Human Corp (OHC) AI OS is powered by the **KAIROS Orchestrator**, a distributed system designed to manage complex agent swarms with zero friction. KAIROS ensures that a single human can orchestrate vast AI teams by providing a unified, aesthetics-first interface for task decomposition, real-time coordination, and long-term memory consolidation.

## 2. Architectural Pillars

### I. Distributed State Machine (Shared Task List)
KAIROS utilizes a database-backed state machine to manage the lifecycle of `shared_tasks`.
- **Hybrid Locking:** PostgreSQL uses `FOR UPDATE SKIP LOCKED` for high-concurrency cloud environments. Standalone mode utilizes SQLite with Go-level Mutexes and explicit transactions to prevent TOCTOU vulnerabilities.
- **UltraPlan Integration:** Tasks move through specialized deliberation phases (`PROPOSE`, `CRITIQUE`, `REVISE`, `APPROVED`, `EXECUTE`) before being claimed by worker agents.
- **DAG Support:** Tasks can have multiple dependencies, forming a Directed Acyclic Graph.

**Shared Task List Schema (PostgreSQL/SQLite):**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks_dag (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies_dag (
    task_id VARCHAR NOT NULL,
    depends_on_task_id VARCHAR NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);
```

### II. Teammate Mesh (Real-time Transport)
The Teammate Mesh provides low-latency communication across the swarm.
- **Unified API:** A single gateway (`POST /api/mesh/broadcast`) handles event routing.
- **OHC-SIP Compliance:** All messages MUST include `agent_id`, `action`, and `status` at the JSON root to ensure compatibility across different agent roles and versions.
- **Hybrid Transport:**
    - **Cloud:** Powered by Redis Pub/Sub connected to Centrifuge hubs for WebSocket propagation to thin clients and sub-agents.
    - **Standalone:** Powered by a sharded in-memory Go transport (`MemoryMeshTransport`) for maximum host-machine efficiency.

### III. autoDream Pipeline (Omni-Context Memory)
The autoDream system consolidates episodic agent memory into a durable vector store.
- **Continuous Sync:** Local SQLite vector embeddings are automatically synced to Cloud pgvector instances. Intermediate worker outputs and session logs are compressed by a background pipeline.
- **Durable Storage:** The pipeline utilizes embeddings, storing them directly in a `pgvector` enabled table (`autodream_memories`) for exact semantic match recall.
- **Observability:** Every consolidation event is recorded via OpenTelemetry, providing "Full-Spectrum Observability" into the swarm's intelligence growth.

### IV. Sub-Agent Orchestration Queue
- **Queue Worker:** Implement a worker that polls `sub_agent_jobs` using `SKIP LOCKED` (Postgres) and Mutexes (SQLite).
- **Isolation:** For Standalone mode, use goroutines with OS-level resource limits. For Cloud, use a sidecar pattern or K8s Jobs.

## 3. Sequence Diagram for Shared Task List
```mermaid
sequenceDiagram
    participant Human
    participant KAIROS
    participant Database as Shared Task List (PG/SQLite)
    participant SubAgentQueue
    participant Worker

    Human->>KAIROS: Request high-level feature
    KAIROS->>Database: Decompose into Shared Tasks (DAG)
    KAIROS->>Database: Insert Tasks (State: PENDING)

    loop Queue Polling
        SubAgentQueue->>Database: Query PENDING tasks (no BLOCKED dependencies)
        SubAgentQueue->>Database: SELECT FOR UPDATE SKIP LOCKED
        Database-->>SubAgentQueue: Lock Acquired
        SubAgentQueue->>Worker: Dispatch Task
        Worker->>Worker: Execute Implementation
        Worker->>Database: UPDATE Task (State: COMPLETED)
        Worker->>Teammate Mesh: Broadcast TaskCompleted
    end
```

---
*Authored by: Principal Product Architect & KAIROS Orchestrator (L7)*
*Identity: One Human Corp*

</div>
