---
status: "PENDING"
priority: P1
agent: "KAIROS Orchestrator"
Title: "Design Doc: OHC KAIROS Teammate Mesh"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The Teammate Mesh provides sub-millisecond Pub/Sub capabilities to orchestrate agents actively working on the Shared Task List, backed by a robust background queue. Realtime communication between agents is critical for the "Zero Friction" swarm experience.

# Research Report
*   **Realtime Transport (`srcs/server/orchestration/hub.go`)**: Implement generic `MeshTransport` interface with `RedisMeshTransport` (Cloud, mapping to production Redis Pub/Sub channels like `mesh:tasks`, `mesh:coordination`) and `MemoryMeshTransport` (Standalone).
*   **Sub-Agent Queue (`srcs/server/orchestration/queue`)**: A Celery/BullMQ-style background worker system. Enqueues jobs via `RedisTaskQueue` (lists/sorted sets) or `SQLiteTaskQueue` (`sub_agent_jobs` table).
*   **Delivery**: Up to 10k msgs/sec multiplexed down to the CEO dashboard via Centrifuge WebSockets and Agent-to-Agent via gRPC.
*   **Security**: Uses SPIFFE/SPIRE for Agent SVID issuance. All internal mesh API routes explicitly demand mTLS interceptor checks.

# Design Doc
## Realtime API Contracts
- **Transport**: WebSockets / gRPC locally, backed by Redis Pub/Sub for horizontal scaling in Cloud-Native Mode.
- **Event Bus Channels**:
  - `mesh:tasks` - Task transitions (CREATE, CLAIM, COMPLETE)
  - `mesh:presence` - Agent health/heartbeats.
- **Message Format (JSON)**:
  ```json
  {
    "event_type": "TASK_CLAIMED",
    "agent_id": "Implementer-1",
    "payload": {
      "task_id": "123e4567-e89b-12d3-a456-426614174000",
      "timestamp": "2026-04-05T22:45:00Z"
    }
  }
  ```
## API Contracts & Protobufs
Agents interact with the Mesh using standard HTTP POSTs and updated gRPC contracts (`srcs/proto/hub.proto`):
*   `AdvertiseCapabilities(AgentCapabilities)`
*   `DiscoverAgents(Query)`
*   `StreamMeshEvents(EventStreamRequest)`

# Implementation Prompt
Implement the Teammate Mesh Realtime API Contracts in `srcs/server/orchestration/hub.go`, and Sub-Agent Queuing in `srcs/server/orchestration/queue`. Add gRPC methods in `srcs/proto/hub.proto` for Agent interaction (`AdvertiseCapabilities`, `DiscoverAgents`, `StreamMeshEvents`). Ensure SPIFFE/SPIRE for auth. Fallback to `MemoryMeshTransport` and `SQLiteTaskQueue` for standalone deployments.
