# KAIROS AI OS Architecture

<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>

This document details the OHC Hybrid Agentic OS architecture, designed to orchestrate a vast swarm of AI agents.

## Phase 1: Shared Task List (The Brain)
A durable, distributed state machine in PostgreSQL or SQLite to track the Shared Task List.

## Phase 2: Realtime Teammate Mesh (The Nerves)
A low-latency communication layer using WebSockets/gRPC with a Redis backplane.

## Phase 3: AutoDream Data Pipelines (The Memory)
A pipeline for long-term memory consolidation using pgvector or similar embeddings.

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
