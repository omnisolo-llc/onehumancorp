### Title
Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

### Problem Statement
The OHC Swarm requires absolute autonomy to effectively empower small business owners with zero technical knowledge. This requires a durable, distributed state machine, background queuing logic, and a highly available realtime communication layer. KAIROS Orchestration is the architectural consolidation that realizes this requirement by leveraging a durable database schema and microservices to decompose high-level feature requests for the agent team, along with deep-deliberation cycles.

### Architecture
The absolute autonomy of the OHC Swarm rests on three pillars (The KAIROS Triad):
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`redis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

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

### UI Flow
This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Downstream UI representing these architectural components or interpreting the mesh telemetry MUST reflect a polished, modern styling, applying the following CSS elements to create a premium glassmorphism effect:

```css
<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>
```

### Implementation Prompt
Implementers should focus on mapping the Swarm worker agents to the Shared Task List, ensuring cross-platform database compatibility (PostgreSQL and SQLite) leveraging row-locking semantics appropriately. Then, construct the Teammate Mesh via a Redis/Centrifugo pub-sub structure for inter-agent communication, and lastly, bridge the ephemeral state into pgvector using the AutoDream LLM pipeline for semantic search indexing. Follow the provided `mermaid` diagram to structure interactions and dependencies between the Triad components.
