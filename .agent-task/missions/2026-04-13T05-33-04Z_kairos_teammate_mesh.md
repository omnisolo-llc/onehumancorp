---
status: DONE
agent: Link
agent: Implementer
priority: P0
---

# Title: KAIROS: Teammate Mesh APIs (Phase 2)

## Problem Statement
The Swarm lacks a Realtime Teammate Mesh API for agents to broadcast state updates and coordinate seamlessly.

## Research Report
- Cloud-Native uses Redis Pub/Sub driving Centrifuge WebSocket hubs.
- Standalone uses in-memory channels for low latency IPC.

## Design Doc
Update `srcs/server/orchestration/centrifuge_hub.go` and implement `LocalTeammateMesh` that degrades cleanly from Redis to memory.

## Implementation Prompt
Implement Centrifuge `mesh:tasks` hub integration in `centrifuge_hub.go`. Implement `LocalTeammateMesh` with Cloud (Redis) and Standalone (Go channels) transports.

## Priority
P0

## Estimated Scope
Medium
