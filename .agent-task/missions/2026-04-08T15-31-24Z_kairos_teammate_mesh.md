---
status: IN_PROGRESS
agent: Jules
Title: "KAIROS Orchestration: Realtime Teammate Mesh APIs"
Priority: P0
Estimated Scope: Large
---

# Title
KAIROS Orchestration: Realtime Teammate Mesh APIs

# Problem Statement
The OHC swarm relies on a "Teammate Mesh" for realtime agent coordination. Currently, there is a lack of formalized, highly available realtime APIs. We need a robust realtime layer to support state synchronization, task broadcasting, and agent discovery across Cloud-Native and Standalone deployments.

# Research Report
* The Teammate Mesh requires resilient realtime capabilities.
* We must migrate away from bare WebSockets to a more robust, scalable pub/sub architecture.
* Memory records indicate that `CentrifugeNode` should be utilized as the central hub for realtime pub/sub task broadcasting.
* In Cloud-Native mode, the system must utilize Redis Pub/Sub (`rueidis`) for horizontal scalability.
* In Standalone mode, the system should gracefully degrade to in-memory or SQLite-backed mechanisms.

# Design Doc
**Architecture Strategy**:
* **Protocol Definitions**: Define structured RPCs for the Teammate Mesh in `srcs/proto/hub.proto`.
* **Transport Layer**: Implement a `MeshTransport` interface in `srcs/server/orchestration/hub.go`.
* Integrate `CentrifugeNode` to manage WebSocket clients and pub/sub routing.
* Implement `RedisMeshTransport` mapping to production Redis Pub/Sub channels (Cloud mode).
* Implement `MemoryMeshTransport` as a fallback mechanism (Standalone mode).

**Data Structures**:
```protobuf
message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

# Implementation Prompt
You are an Implementer agent. Your task is to build the Realtime Teammate Mesh APIs.
1. Update `srcs/proto/hub.proto` with the new `MeshEvent` message type and the necessary streaming RPC endpoints.
2. Run `bazelisk run //:gazelle` or relevant build rules to compile the protobuf definitions to Go.
3. In `srcs/server/orchestration/hub.go`, define the `MeshTransport` interface and integrate `CentrifugeNode`.
4. Implement `RedisMeshTransport` (using `github.com/redis/rueidis` for Redis Pub/Sub).
5. Implement `MemoryMeshTransport` for Standalone mode fallback.
6. Update the `TaskManager` to utilize the new `MeshTransport` for broadcasting coordination events.
7. Add OpenTelemetry metrics for mesh latency and message throughput. Note that in Cloud-Native mode, `telemetry.BufferMetricFunc` is nil, so sync endpoints must route directly to OpenTelemetry.
8. Write comprehensive unit tests for the transport layers and ensure high coverage.
9. Verify your implementation by running `bazelisk test //srcs/server/orchestration/...`.
