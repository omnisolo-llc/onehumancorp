---
status: "PENDING"
Title: "Master Design Doc: KAIROS Unified Orchestration Architecture"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The OHC Swarm requires a centralized "KAIROS" Orchestration layer to break down complex objectives into actionable sub-tasks, coordinate agent activity via realtime pub/sub, and maintain state durability across execution environments. We need a unified architecture document that aligns the `Shared Task List`, `Teammate Mesh`, `AutoDream Pipeline`, and `Sub-Agent Queue` under a single schema supporting both Cloud-Native (multi-tenant Postgres/Redis) and Standalone (local SQLite) modes.

# Research Report
Based on current implementations (`srcs/server/orchestration/tasks_db.go` and `sub_agent.go`):
1. **Shared Task List:** Tasks are tracked in the database (`shared_tasks`). In PostgreSQL, concurrency is managed natively with `FOR UPDATE SKIP LOCKED`. In SQLite, this is gracefully degraded using an application-level mutex (`sync.Mutex` in `TaskOrchestrator`).
2. **Sub-Agent Orchestration:** Handled via the `SubAgentSpawner` interface. Cloud execution assumes distributed workers (e.g., K8s pods or Redis queues), while SQLite uses local goroutines throttled by a semaphore channel.
3. **Teammate Mesh (Realtime Comm):** `CentrifugeNode` handles the websocket distribution layer, publishing events (e.g., `task.assigned`, `SUB_AGENT_COMPLETED`) across instances using Redis Pub/Sub (`rueidis`) or memory transport.
4. **AutoDream Pipeline:** Consolidates ephemeral context (`.agent-task/memory/*.yml` and session data) into a long-term vector database (`autodream_memories`) using pgvector/Minimax embeddings, granting semantic recall.
5. **Observability:** Task transitions (PENDING -> RUNNING -> COMPLETED) must be heavily metered via OpenTelemetry float histograms (`ohc_agent_transition_latency_seconds`) to track execution vs wait times.

# Design Doc

## Architecture Visualization

```mermaid
graph TD
    subgraph Swarm Sub-Agents
        A1[Worker Goroutine 1]
        A2[Worker Goroutine 2]
    end

    subgraph Teammate Mesh (Centrifugo/Redis)
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List DB)]
        Q[SubAgentSpawner]
        AD[AutoDream Pipeline]
        V[(pgvector/Local Embeddings)]
    end

    A1 <-->|Pub/Sub Events| M
    A2 <-->|Pub/Sub Events| M

    A1 -->|ClaimTask| T
    A2 -->|ClaimTask| T

    Q -->|Spawn| A1
    Q -->|Spawn| A2

    T -.->|Completed Context| AD
    AD -->|Embed & Store| V

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V,Q premium;
```

## Implementation Prompt
This is a comprehensive Master Document meant to finalize architectural alignment. If you are an Implementer agent picking this up:
1. Ensure all `shared_tasks` DB operations and migrations support the full lifecycle (`PENDING`, `IN_PROGRESS`, `DELIBERATION`, `COMPLETED`, `FAILED`).
2. Confirm the `ohc_agent_transition_latency_seconds` OpenTelemetry integration is fully wired within `tasks_db.go` or `sip.go` during state mutations.
3. Verify the `SubAgentSpawner` in `sub_agent.go` appropriately manages the semaphore channels in Standalone mode to avoid exhausting local connections.

## Visual Excellence Mandate
Any OHC frontend client visualizing the state of the KAIROS Orchestrator must apply the following CSS variables for the glassmorphism aesthetic:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
