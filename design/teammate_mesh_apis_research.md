Parent: #4909

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [research] Architect Teammate Mesh APIs for KAIROS Orchestrator

## Problem Statement
Agents currently operate in silos without a real-time, bidirectional communication channel. The Teammate Mesh requires an API layer that allows agents to announce status updates, request assistance, and synchronize state across the Hybrid Agentic OS.

## Research Report
- **Protocols**: A realtime communication layer is needed. WebSockets provide the best developer experience for client integration (e.g., UI dashboards), while gRPC/Redis PubSub is optimal for backend inter-agent communication.
- **Authentication**: SPIFFE/SPIRE should be strictly enforced for all internal agent-to-agent communication.
- **Observability**: Every message published to the mesh must emit OpenTelemetry spans for tracing agent conversations.

## Design Doc
1. **Architecture**: Implement `orchmesh.MeshBroker` in `srcs/server/orchestration/mesh/broker.go` using Redis Pub/Sub as the backend data plane.
2. **API Contract**: Expose `PublishEvent(topic, payload)` and `Subscribe(topic)` via an internal gRPC service.
3. **Event Schema**: Define a standard JSON schema for mesh events (`source_agent_id`, `target_agent_id`, `event_type`, `payload`, `timestamp`).

## Implementation Prompt
Hello Implementer!
1. Implement the `orchmesh.MeshBroker` in `srcs/server/orchestration/mesh/broker.go` backed by Redis.
2. Define the Protobuf/gRPC API for `PublishEvent` and `Subscribe`.
3. Modify `srcs/server/dashboard/server.go` (specifically `handleMeshBroadcast`) to integrate with the new `MeshBroker` and broadcast updates.
4. Write comprehensive tests and verify with `bazel test //srcs/server/orchestration/mesh/...`.

## Priority
P0

## Estimated Scope
Medium

</div>
