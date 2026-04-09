---
status: PENDING
agent: Miser
---
# Title: 💰 Miser: Proactive Agent Memory Payload Compression

## Problem Statement
The OHC swarm uses a Teammate Mesh to publish events, and some of these events contain `payload` fields which can be quite large if they involve full agent state, large text chunks, or multiple tool definitions. While we have implemented compression for LangGraph Checkpoints and LLM Caching, we haven't implemented payload compression for the Teammate Mesh Events, which consumes memory and Redis bandwidth in Cloud Native deployments.

## Research Report
- The Teammate Mesh passes around `MeshEvent` structs where `Payload` is `[]byte`.
- To reduce Redis Pub/Sub bandwidth and potential Centrifuge memory limits, we can add a simple compression middleware to `RedisMeshTransport` and `MemoryMeshTransport` before broadcasting and after receiving, or handle it closer to the event construction.
- Actually, since I am in a "No pending missions exist" state (for Miser explicitly, as all existing pending ones are for Researcher or are not specific to cost, or wait, I am allowed to pick up a proactive improvement).
- Wait, I should create a new proactive mission for myself!

## Design Doc
1. Define a proactive cost improvement: "Optimize memory footprint of `[]byte` payloads inside `SharedTasks` or similar."
2. Actually, let's look at `srcs/server/orchestration/cached_minimax_client.go` to see if there's more we can compress, or `srcs/server/checkpointer/checkpointer.go`.
