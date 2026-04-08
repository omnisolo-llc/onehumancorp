---
status: "PENDING"
Title: "KAIROS Phase 2: Realtime Teammate Mesh APIs"
Priority: "P0"
Estimated Scope: "Large"
---

# Title: KAIROS Phase 2: Realtime Teammate Mesh APIs

## Problem Statement
The OHC swarm relies on a "Teammate Mesh" for realtime pub/sub task broadcasting and coordination. We need to formalize the highly available Realtime Teammate Mesh APIs so other feature agents can implement them in production, ensuring resilient and low-latency communication across agents in both Cloud-Native and Standalone modes.

## Research Report
- Current communication must utilize `CentrifugeNode` for realtime pub/sub task broadcasting, alongside Redis Pub/Sub (`rueidis`) for horizontal scalability in Cloud-Native mode.
- The Teammate Mesh must handle agent discovery, state synchronization, and realtime coordination.
- **Cloud-Native Mode:** Requires `RedisMeshTransport` mapping to production Redis Pub/Sub channels.
- **Standalone Mode:** Requires `MemoryMeshTransport` for in-memory / SQLite-backed mechanisms.

## Design Doc
**Architecture:**
- **Protocol Definitions:** Update `srcs/proto/hub.proto` with RPCs for Teammate Mesh operations.
- **Service Interfaces:** Expand `HubService` to support realtime events.
- **Transport Layer (`srcs/server/orchestration/hub.go` / `mesh.go`):**
  - Integrate `CentrifugeNode` to manage WebSocket clients and pub/sub routing.
  - Implement `RedisMeshTransport` using `github.com/redis/rueidis` for Redis Pub/Sub.
  - Implement `MemoryMeshTransport` for Standalone mode fallback.

**Data Structures (Proto):**
```protobuf
message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

## Implementation Prompt
Hello Implementer agent! Your task is to build the Realtime Teammate Mesh APIs.
1. Update `srcs/proto/hub.proto` with the new `MeshEvent` message type and necessary streaming RPCs.
2. Ensure you run the `bazelisk` protobuf generation rules to compile the definitions to Go.
3. In `srcs/server/orchestration/`, ensure `CentrifugeNode` is properly initialized with a `MeshTransport`.
4. Implement `RedisMeshTransport` (using `rueidis`) and fix any missing capabilities in `MemoryMeshTransport`.
5. Ensure `TaskManager` uses the new `MeshTransport` for broadcasting events via the `CentrifugeNode` hub.
6. Instrument all API endpoints with OpenTelemetry metrics (`telemetry.Record...`) for mesh latency and message throughput. Note that in Cloud-Native mode, `telemetry.BufferMetricFunc` is nil, so sync endpoints must route directly to OpenTelemetry.
7. Add unit tests for the transport layers and verify functionality by running: `bazelisk test //srcs/server/orchestration/...`
8. Verify that tests testing `CentrifugeNode` configuration pass.

## Visual Excellence Guidelines
Any frontend representation of the Teammate Mesh later created must strictly apply the OHC "Premium Feel":
```css
backdrop-filter: blur(20px) saturate(200%);
background: rgba(255, 255, 255, 0.03);
font-family: 'Outfit', 'Inter', sans-serif;
```

## Priority
P0

## Estimated Scope
Large
