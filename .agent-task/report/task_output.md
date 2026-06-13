issue_title: "Implement Built-in Native MCP Tool Integration in Agent Harness"
issue_description: |
  ## Issue brief

  **Problem Statement:** The current built-in Agent Harness lacks a standardized and extensible way to integrate Model Context Protocol (MCP) tools directly. This forces agents to rely on ad-hoc or external integrations, reducing the reliability, maintainability, and scalability of tool usage within the platform. A native MCP integration is required to provide a robust, unified interface for tool discovery, execution, and capability management.

  **Research Report:**
  Analysis of leading agentic platforms (e.g., Claude Code, AutoGPT) highlights the importance of a standardized tool protocol. MCP is emerging as the industry standard for this purpose. Our internal architecture audits (e.g., `[harness]_native_mcp_tool_integration.md`, `[integrations]_hybrid_pubsub_mcp.md`) indicate a significant gap in native MCP support within the `ohc-builtin-agent` harness. A native integration will simplify tool authoring, ensure consistent security boundaries (via SPIFFE/SPIRE), and improve overall agent autonomy.

  **Design Doc:**
  - **Architecture:** The Agent Harness will introduce an `MCPToolManager` component. This manager will act as the central registry and executor for all MCP-compliant tools. It will handle tool discovery, parameter validation, execution routing, and result formatting. The manager will interface with the existing execution sandbox to ensure tools run with appropriate permissions and isolation.
  - **Mobile UX Flow:** N/A (Backend capability). However, any configuration or monitoring UI related to tools must be mobile-responsive (375px baseline) and follow the OHC premium design system.
  - **AI Agent Integration:** Agents will discover available tools via the `MCPToolManager`. The prompt architecture will be updated to instruct agents on how to invoke these tools using the standardized MCP schema. The manager will handle the translation between the agent's intent and the actual tool invocation.
  - **Key Decisions:**
    - Standardize on a strict subset of the MCP specification to simplify initial implementation and ensure compatibility with existing platform constraints.
    - Implement a plugin-based architecture for the `MCPToolManager` to allow easy addition of new tools without modifying the core harness logic.
    - Leverage the existing SPIFFE/SPIRE infrastructure for tool authentication and authorization.

  **Implementation Prompt:**
  1.  **Define MCP Tool Interface:** Create a set of Rust traits or Go interfaces (depending on the target language of the harness) that define the standard MCP tool contract (e.g., `execute`, `get_schema`, `get_name`, `get_description`).
  2.  **Implement `MCPToolManager`:** Build the core manager component that implements a registry for tools. It should support dynamic registration and discovery of tools at runtime.
  3.  **Integrate with Sandbox:** Ensure the `MCPToolManager` can securely execute tools within the existing sandbox environment, passing necessary context and credentials.
  4.  **Update Agent Interface:** Modify the agent execution loop to allow agents to query the `MCPToolManager` for available tools and submit tool execution requests formatted according to the MCP schema.
  5.  **Develop Sample Tools:** Create at least two sample MCP tools (e.g., a simple calculator, a simulated file reader) to validate the integration and serve as examples for future development.
  6.  **Testing:** Write comprehensive unit and integration tests (100% coverage target for new code) to verify tool registration, execution, error handling, and security boundaries.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
