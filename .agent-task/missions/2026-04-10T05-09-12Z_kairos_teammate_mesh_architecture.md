---
status: PENDING
Title: "KAIROS Phase 2: Architect Realtime Teammate Mesh APIs"
Priority: P0
Estimated Scope: Large
---

# Problem Statement
For the One Human Corp (OHC) Swarm to function autonomously with true Swarm Intelligence, agents require a highly available, low-latency realtime communication layer called the "Teammate Mesh." Currently, this mesh is conceptually understood but lacks the concrete, implemented APIs necessary for other feature agents to begin integrating it into production. We must move beyond bare WebSockets to a robust, scalable architecture using CentrifugeNode and Redis Pub/Sub (`rueidis`) that works across our Hybrid Architecture.

# Research Report
- Based on the `CLAUDE_OHC.md` and OHC Hybrid Architecture (OHC-HA) requirements, realtime communication needs to support both Cloud-Native Mode (high-concurrency scaling, Redis Pub/Sub) and Standalone Desktop Mode (graceful degradation without heavy dependencies).
- Using bare WebSockets is explicitly discouraged in our memory guidelines. Instead, we must use CentrifugeNode for robust WebSocket management, connection recovery, and scaling.
- Redis Pub/Sub via `rueidis` should be used for cross-node event distribution in cloud deployments.

# Design Doc
**Architecture Components:**
1. **MeshHub (Centrifugo Proxy):** A centralized event dispatcher bridging the backend Go services and the Centrifuge WebSocket layer.
2. **Event Topics:** Standardized channel naming conventions (e.g., `mesh:events:global`, `mesh:events:agent:{id}`) for pub/sub operations.
3. **API Contracts (gRPC/HTTP):**
   - `PublishMeshEvent(ctx, topic, payload)`
   - `SubscribeMeshEvent(ctx, topic) <-chan MeshEvent`
4. **Standalone Degradation:** In SQLite/Standalone mode, the MeshHub should bypass Redis and use an in-memory pub/sub broker to route messages directly between local sub-agents.

**API Interfaces (`srcs/server/orchestration/mesh/mesh.go`):**
```go
package mesh

import (
    "context"
    "encoding/json"
)

type MeshEvent struct {
    Topic   string          `json:"topic"`
    Payload json.RawMessage `json:"payload"`
}

type MeshHub interface {
    Publish(ctx context.Context, topic string, payload []byte) error
    Subscribe(ctx context.Context, topic string) (<-chan MeshEvent, error)
}
```

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the "Teammate Mesh" APIs.
1. Create the `MeshHub` interface and core types in `srcs/server/orchestration/mesh/mesh.go`.
2. Implement a Redis-backed `MeshHub` in `srcs/server/orchestration/mesh/redis_hub.go` using the `rueidis` library. Ensure it integrates with CentrifugeNode logic.
3. Implement an in-memory `MeshHub` for Standalone mode in `srcs/server/orchestration/mesh/memory_hub.go`.
4. Ensure dependency injection based on the `OHC_STANDALONE` environment variable logic.
5. Create comprehensive unit tests for both implementations. Remember to mock Redis or use an embedded test instance.
6. Verify your implementation with `bazelisk test //srcs/server/orchestration/mesh/...`
7. Remember: You are the Lead for your domain. DO NOT ask for approval. Follow all SPIFFE/SPIRE guidelines.

# Visual Excellence Guidelines
Any downstream frontend UI visualizing the Teammate Mesh must apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
