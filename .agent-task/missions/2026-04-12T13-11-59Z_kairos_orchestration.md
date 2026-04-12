---
status: PENDING
agent: Implementer
priority: P0
estimated_scope: Large
---

# Title: OHC Hybrid OS - Realtime Teammate Mesh APIs (Orchestration)

## Problem Statement
The KAIROS Orchestrator requires a robust background queuing and task delegation framework to manage isolated sub-agents and distribute workloads asynchronously. Currently, the orchestrator lacks a background processing system that can handle distributed tasks (via tools like BullMQ/Celery equivalents in Go) and an integrated Teammate Mesh for Pub/Sub coordination among agents.

## Research Report
- OHC's Autonomous Task Definition specifies the need for scalable background queuing logic to spawn isolated sub-agents.
- OHC's Teammate Mesh Architecture mandates a highly available realtime communication layer (WebSockets, gRPC, Redis Pub/Sub).
- The KAIROS Triad designates the Teammate Mesh as "The Nerves" of the system, utilizing CentrifugeNode and Redis Pub/Sub (`rueidis`) for broadcasting state changes.
- Standalone mode requires graceful degradation to local/in-memory message buses.

## Design Doc
1. **Background Job Queue:**
   - Define a `QueueManager` interface in `srcs/server/orchestration/queue.go` for enqueuing and dequeuing agent tasks.
   - Implement `RedisQueueManager` utilizing `rueidis` for distributed K8s mode.
   - Implement `LocalQueueManager` using Go channels/goroutines for Standalone mode.

2. **Teammate Mesh Pub/Sub:**
   - Define a `MeshCoordinator` interface in `srcs/server/orchestration/mesh.go` supporting `Publish(channel, msg)` and `Subscribe(channel, handler)`.
   - Implement `RedisMeshCoordinator` for cloud-native setups.
   - Implement `LocalMeshCoordinator` using broad-casting Go channels for local setups.
   - Connect the mesh to a mock `CentrifugeHub` struct for WebSocket compatibility (placeholder logic to be refined).

3. **Sub-Agent Orchestration:**
   - Implement a basic `AgentSpawner` that listens to the `QueueManager` and simulates dispatching tasks, broadcasting its status over the `MeshCoordinator`.

## Implementation Prompt
Hello Implementer! To build out Phase 2 (Teammate Mesh) and the Sub-Agent Queue logic of KAIROS, implement the `QueueManager` and `MeshCoordinator` interfaces in `srcs/server/orchestration/`. Ensure that both Redis (`rueidis`) and Local (in-memory) implementations exist. Create a simple `AgentSpawner` that ties the queue to the pub/sub mesh. Ensure proper test coverage (>90%) for the Go packages using Bazel, and mock Redis appropriately in tests. Make sure to define Bazel targets in `srcs/server/orchestration/BUILD.bazel`!

## Priority
P0

## Estimated Scope
Large
