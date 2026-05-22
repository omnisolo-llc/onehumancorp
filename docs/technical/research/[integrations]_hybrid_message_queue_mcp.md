# Title: [integrations] Hybrid Message Queue MCP

## Problem Statement
OHC operates in both Cloud-native (Kubernetes) and Standalone (SQLite) environments. A core requirement for agent orchestration is asynchronous task processing and inter-agent communication. In Cloud mode, this requires a scalable message broker (e.g., RabbitMQ, Kafka, or Redis Pub/Sub). In Standalone mode, we need a lightweight local alternative (e.g., SQLite-backed queues or Go channels) to maintain a zero-dependency local footprint. Agents lack a unified MCP Tool for interacting with message queues seamlessly across these environments.

## Research Report
Current agentic architectures generally hardcode their message brokers. Tools like Celery or standard messaging queues tie the agent framework directly to specific infrastructure. OHC's Hybrid Architecture demands an abstraction layer. A Hybrid Message Queue MCP Tool will allow agents to publish and subscribe to tasks without knowing the underlying implementation. The system will dynamically route messages to a Cloud broker (like Redis or RabbitMQ) for multi-tenant environments, or to a local SQLite/in-memory queue for Standalone execution. This ensures true portability and scalability.

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/message_queue/`.
- Introduce a `MessageQueueManager` implementing the MCP Tool interface.
- Dynamically route based on whether the deployment is Cloud or Standalone.
- **Cloud Mode:** Utilize a distributed message broker (e.g., Redis Pub/Sub or Streams).
- **Standalone Mode:** Implement a SQLite-backed task queue or in-memory channel system for single-node execution.

**API Contracts:**
- `Publish(ctx async context, topic string, payload []byte) error`
- `Subscribe(ctx async context, topic string, handler func(payload []byte) error) error`

**Security:**
- Apply tenant prefixes to topics/channels in Cloud mode to enforce tenant isolation.
- Sanitize and apply PII redaction to message payloads before publishing.

## Implementation Prompt
"Implement the Hybrid Message Queue MCP tool in `src/server/lib/integrations/message_queue/`.
1. Create `message_queue.rs` defining the `MessageQueueManager` and its MCP capabilities (`Publish`, `Subscribe`).
2. Implement dynamic routing to initialize the Cloud driver or the Standalone driver based on the environment configuration.
3. For Cloud mode, implement a distributed message broker driver (e.g., Redis Pub/Sub or Streams). Ensure tenant isolation by prefixing topics.
4. For Standalone mode, implement a local SQLite-backed queue or an in-memory queue.
5. Apply PII redaction to payloads.
6. Create comprehensive tests in `message_queue_test.rs` covering both modes with mocks. Ensure 100% test coverage.
7. Update or create the adjacent `BUILD.bazel` file to include the new files and dependencies in the `srcs` array."

## Priority
P1

## Estimated Scope
Medium
