<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Orchestration: Master Design Doc (Hybrid AI OS)

**Author:** Principal Product Architect & KAIROS Orchestrator (L7)
**Version:** 4.0.0
**Status:** Approved

## 1. Executive Summary
KAIROS is the orchestration engine that powers the One Human Corp (OHC) Swarm. It bridges the gap between Cloud-Native Kubernetes clusters and Standalone Desktop deployments through a unified, hybrid architecture.

## 2. The KAIROS Triad

### 2.1 Shared Task List
- **Hybrid Concurrency:** Utilizes PostgreSQL `FOR UPDATE SKIP LOCKED` for cloud and application-level mutexed SQLite for standalone mode.
- **DAG Dependencies:** Enforces strict execution order using the `dependencies` JSONB array.

### 2.2 Teammate Mesh
- **Realtime Pub/Sub:** Powered by `CentrifugeNode` and Redis Pub/Sub (`rueidis`).
- **Resilient Transport:** Seamlessly switches to `MemoryMeshTransport` (Standalone).

### 2.3 AutoDream
- **Memory Consolidation:** Processes ephemeral `.agent-task/memory/*.yml` session data into vectors.
- **Vector Search:** Uses `pgvector` for exact Nearest Neighbor search in Cloud mode, with graceful SQLite degradation.

## 3. High-Level Architecture

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List)]
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
</div>
