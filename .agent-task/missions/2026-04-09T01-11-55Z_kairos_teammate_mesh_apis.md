---
status: PENDING
---

# Title: KAIROS Orchestrator: Realtime Teammate Mesh APIs
## Problem Statement
The KAIROS Orchestrator requires a Realtime Teammate Mesh API to enable other feature agents to implement real-time communication in production. This mesh will allow agents to broadcast state changes, coordinate tasks, and publish findings across the Swarm.

## Research Report
The Teammate Mesh needs to support Pub/Sub patterns. In Cloud-Native mode, this should be backed by Redis Pub/Sub for high availability and scalability. In Standalone mode, it should degrade gracefully to an in-memory event bus or SQLite-backed polling mechanism if real-time sockets are unavailable, though WebSockets/gRPC are preferred.

## Design Doc
We need to define the API contracts and interfaces for the Teammate Mesh.
- **Interface `TeammateMesh`**:
  - `Publish(ctx context.Context, channel string, message interface{}) error`
  - `Subscribe(ctx context.Context, channel string) (<-chan interface{}, error)`
- **Cloud Implementation (`redisMesh`)**: Implements `TeammateMesh` using a Redis client.
- **Standalone Implementation (`localMesh`)**: Implements `TeammateMesh` using Go channels (in-memory).
- **API Endpoints (for external agents/clients)**:
  - `POST /api/v1/mesh/publish`: Publish a message to a channel.
  - `GET /api/v1/mesh/subscribe`: WebSocket endpoint to subscribe to a channel.

## Implementation Prompt
You are an Implementer agent. Your task is to design and implement the Teammate Mesh APIs for the KAIROS Orchestrator.
1. Create a new package `srcs/server/orchestration/mesh/` to house the Teammate Mesh logic.
2. Define the `TeammateMesh` interface and the standard message payload structure (e.g., `MeshMessage`).
3. Implement the `redisMesh` using a Redis client (ensure you have the necessary dependencies in `go.mod`).
4. Implement the `localMesh` using Go channels for the Standalone fallback.
5. Create HTTP/WebSocket handlers in `srcs/server/api/mesh_handler.go` to expose the publish and subscribe functionality.
6. Write comprehensive unit tests for both implementations, including a mock Redis server or `miniredis` if applicable, and test the HTTP/WebSocket endpoints. Ensure >95% test coverage.

## Priority
P0

## Estimated Scope
Large
