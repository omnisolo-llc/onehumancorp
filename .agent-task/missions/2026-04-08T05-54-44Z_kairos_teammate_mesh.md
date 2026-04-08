# Title: Architect the Realtime Teammate Mesh APIs (Phase 2)

## Problem Statement
The KAIROS Orchestrator must support dynamic, real-time coordination between distributed agents. Without a highly available Teammate Mesh, agents cannot be notified of task transitions, state changes, or immediate sub-agent requests, relying entirely on slow database polling.

## Research Report
The current mesh uses basic WebSockets for client-agent communication. However, inter-agent mesh communication requires a robust publish/subscribe system. Industry standards point to Redis Pub/Sub as the backplane for cloud-scale Websockets/gRPC. Standalone mode (local execution) must degrade gracefully to use purely local memory channels. Reference memory guidelines state that "mesh:tasks" should be used for broadcasting.

## Design Doc
**Architecture:**
- **Hub Abstraction:** An orchestration Hub component (`CentrifugeNode` concept) manages connections.
- **Cloud Mode Backplane:** Uses `rueidis` to connect to a Redis instance and subscribe/publish to channels (e.g., `mesh:tasks`, `mesh:presence`).
- **Standalone Mode Backplane:** Replaces Redis with an in-memory Go channel broker.

**Payload Specification:**
Events must follow a structured JSON schema:
```json
{
  "agent_id": "agent-uuid",
  "action": "task_assigned",
  "status": "in_progress",
  "payload": {
    "task_id": "123"
  }
}
```

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the Realtime Teammate Mesh APIs for the KAIROS Orchestrator.
1. Create or update the Hub component in `srcs/server/orchestration/mesh.go`.
2. Implement an interface for the `PubSubBroker` with `Publish(channel string, payload []byte)` and `Subscribe(channel string, handler func([]byte))`.
3. Provide a Redis implementation of `PubSubBroker` using `github.com/redis/rueidis` for Cloud Mode. Connect to the `mesh:tasks` channel.
4. Provide an in-memory implementation of `PubSubBroker` using Go channels for Standalone Mode.
5. Create unit tests mocking the Redis backplane and testing the in-memory broker.
6. Ensure that when a task transitions state, an event is emitted via the Hub to the `mesh:tasks` channel.
7. Use `bazelisk test //srcs/server/orchestration/...` to verify your code.
8. Remember: You are the Lead for your domain. DO NOT ask for approval. Rely entirely on SPIFFE/SPIRE for identity and auth.

## Priority
P0

## Estimated Scope
Medium
