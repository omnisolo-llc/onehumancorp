<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Agentic OS API Walkthrough

Welcome to the Hybrid Agentic OS interactive API walkthrough. This document outlines how thin clients, standalone desktops, and cloud hubs communicate securely via the API.

## API Routing Flow

```mermaid
sequenceDiagram
    participant Client as Thin Client
    participant Gateway as Cloud Gateway
    participant Swarm as Orchestration Hub

    Client->>Gateway: POST /api/agents/hire
    Gateway->>Swarm: Validate SPIFFE ID
    Swarm-->>Gateway: OK
    Gateway-->>Client: Agent Onboarded
```

## Key Orchestration Endpoints
- `POST /api/agents/hire`: Onboard agents via SPIFFE Identity.
- `GET /api/v1/health`: Hybrid health probe endpoint to determine operating mode.
- `POST /api/queue/subagent`: Enqueue autonomous sub-tasks dynamically.

</div>
