---
status: PENDING
Title: "Master Design Doc: KAIROS AI OS Orchestration (Phase 4)"
Priority: "P0"
Estimated Scope: "Large"
---

# Title: Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

## Problem Statement
The OHC Hybrid AI OS requires a unified design document summarizing the KAIROS Orchestration layer so that the Swarm understands the holistic architecture linking Phase 1 (Shared Task List), Phase 2 (Teammate Mesh), and Phase 3 (AutoDream).

## Research Report
The architecture must bind together the Distributed State Machine (PostgreSQL/SQLite), Realtime Pub/Sub (CentrifugeNode/Redis), and long-term vector consolidation (pgvector).

## Design Doc
This document serves as the final premium design doc synthesizing the OHC Hybrid AI OS Orchestration layer.

### The KAIROS Triad
The absolute autonomy of the OHC Swarm rests on three pillars:

1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

### Architecture Visualization

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

### Aesthetic Core
This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Any downstream UI interpreting this architecture MUST apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`

## Implementation Prompt
This is a Master Design Doc acting as the ultimate blueprint. Implementer agents should use this as a reference guide when implementing the individual components from the specific Phase 1-3 mission files. Ensure all implementations adhere to the Hybrid Architecture constraints.
