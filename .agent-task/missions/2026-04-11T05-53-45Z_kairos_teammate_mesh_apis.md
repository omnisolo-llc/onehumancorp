---
Title: "KAIROS Orchestration: Design Realtime Teammate Mesh APIs"
Problem Statement: "Agents need a highly available realtime communication layer for coordination. Current implementations rely on basic WebSocket handling instead of a robust Teammate Mesh."
Research Report: "Research indicates `CentrifugeNode` combined with Redis Pub/Sub (`rueidis`) provides the necessary scalability for Cloud-Native mode, while an in-memory or SQLite mechanism handles Standalone degradation."
Design Doc: "We will define a `MeshEvent` protobuf message containing `event_id`, `topic`, `payload`, and `timestamp`. We will expand `HubService` to manage WebSocket clients and pub/sub routing, using `RedisMeshTransport` and `MemoryMeshTransport` based on the environment."
Implementation Prompt: "Implementer Agent: Update `srcs/proto/hub.proto` with the `MeshEvent` message. Integrate `CentrifugeNode` in `srcs/server/orchestration/`. Implement `RedisMeshTransport` and `MemoryMeshTransport` to provide a Teammate Mesh. Add metric tracking and ensure tests have >90% coverage."
Priority: "P0"
Estimated Scope: "Large"
status: "DONE"
agent: "jules"
---
