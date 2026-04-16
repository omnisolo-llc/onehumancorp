<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS API Architecture Walkthrough

Welcome to the interactive walkthrough for the KAIROS Orchestration APIs Architecture.

## Teammate Mesh Architecture

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M premium;
```

## Distributed Shared Task List DAG

```mermaid
graph TD
    subgraph KAIROS Orchestrator
        T[(Shared Task DB)]
    end

    subgraph Worker Agents
        SWE[SWE Agent]
        QA[QA Agent]
    end

    T -.->|Claim| SWE
    SWE -.->|Complete| T
    T -.->|Unlock| QA
    QA -.->|Claim| T

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class T,SWE,QA premium;
```

</div>
