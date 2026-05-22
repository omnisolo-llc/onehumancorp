<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Document: Dynamic Tool Registration via MCP

## 1. Executive Summary
**Objective:** Enable the Orchestration Hub to dynamically map OpenAPI schemas into agent tool definitions at runtime, preventing the need for hardcoded schemas and tightly coupled agent-tool implementations.
**Scope:** Implement the `MCPRegistryService` and integrate the `Switchboard` routing layer with the Model Context Protocol.

## 2. Architecture & Components
- **MCP Gateway (Switchboard):** The centralized middleware that manages all registered tool servers (e.g., `github-mcp`, `postgres-mcp`).
- **MCP Registry Service:** Maintains an in-memory or Redis-backed cache of all active tool schemas available to the organization.
- **Dynamic Tool Binder:** A LangGraph node interceptor that injects the JSON Schema of dynamically requested tools directly into the agent's system prompt immediately before execution.
- **SPIFFE/SPIRE Issuance:** Generates short-lived SVIDs for the agent to authenticate with the specific MCP server.

## 3. Data Flow
1. **Querying:** Agent outputs an internal thought: `I need to search the database`. It calls the native `query_registry("database search")` tool.
2. **Schema Synthesis:** The `MCPRegistryService` returns the JSON Schema for `tools.postgres.query()`.
3. **Binding:** The `Dynamic Tool Binder` appends this schema to the agent's context window.
4. **Execution:** The agent calls `tools.postgres.query(sql="SELECT * FROM users")`. The Hub receives this, attaches the agent's SPIFFE ID, and routes it to the `postgres-mcp` pod.
5. **Response:** The `postgres-mcp` server returns the results, which the Hub formats and appends to the agent's LangGraph state.

## 4. API & Data Models
### 4.1 MCP Schema Struct
```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct McpSchema {
    name: String,
    description: String,
    parameters: serde_json::Value,
    auth_role: String,
}
```

## 5. Implementation Details
- **Schema Validation:** Use typed serde deserialization and reject unknown or malformed payloads to prevent schema drift or injection attacks.
- **Zero-Lock Paradigm:** The `MCPRegistryService` must support any standard OpenAPI v3 specification to ensure zero vendor lock-in and allow importing custom tools easily.
- **Performance:** Caching of OpenAPI schemas in the `MCPRegistryService` must be implemented to ensure sub-50ms latency routing for tool discovery requests.

</div>
