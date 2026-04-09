# KAIROS AI OS Orchestration: Unified Architecture Master Design

This Master Design Document synthesizes the complete OHC Hybrid AI OS Orchestration layer (KAIROS), fulfilling Phase 4 of the orchestration roadmap.

## 1. The KAIROS Foundation (Hybrid Architecture)

KAIROS acts as the central brain orchestrating the OHC Swarm. It operates strictly under the OHC Hybrid Architecture (OHC-HA):
- **Cloud-Native Mode**: Leverages Kubernetes, PostgreSQL (for durable state & pgvector embeddings), and Redis Pub/Sub (`rueidis` via Centrifuge) to scale multi-tenant operations securely. Uses `FOR UPDATE SKIP LOCKED` for task assignment concurrency.
- **Standalone Desktop Mode**: Degrades gracefully to single-user operation. Uses local SQLite with explicit application-level semaphores and concurrent write locks. Local memory pipelines rely on fallback local LLM embeddings.

## 2. Core Pillars of the Swarm

### 2.1 Shared Task List
A distributed state machine (`shared_tasks`) that stores the directed acyclic graph (DAG) of the Swarm's intentions.
- **Schema**: Stores `task_id`, `dependencies` (JSONB), and `status`.
- **Concurrency**: Agent workers poll tasks sequentially, bounded by the execution mode (Cloud locks vs. SQLite local locks).

### 2.2 Teammate Mesh
The realtime nervous system, powered by `CentrifugeNode`.
- **Channels**: Broadcasts state changes (`mesh:tasks`), presence data, and lifecycle events (`SUB_AGENT_SPAWNED`).
- **Transport**: `RedisMeshTransport` (Cloud) degrades to `MemoryMeshTransport` (Standalone).

### 2.3 AutoDream Pipeline
The long-term memory consolidation mechanism mapping to `autodream_memories`.
- **Flow**: Asynchronously reads `.agent-task/memory/*.yml` and session state.
- **Compute**: Batches and compresses text using `cached_minimax_client`, embedding truths into `pgvector` for semantic querying.

### 2.4 Sub-Agent Queuing
For Task Decomposition (KAIROS Mode), tasks with priority `DELEGATED` spawn isolated `SubAgentSpawner` contexts.
- Enforces strict parent-child task lifecycles via `parent_task_id`.
- Records transient state to `.agent-task/status/{task_id}.yml` to prevent orphan zombies.

### 2.5 UltraPlan Deliberation
A distributed state machine layer tracking massive architectural refactors, voting, and peer review via `statemachine.Transition` before finalizing execution steps into the Shared Task List.

## 3. Observability & Telemetry

Full-spectrum observability is critical:
- **Metrics**: Endpoints capture high-fidelity Prometheus vectors such as `ohc_autodream_sync_duration_seconds` and `ohc_mesh_broadcast_total`.
- **Dimensions**: All metrics explicitly record the `deployment_mode` label (`cloud` or `standalone`).

## 4. Visual Excellence Mandate
All KAIROS UI dashboards interpreting this orchestration state must strictly adhere to the OHC Premium Feel aesthetic.

```html
<style>
body, .kairos-dashboard {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
  color: #ffffff;
  border: 1px solid rgba(255, 255, 255, 0.08);
}
</style>
```

## 5. Sequence Diagram

```mermaid
graph TD
    subgraph Swarm Agents
        A1[Planner Agent]
        A2[Implementer Agent]
    end

    subgraph KAIROS Orchestrator
        TL[(Shared Task List)]
        Mesh[Teammate Mesh / Centrifuge]
        Queue[Sub-Agent Queue]
        AD[AutoDream Pipeline]
        Mem[(pgvector Memories)]
    end

    A1 -->|Decompose & Enqueue| TL
    A1 -->|Publish| Mesh
    TL -->|Route DELEGATED| Queue
    Queue -->|Spawn| A2
    A2 -->|Update State| Mesh
    A2 -->|Write Context| AD
    AD -->|Embed| Mem

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,TL,Mesh,Queue,AD,Mem premium;
```
