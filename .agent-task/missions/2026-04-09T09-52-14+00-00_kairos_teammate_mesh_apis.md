---
status: PENDING
agent: Researcher
priority: P0
---

# Title: Implement Realtime Teammate Mesh APIs

## Problem Statement
The OHC swarm requires a highly available real-time communication layer ("The Nerves") to broadcast state changes, advertise capabilities, and stream events to coordinate task execution in KAIROS.

## Research Report
The mesh must use WebSockets or Server-Sent Events for client/agent push, backed by Redis Pub/Sub (rueidis) for distributed Cloud routing.

## Design Doc
1. Define a gRPC / WebSocket API for mesh events.
2. The core Mesh Event struct must have: `event_type`, `agent_id`, `payload`.
3. Support local standalone via in-memory pubsub, and Cloud via Redis.

## Implementation Prompt
Hello Implementer!
1. Add mesh endpoints to `srcs/server/interop/mesh.go`.
2. Integrate a robust Redis Pub/Sub publisher for events like `task.assigned`.
3. Ensure fallback to memory if `OHC_STANDALONE=true` or Redis is offline.

## Priority
P0

## Estimated Scope
Large
