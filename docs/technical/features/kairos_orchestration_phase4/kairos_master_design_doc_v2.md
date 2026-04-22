<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; color: #fff;">

# Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

## 1. Overview
This document details the phase 4 master design for the OHC KAIROS AI OS Orchestration layer, enabling an autonomous swarm through a hybrid cloud/desktop architecture.

## 2. The KAIROS Triad

### 2.1 Shared Task List (The Brain)
A durable, distributed state machine and task queue.
- **Hybrid Concurrency:** Utilizes PostgreSQL `FOR UPDATE SKIP LOCKED` for cloud scalability and application-level mutexed SQLite for standalone mode.
- **DAG Dependencies:** Enforces strict execution order.

### 2.2 Teammate Mesh (The Nerves)
A highly available, low-latency communication layer.
- **Realtime Pub/Sub:** Powered by `CentrifugeNode` and Redis Pub/Sub (`rueidis`).
- **Resilient Transport:** Seamlessly switches between `RedisMeshTransport` (Cloud) and `MemoryMeshTransport` (Standalone).

### 2.3 AutoDream (The Memory)
The long-term persistence and semantic memory layer.
- **Vector Search:** Uses `pgvector` for exact Nearest Neighbor search in Cloud mode, with graceful SQLite degradation. Embedded memory consolidation.

## 3. High-Level Architecture

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
        Q[Sub-Agent Queue]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V

    A1 -->|Delegate| Q
    Q -->|Process| A2

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V,Q premium;
```

</div>
