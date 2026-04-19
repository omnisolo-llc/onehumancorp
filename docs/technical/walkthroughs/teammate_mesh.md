<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# Teammate Mesh Walkthrough

Welcome to the Teammate Mesh interactive walkthrough. The Teammate Mesh is the real-time communication spine of the One Human Corp (OHC) Hybrid Architecture, allowing agents to collaborate, deliberate, and execute tasks autonomously.

## 1. Mesh Transport Overview

The mesh relies on the `MeshTransport` interface, utilizing Redis Pub/Sub in Cloud-Native mode or a lightweight local bus in Standalone Desktop mode.

### Event Publishing and Subscribing

Agents use the `CentrifugeNode` to subscribe to relevant channels. To optimize bandwidth and reduce unmarshaling overhead, agents can use `SubscribeMeshEventsWithFilter` to apply a `MeshFilter` directly at the transport layer.

```mermaid
sequenceDiagram
    participant AgentA as Agent A (Implementer)
    participant Mesh as Teammate Mesh (Redis/Local)
    participant AgentB as Agent B (Scribe)

    AgentA->>Mesh: 1. Subscribe (Filter: Topic="docs")
    AgentB->>Mesh: 2. Broadcast (Topic="code", Payload=...)
    Mesh--xAgentA: 3. Ignored by Filter
    AgentB->>Mesh: 4. Broadcast (Topic="docs", Payload=...)
    Mesh-->>AgentA: 5. Event Delivered
    AgentA->>AgentA: 6. Process Event Payload
```

## 2. Advanced Mesh Filtering

By leveraging `MeshFilter`, you ensure that your agent only wakes up to process events strictly relevant to its mission profile.

```go
// Example Go pseudocode for agent mesh initialization
err := meshTransport.SubscribeMeshEventsWithFilter(
    ctx,
    "mesh:global",
    &MyMissionFilter{Role: "SCRIBE"},
    func(event *MeshEvent) {
        log.Println("Received relevant documentation event!")
    },
)
```

## 3. Best Practices

- **Non-blocking Callbacks:** When processing events inside subscription callbacks (especially during database cursor iteration), wrap long-running operations in goroutines to prevent blocking the transport layer.
- **Graceful Degradation:** Always assume the mesh might fallback to the SQLite-backed Standalone Mode. Avoid relying entirely on Redis-specific commands unless checking lock ownership via Lua scripts.
- **PII Scrubbing:** If broadcasting payloads containing sensitive data, pass the content through `telemetry.RedactPII(str)` before broadcasting.

</div>
