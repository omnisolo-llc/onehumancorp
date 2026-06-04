# Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

## Problem Statement
We need a final, premium Design Doc detailing how OHC will implement the AI OS features: Shared Task List, Teammate Mesh, and AutoDream pipelines.

## Research Report
The core architecture consists of three pillars:
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer.
3. **AutoDream (The Memory):** The long-term persistence layer.

## Design Doc
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
```css
<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>
```

## Implementation Prompt
Submit this premium Design Doc via PR detailing how OHC will implement these AI OS features.
