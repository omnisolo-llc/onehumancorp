<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Architecture Walkthrough

**Version:** 1.0.0
**Target Audience:** Architects & Orchestration Engineers

## 1. Introduction

The One Human Corp (OHC) Agentic OS utilizes a Hybrid Architecture to empower a human CEO. This walkthrough illustrates the distinction between our **Cloud-Native Mode** and **Standalone Mode**.

## 2. Cloud-Native vs Standalone Mode

Our dual-mode setup ensures that the system is powerful enough to orchestrate an entire company from the cloud, but lightweight enough to be run locally without massive overhead.

```mermaid
graph TD
    subgraph "Cloud-Native Mode (Multi-Tenant)"
        CloudAPI[Gateway & API] --> CloudHub[Orchestration Hub]
        CloudHub --> Postgres[(PostgreSQL)]
        CloudHub --> Redis[(Redis Pub/Sub)]
        CloudHub --> K8s[Kubernetes Cluster]
        K8s --> CloudAgents[Containerized Agents]
    end

    subgraph "Standalone Mode (Single-Tenant Desktop)"
        LocalAPI[Local API Process] --> LocalHub[Local Hub Engine]
        LocalHub --> SQLite[(SQLite DB)]
        LocalHub --> LocalEvents[In-Memory Events]
        LocalHub --> Desktop[Desktop Processes]
        Desktop --> LocalAgents[Local Agents]
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CloudAPI,CloudHub,Postgres,Redis,K8s,CloudAgents,LocalAPI,LocalHub,SQLite,LocalEvents,Desktop,LocalAgents premium;
```

### Key Differences

1. **State Persistence**:
   - *Cloud-Native*: Relies on **PostgreSQL** with pgvector for AutoDream embeddings, providing robust long-term consistency.
   - *Standalone*: Relies on **SQLite** with local memory, optimizing for lower resource footprint while sacrificing some advanced concurrent search capabilities.
2. **Coordination Engine**:
   - *Cloud-Native*: Uses **Redis** for the Teammate Mesh, providing sub-millisecond Pub/Sub for complex agent workflows.
   - *Standalone*: Uses **In-Memory Go Channels** and SQLite semaphores for communication, allowing offline execution.

</div>
