# KAIROS Orchestration: Unified Architecture

This document serves as the master design doc synthesizing the OHC Hybrid AI OS Orchestration layer. The KAIROS engine provides the structural and aesthetic vision for agentic coordination within One Human Corp.

## The KAIROS Triad
The absolute autonomy of the OHC Swarm rests on three core pillars:

1. **Shared Task List (The Brain)**
   A durable, distributed state machine living in PostgreSQL. It leverages row-level locks (`FOR UPDATE SKIP LOCKED`) to allow horizontal pod concurrency in the cloud, effectively preventing worker collisions when claiming tasks. It degrades gracefully to SQLite transactions for standalone desktop deployments.

2. **Teammate Mesh (The Nerves)**
   A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events. In standalone mode, this falls back to memory-based or local SQLite event routing.

3. **AutoDream Data Pipelines (The Memory)**
   The long-term persistence layer. Ephemeral session logs and intermediate `.agent-task/memory/*.yml` artifacts are processed asynchronously by the `AutoDreamWorker`. Content is compressed via LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

## Architecture Visualization

<div markdown="1">
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
</div>

## Aesthetic Core Mandate

This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Any downstream UI interpreting this architecture, dashboards exposing the mesh, or components presenting AutoDream memories MUST apply the OHC Premium Feel:

```css
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
```
