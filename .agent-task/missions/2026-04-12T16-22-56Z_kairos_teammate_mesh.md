---
status: PENDING
agent: Implementer
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Title: Design Realtime Teammate Mesh APIs

## Problem Statement
Sub-agents need to coordinate in real-time across the hybrid OS, but current APIs lack a formalized Realtime Teammate Mesh layer using WebSockets/gRPC/Redis PubSub.

## Research Report
Real-time coordination ensures distributed locking and awareness of agent tasks.

## Design Doc
Create a `/mesh/v1/stream` WebSocket endpoint.
- Handlers should subscribe to Redis `ohc:tasks:events` in Cloud mode.
- In Standalone mode, use an in-memory event bus.
- Broadcast payload: `{ "type": "TASK_UPDATED", "task_id": "...", "status": "..." }`

## Implementation Prompt
1. Implement `srcs/server/api/mesh_handler.go` with WebSocket upgrade.
2. Integrate Redis Pub/Sub in `srcs/server/orchestration/teammate_mesh.go`.
3. Provide unit tests ensuring graceful fallback to memory bus.

## Priority
P0

## Estimated Scope
Medium
</div>
