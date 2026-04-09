<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 2rem; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: inherit;">

# Interactive Walkthrough: OHC Hybrid Architecture (OHC-HA)

Welcome to the definitive visual guide for the **One Human Corp Hybrid Architecture**. The core innovation of the Agentic OS is its ability to seamlessly transition between infinite multi-tenant cloud scalability and absolute single-user local privacy.

## The Problem with the Status Quo
Most Agentic OS solutions force a binary choice:
- **Pure Local (e.g., Claude Code):** Excellent privacy, zero data exfiltration, but fundamentally limited by your laptop's CPU/RAM. You cannot scale a 1,000-agent swarm locally.
- **Pure Cloud (e.g., Replit Agent):** Infinite parallel compute, but mandates surrendering all corporate data sovereignty and intellectual property to a third-party cloud.

## The OHC Solution: Dynamic Escalation
OHC bridges this gap through the **Swarm Intelligence Protocol (OHC-SIP)**.

### 1. Standalone Desktop Mode (The Default)
By default, the human CEO runs the OHC Swarm on their local hardware.
- **Database:** Local SQLite (`standalone.db`).
- **Communication:** In-memory Go channels (no Redis required).
- **Security:** Private. The system works completely offline.
- **Use Case:** Ideation, drafting, and managing low-compute agent tasks.

### 2. Cloud-Native Mode (The Escalation)
When the CEO needs to deploy 500 agents to simultaneously audit a massive codebase or execute a complex market GTM strategy, the local machine cannot handle the load.
- **Database:** Multi-tenant PostgreSQL with row-level locking (`FOR UPDATE SKIP LOCKED`).
- **Communication:** Redis Pub/Sub (`rueidis`) and CentrifugeNode for highly concurrent mesh networking.
- **Security:** SPIFFE/SPIRE zero-trust identity and strict tenant isolation.

### The Bridge: Local-to-Cloud Sync
OHC handles the transition automatically. When "Cloud Escalation" is triggered, the local SQLite state (tasks, context, vector memory) is synchronized to the Cloud PostgreSQL instance.

<div style="background: rgba(0,0,0,0.3); padding: 1.5rem; border-radius: 8px; margin: 2rem 0;">

```mermaid
graph TD
    subgraph Standalone Desktop (Local Hardware)
        CEO[Human CEO]
        App[Flutter Desktop App]
        LocalDB[(SQLite db)]
        LocalSwarm[Local Micro-Swarm]

        CEO --> App
        App --> LocalDB
        LocalDB <--> LocalSwarm
    end

    subgraph The Bridge
        SyncEngine{OHC-SIP Sync Engine}
        LocalDB -.->|Context & Tasks| SyncEngine
        SyncEngine -.->|Aggregated State| CloudDB
    end

    subgraph Cloud-Native K8s (Massive Scale)
        CloudDB[(PostgreSQL / pgvector)]
        Mesh[Redis / Centrifuge Mesh]
        CloudSwarm[Massive Agent Swarm]

        CloudDB <--> Mesh
        Mesh <--> CloudSwarm
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CEO,App,LocalDB,LocalSwarm,SyncEngine,CloudDB,Mesh,CloudSwarm premium;
```

</div>

## Architectural Graceful Degradation
To achieve 100% code reuse, OHC relies on Go interfaces. When the OHC backend boots, it detects the environment (`OHC_STANDALONE=true` vs K8s).
- If Redis is missing, the Teammate Mesh gracefully degrades to using SQLite mutexes and in-memory events.
- If PostgreSQL `pgvector` is missing, the AutoDream pipeline degrades to text-based or local lightweight embedding caches.

This ensures the **exact same binary** powers both the localized desktop app and the massive Kubernetes clusters.

</div>
