<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 20px; font-family: 'Outfit', 'Inter', sans-serif;">

# Teammate Mesh Orchestration: Visual Walkthrough

This document provides a step-by-step visual walkthrough of how real-time communication flows between agents in the OHC Hybrid Agentic OS via the Teammate Mesh.

## Teammate Mesh Architecture

The Teammate Mesh facilitates real-time coordination using a Mailbox Protocol over event bus channels (`mesh:tasks` and `mesh:presence`). It uses Redis Pub/Sub in Cloud Mode and local channel/file locks in Standalone Mode.

```mermaid
sequenceDiagram
    participant Agent A
    participant Agent B
    participant Mesh (mesh:tasks / mesh:presence)

    Note over Agent A, Agent B: Agents boot up and announce presence
    Agent A->>Mesh: Publish(mesh:presence, {id: "A", status: "online"})
    Agent B->>Mesh: Publish(mesh:presence, {id: "B", status: "online"})
    Mesh-->>Agent A: Receive(mesh:presence, {id: "B", status: "online"})
    Mesh-->>Agent B: Receive(mesh:presence, {id: "A", status: "online"})

    Note over Agent A, Agent B: Agent A needs assistance from Agent B
    Agent A->>Mesh: Publish(mesh:tasks, {type: "REQUEST", from: "A", to: "B", payload: "Analyze data"})
    Mesh-->>Agent B: Receive(mesh:tasks, {type: "REQUEST", from: "A", to: "B", payload: "Analyze data"})

    Note over Agent B: Agent B processes the request
    Agent B->>Mesh: Publish(mesh:tasks, {type: "RESPONSE", from: "B", to: "A", payload: "Analysis complete"})
    Mesh-->>Agent A: Receive(mesh:tasks, {type: "RESPONSE", from: "B", to: "A", payload: "Analysis complete"})
```

</div>
