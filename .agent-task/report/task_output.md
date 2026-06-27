issue_title: "Implement MCP Manager and Semantic Bash Sandbox for KAIROS Hub"
issue_description: |
  **Problem Statement**
  The OneHumanCorp (OHC) platform currently uses custom interfaces for tool integration and basic shell wrappers for Bash execution. As a result, the platform lags behind leading AI Agent frameworks (like Claude Code) which leverage the Model Context Protocol (MCP) for dynamic tool discovery and employ semantic AST parsing for secure, robust Bash execution. This lack of standardization and security limits the true autonomy and scalability of OHC agents for business owners (like Maya, Carlos, Priya) who need a unified, invisible AI automation engine.

  **Research Report**
  A gap analysis of OHC against market leaders (e.g., Claude Code v2.1.88) reveals two critical architectural gaps:
  1.  **Native MCP Tool Integration:** OHC requires a standardized way to integrate tools dynamically. Claude Code uses MCP, allowing agents to load external tools (stdio, HTTP, SSE) seamlessly. Adopting MCP will bridge the gap between static toolsets and dynamic discovery, crucial for the "Hybrid Agentic OS".
  2.  **Semantic Bash Execution Sandboxing:** Current OHC Bash execution relies on basic wrappers, creating a "Valley of Vulnerability" and loss of context. Market standards use Pre-execution AST Analysis to block dangerous patterns (e.g., obfuscated variables, legacy expansions) and semantic understanding to classify intent, isolating filesystem and network access dynamically per command.

  **Design Doc**
  *Architecture Design:*
  -   **MCP Manager (`src/server/harness/mcp/`):** A new service in the orchestration layer that implements the Model Context Protocol. This manager will handle tool registration, capability broadcasting, and standardizing tool invocation from the LLM layer, bridging Cloud and Standalone modes.
  -   **Semantic Bash Sandbox (`src/server/harness/bash_sandbox/`):** A new module to replace basic shell wrappers. It will incorporate an AST parser for bash scripts to pre-analyze and validate commands against strict security rules before execution.
  -   **Multi-tenant Isolation:** Both components must adhere to OHC's zero-trust SPIFFE/SPIRE identity framework. The Bash Sandbox must enforce filesystem and network boundaries per tenant, ensuring operations on behalf of different owners (e.g., Carlos vs. Priya) are strictly isolated.

  *Architecture Diagram:*
  ```mermaid
  graph TD
      A[Agent Feed] --> B[MCP Manager]
      B --> C[Bash Sandbox]
      B --> D[Tool Registration API]
      C --> E[Filesystem Sandbox]
      C --> F[Network Sandbox]
      D --> G[SPIFFE/SPIRE Auth]
  ```

  *Entity-Relationship Diagram:*
  ```mermaid
  erDiagram
      Tenant ||--o{ MCPTool : owns
      MCPTool ||--o{ ExecutionLog : generates
      Tenant ||--o{ SandboxPolicy : configures
      SandboxPolicy ||--o{ FileRule : contains
      SandboxPolicy ||--o{ NetworkRule : contains
  ```

  *Mobile UX Flow (375px Viewport):*
  While these are backend infrastructure changes, the outcome directly affects the Agent Feed on the 375px mobile UI. Owners will experience faster, safer agent operations without seeing error boundaries.
  To support this, a new **"Advanced Settings -> AI Hub"** configuration screen will be added:
  - **Layout:** A clean, translucent UniFi-style card layout.
  - **Cards:** "Connected Tools" (showing dynamically discovered MCP tools) and "Security Rules" (showing active sandboxing limits).
  - **Interactive Buttons:** Each tool card will have a toggle switch to enable/disable specific MCP tools, instantly taking effect via the MCP Manager.

  *AI Agent Integration Points:*
  Agents will no longer use ad-hoc function calls but will query the MCP Manager for available tools dynamically based on context and tenant scope, routing execution securely through the Semantic Bash Sandbox when OS-level tasks are required.

  **Implementation Prompt**
  Implement an MCP Manager in `src/server/harness/mcp/` that standardizes tool registration and discovery for OHC agents. Additionally, implement a Semantic Bash Sandbox in `src/server/harness/bash_sandbox/` that performs AST-based security validation of bash commands before execution. Ensure both components integrate with the existing OHC orchestration layer and adhere to strict multi-tenant isolation rules. Verify the implementation with 100% unit test coverage and E2E tests validating secure tool invocation and blocked malicious bash patterns.

  **Priority:** P0 (critical)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
