<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: [integrations] Hybrid WebSockets MCP

## Problem Statement
OHC requires real-time, bi-directional communication between the UI and backend agents. In Cloud-native mode, this is achieved through scalable WebSockets backed by Redis or similar technologies. However, in Standalone mode, running these heavy dependencies is contrary to the single-user, local-execution philosophy. Agents currently lack a unified MCP Tool for real-time WebSocket communication that gracefully adapts to both environments.

## Research Report
Current agentic OS implementations often rely on polling or heavy message brokers for real-time updates. By implementing a Hybrid WebSockets MCP, OHC can provide a seamless real-time experience:
- **Cloud Scale:** Utilizes a distributed backend (e.g., Redis Pub/Sub) to manage WebSocket connections across multiple pods, ensuring high concurrency and multi-tenant isolation.
- **Local Zero-Dependency:** Employs an in-memory WebSocket manager for Standalone mode, eliminating the need for external brokers while maintaining low latency.
- **Dynamic Mode Switching:** The MCP dynamically selects the appropriate backend based on the environment.

## Design Doc
**Architecture:**
- Create a new package `src/server/lib/integrations/websockets/`.
- Introduce a `WebSocketManager` implementing the MCP Tool interface.
- Dynamically select the backend driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Integrate with Redis Pub/Sub to synchronize WebSocket messages across distributed pods.
- **Standalone Mode:** Implement an in-memory WebSocket registry for local, single-node execution.

**API Contracts:**
- `Broadcast(ctx async context, topic string, payload []byte) error`
- `RegisterConnection(ctx async context, conn *websocket.Conn, topic string) error`

**Security:**
- Enforce `organization_id` based topic isolation in Cloud mode to prevent cross-tenant data leakage.

## Implementation Prompt
"Implement the Hybrid WebSockets MCP tool in `src/server/lib/integrations/websockets/`.
1. Create `websockets.rs` defining the `WebSocketManager` and its MCP capabilities (`Broadcast`, `RegisterConnection`).
2. Implement environment-agnostic logic, checking `os.Getenv(\"OHC_MULTITENANT\") == \"true\"` for Cloud mode.
3. For Cloud mode, implement Redis Pub/Sub integration for message distribution, ensuring `organization_id` prefixes on topics.
4. For Standalone mode, implement an in-memory connection registry.
5. Create comprehensive tests in `websockets_test.rs`, verifying both Cloud (mocked Redis) and Standalone behaviors. Ensure 100% test coverage.
6. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P1

## Estimated Scope
Medium

</div>
