---
status: PENDING
priority: P0
scope: Medium
title: "KAIROS: Design Realtime Teammate Mesh APIs"
---

# Title: Design Realtime Teammate Mesh APIs

## Problem Statement
For the OHC Swarm to orchestrate efficiently, agents must communicate in realtime. Phase 2 of the KAIROS Orchestrator playbook requires designing the Realtime Teammate Mesh APIs. These APIs must allow agents to coordinate, share status updates, and negotiate locks. This system must utilize Redis Pub/Sub in Cloud-Native Mode and gracefully degrade to local mechanisms in Standalone Desktop Mode.

## Research Report
- Current communication relies heavily on database state or filesystem polling (`.agent-task`).
- Realtime pub/sub reduces latency and polling overhead.
- Redis is available in Cloud-Native Mode (`provider.IsRedis()` style checks).
- In Standalone Desktop Mode, we lack Redis. We must design an interface that abstracts the transport layer. A local in-memory event bus or simulated pub/sub over SQLite (polling for local mode only) is necessary.

## Design Doc
1.  **Interface Definition:**
    - Create a `TeammateMesh` interface in `srcs/server/orchestration/mesh.go` (or similar).
    - Methods:
      - `Publish(channel string, message []byte) error`
      - `Subscribe(channel string, handler func(message []byte)) error`
      - `RequestLock(resource string) (bool, error)`
      - `ReleaseLock(resource string) error`
2.  **API Endpoints (Thin Client / SubAgent Support):**
    - Define REST/WebSocket endpoints for external agents to connect to the mesh.
    - `POST /api/v1/mesh/broadcast` - Broadcast a message to a channel.
    - `GET /api/v1/mesh/stream` - WebSocket endpoint for realtime events.
3.  **Hybrid Architecture Implementations:**
    - `RedisMesh`: Implements `TeammateMesh` using Redis Pub/Sub and Redis distributed locks.
    - `LocalMesh`: Implements `TeammateMesh` using Go channels (in-memory) and `sync.Mutex` for locks, tailored for SQLite/Standalone environments.

## Implementation Prompt
- Define the `TeammateMesh` interface in a new file `srcs/server/orchestration/mesh/mesh.go`.
- Implement `RedisMesh` in `srcs/server/orchestration/mesh/redis_mesh.go`.
- Implement `LocalMesh` in `srcs/server/orchestration/mesh/local_mesh.go`.
- Define the `Broadcast` REST endpoint handler in `srcs/server/api/mesh_handlers.go`.
- Write tests in `srcs/server/orchestration/mesh/mesh_test.go` to ensure both implementations conform to the interface.
