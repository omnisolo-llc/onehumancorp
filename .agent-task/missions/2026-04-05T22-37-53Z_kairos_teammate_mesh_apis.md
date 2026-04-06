---
agent: Jules
---
title: "Realtime Teammate Mesh APIs (gRPC/WebSockets/Redis)"
status: DONE
priority: "P0"
estimated_scope: "Large"
---

# Problem Statement
The OHC swarm relies on a "Teammate Mesh" for agent coordination. We need to formalize the Realtime Teammate Mesh APIs so other feature agents can implement them in production, ensuring resilient and low-latency communication across agents, regardless of whether OHC is running in Cloud-Native (Multi-tenant) or Standalone (Local) mode.

# Research Report
- Current communication relies on basic interfaces, but requires strict protocol definitions (gRPC/WebSockets) for real-time mesh functionality.
- The Teammate Mesh must handle agent discovery, capability advertising, task delegation, and state synchronization.
- **Cloud-Native Mode:** Requires Redis Pub/Sub (`rueidis`) for horizontal scalability and broadcasting across pods.
- **Standalone Mode:** Requires in-memory or SQLite-backed mechanisms to simulate Pub/Sub without external dependencies.
- **Realtime Transport:** WebSockets for frontend/client observing, and gRPC for agent-to-agent backend communication.

# Design Doc
**Architecture:**
- **Protocol Definitions:** Update `srcs/proto/hub.proto` with comprehensive RPCs for Teammate Mesh operations.
- **Service Interfaces:** Expand `HubService` to support:
  - `AdvertiseCapabilities(AgentCapabilities)`
  - `DiscoverAgents(Query)`
  - `StreamMeshEvents(EventStreamRequest)`
- **Transport Layer (`srcs/server/orchestration/hub.go`):**
  - Define a generic `MeshTransport` interface.
  - Implement `RedisMeshTransport` (Cloud) mapping to `production Redis Pub/Sub channels`.
  - Implement `MemoryMeshTransport` (Standalone).

**Data Structures (Proto):**
```protobuf
message AgentCapabilities {
  string agent_id = 1;
  repeated string supported_skills = 2;
  int32 max_concurrent_tasks = 3;
}

message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

# Implementation Prompt
You are an Implementer agent. Your task is to build the Realtime Teammate Mesh APIs.
1. Update `srcs/proto/hub.proto` with the new message types (`AgentCapabilities`, `MeshEvent`) and RPCs (`AdvertiseCapabilities`, `DiscoverAgents`, `StreamMeshEvents`).
2. Run `bazelisk test //srcs/proto/...` or equivalent generation scripts to compile the protobuf files to Go.
3. In `srcs/server/orchestration/`, define the `MeshTransport` interface and implement `RedisMeshTransport` (using `github.com/redis/rueidis`) and `MemoryMeshTransport`.
4. Update `CentrifugeNode` and `TaskManager` to utilize the new `MeshTransport` for broadcasting events, ensuring `production Redis Pub/Sub channels` are used for the Teammate Mesh in cloud mode.
5. Add unit tests for the transport layers, verifying that events published are received by subscribers. Ensure >90% coverage.
6. Instrument all API endpoints with OpenTelemetry metrics for mesh latency and message throughput.
7. Verify functionality by writing a test using Bazel: `bazelisk test //srcs/server/orchestration/...`