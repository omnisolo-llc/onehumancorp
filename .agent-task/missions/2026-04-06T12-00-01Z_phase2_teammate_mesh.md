---
status: PENDING
agent: null
Title: "KAIROS Phase 2: Realtime Teammate Mesh APIs"
Priority: P0
Estimated Scope: Medium
---

# Problem Statement
The OHC Swarm requires a robust Realtime Teammate Mesh API to coordinate across disparate pods and local workers. A high-performance pub/sub broker interface is needed to dispatch events (like `task.assigned` or `task.completed`) across the Swarm, ensuring that any state machine transitions in the `shared_tasks` table immediately unblock dependent agents.

# Research Report
- OHC leverages a Teammate Mesh built upon `CentrifugeNode` for client-facing realtime broadcasts (replacing raw WebSockets).
- For Cloud-Native horizontal scaling, Redis Pub/Sub (`rueidis`) is utilized. In standalone mode, an in-memory or SQLite-backed polling mesh acts as a graceful fallback.
- The `state_machine_transitions` must emit events to this Teammate Mesh upon completion of a transaction.

# Design Doc
**Mesh API (`TeammateMesh` Interface):**
```go
package orchestration

import "context"

type MeshEvent struct {
    Topic   string `json:"topic"`
    Payload []byte `json:"payload"`
}

type TeammateMesh interface {
    Publish(ctx context.Context, event MeshEvent) error
    Subscribe(ctx context.Context, topic string, handler func(MeshEvent)) error
}
```

# Implementation Prompt
You are an Implementer agent. Your mission is to establish the core Realtime Teammate Mesh logic:
1. Implement the `TeammateMesh` interface in `srcs/server/orchestration/mesh.go`.
2. Connect `Publish` events to Redis Pub/Sub when running in Cloud Mode (`OHC_MULTITENANT=true` and `dbWrapper.Provider().IsSQLite() == false`).
3. Connect `Publish` events to a local Centrifuge in-memory channel in Standalone Mode.
4. Integrate the `TeammateMesh` into `tasks_db.go` so that a `Publish` call happens immediately after `ClaimTask` successfully commits its database transaction.
5. Create unit tests mocking Redis and verifying the event distribution.

# Priority
P0

# Estimated Scope
Medium
