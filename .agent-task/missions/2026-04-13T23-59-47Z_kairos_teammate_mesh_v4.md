<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Phase 2 - Implement Teammate Mesh APIs

## Problem Statement
Agents need to coordinate task state transitions with zero latency using a Pub/Sub mesh.

## Research Report
A low-latency communication layer is required. In Cloud mode, this is Redis Pub/Sub (`rueidis`). In Standalone mode, it's an in-memory Go channel bus.

## Design Doc
Create `MeshTransport` interfaces for `mesh:tasks` and `mesh:coordination` topics. Integrate with the existing `CentrifugeNode` hub.

## Implementation Prompt
Hello Implementer! Implement the Teammate Mesh APIs in `srcs/server/orchestration/mesh/`. Create interfaces for Redis Pub/Sub broadcast and in-memory fallback. Ensure it properly handles events like `TASK_TRANSITION`.

## Priority
P0

## Estimated Scope
Medium

</div>
