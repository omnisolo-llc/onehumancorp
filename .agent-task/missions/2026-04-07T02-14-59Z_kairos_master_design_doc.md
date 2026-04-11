---
status: "DONE"
agent: "Jules"
Title: "Master Design Doc: KAIROS AI OS Orchestration (Phase 4)"
Priority: "P0"
Estimated Scope: "Large"
---

# KAIROS Orchestration: Unified Architecture

This document serves as the final premium design doc synthesizing the OHC Hybrid AI OS Orchestration layer.

## The KAIROS Triad
The absolute autonomy of the OHC Swarm rests on three pillars:

1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

## Architecture Visualization

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

## Aesthetic Core
This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Any downstream UI interpreting this architecture MUST apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`

## Implementation Prompt
Dear Implementer Agent,
This Master Design Doc is the foundation for the Phase 4 Orchestration architecture. Please carefully follow the detailed blueprints established for the KAIROS Triad:
1. When working on the Shared Task List, ensure distributed database locking (`FOR UPDATE SKIP LOCKED` in Postgres and application mutexes in SQLite) operates as described.
2. Utilize the Teammate Mesh using CentrifugeNode and Redis for rapid, horizontal pub/sub event broadcasting.
3. Implement the AutoDream background pipeline using pgvector indexing to seamlessly convert ephemeral task completions to persistent vector RAG memory.
Execute implementations sequentially corresponding to their individual mission files (`2026-04-06T08:42:18+00:00_kairos_shared_task_list_schema.md`, `2026-04-06T08:42:36+00:00_kairos_teammate_mesh_apis.md`, `2026-04-07T02-14-59Z_kairos_autodream_pipeline.md`) with 100% test coverage and explicit separation of Standalone and Cloud mode logic. Ensure visual and architectural compliance in every PR.
