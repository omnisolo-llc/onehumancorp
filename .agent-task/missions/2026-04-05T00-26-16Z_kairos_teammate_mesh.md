---
status: PENDING
agent: Implementer
---

# Title: KAIROS Orchestration: Implement Teammate Mesh APIs

## Problem Statement
Agents require a high-throughput realtime event bus to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously. Without this, agent coordination either deadlocks locally or loses state in the cloud. We need an Agent-to-Agent (A2A) Interaction Protocol for autonomous agents to discover, collaborate, and exchange structured data with cryptographic certainty of identity.

## Research Report
The KAIROS Orchestration layer must integrate a "Teammate Mesh Architecture":
1.  **Cloud Architecture**: Agents publish to production Redis Pub/Sub channels (e.g., `mesh:tasks`, `mesh:coordination`). Centrifuge handles downstream WebSocket propagation to the human CEO dashboard. Redis handles bursts up to 10k messages/sec per Hub instance.
2.  **Standalone Architecture**: Fallbacks to in-memory Go channels natively, ensuring the OS functions offline.
3.  **Zero Secrets**: Relies entirely on SPIFFE/SPIRE Workload APIs to establish mTLS mesh identities. Every message must be signed with a SPIFFE SVID.
4.  **Message Schema**: Use Protobuf-backed schemas (e.g. `srcs/proto/agent.proto`) for inter-agent communication carrying `id`, `sender_id`, `recipient_id`, `type`, `payload`, and `metadata`.

## Design Doc
1. **Teammate Mesh API**: Define Go API handlers for `mesh:tasks` and `mesh:coordination` broadcasts. Endpoints like `/api/mesh/broadcast` must enforce the OHC-SIP JSON structure (`agent_id`, `action`, `status` at the root level).
2. **Redis Integration**: Integrate `go-redis` or `rueidis` pub/sub if Redis is available in the multitenant environment.
3. **In-Memory Fallback**: Use Go channels to propagate messages locally in Standalone Mode.
4. **Centrifuge**: Map mesh channels to Centrifuge WebSocket channels for UI updates.
5. **Aesthetic Core**: Any dashboard UI for the Mesh must enforce: `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`, `backdrop-filter: blur(20px) saturate(200%)`.

## Implementation Prompt
Hello Implementer agent! Please implement the Teammate Mesh APIs:
1. Check `srcs/server/orchestration/mesh.go` and `srcs/server/dashboard/server.go` for Teammate Mesh endpoints.
2. Implement the Teammate Mesh API endpoints. Support broadcasting events with `agent_id`, `action`, and `status` at the root JSON level to Centrifuge channels. Utilize Protobufs as defined in `srcs/proto/agent.proto` where applicable.
3. If Redis is configured, subscribe to channels and broadcast messages across cluster nodes. If not, use an internal Go channel dispatcher.
4. Apply SPIFFE/SPIRE authentication logic via `auth.RequireRole("system", ...)` middleware for internal agent-to-agent communication endpoints. Enforce mTLS checks on incoming A2A calls where relevant.
5. Write rigorous tests (>90% coverage) to verify both Redis and local-fallback behaviors. Use `bazelisk test //srcs/server/...`. Ensure graceful degradation and exponential backoff retry.

## Priority
P0

## Estimated Scope
Medium
