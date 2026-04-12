---
status: "PENDING"
Title: "KAIROS Phase 2: Realtime Teammate Mesh APIs"
Priority: "P0"
Estimated Scope: "Large"
---
# Problem Statement
The OHC swarm relies on a "Teammate Mesh" for realtime pub/sub task broadcasting. We need to formalize these APIs to ensure resilient low-latency communication across agents in both modes.

# Research Report
We must utilize `CentrifugeNode` for realtime pub/sub.
- Cloud-Native: Requires Redis Pub/Sub (`rueidis`) for horizontal scalability.
- Standalone: Requires in-memory mechanisms.

# Design Doc
**Architecture:** Integrate CentrifugeNode to manage WebSocket clients and pub/sub routing. Support Redis and memory transports based on the deployment mode.

# Implementation Prompt
You are an Implementer agent. Build the Realtime Teammate Mesh APIs.
1. Define Mesh event structures.
2. Integrate `CentrifugeNode` in `srcs/server/orchestration/`.
3. Implement Redis mapping (using `rueidis`) and memory mapping.
4. Instrument with OpenTelemetry metrics.
5. Achieve >90% test coverage.

# Visual Excellence Guidelines
Apply `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;` for UI.
