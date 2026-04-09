---
Title: "Implement KAIROS Teammate Mesh Architecture"
Problem Statement: "Agents need a highly available realtime communication layer (e.g., WebSockets, gRPC, Redis Pub/Sub) for coordination. The current system relies on basic polling or incomplete pub/sub integrations."
Research Report: "The Teammate Mesh (The Nerves) is a highly available, low-latency communication layer. Using CentrifugeNode and Redis Pub/Sub (rueidis), agents broadcast state changes, advertise capabilities, and stream events."
Design Doc: "1. CentrifugeNode Integration: Set up Centrifugo or an embedded Centrifuge Node for realtime messaging. 2. Redis Pub/Sub: Use `rueidis` for scalable pub/sub in Cloud-Native mode. 3. API Contracts: Define gRPC or REST + WebSocket endpoints for agents to subscribe to channels. 4. Graceful Degradation: In Standalone mode, provide a local-only pub/sub mechanism (e.g., in-memory event bus or SQLite-backed queue)."
Implementation Prompt: "1. Implement the Teammate Mesh hub in `srcs/server/orchestration/hub.go`. 2. Integrate Centrifuge for WebSocket communication. 3. Configure Redis pub/sub using `rueidis`. 4. Expose APIs for agent subscription and publishing. 5. Ensure OpenTelemetry metrics for mesh latency and message throughput are recorded. 6. Write tests."
Priority: "P0"
Estimated Scope: "Large"
---
