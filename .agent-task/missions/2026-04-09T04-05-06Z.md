---
status: PENDING
Title: "KAIROS Orchestration: Shared Task List, Teammate Mesh, and AutoDream Architectural Consolidation"
Priority: P0
Estimated Scope: Large
---

# Title: KAIROS Orchestration: Shared Task List, Teammate Mesh, and AutoDream Architectural Consolidation

## Problem Statement
The OHC Hybrid Agentic OS requires an autonomous, resilient backbone to seamlessly decompose massive human goals into isolated, parallel agentic workflows. "KAIROS Orchestration" is this unified architecture, driving "Shared Task Lists", "Teammate Mesh", "Sub-Agent Queues", "Distributed State Machines", and "AutoDream" pipelines across both Kubernetes/PostgreSQL clouds and local SQLite standalone footprints. We lack a robust distributed state machine to track asynchronous tasks across the swarm with exact sequence and DAG dependencies. Agents cannot effectively orchestrate complex, multi-step workflows. We also lack a real-time Teammate Mesh to support inter-agent communication, and the AutoDream vector data pipeline for long-term memory consolidation.

## Research Report
Based on `docs/KAIROS_AI_OS_ARCHITECTURE.md` and `docs/KAIROS_ORCHESTRATOR_DESIGN.md`, the platform utilizes a "Hybrid Architecture" (OHC-HA).
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

## Design Doc
**1. Shared Task List (`shared_tasks` and `task_dependencies`)**
```sql
CREATE TABLE shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    locked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE task_dependencies (
    task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
```

**2. AutoDream (`autodream_memories`)**
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

**3. Teammate Mesh (Redis/Memory)**
- Event Bus Channels: `mesh:tasks`, `mesh:presence`
- Implement `MeshTransport` interface with `RedisMeshTransport` and `MemoryMeshTransport`.

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the backend database designs and sequence diagrams for the KAIROS Orchestration feature.

1.  **Phase 1 (UltraPlan/Decomposition):**
    - Create the SQL migration file for `shared_tasks` and `task_dependencies` in `srcs/server/db/migrations/`. Name it appropriately (e.g., `015_shared_tasks.sql`).
    - Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
    - Create the data access layer in `srcs/server/orchestration/tasks_db.go`.
    - Implement a `ClaimTask` method using `SELECT * FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED` for Postgres and appropriate mutex/transaction locking for SQLite.
    - Create unit tests for `tasks_db.go`.

2.  **Phase 2 (Orchestration):**
    - Design the Realtime Teammate Mesh APIs.
    - Implement `MeshTransport` interface (`srcs/server/orchestration/hub.go`) with `RedisMeshTransport` and `MemoryMeshTransport`.
    - Define gRPC contracts in `srcs/proto/hub.proto` (e.g., `AdvertiseCapabilities`, `DiscoverAgents`, `StreamMeshEvents`).

3.  **Phase 3 (autoDream):**
    - Create the SQL migration file for `autodream_memories` in `srcs/server/db/migrations/` (e.g., `016_autodream.sql`).
    - Add the migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
    - Architect the data pipelines for OHC's long-term memory consolidation system (`srcs/server/orchestration/autodream/pipeline.go`).

4.  **Finalize:** Submit a PR. Remember: You are the Lead for your domain. DO NOT ask for approval.

## Visual Excellence Guidelines
Any frontend representation of the KAIROS Orchestration must apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
