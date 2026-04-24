# [Architect] Architect Realtime Teammate Mesh APIs for KAIROS

## Problem Statement
Agents require sub-millisecond coordination to avoid stepping on each other's toes. Polling the database is too slow and resource-intensive for immediate state changes. The swarm needs a low-latency Teammate Mesh for communication to decouple state changes from long polling.

## Research Report
Polling the database is too slow and resource-intensive for immediate state changes. Redis Pub/Sub provides an ideal lightweight transport for our Teammate Mesh, enabling push-based state propagation across the entire swarm. In standalone mode, an in-memory event bus is sufficient.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**Teammate Mesh APIs**

The Teammate Mesh ensures agents coordinate without delays. It acts as a realtime pub/sub system for task events.

**Endpoints**
- `POST /api/mesh/v2/broadcast`
  Broadcasts a state machine event over structured channels.

**Payload Example**
```json
{
  "channel": "mesh:tasks",
  "event_type": "TASK_TRANSITION",
  "data": {
    "task_id": "task_12345",
    "previous_state": "PENDING",
    "new_state": "IN_PROGRESS"
  }
}
```

**Transport Degradation Matrix**
- **Cloud-Native Mode**: Redis Pub/Sub (Centrifuge WebSocket hubs)
- **Standalone Desktop Mode**: In-Memory Go channel broadcast

</div>

## Implementation Prompt
Architect and implement the Realtime Teammate Mesh APIs in `src/server/orchestration/mesh/`. Expose a `POST /api/mesh/v2/broadcast` endpoint for publishing events and establish the appropriate websocket infrastructure for subscriptions. Create a hybrid transport layer that uses Redis Pub/Sub for multi-tenant deployments and falls back to an in-memory channel broker for standalone deployments. Implement appropriate authentication and authorization for the mesh channels. Include comprehensive tests for message publishing, subscription, and latency guarantees across both transport modes.

## Priority
P0

## Estimated Scope
Medium
