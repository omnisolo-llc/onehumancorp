---
status: PENDING
agent:
---

# Title: KAIROS Phase 2: Implement Realtime Teammate Mesh APIs
## Problem Statement
The One Human Corp (OHC) Swarm requires a robust, distributed realtime communication layer (Teammate Mesh) for agent coordination. Agents need to be able to broadcast state changes, share coordination messages, and lock resources to avoid race conditions.

## Research Report
- OHC's Teammate Mesh utilizes `CentrifugeNode` for realtime pub/sub broadcasting (replacing bare WebSockets).
- In Cloud-Native mode, it uses Redis Pub/Sub (`rueidis`) for horizontal scalability and distributed locking (`SET NX EX`).
- In Standalone mode, it degrades to local Go channels and local SQLite/PostgreSQL transactions.
- State machines tracking agent transitions are essential to ensure deterministic behavior (e.g., `PENDING` -> `ASSIGNED` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED`).

## Design Doc
**Architecture:**
- **Hub:** `srcs/server/orchestration/hub.go` containing `CentrifugeNode` integration.
- **State Tracker:** A distributed State Machine backed by Redis locks (Cloud) or local Mutexes (Standalone) prior to state transitions.
- **Event Schema:**
```json
{
  "topic": "task.assigned",
  "payload": {
    "task_id": "123",
    "agent_id": "worker-1",
    "state": "IN_PROGRESS"
  }
}
```

**Visual Excellence Guidelines:**
Any UI developed for this feature must enforce:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the Teammate Mesh APIs for the KAIROS Orchestration layer.
1. Implement the real-time event broadcasting mechanism using `CentrifugeNode` in `srcs/server/orchestration/hub.go`.
2. Implement distributed locking logic in `srcs/server/orchestration/state_machine.go`. Use Redis `rueidis` `SET NX EX` for Cloud mode.
3. Integrate the Teammate Mesh with the newly created `shared_tasks` database layer, so state changes broadcast events over the mesh.
4. Provide unit tests using Go's `slog.NewJSONHandler` for unified logging observability. Use `db.NewTestProvider(t)` for DB context.
5. Verify tests pass via `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Medium
