---
status: DONE
agent: Link
---

# Mission: Realtime Teammate Mesh API Architecture

**Title:** Realtime Teammate Mesh API Architecture
**Problem Statement:** The current Teammate Mesh is split between a legacy WebSocket implementation and a newer Redis/In-memory transport that lacks a unified API gateway and proper OHC-SIP compliance (agent_id, action, status at root).
**Research Report:**
- `srcs/server/orchestration/mesh.go` contains multiple implementations (`RedisMeshTransport`, `MemoryMeshTransport`).
- OHC-SIP compliance is mentioned but not strictly enforced by types.
- Agents need a way to discover other agents' capabilities to delegate tasks effectively.
**Design Doc:**
- **Unified Interface:** Refactor `MeshTransport` to include `AdvertiseCapabilities` and `DiscoverAgents`.
- **API Gateway:** Implement `POST /api/mesh/broadcast` that validates OHC-SIP root fields.
- **Heartbeat:** Implement an automatic heartbeat mechanism in the mesh client that updates the `AgentRegistry`.
- **Hybrid Support:** Ensure `MemoryMeshTransport` uses sharding to match Redis performance characteristics for Standalone mode.
**Implementation Prompt:**
- Refactor `srcs/server/orchestration/mesh.go` to ensure `MeshMessage` and `Task` structs have `agent_id`, `action`, and `status` at the JSON root.
- Implement `DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error)` in `TeammateMesh`.
- Add a REST handler in `api/mesh/mesh_handler.go` for `broadcast` and `capabilities`.
- Write a system test in `srcs/server/orchestration/mesh_system_test.go` verifying a broadcast from a Standalone client reaches a Cloud client (simulated).
**Priority:** P1
**Estimated Scope:** Medium
