---
status: "DONE"
agent: "Jules"
Title: "Master Design Doc: KAIROS AI OS Orchestration (Phase 4)"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The OHC swarm requires a unified orchestrator design that brings together the three core pillars of the AI OS (Shared Task List, Teammate Mesh, AutoDream). Previous phases drafted these systems independently; they must now be synthesized into a cohesive, highly-concurrent architecture that respects both Cloud-Native and Standalone execution modes.

# Research Report
- Based on `CLAUDE_OHC.md` and `README.md`, OHC operates in a "Hybrid Architecture" (`OHC-HA`).
- Phase 1 defined the Shared Task List using PostgreSQL `FOR UPDATE SKIP LOCKED` for Cloud-Native mode and SQLite mutexes for Standalone mode.
- Phase 2 defined the Teammate Mesh using `CentrifugeNode` and Redis Pub/Sub (`rueidis`) for Realtime state broadcasting.
- Phase 3 defined the AutoDream pipeline using `pgvector` for exact semantic search of consolidated agent memory.
- The KAIROS Triad must be brought together into a single master orchestration flow.

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

# Implementation Prompt
You are an Implementer agent. Your mission is to verify the architectural cohesion of the KAIROS Triad.
1. Ensure that the database migrations for `shared_tasks`, `autodream_memories`, and `state_machine_transitions` exist and degrade gracefully in SQLite mode.
2. Verify that the `TaskManager` correctly delegates sub-tasks to the `queue.Job` worker queue, and publishes mesh events.
3. Verify that `AutoDreamWorker` consolidates epoch memory properly.
4. Ensure full-spectrum observability via `telemetry.Record...` calls.

# Visual Excellence Guidelines
Any downstream UI interpreting this architecture MUST apply the following aesthetic core:
```css
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
```
