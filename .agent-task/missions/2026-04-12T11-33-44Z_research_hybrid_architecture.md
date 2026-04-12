---
status: PENDING
priority: P0
scope: Large
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# Title: OHC Competitive Architecture Research & Feature Disruption Synthesis

## Problem Statement
Rival Agentic Operating Systems (like Claude Code, OpenClaw, Replit Agent) are gaining traction, yet they lack comprehensive Hybrid Architectures that blend Cloud-Native Orchestration with true Standalone Edge/Local execution capabilities. One Human Corp (OHC) must benchmark these platforms to identify actionable disruption opportunities, particularly focusing on our local-private RAG capabilities that securely sync with scalable cloud backends. A gap exists in articulating precisely how OHC’s SQLite/Postgres hybridity decisively outperforms market defaults.

## Research Report

### Competitive Analysis Table

| Feature / Platform          | One Human Corp (OHC) | Claude Code | OpenClaw | Replit Agent |
|-----------------------------|----------------------|-------------|----------|--------------|
| **Standalone Mode**         | Native (SQLite)      | None        | Partial  | None         |
| **Cloud Scale**             | Kubernetes/Postgres  | High        | Low      | High         |
| **Teammate Mesh**           | Hybrid (Local/Cloud) | Basic       | None     | Cloud Only   |
| **Local RAG & Embeddings**  | Edge Vector DB sync  | Cloud Only  | Basic    | Cloud Only   |

### System Architecture Synthesis

```mermaid
graph TD;
    subgraph OHC Hybrid Edge
        Local[Local Agent Mesh] --> SQL[Local SQLite DB];
        SQL --> OfflineQ[Offline Mesh Queue];
    end
    subgraph OHC Cloud Native
        CloudMesh[Cloud Agent Mesh] --> PG[PostgreSQL / pgvector];
    end
    OfflineQ -->|AutoDream Sync| CloudMesh;

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Local,SQL,OfflineQ,CloudMesh,PG premium;
```

- **Market Default:** Competitors rely almost exclusively on managed cloud environments or fully isolated local tools with limited swarm potential.
- **OHC Advantage:** The OHC Hybrid Architecture (OHC-HA) seamlessly transitions between Kubernetes (PostgreSQL, Redis) and Desktop (SQLite) modes, giving users total data sovereignty without sacrificing Swarm Intelligence scalability.
- **Gap Identified:** The ability to execute sub-agent tasks completely offline while maintaining a local Vector DB representation, then probabilistically reconciling state via Teammate Mesh upon reconnection, is missing in rival solutions.

## Design Doc
1. **Offline State Reconciliation Mechanism:**
   - Extend the Teammate Mesh to support a queue-and-forward model for local execution.
   - Design a local edge Vector DB schema (based on pgvector parity) using SQLite for "AutoDream" offline mode.
2. **Hybrid Protocol Buffers:** Ensure API contracts gracefully handle missing `mesh:tasks` acknowledgments during intermittent connectivity.
3. **UI Enhancements:** Surface an offline indicator and "Sync pending" state in the frontend, adhering to the Visual Excellence Mandate (backdrop-filter 20px blur).

## Implementation Prompt
Implementer Agent:
1. Update `srcs/server/orchestration/sip.go` to intercept task complete events when Redis is unavailable.
2. Store pending swarm memory updates in a new SQLite table `offline_mesh_events`.
3. Add a background goroutine in Standalone Mode that polls `offline_mesh_events` and attempts to flush them to the Cloud API once connectivity is restored.
4. Ensure comprehensive test coverage (`bazelisk test //srcs/server/...`) for the synchronization logic.

## Priority
P0

## Estimated Scope
Large

</div>
