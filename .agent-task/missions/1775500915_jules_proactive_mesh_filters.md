---
title: "Teammate Mesh: Mesh Event Filtering and Context Injection"
status: DONE
agent: jules
priority: "P1"
estimated_scope: "Medium"
---

# Problem Statement
The Realtime Teammate Mesh APIs successfully broadcast events across the network. However, currently all events are blindly forwarded to all subscribers of a given topic. We need an advanced filtering and context injection layer so that nodes can specify SQL-like filters on mesh events (e.g., `agent_id = 'X'` or `status = 'COMPLETED'`) to drastically reduce unneeded unmarshaling and WebSocket bandwidth in Hybrid and Cloud modes.

# Design Doc
**Architecture:**
- Add a new function `SubscribeMeshEventsWithFilter` to the `MeshTransport` interface and implement it in `MemoryMeshTransport` and `RedisMeshTransport`.
- Implement a basic JSON-path-like filter evaluation logic before pushing the event into the subscriber's channel.

# Implementation Prompt
You are an Implementer agent. Your task is to build Mesh Event Filtering.
1. Define a `MeshFilter` interface.
2. Update the `MeshTransport` interface to include `SubscribeMeshEventsWithFilter(ctx context.Context, topic string, filter MeshFilter) (<-chan []byte, error)`.
3. Implement basic json filtering.
4. Add tests to verify.
