<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Sub-Agent Orchestration Queue
Design for routing, backoff, and timeouts for spawned worker agents across distributed deployments.

## Architecture Flow
```mermaid
graph TD
    subgraph KAIROS Orchestrator
        A[Task Manager] -->|Enqueue| Q{Sub-Agent Queue Interface}
    end

    Q -->|Cloud| Redis[(Redis ZSETs)]
    Q -->|Standalone| DB[(SQLite Mutexed Table)]

    Redis -->|Dequeue| W1[Worker Pod]
    DB -->|Dequeue| W2[Local Worker]

    W1 -->|Transition Event| M[Teammate Mesh / Centrifuge]
    W2 -->|Transition Event| M

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,Q,Redis,DB,W1,W2,M premium;
```
</div>
