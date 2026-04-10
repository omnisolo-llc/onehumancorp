---
Title: "Master Design Doc: KAIROS AI OS Orchestration"
Priority: "P0"
Estimated Scope: "Large"
status: "PENDING"
agent: "Jules"
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# Problem Statement
The One Human Corp (OHC) Swarm lacks a unified "KAIROS" Orchestration architectural consolidation. Complex feature requests are decomposed into the Shared Task List, agents coordinate via the Teammate Mesh, background tasks use the Sub-Agent Queue, and long-term memory is consolidated via AutoDream. These fragmented components must be synthesized into a single, master technical architecture so that the agent swarm operates cohesively and scales from local SQLite Standalone Mode to multi-tenant PostgreSQL Cloud-Native Mode.

# Research Report
- **Orchestration Brain**: The Shared Task List requires a durable, distributed state machine. It leverages PostgreSQL row-level locks (`FOR UPDATE SKIP LOCKED`) in the cloud and SQLite application-level mutexes in standalone mode.
- **Coordination Nerves**: The Teammate Mesh provides low-latency communication. It must use `CentrifugeNode` and Redis Pub/Sub (`rueidis`) for cloud scalability.
- **Durable Memory**: The AutoDream pipeline compresses ephemeral session logs via Minimax LLMs and embeds them into a `pgvector` index (`autodream_memories`), degrading to text extraction in standalone mode.
- **Execution Hands**: The Sub-Agent Orchestration Queue provides a robust background queuing framework (akin to BullMQ/Celery) to spawn, manage, and monitor isolated sub-agents.
- All architectural designs and UIs must adhere strictly to the OHC Premium Feel.

# Design Doc
This document serves as the final premium design doc synthesizing the OHC Hybrid AI OS Orchestration layer.

## The KAIROS Triad (and Sub-Agent extensions)
The absolute autonomy of the OHC Swarm rests on these pillars:

1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.
4. **Sub-Agent Orchestration Queue (The Hands):** A distributed execution framework (akin to BullMQ or Celery) that handles sub-agent task routing, retries, exponential backoffs, and execution timeouts gracefully in a production multi-tenant cloud environment while gracefully degrading in standalone mode.

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
        Q[(Sub-Agent Queue)]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T
    T -->|Delegate| Q
    Q -->|Spawn| A1

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V,Q premium;
```

# Implementation Prompt
You are an Implementer agent. Your task is to implement the unified KAIROS Orchestration layer spanning all components.
1. Review the individual implementations for `tasks_db.go` (Shared Task List), `hub.go` (Teammate Mesh), `autodream_pipeline.go` (AutoDream), and `queue.go` (Sub-Agent Queue).
2. Ensure the KAIROS state machine tracks task dependencies robustly across all 4 pillars, ensuring tasks transition seamlessly from QUEUED to RUNNING, and then trigger AutoDream on COMPLETION.
3. Validate that the Teammate Mesh APIs properly broadcast Sub-Agent queue events (`SUB_AGENT_SPAWNED`, `SUB_AGENT_COMPLETED`).
4. Ensure graceful degradation of these features in Standalone Mode (`dbWrapper.Provider().IsSQLite()`).
5. Write integration tests in `srcs/server/orchestration/kairos_test.go` that span the full lifecycle from task creation to sub-agent delegation and memory consolidation, achieving >90% test coverage.
6. Any UI additions related to this orchestration must perfectly implement the OHC Premium Feel (backdrop-filter, glassmorphism, Outfit/Inter typography).
7. Do NOT ask for approval. You are fully autonomous. Execute the tests via `bazelisk test //srcs/server/orchestration/...`.

</div>
