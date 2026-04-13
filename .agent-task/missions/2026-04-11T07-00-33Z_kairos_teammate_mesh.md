---
status: BLOCKED
agent: Jules
blockers:
  - The mission requirements demand modifications to files in `srcs/server/orchestration/` (e.g., `hub.go`, `mesh.go`), which is outside my designated domain ownership (`lib/integrations/`, `services/webhooks/`, `api/`). As a Link (L7) agent, I must not modify files in the orchestration package to prevent merge conflicts.
---

# Title: KAIROS Phase 2: Realtime Teammate Mesh APIs & Sub-Agent Queuing

## Problem Statement
To coordinate task execution, agents need a high-throughput realtime event bus to broadcast intent and perform lock arbitration without continuously polling the database. Additionally, we need a scalable background queue to spawn isolated sub-agents.

## Research Report
In Cloud-Native mode, we will utilize `CentrifugeNode` and Redis Pub/Sub (`rueidis`) for the Teammate Mesh. In Standalone mode, it falls back to in-memory Go channels. For Sub-Agent queuing, Celery/BullMQ-style background workers via `RedisTaskQueue` or `SQLiteTaskQueue` (`sub_agent_jobs`) are needed.

## Design Doc
- **Teammate Mesh Transport**: Map generic `MeshTransport` interface to Redis (`mesh:tasks`, `mesh:coordination`) and Memory fallbacks.
- **Centrifuge WebSocket Delivery**: Multiplex events to the human CEO dashboard.
- **Sub-Agent Queuing**: Enqueue jobs into `sub_agent_jobs`.
- **SPIFFE/SPIRE**: Ensure zero-trust authentication on Mesh API routes.

## Implementation Prompt
Hello Implementer agent!
1. Expand `HubService` in `srcs/server/orchestration/hub.go` to support realtime events via `CentrifugeNode` and `RedisMeshTransport` or `MemoryMeshTransport`.
2. Implement sub-agent background queuing logic fetching from `sub_agent_jobs` using Redis or SQLite fallbacks.
3. Secure endpoints with SPIFFE interceptors.
4. Verify functionality using `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Large
