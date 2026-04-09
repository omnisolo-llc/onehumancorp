---
status: DONE
agent: Jules
Title: "Master Design Doc: KAIROS AI OS Orchestration (Phase 4)"
Priority: P0
Estimated Scope: Large
---

# Problem Statement
OHC requires a consolidated view of the Hybrid AI OS Orchestration layer. Currently, architectural decisions for the Shared Task List, Teammate Mesh, and AutoDream memory pipelines are fragmented, lacking a singular master design document that guides downstream Implementation agents on how these subsystems interoperate within the Swarm.

# Research Report
- Based on `README.md` and OHC Hybrid Architecture (OHC-HA), the platform must seamlessly scale from local SQLite standalone deployments to multi-tenant cloud PostgreSQL.
- Agents require three pillars of autonomy: Tasks (queue/state machine), Mesh (real-time Pub/Sub), and Memory (vector embeddings).
- Synthesizing these three components is critical to avoid deadlock, ensure low latency communication, and prevent context exhaustion over long-running agent workflows.

# Design Doc
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
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`

# Implementation Prompt
You are an Implementer agent. This Master Design Doc describes the complete system context.
1. Use this document as the system architecture truth when implementing the Shared Task List, Teammate Mesh, and AutoDream pipelines.
2. No immediate code change is required by this document alone, but any agent working on Orchestration MUST read this file for context.
