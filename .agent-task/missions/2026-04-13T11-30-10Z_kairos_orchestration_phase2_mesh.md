---
status: DONE
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Phase 2 - Architect Teammate Mesh APIs

## Problem Statement
For realtime agent coordination, the OHC Swarm requires a high-throughput, low-latency communication bus. Currently, agents lack standard APIs to negotiate states or update task progress interactively.

## Research Report
*   **Protocol**: WebSockets offer the required full-duplex communication.
*   **Scale**: In Cloud Mode, multiple instances of the backend service require a Pub/Sub backplane (Redis) to broadcast WebSocket messages across the cluster. In Standalone Mode, an in-memory channel event bus suffices.

## Design Doc
### Teammate Mesh Architecture
**API Endpoints**:
*   `GET /api/v1/mesh/ws`: WebSocket upgrader for agent connections.
*   `POST /api/v1/mesh/publish`: HTTP endpoint to publish a message to a specific topic (e.g., a specific task ID).

**Message Format (JSON)**:
```json
{
  "event_type": "TASK_UPDATED",
  "task_id": "uuid",
  "payload": {
    "status": "IN_PROGRESS",
    "agent_id": "uuid"
  }
}
```

**Backplane**:
*   `MeshPubSub` interface: `Publish(topic, message)`, `Subscribe(topic)`.
*   Implementations: `RedisPubSub` (Cloud) and `MemoryPubSub` (Standalone).

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Build the Realtime Teammate Mesh APIs and Pub/Sub backplane.
**Instructions**:
1. Define the `MeshPubSub` interface in `srcs/server/api/mesh/pubsub.go`.
2. Implement `RedisPubSub` (using `go-redis/redis`) and `MemoryPubSub`.
3. Create the WebSocket handler in `srcs/server/api/mesh/ws.go` using `gorilla/websocket`.
4. Create the HTTP publish handler in `srcs/server/api/mesh/publish.go`.
5. Ensure the WebSocket handler securely authenticates agents using SPIFFE/SPIRE context.
**Acceptance Criteria**:
*   WebSocket connections can successfully establish and receive broadcast messages.
*   Unit tests for both PubSub implementations pass.

## Priority
P0

## Estimated Scope
Large

</div>
