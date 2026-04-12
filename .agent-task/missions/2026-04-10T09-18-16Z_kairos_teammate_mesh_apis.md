---
Title: "KAIROS Phase 2: Realtime Teammate Mesh API Design"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
For Swarm Intelligence to function effectively, agents must communicate in real-time across the hybrid OS architecture. Without a centralized "Teammate Mesh", agents cannot broadcast task state changes, advertise capabilities, or coordinate execution seamlessly across Cloud-Native and Standalone deployments.

# Research Report
- OHC employs `rueidis` for Redis Pub/Sub in Cloud Mode and degraded local channels for Standalone Mode.
- We need robust channels specifically dedicated to: `mesh:tasks` (task assignments), `mesh:coordination` (capability advertising), and `mesh:events` (general system events).
- Communication payloads should be standardized via Protobuf (`srcs/proto/`) or strongly typed JSON to ensure reliable parsing by agents in different domains.

# Design Doc
**Architecture:**
- Create `srcs/server/orchestration/teammate_mesh_hub.go`.
- Interface `MeshHub` with `Publish(topic string, payload []byte)` and `Subscribe(topic string) <-chan []byte`.
- Use a Redis backend for the `MeshHub` in Cloud-Native mode (`rueidis`).
- Implement a local Go channel-based broker for Standalone Desktop Mode.

# Implementation Prompt
Hello Implementer agent, your mission is to implement the Teammate Mesh APIs.
1. Implement the `MeshHub` interface in `srcs/server/orchestration/mesh.go`.
2. Ensure you branch logic based on `dbProvider.IsSQLite()` or a similar configuration flag to toggle between Redis and local Go channels.
3. Write test files verifying publish/subscribe mechanisms in both modes.
4. Execute `bazelisk test //srcs/server/orchestration/...` to verify your implementation.

# Visual Excellence Guidelines
Any frontend representation of the Teammate Mesh (e.g., debug panels) must adhere to:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`
