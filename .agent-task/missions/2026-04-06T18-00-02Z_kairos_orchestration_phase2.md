---
Title: "KAIROS Phase 2: Realtime Teammate Mesh APIs"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
To function properly, the swarm requires a low-latency, highly available Teammate Mesh. Agents must be able to broadcast state changes, advertise capabilities, and coordinate work in realtime using WebSockets, CentrifugeNode, and Redis Pub/Sub across both Cloud-Native and Standalone architectures.

# Research Report
- Current implementations require scaling. A robust pub/sub architecture must be built upon `CentrifugeNode`.
- In Cloud-Native mode, the Teammate Mesh must route through Redis Pub/Sub (`rueidis`) using `RedisMeshTransport` for horizontal scale.
- In Standalone Mode, communication relies on a graceful fallback `MemoryMeshTransport`.
- Agents communicate using a standardized `MeshEvent` format to synchronize orchestration updates.

# Design Doc
**Architecture & APIs:**
- **Hub Proto:** Introduce `MeshEvent` into `srcs/proto/hub.proto` to standardize topics and payloads.
- **Transports:** Establish the `MeshTransport` Go interface in `srcs/server/orchestration/mesh.go` with methods for `BroadcastMeshEvent` and `SubscribeMeshEvents`.
- **Observability:** All Pub/Sub operations must emit high-fidelity OpenTelemetry metrics.

**MeshEvent Definition:**
```protobuf
message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

# Implementation Prompt
You are an Implementer agent. Execute the following:
1. Append the `MeshEvent` structure and related streaming RPCs to `srcs/proto/hub.proto`.
2. Expand the `MeshTransport` interface in `srcs/server/orchestration/mesh.go`.
3. Implement `BroadcastMeshEvent` and `SubscribeMeshEvents` across `MemoryMeshTransport` and `RedisMeshTransport`. Ensure channel locking is robust.
4. Ensure OpenTelemetry metrics instrument the payload throughput.
5. Verify execution via `bazelisk test //srcs/server/orchestration/...`

# Visual Excellence Mandate
Any UI displaying Mesh Activity must apply:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
