---
agent: "KAIROS Orchestrator"
status: "PENDING"
Title: "Implement Teammate Mesh API"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
Agents require a highly available, low-latency communication layer (The Nerves) to broadcast state changes, coordinate tasks, and avoid redundant work.

# Research Report
Redis Pub/Sub (`rueidis`) combined with the `CentrifugeNode` structure for websocket delivery is the optimal stack for our Hybrid OS. This allows seamless coordination in the cloud and gracefully degrades to in-memory channels in standalone mode.

# Design Doc
**Architecture:**
- **Hub:** Mesh Hub managing active agent WebSocket/gRPC connections.
- **Pub/Sub Layer:** Redis (Cloud) or Go native channels (Standalone).

**API Contracts:**
1. **BroadcastStateChange**: Agents emit `{ "type": "STATE_CHANGE", "agentId": "...", "state": "...", "timestamp": "..." }`
2. **AdvertiseCapabilities**: Agents broadcast `{ "type": "CAPABILITY", "skills": ["..."] }`

**Sequence:**
1. Agent A connects to Teammate Mesh -> Authenticates via SPIFFE.
2. Agent A claims a task from Shared Task List -> Emits `STATE_CHANGE` -> Mesh broadcasts to Agent B, C.
3. Agent B updates local memory state based on Agent A's broadcast.

# Implementation Prompt
Implement `srcs/server/api/teammate_mesh.go` containing the gRPC/WebSocket handlers for the Mesh API. Integrate `rueidis` for the pub/sub backbone, ensuring a fallback to Go's `sync.Cond` or channels if Redis is unavailable (Standalone mode). Ensure 100% test coverage in `srcs/server/api/teammate_mesh_test.go` and use the `otel.Meter` instance for telemetry tracking of messages sent/received.
