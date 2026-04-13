---
status: PENDING
agent: Implementer
priority: P0
---

# Title: Phase 2 - Teammate Mesh APIs (Orchestration)

## Problem Statement
For the OHC Swarm to coordinate efficiently without delays, we need a Realtime Teammate Mesh. Agents currently lack a standardized way to broadcast their state and coordinate in a peer-to-peer or pub/sub fashion across distributed nodes.

## Research Report
- Cloud-Native mode requires Redis Pub/Sub driving Centrifuge WebSocket hubs (`mesh:tasks`, `mesh:coordination`).
- Standalone mode requires an in-memory channel broadcast ensuring low-latency IPC without requiring Redis.

## Design Doc
**Architecture:**
- **Centrifuge Hub Integration**: Update `srcs/server/orchestration/centrifuge_hub.go` to support agent coordination channels.
- **Mesh Transport**: Implement `LocalTeammateMesh` in `srcs/server/orchestration/mesh.go` that switches between Redis Pub/Sub (Cloud) and Go Channels (Standalone).

## Implementation Prompt
Implement the Realtime Teammate Mesh APIs. Update `srcs/server/orchestration/centrifuge_hub.go` to handle Centrifuge channels (`mesh:tasks`, etc.). Implement `LocalTeammateMesh` interface in `srcs/server/orchestration/mesh.go` to handle broadcasting and subscribing, degrading to in-memory channels if Redis is unavailable. Ensure full test coverage >90%.

## Priority
P0

## Estimated Scope
Medium
