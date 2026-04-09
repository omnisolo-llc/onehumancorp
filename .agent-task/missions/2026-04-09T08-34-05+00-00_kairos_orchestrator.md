---
title: "KAIROS Orchestrator: Unified Architecture"
status: PENDING
agent: Implementer
priority: P0
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

## Problem Statement
The OHC Hybrid Architecture requires a unified orchestration layer to manage the Shared Task List, Teammate Mesh, and AutoDream pipelines.

## Research Report
- **Shared Task List:** Needs robust locking mechanisms. `FOR UPDATE SKIP LOCKED` for PostgreSQL and explicit transaction isolation for SQLite.
- **Teammate Mesh:** Requires a realtime communication layer via WebSockets or Redis Pub/Sub.
- **AutoDream:** Must persist ephemeral session logs and intermediate artifacts into a `pgvector` index for semantic search.

## Design Doc
- **Module Path:** `srcs/server/orchestration`
- **Architecture:**
  - `Shared Task List`: Implement database schema and microservices mapping for task tracking.
  - `Teammate Mesh APIs`: Design realtime communication APIs for agent coordination.
  - `AutoDream Data Pipelines`: Architect data pipelines for long-term memory consolidation using pgvector and LLM embeddings.

## Implementation Prompt
Hello Implementer agent!
1. Review the existing implementation in `srcs/server/orchestration/tasks.go` and ensure proper locking mechanisms are used (`FOR UPDATE SKIP LOCKED` for Postgres, concurrent read/write locks for SQLite).
2. Implement the Teammate Mesh APIs to facilitate realtime communication between agents.
3. Develop the AutoDream data pipelines to aggregate and embed session logs into the `autodream_memories` table.

## Priority
P0

## Estimated Scope
Large
