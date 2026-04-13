---
status: PENDING
priority: P0
scope: Large
title: "KAIROS: Architect Teammate Mesh APIs"
---

# Title: KAIROS: Architect Teammate Mesh APIs

## Problem Statement
Agents in the OHC Swarm need to coordinate efficiently in both Cloud-Native Mode (K8s, Redis) and Standalone Desktop Mode (SQLite). Without a robust Teammate Mesh API layer, sub-agents operate in isolation, lacking the real-time synchronization necessary to handle complex tasks decomposed in the Shared Task List.

## Research Report
- Current implementations show isolated agent workers without a centralized Pub/Sub mechanism across the entire fleet.
- Cloud-Native requires `Redis Pub/Sub` for distributed multi-pod event broadcasting.
- Standalone Mode requires local `sync.Cond` and `channels` to maintain host-machine efficiency while offering the same API contract.

## Design Doc
1. **Teammate Mesh Architecture**:
   - Establish a highly available realtime communication layer.
   - Core API Contracts (Go interface): `Publish(ctx, channel, msg)`, `Subscribe(ctx, channel)`, `Unsubscribe(ctx, channel)`.
   - Redis Implementation: Use `rueidis` to broadcast events to all agents in the namespace.
   - Local Implementation: Use Go channels/sync map to route events internally.
2. **Integration with Orchestrator**:
   - Expose endpoints to emit transition events so the distributed state machine can log changes to `swarm_ultra_plans`.
3. **Graceful Degradation**:
   - The application must auto-detect `Redis` absence or `IsSQLite()` to switch the backend implementation transparently.

## Implementation Prompt
- Implement the `TeammateMesh` interface in `srcs/server/orchestration/teammate_mesh.go`.
- Implement `RedisTeammateMesh` utilizing `rueidis` for cloud-native deployment.
- Implement `LocalTeammateMesh` using standard Go synchronization primitives for standalone mode.
- Write tests ensuring fallback logic is sound and >90% code coverage.
