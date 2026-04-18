# Title: Integration Blueprint: Hybrid PubSub MCP

## Problem Statement
OHC supports both Cloud-native (multi-tenant Postgres/Redis) and Standalone (single-user SQLite) modes. Currently, agents lack a unified, asynchronous messaging pattern for event-driven workflows (e.g., subscribing to system events or publishing broadcast messages to other agents). Cloud deployments rely on message brokers (like Redis Pub/Sub) to ensure cross-pod communication, while Standalone agents need a lightweight, zero-dependency local event bus. A unified Model Context Protocol (MCP) tool is missing to abstract these messaging paradigms securely and dynamically based on the execution environment.

## Research Report
Market analysis shows that most agentic frameworks tightly couple their eventing systems to either purely local in-memory channels (like CrewAI's local signals) or strictly distributed cloud brokers. OHC's Hybrid Architecture demands an MCP Tool that can intelligently route `Publish` and `Subscribe` requests. By introducing a Hybrid PubSub MCP, agents can emit and react to events seamlessly. The driver will dynamically resolve to a Redis-based Pub/Sub mechanism when `OHC_MULTITENANT` is active, and a local in-memory Go channel mechanism when running in standalone mode.

## Design Doc
**Architecture:**
- Add a new package `srcs/server/lib/integrations/hybrid_pubsub/`.
- Introduce a `PubSubManager` that implements the MCP Tool interface.
- Dynamically load the appropriate driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
  - `Standalone`: Local in-memory channel broker.
  - `Cloud`: Redis Pub/Sub driver (via go-redis).

**API Contracts:**
- `Publish(ctx context.Context, topic string, payload []byte) error`
- `Subscribe(ctx context.Context, topic string) (<-chan []byte, error)`

**DB Schema Changes:**
- None required.

**Security:**
- Ensure `organization_id` prefixes are strictly applied to all topic strings in Cloud mode to enforce cross-tenant data isolation.
- Payloads MUST be validated for PII leakage using standard redaction techniques.

## Implementation Prompt
"Implement the Hybrid PubSub MCP tool in `srcs/server/lib/integrations/hybrid_pubsub/`.
1. Create `pubsub.go` defining the `PubSubManager` and its MCP capabilities (`Publish` and `Subscribe`).
2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud mode, implement a Redis-backed Pub/Sub mechanism ensuring `organization_id` is automatically prefixed to the topic to maintain tenant isolation.
4. For Standalone mode, implement a robust in-memory Go channel-based Pub/Sub broker.
5. Create comprehensive unit tests in `pubsub_test.go`, verifying both the in-memory fallback and the Redis implementation (using mocks). Ensure 100% unit test coverage.
6. Create an E2E test starting from UI interaction to verify the publish/subscribe flow functions as intended across agents.
7. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P1

## Estimated Scope
Medium
