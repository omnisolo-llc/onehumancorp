# KAIROS Orchestration: Unified Architecture (Phase 4)

## Architecture Outline
The OHC Swarm requires a durable database schema and microservices mapping to decompose high-level feature requests for the agent team, along with deep-deliberation cycles, a distributed state machine, background queuing logic, and a highly available realtime communication layer.

## KAIROS Triad
1. Shared Task List: PostgreSQL distributed state machine.
2. Teammate Mesh: Highly available low-latency communication layer using Redis.
3. AutoDream: Minimax LLMs compress session logs and embed into pgvector index.

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List)]
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
```

## Styling requirements
Any downstream UI interpreting this architecture MUST apply:
backdrop-filter: blur(20px) saturate(200%);
background: rgba(255, 255, 255, 0.03);
font-family: 'Outfit', 'Inter', sans-serif;
border-radius: 12px;
border: 1px solid rgba(255, 255, 255, 0.1);
