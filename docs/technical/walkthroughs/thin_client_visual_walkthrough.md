<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Thin Client Architecture Visual Walkthrough

Welcome to the visual walkthrough of the OHC Thin Client Architecture. This guide illustrates how the UI-only Thin Client (Mobile/Desktop) connects securely to the Cloud-Native backend via API and OAuth.

## 1. Zero Trust Connection Flow

The Thin Client initiates a secure connection to the Cloud Gateway, relying entirely on remote API endpoints and SPIFFE/SPIRE for identity validation.

```mermaid
graph TD
    UI[Thin Client UI] -->|OAuth / SPIFFE| Gateway[OHC Cloud Gateway]
    Gateway --> Auth{Identity Provider}
    Auth -->|Valid SVID| Orchestrator[KAIROS Hub]
    Auth -->|Invalid| 401[401 Unauthorized]
    Orchestrator --> Redis[(Teammate Mesh)]
    Orchestrator --> Swarm[Agent Swarm]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class UI,Gateway,Auth,401,Orchestrator,Redis,Swarm premium;
```

## 2. Real-Time Telemetry Streaming

The Thin Client connects to the Teammate Mesh via Server-Sent Events (SSE) or WebSockets to display live agent actions with zero-latency.

```mermaid
sequenceDiagram
    participant Client as Thin Client
    participant Mesh as Teammate Mesh (Redis/Centrifugo)
    participant Agent as Swarm Agent

    Client->>Mesh: Subscribe to `mesh:tasks` (SSE/WS)
    Agent->>Mesh: Publish Status Update
    Mesh-->>Client: Stream Event Payload
    Client->>Client: Update Dashboard UI

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
```

</div>
