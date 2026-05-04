# Research Report: Hybrid PubSub MCP Integrations

## Overview
The Hybrid PubSub MCP provides a standardized interface for pub/sub messaging across different environments (cloud-native and standalone).

## Architecture
- **Cloud-native**: Utilizes Redis Pub/Sub for distributed messaging, providing scalable real-time events across worker nodes. Topics are prefixed with the `tenant_id` to enforce strict cross-tenant data isolation.
- **Standalone**: Utilizes an in-memory pub/sub mechanism (`MemoryTransport`), requiring zero external dependencies and allowing seamless local execution for single-user scenarios.

## Integrations
The system implements the following components:
- **`PubSubManager`**: A tool manager built in Rust inside `src/server/integrations/pubsub/mcp.rs`.
- **Dynamic Routing**: The component reads the `OHC_MULTITENANT` configuration to determine whether it is operating in cloud mode. Based on this, it seamlessly switches between `RedisTransport` and `MemoryTransport` through the `MeshTransport` interface in `src/server/mesh/transport.rs`.
- **Publishers/Subscribers**: Exposes asynchronous `publish(tenant_id, topic, payload)` and `subscribe(tenant_id, topic, handler)` endpoints.
- **Distributed Locking**: Provides `acquire_lock(tenant_id, resource, owner, ttl_seconds)` and `release_lock(tenant_id, resource, owner)` to ensure safe, cross-node mutual exclusion over tenant-scoped resources. Resources are automatically prefixed with `tenant_id` in cloud mode.
- **Health/Presence Monitoring**: Includes `register_presence(tenant_id, agent_id, status, ttl_seconds)` and `get_active_agents(tenant_id)` to track alive agents and microservices inside the tenant's execution scope.

## Implementation Details
The code is thoroughly tested via `#[tokio::test]`, with isolated scenarios mocking both standalone mode and cloud mode to assert proper topic prefixing, satisfying the multi-tenancy requirements for the AI agent orchestration.

*Issue #8507*

## Message Serialization/Deserialization
- To ensure cross-platform compatibility and efficient network transport, messages are expected to be serialized using **Protobuf** prior to invoking the `publish` tool, and deserialized back into typed objects upon receiving them from `subscribe`.
- For specific frontend bridging scenarios or lightweight tasks, standard **JSON** payloads are also natively supported by formatting them into raw byte arrays (`Vec<u8>`).
- The `PubSubManager` interface is deliberately kept agnostic to the payload content, treating all incoming data as `Vec<u8>` to provide maximum flexibility to the calling agents or external services.
