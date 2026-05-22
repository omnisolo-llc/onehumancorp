# NATS: Hybrid Event Mesh

## Title
NATS 🚀 (Hybrid Event Mesh Integration)

## Problem Statement
The OHC Hybrid Architecture requires a robust and high-performance eventing system to handle real-time communication between Cloud-Native and Standalone Desktop nodes. Currently, there is a gap in achieving low-latency, scalable, and decentralized event routing that works seamlessly across multi-tenant cloud environments (K8s) and local desktop instances (SQLite-backed). We need an event mesh capable of bridging these distinct environments without heavy dependencies on centralized brokers in offline-first scenarios.

## Research Report
- **Goal**: Integrate NATS (and JetStream) as the primary Hybrid Event Mesh to facilitate real-time messaging, KV storage, and event streaming across the OHC ecosystem.
- **Capabilities**:
  - **Decentralized Pub/Sub**: High-throughput message routing with support for dynamic topologies (leaf nodes for desktop clients).
  - **JetStream Persistence**: Durable message queues for reliable delivery, enabling offline-first operations where events are cached locally and synchronized upon reconnection.
  - **Multi-Tenant Support**: Strong isolation using NATS accounts for Cloud-Native deployments.
  - **Low Footprint**: Extremely lightweight binary, suitable for embedding within the Standalone Desktop Mode.
- **Architecture Validation**:
  - Existing infrastructure uses tools like Redis and PostgreSQL, which are excellent for state but lack the ultra-low latency and dynamic routing of a dedicated event mesh.
  - NATS leaf nodes can run alongside the Standalone SQLite instance, forwarding events to the Cloud cluster transparently when network connectivity is available.

## Design Doc
1. **Architecture Update**: Introduce a `NatsProvider` within the `src/server/integrations/` directory, conforming to the integration blueprints.
2. **Component Integration**:
   - Cloud: NATS cluster with JetStream enabled for global event distribution.
   - Standalone: Embedded NATS server acting as a leaf node to the cloud cluster.
3. **Data Schema (KV/Object Store)**:
   - Define buckets for transient state synchronization and agent presence metrics.
4. **API Contracts**:
   - `Publish(subject string, data []byte)`
   - `Subscribe(subject string, handler func(msg))`
5. **UI Wireframes**: "Event Mesh Status" indicator visualizing active connections and message throughput in the admin dashboard.

## Implementation Prompt
"Implement the NATS Event Mesh module in `src/server/integrations/nats/`. The module must provide a `NatsIntegration` struct conforming to the `Integration` interface in `catalog.rs`. It should support connecting to a remote cluster via credentials and configuring a local embedded instance as a fallback/leaf node. Ensure OpenTelemetry metrics (`ohc.nats.messages_published`, `ohc.nats.messages_received`) are instrumented. Write comprehensive E2E tests validating event propagation between a mock Cloud node and a Standalone instance."

## Priority
P1

## Estimated Scope
Large
