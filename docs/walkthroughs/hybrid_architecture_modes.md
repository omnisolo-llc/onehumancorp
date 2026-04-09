<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Visual Walkthrough: Hybrid Architecture Modes

The One Human Corp (OHC) Agentic OS uses a flexible Hybrid Architecture (OHC-HA) designed to adapt to your deployment needs. This walkthrough covers the three primary modes of operation.

## 1. Cloud-Native Mode

**Best for:** Large-scale enterprise deployments, multi-tenant environments, and high-concurrency tasks.

In Cloud-Native mode, OHC leverages Kubernetes for orchestration and scalable services like PostgreSQL and Redis. The Swarm can infinitely scale horizontally.

```mermaid
graph TD
    CEO[Human CEO] --> Ingress[K8s Ingress]
    Ingress --> Hub[Orchestration Hub]
    Hub --> DB[(PostgreSQL)]
    Hub --> Cache[(Redis Pub/Sub)]
    Hub --> Agents[AI Swarm]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CEO,Ingress,Hub,DB,Cache,Agents premium;
```

## 2. Standalone Desktop Mode

**Best for:** Local development, strict privacy, and offline capable execution.

Standalone mode is designed to run on a single host machine with minimal resource overhead. It uses local SQLite instead of Postgres and in-memory structures instead of Redis. It syncs state to the cloud when online.

```mermaid
graph TD
    CEO[Human CEO] --> DesktopApp[Standalone Desktop App]
    DesktopApp --> LocalHub[Local Orchestrator]
    LocalHub --> LocalDB[(SQLite)]
    LocalHub --> LocalAgents[Local AI Swarm]
    LocalDB -.->|Sync| CloudDB[(Cloud PostgreSQL)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CEO,DesktopApp,LocalHub,LocalDB,LocalAgents,CloudDB premium;
```

## 3. Thin Client Mode

**Best for:** Mobile users or light desktop users connecting to a powerful remote server.

Thin Client mode runs purely the presentation layer locally. All execution and heavy lifting happens remotely via API/OAuth connections.

```mermaid
graph TD
    CEO[Human CEO] --> UI[Thin Client UI]
    UI -->|API Calls / OAuth| RemoteGateway[Remote Cloud Gateway]
    RemoteGateway --> RemoteHub[Remote Orchestrator]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CEO,UI,RemoteGateway,RemoteHub premium;
```

</div>
